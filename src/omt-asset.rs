use clap::{Arg, App, SubCommand};
use std::process;

use omt::asset::AssetBuilder;

fn main() {
// omt-asset build --content-directory Content --temp-directory Temp --data-directory Data --archive App/data/base.omar --paklist Data/data.paklist

	let matches = App::new("omt-asset")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Handles assets")
					.subcommand(SubCommand::with_name("build")
						.arg(Arg::with_name("content-directory")
							.long("content-directory")
							.value_name("CONTENT-DIRECTORY")
							.help("Set the content directory")
							.takes_value(true)
						)
						.arg(Arg::with_name("data-directory")
							.long("data-directory")
							.value_name("DATA-DIRECTORY")
							.help("Set the data directory")
							.takes_value(true)
						)
						.arg(Arg::with_name("temp-directory")
							.long("temp-directory")
							.value_name("TEMP-DIRECTORY")
							.help("Set the temp directory")
							.takes_value(true)
						)
						.arg(Arg::with_name("archive")
							.long("archive")
							.value_name("archive")
							.help("Set the archive filename")
							.takes_value(true)
						)
						.arg(Arg::with_name("paklist")
							.long("paklist")
							.value_name("PAKLIST")
							.help("Set the pakelist name")
							.takes_value(true)
						)
						.arg(Arg::with_name("dry-run")
							.long("dry-run")
							.value_name("dry-run")
							.help("Enable dry run to show commands without actually running them")
							.takes_value(false)
						)
					)
					.get_matches();

//	println!("{:?}", matches);
//	println!("{:?}", matches.subcommand());

	if let ("build", Some( sub_matches ) ) = matches.subcommand() {
		let content_directory = sub_matches.value_of("content-directory").unwrap_or(".").to_string();
		let data_directory = sub_matches.value_of("data-directory").unwrap_or(".").to_string();
		let temp_directory = sub_matches.value_of("temp-directory").unwrap_or(".").to_string();
		let archive = sub_matches.value_of("archive").unwrap_or("out.omar").to_string();
		let paklist = sub_matches.value_of("paklist").unwrap_or("").to_string();
		let dry_run = sub_matches.occurrences_of("dry-run") > 0;

		println!("content_directory: {:?}", content_directory );
		println!("data_directory   : {:?}", data_directory );
		println!("temp_directory   : {:?}", temp_directory );
		println!("archive          : {:?}", archive );
		println!("paklist          : {:?}", paklist );
		println!("dry_run          : {:?}", dry_run );

		let asset_builder = AssetBuilder::new(
			&content_directory,
			&data_directory,
			&temp_directory,
			&archive,
			&paklist,
			&dry_run,
		);

		match AssetBuilder::build(
			&asset_builder,
		) {
			Ok( number_of_files ) => {
					println!("📁 ✅ ~{:?} assets build", number_of_files );
					process::exit( 0 );
				},
			Err( e ) => {
				println!("📁 ‼️ Error {:?}", e );
				process::exit( -1 );
			},
		}
	}
}
