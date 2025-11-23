use std::process;

use clap::{Arg, Command};
use omt::font::Font;
use omt::font::FontPreviewer;

fn main() {
	//		${madfonter} $TEXSIZE $SIZE "${TTF}" ${TMP_DIR}/${NAME}.tga ${TMP_DIR}/${NAME}.omfont
	// omt-font create --output test-font --texsize 1024 --size 40 --ttf test.ttf

	const VERSION: &str = env!("CARGO_PKG_VERSION");
	let matches = Command::new("omt-font")
		.version(VERSION)
		.author("Andreas N. <andreas@omni-mad.com>")
		.about("Handles fonts")
		.subcommand(
			Command::new("create")
				.arg(
					Arg::new("output")
						.long("output")
						.value_name("OUTPUT")
						.help("Set the output")
						.num_args(1),
				)
				.arg(
					Arg::new("size")
						.long("size")
						.value_name("SIZE")
						.help("Set the font size")
						.num_args(1),
				)
				.arg(
					Arg::new("texsize")
						.long("texsize")
						.value_name("TEXSIZE")
						.help("Set the texture size")
						.num_args(1),
				)
				.arg(
					Arg::new("border")
						.long("border")
						.value_name("BORDER")
						.help("Set the border size")
						.num_args(1),
				)
				.arg(
					Arg::new("distancefield-scale")
						.long("distancefield-scale")
						.value_name("DISTANCEFIELD-SCALE")
						.help("Set/enable the distancefield scale")
						.num_args(1),
				)
				.arg(
					Arg::new("distancefield-max-distance")
						.long("distancefield-max-distance")
						.value_name("DISTANCEFIELD-MAX-DISTANCE")
						.help("Set the distancefield maximum distance")
						.num_args(1),
				)
				.arg(
					Arg::new("input")
						.long("input")
						.value_name("INPUT")
						.help("Set the input font(s) (only .ttf supported)")
						.num_args(1..),
				),
		)
		.subcommand(
			Command::new("preview").arg(
				Arg::new("input")
					.long("input")
					.value_name("INPUT")
					.help("Set the input")
					.num_args(1),
			),
		)
		.get_matches();

	if let Some(("create", sub_matches)) = matches.subcommand() {
		let output = sub_matches
			.get_one::<String>("output")
			.map(String::as_str)
			.unwrap_or("output-font")
			.to_string();
		let texsize = sub_matches
			.get_one::<String>("texsize")
			.map(String::as_str)
			.unwrap_or("1024")
			.to_string();
		let size = sub_matches
			.get_one::<String>("size")
			.map(String::as_str)
			.unwrap_or("16")
			.to_string();
		let border = sub_matches
			.get_one::<String>("border")
			.map(String::as_str)
			.unwrap_or("0")
			.to_string();
		let df_scale = sub_matches
			.get_one::<String>("distancefield-scale")
			.map(String::as_str)
			.unwrap_or("4")
			.to_string();
		let df_max_distance = sub_matches
			.get_one::<String>("distancefield-max-distance")
			.map(String::as_str)
			.unwrap_or("2")
			.to_string();
		let input = sub_matches
			.get_many::<String>("input")
			.unwrap()
			.map(String::as_str)
			.collect::<Vec<_>>();

		let texsize = match u32::from_str_radix(&texsize, 10) {
			Ok(n) => n,
			x => {
				println!("Error parsing texsize {:?} >{}<", x, texsize);
				process::exit(-1);
			},
		};

		let size = match u32::from_str_radix(&size, 10) {
			Ok(n) => n,
			x => {
				println!("Error parsing size {:?} >{}<", x, size);
				process::exit(-1);
			},
		};

		let border = match u32::from_str_radix(&border, 10) {
			Ok(n) => n,
			x => {
				println!("Error parsing border {:?} >{}<", x, border);
				process::exit(-1);
			},
		};

		let df_scale = match u16::from_str_radix(&df_scale, 10) {
			Ok(n) => n,
			x => {
				println!("Error parsing df_scale {:?} >{}<", x, df_scale);
				process::exit(-1);
			},
		};

		let df_max_distance = match u16::from_str_radix(&df_max_distance, 10) {
			Ok(n) => n,
			x => {
				println!(
					"Error parsing df_max_distance {:?} >{}<",
					x, df_max_distance
				);
				process::exit(-1);
			},
		};

		println!("output         : {:?}", output);
		println!("texsize        : {:?}", texsize);
		println!("size           : {:?}", size);
		println!("border         : {:?}", border);
		println!("df_scale  	 : {:?}", df_scale);
		println!("df_max_distance: {:?}", df_max_distance);
		//		println!("input          : {:?}", input );
		println!("input          : [");
		for i in &input {
			println!("\t{:?}", i);
		}
		println!("]");

		match Font::create(
			&output,
			texsize,
			size,
			border,
			df_scale,
			df_max_distance,
			&input,
		) {
			Ok(1) => {
				println!("1 font created");
				process::exit(0);
			},
			Ok(n) => {
				println!("{:?} fonts created", n);
				process::exit(0);
			},
			Err(e) => {
				println!("Error creating font >{:?}<", e);
				process::exit(-1);
			},
		}
	}
	if let Some(("preview", sub_matches)) = matches.subcommand() {
		let input = sub_matches
			.get_one::<String>("input")
			.map(String::as_str)
			.unwrap_or("output-font")
			.to_string();
		println!("input         : {:?}", input);
		match FontPreviewer::preview(&input) {
			Ok(_) => {
				process::exit(0);
			},
			Err(e) => {
				println!("Error getting info from font. {}", &e);
				process::exit(-1);
			},
		}
	}
	process::exit(-1);
}
