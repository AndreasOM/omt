use crate::util::OmError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
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
		let id_crc = CrcHelper::crc_from_name_upcase_underscore( &id );
		println!("{:?} -> {:?}", id, id_crc );
		Entry {
			id:				id.to_string(),
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
}

impl Soundbank {
	pub fn new() -> Soundbank {
		Soundbank {
			entries: Vec::new(),
		}
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
			0x46, 0x49, 0x53, 0x48, 0x53, 0x42, 0x4e,	// FISHSBN(K)
			compress as u8,								// K or Z
			0x02, 0x00, 0x00, 0x00,

		]).unwrap();
		f.write_u16::<LittleEndian>( self.entries.len() as u16 ).unwrap();
		for e in &self.entries {
// tmp += [ sound[ 1 ], sound[ 2 ], sound[ 3 ], sound[ 4 ], sound[ 5 ] ].pack( 'La32SSL' )
// # uint32, string 32, uint16, uint16, uint32 - native order
			
			f.write_u32::<LittleEndian>( e.id_crc );
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
			f.write_u16::<LittleEndian>( e.max_instances );
			let drop_mode_u16 = match e.drop_mode {
				DropMode::Drop => 1,
				DropMode::Oldest => 0,
			};
			f.write_u16::<LittleEndian>( drop_mode_u16 );

			let mut flags = 0;
			if e.do_loop {
				flags |= 0x01;
			}			
			f.write_u32::<LittleEndian>( flags );
		}
		Ok( 0 )
	}

	pub fn build(
		output: &str,
		input: &str,
		header: &str,
	) -> Result<u32, OmError> {

		let lines = match FileHelper::lines_in_file( input ) {
			Err( e ) => return Err( OmError::Generic(e.to_string()) ),
			Ok( l ) => l,
		};

		let mut soundbank = Soundbank::new();

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

		soundbank.save_sbk( output );

		Ok( 0 )
	}
}