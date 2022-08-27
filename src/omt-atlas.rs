use std::process;

use clap::{Parser, Subcommand};
use omt::atlas::Atlas;
use omt::atlas::AtlasPreviewer;

#[derive(Debug, Parser)]
#[clap(name = "omt-atlas")]
#[clap(author, version)]
#[clap(about = "Part of the OMT suite of game tools. Handles texture atlases.", long_about = None)]
struct Cli {
	#[clap(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Combine {
		#[clap(short, long, action)]
		output: std::path::PathBuf,
		#[clap(short, long, action)]
		size:   u32,
		#[clap(short, long, action, default_value_t = 0)]
		border: u32,
		#[clap(short, long, min_values = 1, required = true)]
		//		#[clap(short, long, required = true)] // use above, since this is not good enough
		input: Vec<std::path::PathBuf>,
	},
	Info {
		#[clap(short, long, action)]
		input: String, // :TODO: std::path::PathBuf,
	},
	Preview {
		#[clap(short, long, action)]
		input: String, // :TODO: std::path::PathBuf,
	},
}

fn main() -> anyhow::Result<()> {
	// omt-atlas combine --output test-atlas-%d --size 2048 --border 0 --input ../Content/test.png

	let cli = Cli::parse();
	//dbg!(&cli);
	match cli.command {
		Some(command) => {
			//dbg!(&command);
			match command {
				Commands::Combine {
					output,
					size,
					border,
					input,
				} => {
					//println!("combine {:?} {} {} {:?}", &output, &size, &border, &input);
					println!("combine");
					println!("output         : {:?}", output);
					println!("size           : {:?}", size);
					println!("border         : {:?}", border);
					//		println!("input          : {:?}", input );
					println!("input          : [");
					for i in &input {
						println!("\t{:?}", i);
					}
					println!("]");
					match Atlas::combine(
						&output, size, border,
						//&input.iter().map(String::as_str).collect(),
						&input,
					) {
						Ok(1) => {
							println!("1 atlas created");
							process::exit(0);
						},
						Ok(n) => {
							println!("{:?} atlases created", n);
							process::exit(0);
						},
						Err(e) => {
							println!("Error combining atlas >{:?}>", e);
							process::exit(-1);
						},
					}
				},
				Commands::Info { input } => {
					println!("info");
					println!("input         : {:?}", input);
					match Atlas::info(&input) {
						Ok(_) => {
							process::exit(0);
						},
						Err(e) => {
							println!("Error getting info from atlas: {}", &e);
							process::exit(-1);
						},
					}
				},
				Commands::Preview { input } => {
					println!("preview");
					println!("input         : {:?}", input);
					match AtlasPreviewer::preview(&input) {
						Ok(_) => {
							process::exit(0);
						},
						Err(e) => {
							println!("Error getting info from atlas: {}", &e);
							process::exit(-1);
						},
					}
				},
			};
		},
		None => {},
	};

	Ok(())
}
