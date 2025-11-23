use std::process;

use clap::{Arg, Command};
use omt::asset::AssetBuilder;

fn main() {
	// omt-asset build --content-directory Content --temp-directory Temp --data-directory Data --archive App/data/base.omar --paklist Data/data.paklist

	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = Command::new("omt-asset")
		.version(VERSION)
		.author("Andreas N. <andreas@omni-mad.com>")
		.about("Handles assets")
		.subcommand(
			Command::new("build")
				.arg(
					Arg::new("content-directory")
						.long("content-directory")
						.value_name("CONTENT-DIRECTORY")
						.help("Set the content directory")
						.num_args(1),
				)
				.arg(
					Arg::new("data-directory")
						.long("data-directory")
						.value_name("DATA-DIRECTORY")
						.help("Set the data directory")
						.num_args(1),
				)
				.arg(
					Arg::new("temp-directory")
						.long("temp-directory")
						.value_name("TEMP-DIRECTORY")
						.help("Set the temp directory")
						.num_args(1),
				)
				.arg(
					Arg::new("archive")
						.long("archive")
						.value_name("archive")
						.help("Set the archive filename")
						.num_args(1),
				)
				.arg(
					Arg::new("paklist")
						.long("paklist")
						.value_name("PAKLIST")
						.help("Set the pakelist name")
						.num_args(1),
				)
				.arg(
					Arg::new("dry-run")
						.long("dry-run")
						.value_name("dry-run")
						.help("Enable dry run to show commands without actually running them")
						.num_args(0)
						.action(clap::ArgAction::SetTrue),
				),
		)
		.get_matches();

	//	println!("{:?}", matches);
	//	println!("{:?}", matches.subcommand());

	if let Some(("build", sub_matches)) = matches.subcommand() {
		let content_directory = sub_matches
			.get_one::<String>("content-directory")
			.map(String::as_str)
			.unwrap_or(".")
			.to_string();
		let data_directory = sub_matches
			.get_one::<String>("data-directory")
			.map(String::as_str)
			.unwrap_or(".")
			.to_string();
		let temp_directory = sub_matches
			.get_one::<String>("temp-directory")
			.map(String::as_str)
			.unwrap_or(".")
			.to_string();
		let archive = sub_matches
			.get_one::<String>("archive")
			.map(String::as_str)
			.unwrap_or("out.omar")
			.to_string();
		let paklist = sub_matches
			.get_one::<String>("paklist")
			.map(String::as_str)
			.unwrap_or("")
			.to_string();
		let dry_run = sub_matches.get_flag("dry-run");

		println!("content_directory: {:?}", content_directory);
		println!("data_directory   : {:?}", data_directory);
		println!("temp_directory   : {:?}", temp_directory);
		println!("archive          : {:?}", archive);
		println!("paklist          : {:?}", paklist);
		println!("dry_run          : {:?}", dry_run);

		let asset_builder = AssetBuilder::new(
			&content_directory,
			&data_directory,
			&temp_directory,
			&archive,
			&paklist,
			&dry_run,
		);

		match AssetBuilder::build(&asset_builder) {
			Ok(number_of_files) => {
				println!("📁 ✅ ~{:?} assets build", number_of_files);
				process::exit(0);
			},
			Err(e) => {
				println!("📁 ‼️ Error {:?}", e);
				process::exit(-1);
			},
		}
	}
}
