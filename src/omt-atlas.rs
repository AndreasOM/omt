use std::process;

use clap::{Parser, Subcommand};
use omt::atlas::Atlas;
use omt::atlas::AtlasPreviewer;
use omt::atlas::AtlasSet;

#[derive(Debug, Parser)]
#[command(name = "omt-atlas")]
#[command(author, version)]
#[command(about = "Part of the OMT suite of game tools. Handles texture atlases.", long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Combine {
		#[arg(short, long)]
		output:         std::path::PathBuf,
		#[arg(short, long)]
		size:           Option<u32>,
		#[arg(short, long)]
		maximum_size:   Option<u32>,
		#[arg(short, long, default_value_t = 0)]
		border:         u32,
		#[arg(short, long, num_args = 1.., required = true)]
		//		#[clap(short, long, required = true)] // use above, since this is not good enough
		input: Vec<std::path::PathBuf>,
		#[arg(short = 'r', long)]
		reference_path: Option<std::path::PathBuf>,
	},
	Info {
		#[arg(short, long)]
		input: String, // :TODO: std::path::PathBuf,
	},
	Preview {
		#[arg(short, long)]
		input: String, // :TODO: std::path::PathBuf,
	},
	Uncombine {
		#[arg(short, long)]
		input:       String, // :TODO: std::path::PathBuf,
		#[arg(short, long, default_value = ".")]
		output_path: std::path::PathBuf,
		#[arg(short, long)]
		force:       bool,
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
					maximum_size,
					border,
					input,
					reference_path,
				} => {
					//println!("combine {:?} {} {} {:?}", &output, &size, &border, &input);
					println!("combine");
					println!("output         : {:?}", output);
					println!("size           : {:?}", size);
					println!("border         : {:?}", border);
					//println!("write reference: {}", if write_reference { "YES" } else { "NO" } );
					if let Some(rp) = &reference_path {
						println!("reference_path : {}", rp.display());
					}
					//		println!("input          : {:?}", input );
					println!("input          : [");
					for i in &input {
						println!("\t{:?}", i);
					}
					println!("]");
					let mut atlas_set = AtlasSet::default()
						.with_border(border)
						.with_inputs(input.iter().map(|p| p.as_path()).collect());
					if let Some(size) = &size {
						atlas_set = atlas_set.with_target_size(*size);
					};
					println!("{:?}", maximum_size);
					if let Some(maximum_size) = &maximum_size {
						atlas_set = atlas_set.with_maximum_size(*maximum_size);
						atlas_set.autosize()?;
					};
					atlas_set.refit()?;
					/*
					match Atlas::combine(
						&output,
						size,
						border,
						//&input.iter().map(String::as_str).collect(),
						&input,
						reference_path.as_ref(),
					) {
						*/
					match atlas_set.save(&output, reference_path.as_ref().map(|p| p.as_path())) {
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
				Commands::Uncombine {
					input,
					output_path,
					force,
				} => {
					println!("uncombine");
					println!("input       : {:?}", input);
					println!("output_path : {:?}", output_path);
					println!("force       : {:?}", force);
					match Atlas::uncombine(&input, &output_path, force) {
						Ok((extracted, skipped)) => {
							println!("\nSummary:");
							println!("  Extracted: {} files", extracted);
							if skipped > 0 {
								println!("  Skipped  : {} files", skipped);
							}
							process::exit(0);
						},
						Err(e) => {
							println!("Error extracting from atlas: {}", &e);
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
