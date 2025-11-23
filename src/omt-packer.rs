use std::process;

use clap::{Arg, Command};
use omt::packer::command_packer::CommandPacker;
use omt::packer::command_packer_list::CommandPackerList;
use omt::packer::command_packer_pack::CommandPackerPack;
use omt::packer::command_packer_unpack::CommandPackerUnpack;

fn main() {
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = Command::new("omt-packer")
		.version(VERSION)
		.author("Andreas N. <andreas@omni-mad.com>")
		.about("Packs data into archive, or unpacks data from archive")
		.subcommand(
			Command::new("pack")
				.arg(
					Arg::new("basepath")
						.long("basepath")
						.value_name("BASEPATH")
						.required(true)
						.help("Set the base path (for relative names)")
						.num_args(1),
				)
				.arg(
					Arg::new("output")
						.long("output")
						.value_name("OUTPUT")
						.required(true)
						.help("Set the output filename")
						.num_args(1),
				)
				.arg(
					Arg::new("paklist")
						.long("paklist")
						.value_name("PAKLIST")
						.required(true)
						.help("Set the pakelist name")
						.num_args(1),
				)
				.arg(
					Arg::new("name-map")
						.long("name-map")
						.value_name("NAME_MAP")
						.help("Set the (optional) name map file")
						.num_args(1),
				),
		)
		.subcommand(
			Command::new("unpack")
				.arg(
					Arg::new("targetpath")
						.long("targetpath")
						.value_name("TARGETPATH")
						.required(true)
						.help("Set the target path (for relative names)")
						.num_args(1),
				)
				.arg(
					Arg::new("input")
						.long("input")
						.value_name("INPUT")
						.required(true)
						.help("Set the input filename")
						.num_args(1),
				)
				.arg(
					Arg::new("name-map")
						.long("name-map")
						.value_name("NAME_MAP")
						.help("Set the (optional) name map file")
						.num_args(1),
				)
				.arg(
					Arg::new("names-only")
						.long("names-only")
						.help("Use names for file instead of symlinking")
						.action(clap::ArgAction::SetTrue),
				),
		)
		.subcommand(
			Command::new("list")
				.arg(
					Arg::new("input")
						.long("input")
						.value_name("INPUT")
						.required(true)
						.help("Set the input filename")
						.num_args(1),
				)
				.arg(
					Arg::new("name-map")
						.long("name-map")
						.value_name("NAME_MAP")
						.help("Set the (optional) name map file")
						.num_args(1),
				),
		)
		.get_matches();

	//	println!("{:?}", matches);
	//	println!("{:?}", matches.subcommand());

	let r = match matches.subcommand() {
		Some(("pack", sub_matches)) => {
			Some((
				Box::new(CommandPackerPack::new()) as Box<dyn CommandPacker>,
				sub_matches, /*, &|n| {
								 println!("{} files added to archive", n);
							 }*/
			))
		},
		Some(("unpack", sub_matches)) => {
			Some((
				Box::new(CommandPackerUnpack::new()) as Box<dyn CommandPacker>,
				sub_matches, /*, &|n| {
								 println!("{} files extracted from archive", n);
							 }*/
			))
		},
		Some(("list", sub_matches)) => {
			Some((
				Box::new(CommandPackerList::new()) as Box<dyn CommandPacker>,
				sub_matches, /*&|n| {
								 println!("{} files found in archive", n);
							 }*/
			))
		},
		Some((o, _sub_matches)) => {
			println!("SubCommand {} is not supported", o);
			None
		},
		None => {
			println!("No SubCommand given. Try help.");
			None
		},
	};

	if let Some((mut command, sub_matches /*, ok_func*/)) = r {
		if let Some(input) = sub_matches.try_get_one::<String>("input").ok().flatten() {
			command.set_input(&input);
		}
		if let Some(output) = sub_matches.try_get_one::<String>("output").ok().flatten() {
			command.set_output(&output);
		}
		if let Some(name_map) = sub_matches.try_get_one::<String>("name-map").ok().flatten() {
			command.set_name_map(&name_map);
		}
		if let Some(basepath) = sub_matches.try_get_one::<String>("basepath").ok().flatten() {
			command.set_basepath(&basepath);
		}
		if let Some(paklist) = sub_matches.try_get_one::<String>("paklist").ok().flatten() {
			command.set_paklist(&paklist);
		}
		if let Some(targetpath) = sub_matches
			.try_get_one::<String>("targetpath")
			.ok()
			.flatten()
		{
			command.set_targetpath(&targetpath);
		}

		if let Some(names_only) = sub_matches.try_get_one::<bool>("names-only").ok().flatten() {
			command.set_names_only(*names_only);
		}

		match command.run() {
			Ok(_n) => {
				//ok_func(n);
				process::exit(0);
			},
			Err(e) => {
				println!("Error {:?}", e);
				process::exit(-1);
			},
		}
	}
}
