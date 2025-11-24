use std::process;

use clap::{Parser, Subcommand};
use omt::asset::AssetBuilder;

#[derive(Debug, Parser)]
#[command(name = "omt-asset")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Handles assets")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Build {
		#[arg(
			long,
			value_name = "CONTENT-DIRECTORY",
			help = "Set the content directory",
			default_value = "."
		)]
		content_directory: String,
		#[arg(
			long,
			value_name = "DATA-DIRECTORY",
			help = "Set the data directory",
			default_value = "."
		)]
		data_directory:    String,
		#[arg(
			long,
			value_name = "TEMP-DIRECTORY",
			help = "Set the temp directory",
			default_value = "."
		)]
		temp_directory:    String,
		#[arg(
			long,
			value_name = "archive",
			help = "Set the archive filename",
			default_value = "out.omar"
		)]
		archive:           String,
		#[arg(
			long,
			value_name = "PAKLIST",
			help = "Set the pakelist name",
			default_value = ""
		)]
		paklist:           String,
		#[arg(
			long,
			value_name = "dry-run",
			help = "Enable dry run to show commands without actually running them"
		)]
		dry_run:           bool,
	},
}

fn main() {
	// omt-asset build --content-directory Content --temp-directory Temp --data-directory Data --archive App/data/base.omar --paklist Data/data.paklist

	let cli = Cli::parse();

	if let Some(Commands::Build {
		content_directory,
		data_directory,
		temp_directory,
		archive,
		paklist,
		dry_run,
	}) = cli.command
	{
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
