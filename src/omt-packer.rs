use std::process;

use clap::{App, Arg, SubCommand};
use omt::packer::Packer;

fn main() {
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = App::new("omt-packer")
		.version(VERSION)
		.author("Andreas N. <andreas@omni-mad.com>")
		.about("Packs data into archive, or unpacks data from archive")
		.subcommand(
			SubCommand::with_name("pack")
				.arg(
					Arg::with_name("basepath")
						.long("basepath")
						.value_name("BASEPATH")
						.help("Set the base path (for relative names)")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("output")
						.long("output")
						.value_name("OUTPUT")
						.help("Set the output filename")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("paklist")
						.long("paklist")
						.value_name("PAKLIST")
						.help("Set the pakelist name")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("name-map")
						.long("name-map")
						.value_name("NAME_MAP")
						.help("Set the (optional) name map file")
						.takes_value(true),
				),
		)
		.subcommand(
			SubCommand::with_name("unpack")
				.arg(
					Arg::with_name("targetpath")
						.long("targetpath")
						.value_name("TARGETPATH")
						.help("Set the target path (for relative names)")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("input")
						.long("input")
						.value_name("INPUT")
						.help("Set the input filename")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("name-map")
						.long("name-map")
						.value_name("NAME_MAP")
						.help("Set the (optional) name map file")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("names-only")
						.long("names-only")
						.help("Use names for file instead of symlinking")
						.action(clap::ArgAction::SetTrue),
				),
		)
		.subcommand(
			SubCommand::with_name("list")
				.arg(
					Arg::with_name("input")
						.long("input")
						.value_name("INPUT")
						.help("Set the input filename")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("name-map")
						.long("name-map")
						.value_name("NAME_MAP")
						.help("Set the (optional) name map file")
						.takes_value(true),
				),
		)
		.get_matches();

	//	println!("{:?}", matches);
	//	println!("{:?}", matches.subcommand());

	match matches.subcommand() {
		Some(("pack", sub_matches)) => {
			let basepath = sub_matches.value_of("basepath").unwrap_or(".").to_string();
			let output = sub_matches
				.value_of("output")
				.unwrap_or("out.omar")
				.to_string();
			let paklist = sub_matches.value_of("paklist").unwrap_or("").to_string();
			let name_map = sub_matches.value_of("name-map");

			println!("basepath : {:?}", basepath);
			println!("output   : {:?}", output);
			println!("paklist  : {:?}", paklist);
			println!("name-map : {:?}", name_map);

			match Packer::pack(&basepath, &paklist, &output, name_map) {
				Ok(number_of_files) => {
					println!("{:?} files added to archive", number_of_files);
					process::exit(0);
				},
				Err(e) => {
					println!("Error {:?}", e);
					process::exit(-1);
				},
			}
		},
		Some(("unpack", sub_matches)) => {
			let targetpath = sub_matches
				.value_of("targetpath")
				.unwrap_or(".")
				.to_string();
			let input = sub_matches
				.value_of("input")
				.unwrap_or("in.omar")
				.to_string();
			let name_map = sub_matches.value_of("name-map");
			let names_only = sub_matches.get_flag("names-only");

			println!("targetpath: {:?}", targetpath);
			println!("input     : {:?}", input);
			println!("name-map  : {:?}", name_map);
			println!("names-only: {:?}", names_only);

			match Packer::unpack(&input, &targetpath, name_map, names_only) {
				Ok(number_of_files) => {
					println!("{:?} files extracted to archive", number_of_files);
					process::exit(0);
				},
				Err(e) => {
					println!("Error {:?}", e);
					process::exit(-1);
				},
			}
		},
		Some(("list", sub_matches)) => {
			let input = sub_matches
				.value_of("input")
				.unwrap_or("in.omar")
				.to_string();
			let name_map = sub_matches.value_of("name-map");

			println!("input    : {:?}", input);
			println!("name-map : {:?}", name_map);
			match Packer::list(&input, name_map) {
				Ok(number_of_files) => {
					println!("{:?} files found in archive", number_of_files);
					process::exit(0);
				},
				Err(e) => {
					println!("Error {:?}", e);
					process::exit(-1);
				},
			}
		},
		Some((o, _sub_matches)) => {
			println!("SubCommand {} is not supported", o);
		},
		None => todo!(),
	};
}
