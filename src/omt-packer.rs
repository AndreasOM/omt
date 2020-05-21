use clap::{Arg, App, SubCommand};
use std::process;

use omt::packer::Packer;

fn main() {
	let matches = App::new("omt-packer")
					.version("0.3")
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

		match Packer::pack( &basepath, &paklist, &output ) {
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
		match Packer::unpack( &input, &targetpath ) {
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
