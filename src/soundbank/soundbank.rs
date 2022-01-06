use crate::util::OmError;
use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use crate::util::{CrcHelper,FileHelper};

#[derive(Debug,Clone)]
pub enum DropMode {
	Drop,
	Oldest,
}

#[derive(Debug,Clone)]
pub struct Entry {
	id: String,
	id_crc: u32,
	filename: String,
	max_instances: u16,
	drop_mode: DropMode,
	do_loop: bool,
}

impl Entry {
	pub fn new(
		id: &str,
		filename: &str,
		max_instances: u16,
		drop_mode: DropMode,
		do_loop: bool,
	) -> Entry {
		let clean_name = CrcHelper::clean_name_from_name_upcase_underscore( &id );
		let id_crc = CrcHelper::crc_from_name_upcase_underscore( &id );

		println!("{:?} -> {:?}", id, id_crc );
		Entry {
			id:				clean_name.to_string(),
			id_crc:			id_crc,
			filename:		filename.to_string(),
			max_instances:	max_instances,
			drop_mode:		drop_mode,
			do_loop:		do_loop,
		}
	}
}
#[derive(Debug)]
pub struct Soundbank {

	entries: Vec<Entry>,
	version: u8,
}

impl Soundbank {
	pub fn new() -> Soundbank {
		Soundbank {
			entries: Vec::new(),
			version: 2,
		}
	}

	pub fn set_version( &mut self, version: u8 ) {
		self.version = version;
	}

	pub fn add_entry( &mut self, entry: &Entry ) {
		self.entries.push( entry.clone() );
	}

	fn save_sbk( &self, filename: &str ) -> Result< u32, OmError > {
		let mut f = match File::create(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};
		f.write_u16::<LittleEndian>( 0x4f53 ).unwrap();
		f.write_u16::<LittleEndian>( 0x0001 ).unwrap();
		let compress = 'K';
		f.write_all(&[
			0x4f, 0x4d, 0x53, 0x4e, 0x44, 0x42, 0x4e,	// OMSNDBN(K)
			compress as u8,								// K or Z
			self.version, 0x00, 0x00, 0x00,

		]).unwrap();
		f.write_u16::<LittleEndian>( self.entries.len() as u16 ).unwrap();
		for e in &self.entries {
// tmp += [ sound[ 1 ], sound[ 2 ], sound[ 3 ], sound[ 4 ], sound[ 5 ] ].pack( 'La32SSL' )
// # uint32, string 32, uint16, uint16, uint32 - native order
			
			f.write_u32::<LittleEndian>( e.id_crc ).unwrap();
			let n = &e.filename;
			let mut c = 0;
			for nn in n.as_bytes() {
				f.write_u8( *nn ).unwrap();
				c += 1;
			}
			while c < 32 {
				f.write_u8( 0 ).unwrap();
				c += 1;
			}

			if self.version >= 3 {
	//			dbg!(&e.id);

				let n = &e.id;
				let mut c = 0;
				for nn in n.as_bytes() {
					f.write_u8( *nn ).unwrap();
					c += 1;
				}
				while c < 32 {
					f.write_u8( 0 ).unwrap();
					c += 1;
				}
			}

			f.write_u16::<LittleEndian>( e.max_instances ).unwrap();
			let drop_mode_u16 = match e.drop_mode {
				DropMode::Drop => 1,
				DropMode::Oldest => 0,
			};
			f.write_u16::<LittleEndian>( drop_mode_u16 ).unwrap();

			let mut flags = 0;
			if e.do_loop {
				flags |= 0x01;
			}			
			f.write_u32::<LittleEndian>( flags ).unwrap();
		}
		Ok( 0 )
	}

	fn save_header( &self, filename: &str ) -> Result< u32, OmError > {
		let mut f = match File::create(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};
		write!( &mut f, "#pragma once\n" ).unwrap();
		write!( &mut f, "namespace om\n{{\n" ).unwrap();

		write!( &mut f, "\t#if !defined( SOUNDBANK_DEFINES )\n" ).unwrap();
		write!( &mut f, "\t\t#define SOUNDBANK_DEFINES\n" ).unwrap();
		write!( &mut f, "\t\tenum SoundBankLoopMode\n" ).unwrap();
		write!( &mut f, "\t\t{{\n" ).unwrap();
		write!( &mut f, "\t\t\tOldest = 0,\n" ).unwrap();
		write!( &mut f, "\t\t\tDrop   = 1,\n" ).unwrap();
		write!( &mut f, "\t\t}};\n" ).unwrap();
		write!( &mut f, "\t\t#define SOUNDBANK_ENTRY_LOOP_BIT 1\n" ).unwrap();
		write!( &mut f, "\t#endif\n" ).unwrap();

		for e in &self.entries {
			write!( &mut f, "\t#define CRC_{:} 0x{:x}\n", e.id, e.id_crc ).unwrap();
		}
		write!( &mut f, "}} // namespace om\n" ).unwrap();
		Ok( 0 )
	}

	pub fn build(
		output: &str,
		input: &str,
		header: &str,
		use_version: u8,
	) -> Result<u32, OmError> {

		let lines = match FileHelper::lines_in_file( input ) {
			Err( e ) => return Err( OmError::Generic(e.to_string()) ),
			Ok( l ) => l,
		};

		let mut soundbank = Soundbank::new();

		soundbank.set_version( use_version );
		for line in lines {
			let line = line.trim();
			if line.len() == 0 {

			} else if line.starts_with( "#" ) {

			} else {
//				println!("{:?}", line );
				let parts: Vec<&str> = line.split(";").collect();
//				println!("parts.size() = {:?}", parts.len());
				if parts.len() == 6 {
					println!("{:?}", parts );
// # id;file;max instances;drop mode;flags (loop);
					let id = parts[ 0 ];
					let filename = parts[ 1 ];
					let max_instances = u16::from_str(parts[ 2 ]).unwrap_or( 0 );
					let drop_mode = match parts[ 3 ] {
						"OLDEST" => DropMode::Oldest,
						_ => DropMode::Drop,
					};
					let do_loop = match parts[ 4 ] {
						"LOOP" => true,
						_ => false,
					};

					let entry = Entry::new( id, filename, max_instances, drop_mode, do_loop );

//					println!("Entry {:#?}", entry);

					soundbank.add_entry( &entry );
				}
			}
		}

		println!("{:#?}", soundbank );

		match soundbank.save_sbk( output ) {
			Ok( _ ) => {},
			Err( e ) => return Err( e ),
		};
		
		if header.len() > 0 {
			match soundbank.save_header( header ) {
				Err( e ) => return Err( e ),
				Ok( n ) => return Ok( n ),
			}
		}

		Ok( 0 )
	}
}