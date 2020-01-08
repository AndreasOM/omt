
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

extern crate clap;
use clap::{Arg, App, SubCommand};

use crc::{crc32, Hasher32};

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::process;
use std::string::String;

#[derive(Debug)]
struct Entry {
	basepath:String,
	filename:String,
	crc:u32,
	size:u32,
	pos:u32,
	data: Vec<u8>,
}

impl Entry {
	fn create(basepath:&String, filename:&String) -> Entry {

		let fullfilename = format!( "{}/{}", basepath, filename );

		 // :TODO: better error handling
		let size = match fs::metadata( fullfilename ) {
			Ok( metadata ) => metadata.len() as u32,
			Err( _ ) => 0,
		};

		// :TODO: calculate actual CRC name
		let downcase_name = filename.to_lowercase();
		// Ruby: .gsub( /\W\./, ' ' ) // should be 'a-zA-Z0-9_', but actual code behaves differently
		let clean_name: String = downcase_name.chars().map(|c| match c {
			'0'..='9' => c,
			'a'..='z' => c,
	//			'A'..='Z' => c,	// already downcase
			'!'..='@' => c,
			'['..='`' => c,
			'{'..='~' => c,
	//		0x7f => c,			// ignore DEL
			_ => ' '
		}).collect();
		let crc = crc32::checksum_ieee(clean_name.as_bytes());
		println!("CRC: {:?} -> {:?} crc: {:?} {:#10X}\n", filename, clean_name, crc, crc );
//	      puts "CRC: " + filename + " -> " + name + " crc: " + @crc.to_s


		Entry {
			basepath: basepath.to_string(),
			filename: filename.to_string(),
			crc: crc,
			size: size,
			pos: 0,
			data: Vec::new(),
		}
	}

	fn create_from_archive(crc: u32, pos: u32, size: u32) -> Entry {
		Entry {
			basepath: String::new(),
			filename: String::new(),
			crc: crc,
			size: size,
			pos: pos,
			data: Vec::new(),
		}		
	}

	fn load_from_archive( &mut self, data: &Vec<u8> ) -> bool {
		// :TODO: find a more idomatic way
		let start = self.pos as usize;
		let end = start + self.size as usize;
		self.data.resize( self.size as usize, 0 );
		self.data.clone_from_slice(&data[start..end]);

		true
	}

	fn display(&self) {
		println!("Displaying Entry for filename {:?}", self.filename );
		print!("{:?}\n", self);
	}
}

#[derive(Debug)]
struct Archive {
	basepath: String,
	entries: Vec<Entry>
}

impl Archive {
	fn create(basepath:&String) -> Archive {
		Archive {
			basepath: basepath.clone(),
			entries: Vec::new(),
		}
	}
	fn add_entry(&mut self, filename:&String) -> bool {
		let entry = Entry::create(
			&self.basepath,
			&filename,
		);

		self.entries.push(entry);
		true
	}

	fn add_entry_from_archive(&mut self, crc: u32, pos: u32, size: u32 ) -> bool {
		let entry = Entry::create_from_archive(
			crc,
			pos,
			size,
		);

		self.entries.push(entry);
		true
	}

	fn save(&self, output: &String) -> Result<u32,&'static str> {
		// write output
		let output_file = File::create(output);
		// :TODO: rethink error handling
		let mut output_file = match output_file {
			Ok( p ) => p,
			Err( _e ) => return Err("Error writing file"),
		};

		// :TODO: add error handling
		let write_names = false;
		let mut flags: u8 = 0;
		if write_names {
			flags |= 1
		}
		let number_of_files: u32 = self.entries.len() as u32;

		output_file.write_all(&[
			0x4f, 0x4d, 0x41, 0x52, 	// magic header
			2,							// version
			flags,						// flags
			0, 0,						// reserved
		]);
		output_file.write_u32::<LittleEndian>( number_of_files ).unwrap();

		// write the directory
		let mut pos = 0;
		for entry in &self.entries {
			// crc, pos, size all as LittleEndian u32
			output_file.write_u32::<LittleEndian>( entry.crc ).unwrap();
			output_file.write_u32::<LittleEndian>( pos ).unwrap();
			output_file.write_u32::<LittleEndian>( entry.size ).unwrap();

			pos += entry.size;
		}

		// write data to output

		for entry in &self.entries {

			let filename = format!( "{}/{}", self.basepath, entry.filename );
	//		println!("{:?}", filename );
			let data_file = File::open(filename);
			// :TODO: rethink error handling
			let mut data_file = match data_file {
				Ok( p ) => p,
				Err( _e ) => return Err("Error reading data file"),
			};
			let mut buffer = Vec::<u8>::new();
			data_file.read_to_end(&mut buffer);
			output_file.write_all( &buffer );
		}

