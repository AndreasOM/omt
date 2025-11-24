use std::process;

use clap::{Parser, Subcommand};
use omt::font::Font;
use omt::font::FontPreviewer;

#[derive(Debug, Parser)]
#[command(name = "omt-font")]
#[command(version)]
#[command(author = "Andreas N. <andreas@omni-mad.com>")]
#[command(about = "Handles fonts")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Create {
		#[arg(long, value_name = "OUTPUT", help = "Set the output", default_value = "output-font")]
		output: String,
		#[arg(long, value_name = "SIZE", help = "Set the font size", default_value_t = 16)]
		size: u32,
		#[arg(long, value_name = "TEXSIZE", help = "Set the texture size", default_value_t = 1024)]
		texsize: u32,
		#[arg(long, value_name = "BORDER", help = "Set the border size", default_value_t = 0)]
		border: u32,
		#[arg(long, value_name = "DISTANCEFIELD-SCALE", help = "Set/enable the distancefield scale", default_value_t = 4)]
		distancefield_scale: u16,
		#[arg(long, value_name = "DISTANCEFIELD-MAX-DISTANCE", help = "Set the distancefield maximum distance", default_value_t = 2)]
		distancefield_max_distance: u16,
		#[arg(long, value_name = "INPUT", help = "Set the input font(s) (only .ttf supported)", num_args = 1.., required = true)]
		input: Vec<String>,
	},
	Preview {
		#[arg(long, value_name = "INPUT", help = "Set the input", default_value = "output-font")]
		input: String,
	},
}

fn main() {
	//		${madfonter} $TEXSIZE $SIZE "${TTF}" ${TMP_DIR}/${NAME}.tga ${TMP_DIR}/${NAME}.omfont
	// omt-font create --output test-font --texsize 1024 --size 40 --ttf test.ttf

	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Create {
			output,
			size,
			texsize,
			border,
			distancefield_scale,
			distancefield_max_distance,
			input,
		}) => {
			println!("output         : {:?}", output);
			println!("texsize        : {:?}", texsize);
			println!("size           : {:?}", size);
			println!("border         : {:?}", border);
			println!("df_scale  	 : {:?}", distancefield_scale);
			println!("df_max_distance: {:?}", distancefield_max_distance);
			//		println!("input          : {:?}", input );
			println!("input          : [");
			for i in &input {
				println!("\t{:?}", i);
			}
			println!("]");

			let input_refs: Vec<&str> = input.iter().map(String::as_str).collect();

			match Font::create(
				&output,
				texsize,
				size,
				border,
				distancefield_scale,
				distancefield_max_distance,
				&input_refs,
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
		},
		Some(Commands::Preview { input }) => {
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
		},
		None => {
			process::exit(-1);
		},
	}
}
