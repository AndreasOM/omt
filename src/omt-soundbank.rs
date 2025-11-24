use std::process;

use clap::{Parser, Subcommand};
use omt::soundbank::Soundbank;

#[derive(Debug, Parser)]
#[command(name = "omt-soundbank")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Handles soundbank")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Build {
		#[arg(
			long,
			value_name = "OUTPUT",
			help = "Set the output",
			default_value = "test.sbk"
		)]
		output:        String,
		#[arg(
			long,
			value_name = "INPUT",
			help = "Set the input",
			default_value = "test.soundbank"
		)]
		input:         String,
		#[arg(
			long,
			value_name = "OUTPUT-HEADER",
			help = "Set the output header",
			default_value = ""
		)]
		output_header: String,
		#[arg(
			long,
			value_name = "USE-VERSION",
			help = "Set the version to use",
			default_value_t = 2
		)]
		use_version:   u8,
	},
	Info {
		#[arg(long, value_name = "INPUT", help = "Set the input")]
		input: Option<String>,
	},
}

fn main() {
	// omt-soundbank convert --input Data/test.soundbank --output Data/test.sbk --header Data/test_sound.h

	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Build {
			output,
			input,
			output_header,
			use_version,
		}) => {
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
		},
		/*
		Some(Commands::Info { input }) => {
			let input = input.unwrap_or_else(|| "input-atlas-%d".to_string());
			println!("input         : {:?}", input);
			match Atlas::info(&input) {
				Ok(_) => {
					process::exit(0);
				},
				Err(e) => {
					println!("Error getting info from  atlas: {}", &e);
					process::exit(-1);
				}
			}
		},
		*/
		_ => {
			process::exit(-1);
		},
	}
}
