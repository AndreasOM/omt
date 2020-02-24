use clap::{Arg, App, SubCommand};
use std::process;

use omt::font::Font;
use omt::font::FontPreviewer;
use omt::util::OmError;

fn main() {
//		${madfonter} $TEXSIZE $SIZE "${TTF}" ${TMP_DIR}/${NAME}.tga ${TMP_DIR}/${NAME}.omfont
// omt-font create --output test-font --texsize 1024 --size 40 --ttf test.ttf

	let matches = App::new("omt-font")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Handles fonts")
					.subcommand(SubCommand::with_name("create")
						.arg(Arg::with_name("output")
							.long("output")
							.value_name("OUTPUT")
							.help("Set the output")
							.takes_value(true)
						)
						.arg(Arg::with_name("size")
							.long("size")
							.value_name("SIZE")
							.help("Set the font size")
							.takes_value(true)
						)
						.arg(Arg::with_name("texsize")
							.long("texsize")
							.value_name("TEXSIZE")
							.help("Set the texture size")
							.takes_value(true)
						)
						.arg(Arg::with_name("border")
							.long("border")
							.value_name("BORDER")
							.help("Set the border size")
							.takes_value(true)
						)
						.arg(Arg::with_name("distancefield-scale")
							.long("distancefield-scale")
							.value_name("DISTANCEFIELD-SCALE")
							.help("Set/enable the distancefield scale")
							.takes_value(true)
						)
						.arg(Arg::with_name("distancefield-max-distance")
							.long("distancefield-max-distance")
							.value_name("DISTANCEFIELD-MAX-DISTANCE")
							.help("Set the distancefield maximum distance")
							.takes_value(true)
						)
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input font(s) (only .ttf supported)")
							.takes_value(true)
							.multiple(true)
						)
					)
					.subcommand(SubCommand::with_name("preview")
						.arg(Arg::with_name("input")
							.long("input")
							.value_name("INPUT")
							.help("Set the input")
							.takes_value(true)
						)
					)
					.get_matches();

	if let ("create", Some( sub_matches ) ) = matches.subcommand() {
		let output			= sub_matches.value_of("output").unwrap_or("output-font").to_string();
		let texsize 		= sub_matches.value_of("texsize").unwrap_or("1024").to_string();
		let size			= sub_matches.value_of("size").unwrap_or("16").to_string();
		let border			= sub_matches.value_of("border").unwrap_or("0").to_string();
		let df_scale		= sub_matches.value_of("distancefield-scale").unwrap_or("4").to_string();
		let df_max_distance	= sub_matches.value_of("distancefield-max-distance").unwrap_or("2").to_string();
		let input			= sub_matches.values_of("input").unwrap().collect::<Vec<_>>(); 

		let texsize = match u32::from_str_radix( &texsize, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing texsize {:?} >{}<", x, texsize );
				process::exit( -1 );
			}
		};

		let size = match u32::from_str_radix( &size, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing size {:?} >{}<", x, size );
				process::exit( -1 );
			}
		};

		let border = match u32::from_str_radix( &border, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing border {:?} >{}<", x, border );
				process::exit( -1 );
			}
		};

		let df_scale = match u16::from_str_radix( &df_scale, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing df_scale {:?} >{}<", x, df_scale );
				process::exit( -1 );
			}
		};

		let df_max_distance = match u16::from_str_radix( &df_max_distance, 10 ) {
			Ok( n ) => n,
			x => {
				println!("Error parsing df_max_distance {:?} >{}<", x, df_max_distance );
				process::exit( -1 );
			}
		};

		println!("output         : {:?}", output );
		println!("texsize        : {:?}", texsize );
		println!("size           : {:?}", size );
		println!("border         : {:?}", border );
		println!("df_scale  	 : {:?}", df_scale );
		println!("df_max_distance: {:?}", df_max_distance );
//		println!("input          : {:?}", input );
		println!("input          : [" );
		for i in &input {
			println!("\t{:?}", i );
		}
		println!("]" );

		match Font::create( &output, texsize, size, border, df_scale, df_max_distance, &input ) {
			Ok( 1 ) => {
				println!("1 font created" );
				process::exit( 0 );
			},
			Ok( n ) => {
				println!("{:?} fonts created", n );
				process::exit( 0 );
			},
			Err( e ) => {
				println!("Error creating font >{:?}<", e );
				process::exit( -1 );
			}
		}
	}
	if let ("preview", Some( sub_matches ) ) = matches.subcommand() {
		let input = sub_matches.value_of("input").unwrap_or("output-font").to_string();
		println!("input         : {:?}", input );
		match FontPreviewer::preview( &input ) {
			Ok( _ ) => {
				process::exit( 0 );
			},
			Err( e ) => {
				println!("Error getting info from font." );
				match e {
					OmError::NotImplemented( e ) => println!("NotImplemented: {:?}", e ),
					OmError::Generic( e ) => println!("Generic: {:?}", e ),
				};
				process::exit( -1 );
			}
		}
	}
	process::exit( -1 );
}
