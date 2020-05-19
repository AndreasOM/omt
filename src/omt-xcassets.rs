use clap::{Arg, App, SubCommand};
use std::process;

use omt::xcassets::Xcassets;
use omt::util::OmError;

fn main() {
	let matches = App::new("omt-xcassets")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Handles .xcassets")
					.subcommand(SubCommand::with_name("generate")
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input high-resolution image")
							.takes_value(true)
						)
						.arg(Arg::with_name("output")
							.long("output")
							.value_name("OUTPUT")
							.help("Set the output .xcassets")
							.takes_value(true)
						)
						.arg(Arg::with_name("mode")
							.long("mode")
							.value_name("mode")
							.help("Set the mode: [fill|update|force]")
							.takes_value(true)
						)
					)
					.get_matches();

//	println!("{:?}", matches);
//	println!("{:?}", matches.subcommand());

	if let ("generate", Some( sub_matches ) ) = matches.subcommand() {
		let mode		= sub_matches.value_of("mode").unwrap_or("fill").to_string();
		let output		= sub_matches.value_of("output").unwrap_or("").to_string();
		let input		= sub_matches.value_of("input").unwrap_or("").to_string();

		println!("mode    : {:?}", mode );
		println!("output  : {:?}", output );
		println!("input   : {:?}", input );

		match Xcassets::generate( &input, &mode, &output ) {
			Ok( number_of_files ) => {
					println!("{:?} sub-assets generated", number_of_files );
					process::exit( 0 );
				},
			Err( e ) => {
				println!("Error {:?}", e );
				process::exit( -1 );
			},
		}
	}

}
