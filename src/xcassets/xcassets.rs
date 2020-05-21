use crate::util::OmError;
use crate::xcassets::appiconset::AppIconSet;

use std::fs::File;

use resize::{Pixel, Type};

pub struct Xcassets {

}

impl Xcassets {

	pub fn generate(
		input: &str,
		mode: &str,
		output: &str
	) -> Result<u32, OmError>{

		let content_json_filename = format!("{}/Contents.json", output);
		let mut app_icon_set = match AppIconSet::load( &content_json_filename ) {
			Ok( ais ) => ais,
			Err( e ) => {
				println!("ERROR: AppIconSet::load failed with {:?}", e);
				return Err( OmError::Generic( "problem with output file".to_string() ) );
			},
		};

		// load the source // :TODO: handle errors better
	    let decoder = png::Decoder::new(File::open(&input).unwrap());
	    let (info, mut reader) = decoder.read_info().unwrap();
	    let mut src = vec![0; info.buffer_size()];
	    reader.next_frame(&mut src).unwrap();

	    println!("{:#?}", info);

	    let src = src;

		let basename = "icon";
		let suffix = ".png";

		let mut number_of_changes = 0;
		let force = mode == "force";
		for mut ie in app_icon_set.images.iter_mut() {
//			println!("{:?}", ie );
			let do_update = force;
			if !do_update {
				// :TODO: check fill
				// :TODO: check update
			}

			if do_update {
				// figure out size
				let scale = match &ie.scale[..] {
					"1x" => 1.0,
					"2x" => 2.0,
					"3x" => 3.0,
					_ => return Err( OmError::Generic( "Unsupported scale".to_string() ) ),
				};
				let size = ie.size.split("x").collect::<Vec<&str>>();
				let (w,h) = (size[0],size[1]);
				let w: f32 = w.parse::<f32>().expect("");
				let h: f32 = h.parse::<f32>().expect("");

//				let scale = 1.0;
//				let w = 1024.0;
//				let h = 1024.0;

				println!("scale: {:?}, size: {:?} x {:?}", scale, w, h);

				let sw = ( w * scale ) as usize;
				let sh = ( h * scale ) as usize;

				// figure out filename
				let name = format!("{}-{}x{}@{}x{}", basename, sw, sh, scale, suffix);
				println!("{} <= scale: {:?}, size: {:?} x {:?}", name, scale, sw, sh);

				// create image at correct size


				let mut dst = vec![0; 3 * sw * sh];

				resize::resize(
					info.width as usize, info.height as usize,
					sw, sh,
					Pixel::RGB24,
					Type::Lanczos3,
				//	Type::Mitchell,
					&src,
					&mut dst
				);

				let scaled_filename = format!("{}/{}", output, name);

 				let outfh = File::create(scaled_filename).unwrap();
    			let mut encoder = png::Encoder::new(outfh, sw as u32, sh as u32);
    			encoder.set_color(png::ColorType::RGB);
				encoder.set_depth(png::BitDepth::Eight);
    			encoder.write_header().unwrap().write_image_data(&dst).unwrap();
//    			encoder.write_header().unwrap().write_image_data(&src).unwrap();

				ie.filename = Some(name);
				number_of_changes += 1;
			}
		}

		if number_of_changes > 0 {
//			println!("{:#?}", app_icon_set);
			match app_icon_set.save( &content_json_filename ) {
				Ok( _ ) => {},
				Err( _e ) => return Err( OmError::Generic("Error saving app icon set".to_string()) ),
			}
		}

		Ok( number_of_changes )
	}
}
