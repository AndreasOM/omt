use std::process;

use clap::{Parser, Subcommand};
use omt::packer::command_packer::CommandPacker;
use omt::packer::command_packer_list::CommandPackerList;
use omt::packer::command_packer_pack::CommandPackerPack;
use omt::packer::command_packer_unpack::CommandPackerUnpack;

#[derive(Debug, Parser)]
#[command(name = "omt-packer")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Packs data into archive, or unpacks data from archive")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Pack {
		#[arg(
			long,
			value_name = "BASEPATH",
			required = true,
			help = "Set the base path (for relative names)"
		)]
		basepath: String,
		#[arg(
			long,
			value_name = "OUTPUT",
			required = true,
			help = "Set the output filename"
		)]
		output:   String,
		#[arg(
			long,
			value_name = "PAKLIST",
			required = true,
			help = "Set the pakelist name"
		)]
		paklist:  String,
		#[arg(
			long,
			value_name = "NAME_MAP",
			help = "Set the (optional) name map file"
		)]
		name_map: Option<String>,
	},
	Unpack {
		#[arg(
			long,
			value_name = "TARGETPATH",
			required = true,
			help = "Set the target path (for relative names)"
		)]
		targetpath: String,
		#[arg(
			long,
			value_name = "INPUT",
			required = true,
			help = "Set the input filename"
		)]
		input:      String,
		#[arg(
			long,
			value_name = "NAME_MAP",
			help = "Set the (optional) name map file"
		)]
		name_map:   Option<String>,
		#[arg(long, help = "Use names for file instead of symlinking")]
		names_only: bool,
	},
	List {
		#[arg(
			long,
			value_name = "INPUT",
			required = true,
			help = "Set the input filename"
		)]
		input:    String,
		#[arg(
			long,
			value_name = "NAME_MAP",
			help = "Set the (optional) name map file"
		)]
		name_map: Option<String>,
	},
}

fn main() {
	let cli = Cli::parse();

	//	println!("{:?}", matches);
	//	println!("{:?}", matches.subcommand());

	let r: Option<Box<dyn CommandPacker>> = match cli.command {
		Some(Commands::Pack {
			basepath,
			output,
			paklist,
			name_map,
		}) => {
			let mut command = Box::new(CommandPackerPack::new()) as Box<dyn CommandPacker>;
			command.set_basepath(&basepath);
			command.set_output(&output);
			command.set_paklist(&paklist);
			if let Some(name_map) = name_map {
				command.set_name_map(&name_map);
			}
			Some(command)
		},
		Some(Commands::Unpack {
			targetpath,
			input,
			name_map,
			names_only,
		}) => {
			let mut command = Box::new(CommandPackerUnpack::new()) as Box<dyn CommandPacker>;
			command.set_targetpath(&targetpath);
			command.set_input(&input);
			if let Some(name_map) = name_map {
				command.set_name_map(&name_map);
			}
			command.set_names_only(names_only);
			Some(command)
		},
		Some(Commands::List { input, name_map }) => {
			let mut command = Box::new(CommandPackerList::new()) as Box<dyn CommandPacker>;
			command.set_input(&input);
			if let Some(name_map) = name_map {
				command.set_name_map(&name_map);
			}
			Some(command)
		},
		None => {
			println!("No SubCommand given. Try help.");
			None
		},
	};

	if let Some(mut command) = r {
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
