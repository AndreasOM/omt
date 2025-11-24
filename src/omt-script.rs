use std::process;

use clap::{Parser, Subcommand};
use omt::script::Script;

#[derive(Debug, Parser)]
#[command(name = "omt-script")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Handles scripts")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Build {
		#[arg(
			long,
			value_name = "INPUT",
			help = "Set the input filename",
			default_value = ""
		)]
		input:  String,
		#[arg(
			long,
			value_name = "OUTPUT",
			help = "Set the output filename",
			default_value = ""
		)]
		output: String,
		#[arg(
			long,
			value_name = "mode",
			help = "Set the mode: [copy|crush]",
			default_value = "copy"
		)]
		mode:   String,
	},
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Build {
			input,
			output,
			mode,
		}) => {
			println!("mode    : {:?}", mode);
			println!("output  : {:?}", output);
			println!("input   : {:?}", input);

			match Script::build(&input, &mode, &output) {
				Ok(number_of_files) => {
					println!("{:?} scripts build", number_of_files);
					process::exit(0);
				},
				Err(e) => {
					println!("Error {:?}", e);
					process::exit(-1);
				},
			}
		},
		None => {
			process::exit(-1);
		},
	}
}
