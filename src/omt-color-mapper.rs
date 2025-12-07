use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use omt::color_mapper::ColorMapper;

#[derive(Debug, Parser)]
#[command(name = "omt-color-mapper")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Maps colors from one palette to another")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Map {
		#[arg(
			long,
			value_name = "SOURCE_PAL",
			help = "Set the source palette image (.png)",
			default_value = ""
		)]
		source_pal:      PathBuf,
		#[arg(
			long,
			value_name = "TARGET_PAL",
			help = "Set the target palette image (.png)",
			default_value = ""
		)]
		target_pal:      PathBuf,
		#[arg(
			long,
			value_name = "INPUT",
			help = "Set the input image (.png)",
			default_value = ""
		)]
		input:           PathBuf,
		#[arg(
			long,
			value_name = "OUTPUT",
			help = "Set the output image (.png)",
			default_value = ""
		)]
		output:          PathBuf,
		#[arg(
			long,
			help = "Use Euclidean distance instead of weighted distance",
			default_value_t = false
		)]
		euclidean:       bool,
		#[arg(
			long,
			value_name = "WEIGHT",
			help = "Weight for lightness channel in distance calculation (ignored if --euclidean is set)",
			default_value_t = 2.0
		)]
		lightness_weight: f32,
	},
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Map {
			source_pal,
			target_pal,
			input,
			output,
			euclidean,
			lightness_weight,
		}) => {
			println!("source_pal      : {:?}", source_pal);
			println!("target_pal      : {:?}", target_pal);
			println!("input           : {:?}", input);
			println!("output          : {:?}", output);
			println!("euclidean       : {:?}", euclidean);
			println!("lightness_weight: {:?}", lightness_weight);

			match ColorMapper::process(
				&source_pal,
				&target_pal,
				&input,
				&output,
				euclidean,
				lightness_weight,
			) {
				Ok(_) => {
					println!("Image processed successfully");
					process::exit(0);
				},
				Err(e) => {
					println!("Error: {:?}", e);
					process::exit(-1);
				},
			}
		},
		None => {
			process::exit(-1);
		},
	}
}
