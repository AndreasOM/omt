use clap::{Arg, App, SubCommand};
use std::process;

use omt::atlas::Atlas;
use omt::util::OmError;

fn main() {
// omt-atlas combine --output test-atlas-%d --size 2048 --border 0 --input ../Content/test.png
	let matches = App::new("omt-atlas")
					.version("0.2")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Handles atlases")
					.subcommand(SubCommand::with_name("combine")
						.arg(Arg::with_name("output")
							.long("output")
							.value_name("OUTPUT")
							.help("Set the output")
							.takes_value(true)
						)
						.arg(Arg::with_name("size")
							.long("size")
							.value_name("SIZE")
							.help("Set the size")
							.takes_value(true)
						)
						.arg(Arg::with_name("border")
							.long("border")
							.value_name("BORDER")
							.help("Set the border")
							.takes_value(true)
						)
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input")
							.takes_value(true)
							.multiple(true)
						)
					)
					.subcommand(SubCommand::with_name("info")
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input")
							.takes_value(true)
						)
					)
					.get_matches();

	if let ("combine", Some( sub_matches ) ) = matches.subcommand() {
		let output = sub_matches.value_of("output").unwrap_or("output-atlas-%d").to_string();
		let size   = sub_matches.value_of("size").unwrap_or("2048").to_string();
		let border = sub_matches.value_of("border").unwrap_or("0").to_string();
		let input  = sub_matches.values_of("input").unwrap().collect::<Vec<_>>(); 

		let size = match u32::from_str_radix( &size, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing size {:?} >{}<", x, size );
				process::exit( -1 );
			}
		};

		let border = match u32::from_str_radix( &border, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing border {:?} >{}<", x, border );
				process::exit( -1 );
			}
		};

		println!("output         : {:?}", output );
		println!("size           : {:?}", size );
		println!("border         : {:?}", border );
//		println!("input          : {:?}", input );
		println!("input          : [" );
		for i in &input {
			println!("\t{:?}", i );
		}
		println!("]" );

		match Atlas::combine( &output, size, border, &input ) {
			Ok( 1 ) => {
				println!("1 atlas created" );
				process::exit( 0 );
			},
			Ok( n ) => {
				println!("{:?} atlases created", n );
				process::exit( 0 );
			},
			Err( e ) => {
				println!("Error combining atlas >{:?}>", e );
				process::exit( -1 );
			}
		}

	}
	if let ("info", Some( sub_matches ) ) = matches.subcommand() {
		let input = sub_matches.value_of("input").unwrap_or("input-atlas-%d").to_string();
		println!("input         : {:?}", input );
		match Atlas::info( &input ) {
			Ok( _ ) => {
				process::exit( 0 );
			},
			Err( e ) => {
				println!("Error getting info from  atlas." );
				match e {
					OmError::NotImplemented( e ) => println!("NotImplemented: {:?}", e ),
					OmError::Generic( e ) => println!("Generic: {:?}", e ),
				};
				process::exit( -1 );
			}
		}
	}
	process::exit( -1 );
}
