use std::process;

use clap::{Parser, Subcommand};
use omt::xcassets::Xcassets;

#[derive(Debug, Parser)]
#[command(name = "omt-xcassets")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Handles .xcassets")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Generate {
		#[arg(long, value_name = "INPUT", help = "Set the input high-resolution image", default_value = "")]
		input: String,
		#[arg(long, value_name = "OUTPUT", help = "Set the output .xcassets", default_value = "")]
		output: String,
		#[arg(long, value_name = "mode", help = "Set the mode: [fill|update|force]", default_value = "fill")]
		mode: String,
	},
}

fn main() {
	let cli = Cli::parse();

	if let Some(Commands::Generate { input, output, mode }) = cli.command {
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
