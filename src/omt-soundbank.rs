use clap::{Arg, App, SubCommand};
use std::process;

use omt::soundbank::Soundbank;

fn main() {
// omt-soundbank convert --input Data/test.soundbank --output Data/test.sbk --header Data/test_sound.h
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = App::new("omt-soundbank")
					.version(VERSION)
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Handles soundbank")
					.subcommand(SubCommand::with_name("build")
						.arg(Arg::with_name("output")
							.long("output")
							.value_name("OUTPUT")
							.help("Set the output")
							.takes_value(true)
						)
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input")
							.takes_value(true)
						)
						.arg(Arg::with_name("output-header")
							.long("output-header")
							.value_name("OUTPUT-HEADER")
							.help("Set the output header")
							.takes_value(true)
						)
						.arg(Arg::with_name("use-version")
							.long("use-version")
							.value_name("USE-VERSION")
							.help("Set the version to use")
							.takes_value(true)
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

	if let Some( ("build", sub_matches ) ) = matches.subcommand() {
		let output = sub_matches.value_of("output").unwrap_or("test.sbk").to_string();
		let input = sub_matches.value_of("input").unwrap_or("test.soundbank").to_string();
		let output_header = sub_matches.value_of("output-header").unwrap_or("").to_string();
		let use_version: u8 = sub_matches.value_of("use-version").unwrap_or("2").to_string().parse().unwrap_or(2);

		println!("output         : {:?}", output );
		println!("input          : {:?}", input );
		println!("output-header  : {:?}", output_header );
		println!("use-version    : {:?}", use_version );

		match Soundbank::build( &output, &input, &output_header, use_version ) {
			Ok( _ ) => {
				println!("soundbank created" );
				process::exit( 0 );
			},
			Err( e ) => {
				println!("Error combining atlas >{:?}>", e );
				process::exit( -1 );
			}
		}

	}
/*	
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
*/	
	process::exit( -1 );
}