		Ok(number_of_files)
	} 

	fn load(&mut self, filename: &String ) -> Result<u32,&'static str> {
		let file = File::open(filename);

		let file = match file {
			Ok( p ) => p,
			Err( _e ) => return Err("Error reading file"),
		};

		let mut bufreader = BufReader::new(file);

		// read header
		// check magic
		let magic = [ 0x4fu8, 0x4d, 0x41, 0x52 ];
		for m in &magic {
			let b = bufreader.read_u8().unwrap_or( 0 );
			if( b != *m ) {
				return Err( "Broken magic" );
			}
		}

		let v = bufreader.read_u8().unwrap_or( 0 );
		if( v != 2 ) {
			return Err( "Wrong version" );
		}

		let flags = bufreader.read_u8().unwrap_or( 0 );
		if( flags != 0 ) {
			return Err( ":TODO: Flags not implemented" );
		}

		for reserved in 0..2 {
			let r = bufreader.read_u8().unwrap_or( 0 );
			if( r != 0 ) {
				return Err( ":TODO: Reserved field not zero" );
			}
		}

		let number_of_files = bufreader.read_u32::<LittleEndian>().unwrap_or( 0 );
		println!("Reading {:?} files from archive", number_of_files );

		for e in 0..number_of_files {
			let crc = bufreader.read_u32::<LittleEndian>().unwrap_or( 0 );
			let pos = bufreader.read_u32::<LittleEndian>().unwrap_or( 0 );
			let size = bufreader.read_u32::<LittleEndian>().unwrap_or( 0 );
			self.add_entry_from_archive(crc, pos, size);
		}

		let mut data = Vec::new();
		bufreader.read_to_end(&mut data);
		let mut pos = 0;
		for entry in &mut self.entries {
			(*entry).load_from_archive( &data );
		}

		Ok(0)
	}

	fn unpack(&self, targetpath: &String ) -> Result<u32,&'static str> {
		for entry in &self.entries {
			let filename = format!( "{}/{:#10X}", targetpath, entry.crc );
			println!("{:?}", filename );

			let output_file = File::create(filename);
			// :TODO: rethink error handling
			let mut output_file = match output_file {
				Ok( p ) => p,
				Err( _e ) => return Err("Error writing file"),
			};

			output_file.write_all( &entry.data );
		}
		Ok(0)
	}
}

struct Helper {

}

impl Helper {
	fn filenames_in_file(filename: &String) -> Result<Vec<String>, &'static str> {
		let file = File::open(filename);

		let file = match file {
			Ok( p ) => p,
			Err( _e ) => return Err("Error reading file"),
		};

		let bufreader = BufReader::new(file);

		let mut files: Vec<String> = Vec::new();
		for line in bufreader.lines() {
			let filename = line.unwrap();
			files.push( filename );
		}

		Ok(files)
	}
}

fn packer(
		basepath:&String,
		paklist:&String,
		output:&String,
) -> Result<u32,&'static str> {
	let mut archive = Archive::create(basepath);

	for filename in Helper::filenames_in_file(paklist).unwrap_or( Vec::new() ) {	// :TODO: add better error handling
		println!("{:?}", filename );
		archive.add_entry( &filename );		
	}

	archive.save( output )
}

fn unpacker(
		input:&String,
		targetpath:&String,
) -> Result<u32,&'static str> {

	let metadata = match fs::metadata(targetpath) {
		Err( err ) => return Err( "Targetpath not found" ), // :TODO: implement
		Ok( md ) => md,
	};

	if( !metadata.is_dir() ) {
		return Err( "Targetpath is not a directory" );
	}

	let metadata = match fs::metadata(input) {
		Err( err ) => return Err( "Input not found" ),
		Ok( md ) => md,
	};

	if( !metadata.is_file() ) {
		return Err( "Input is not a file" );
	}

	let mut archive = Archive::create( &String::new() );
	match archive.load( input ) {
		Err( e ) => {
			println!("Error in load");
			return Err( e );
		},
		Ok( _ ) => {},
	};
	archive.unpack( targetpath );

	Ok(0)
}

fn main() {
	let matches = App::new("omt-packer")
					.version("0.2")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Packs data into archive, or unpacks data from archive")
					.subcommand(SubCommand::with_name("pack")
						.arg(Arg::with_name("basepath")
							.long("basepath")
							.value_name("BASEPATH")
							.help("Set the base path (for relative names)")
							.takes_value(true)
						)
						.arg(Arg::with_name("output")
							.long("output")
							.value_name("OUTPUT")
							.help("Set the output filename")
							.takes_value(true)
						)
						.arg(Arg::with_name("paklist")
							.long("paklist")
							.value_name("PAKLIST")
							.help("Set the pakelist name")
							.takes_value(true)
						)
					)
					.subcommand(SubCommand::with_name("unpack")
						.arg(Arg::with_name("targetpath")
							.long("targetpath")
							.value_name("TARGETPATH")
							.help("Set the target path (for relative names)")
							.takes_value(true)
						)
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input filename")
							.takes_value(true)
						)
					)
					.get_matches();

//	println!("{:?}", matches);
//	println!("{:?}", matches.subcommand());

	if let ("pack", Some( sub_matches ) ) = matches.subcommand() {
		let basepath = sub_matches.value_of("basepath").unwrap_or(".").to_string();
		let output = sub_matches.value_of("output").unwrap_or("out.omar").to_string();
		let paklist = sub_matches.value_of("paklist").unwrap_or("").to_string();

		println!("basepath: {:?}", basepath );
		println!("output  : {:?}", output );
		println!("paklist : {:?}", paklist );

		match packer( &basepath, &paklist, &output ) {
			Ok( number_of_files ) => {
					println!("{:?} files added to archive", number_of_files );
					process::exit( 0 );
				},
			Err( e ) => {
				println!("Error {:?}", e );
				process::exit( -1 );
			},
		}
	}

	if let ("unpack", Some( sub_matches ) ) = matches.subcommand() {
		let targetpath = sub_matches.value_of("targetpath").unwrap_or(".").to_string();
		let input = sub_matches.value_of("input").unwrap_or("in.omar").to_string();


		println!("targetpath: {:?}", targetpath );
		println!("input  : {:?}", input );
		match unpacker( &input, &targetpath ) {
			Ok( number_of_files ) => {
					println!("{:?} files extracted to archive", number_of_files );
					process::exit( 0 );
				},
			Err( e ) => {
				println!("Error {:?}", e );
				process::exit( -1 );
			},
		}
	}
}
