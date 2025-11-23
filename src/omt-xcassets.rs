use std::process;

use clap::{Arg, Command};
use omt::xcassets::Xcassets;

fn main() {
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = Command::new("omt-xcassets")
		.version(VERSION)
		.author("Andreas N. <andreas@omni-mad.com>")
		.about("Handles .xcassets")
		.subcommand(
			Command::new("generate")
				.arg(
					Arg::new("input")
						.long("input")
						.value_name("INPUT")
						.help("Set the input high-resolution image")
						.num_args(1),
				)
				.arg(
					Arg::new("output")
						.long("output")
						.value_name("OUTPUT")
						.help("Set the output .xcassets")
						.num_args(1),
				)
				.arg(
					Arg::new("mode")
						.long("mode")
						.value_name("mode")
						.help("Set the mode: [fill|update|force]")
						.num_args(1),
				),
		)
		.get_matches();

	//	println!("{:?}", matches);
	//	println!("{:?}", matches.subcommand());

	if let Some(("generate", sub_matches)) = matches.subcommand() {
		let mode = sub_matches
			.get_one::<String>("mode")
			.map(String::as_str)
			.unwrap_or("fill")
			.to_string();
		let output = sub_matches
			.get_one::<String>("output")
			.map(String::as_str)
			.unwrap_or("")
			.to_string();
		let input = sub_matches
			.get_one::<String>("input")
			.map(String::as_str)
			.unwrap_or("")
			.to_string();

		println!("mode    : {:?}", mode);
		println!("output  : {:?}", output);
		println!("input   : {:?}", input);

		match Xcassets::generate(&input, &mode, &output) {
			Ok(number_of_files) => {
				println!("{:?} sub-assets generated", number_of_files);
				process::exit(0);
			},
			Err(e) => {
				println!("Error {:?}", e);
				process::exit(-1);
			},
		}
	}
}
