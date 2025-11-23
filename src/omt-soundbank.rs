use std::process;

use clap::{Arg, Command};
use omt::soundbank::Soundbank;

fn main() {
	// omt-soundbank convert --input Data/test.soundbank --output Data/test.sbk --header Data/test_sound.h
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = Command::new("omt-soundbank")
		.version(VERSION)
		.author("Andreas N. <andreas@omni-mad.com>")
		.about("Handles soundbank")
		.subcommand(
			Command::new("build")
				.arg(
					Arg::new("output")
						.long("output")
						.value_name("OUTPUT")
						.help("Set the output")
						.num_args(1),
				)
				.arg(
					Arg::new("input")
						.long("input")
						.value_name("INPUT")
						.help("Set the input")
						.num_args(1),
				)
				.arg(
					Arg::new("output-header")
						.long("output-header")
						.value_name("OUTPUT-HEADER")
						.help("Set the output header")
						.num_args(1),
				)
				.arg(
					Arg::new("use-version")
						.long("use-version")
						.value_name("USE-VERSION")
						.help("Set the version to use")
						.num_args(1),
				),
		)
		.subcommand(
			Command::new("info").arg(
				Arg::new("input")
					.long("input")
					.value_name("INPUT")
					.help("Set the input")
					.num_args(1),
			),
		)
		.get_matches();

	if let Some(("build", sub_matches)) = matches.subcommand() {
		let output = sub_matches
			.get_one::<String>("output")
			.map(String::as_str)
			.unwrap_or("test.sbk")
			.to_string();
		let input = sub_matches
			.get_one::<String>("input")
			.map(String::as_str)
			.unwrap_or("test.soundbank")
			.to_string();
		let output_header = sub_matches
			.get_one::<String>("output-header")
			.map(String::as_str)
			.unwrap_or("")
			.to_string();
		let use_version: u8 = sub_matches
			.get_one::<String>("use-version")
			.map(String::as_str)
			.unwrap_or("2")
			.to_string()
			.parse()
			.unwrap_or(2);

		println!("output         : {:?}", output);
		println!("input          : {:?}", input);
		println!("output-header  : {:?}", output_header);
		println!("use-version    : {:?}", use_version);

		match Soundbank::build(&output, &input, &output_header, use_version) {
			Ok(_) => {
				println!("soundbank created");
				process::exit(0);
			},
			Err(e) => {
				println!("Error combining atlas >{:?}>", e);
				process::exit(-1);
			},
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
					println!("Error getting info from  atlas: {}", &e );
					process::exit( -1 );
				}
			}
		}
	*/
	process::exit(-1);
}
