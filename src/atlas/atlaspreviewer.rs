use crate::util::OmError;

use crate::atlas::Atlas;
use image::{ DynamicImage, ImageFormat, GenericImage, GenericImageView };
use minifb::{Key, Window, WindowOptions};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AtlasPreviewer {

}

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const GRID_SIZE: usize = 64;
impl AtlasPreviewer {

	fn mixByte( a: u8, b: u8, f: u8 ) -> u8 {
		let f = ( f as f32 )/255.0;
		let fa = a as f32 * f;
		let fb = b as f32 * (1.0-f);

		( fa + fb ) as u8
	}

	fn mixRgba( a: u32, b: u32, f: f32 ) -> u32 {
		let ra = ( ( a >> 24 )&0x000000ff ) as u8;
		let ga = ( ( a >> 16 )&0x000000ff ) as u8;
		let ba = ( ( a >>  8 )&0x000000ff ) as u8;
		let aa = ( ( a >>  0 )&0x000000ff ) as u8;

		let rb = ( ( b >> 24 )&0x000000ff ) as u8;
		let gb = ( ( b >> 16 )&0x000000ff ) as u8;
		let bb = ( ( b >>  8 )&0x000000ff ) as u8;
		let ab = ( ( b >>  0 )&0x000000ff ) as u8;

		let r = ( ( ra as f32 ) * f + ( rb as f32 ) * ( 1.0 -f ) ) as u32;
		let g = ( ( ga as f32 ) * f + ( gb as f32 ) * ( 1.0 -f ) ) as u32;
		let b = ( ( ba as f32 ) * f + ( bb as f32 ) * ( 1.0 -f ) ) as u32;
		let a = ( ( aa as f32 ) * f + ( ab as f32 ) * ( 1.0 -f ) ) as u32;

		let rgba = ( r << 24 )|( g << 16 )|( b << 8 )|a;

		rgba
	}

	fn blit( buffer: &mut Vec< u32 >, w: usize, h: usize, scale: f32, img: &DynamicImage ) {
		let mut pos = 0;
		for y in 0..h {
			for x in 0..w {
				let sx = ( x as f32 * scale ).trunc() as u32;
				let sy = ( y as f32 * scale ).trunc() as u32;
				let pixel = img.get_pixel( sx, sy );
				let r = pixel[ 0 ];// as u32;
				let g = pixel[ 1 ];// as u32;
				let b = pixel[ 2 ];// as u32;
				let a = pixel[ 3 ];// as u32;
				let bg = ( 0x20 + (( x/GRID_SIZE + y/GRID_SIZE )%2)*0xB0 ) as u8;
				let bg = [bg,bg,bg];

				let r = AtlasPreviewer::mixByte( r, bg[ 0 ], a ) as u32;
				let g = AtlasPreviewer::mixByte( g, bg[ 1 ], a ) as u32;
				let b = AtlasPreviewer::mixByte( b, bg[ 2 ], a ) as u32;

				let rgb: u32 = ( r << 16 )|( g << 8 )|( b << 0 );
				buffer[ pos ] = rgb;
				pos += 1;
			}
		}
	}

	fn draw_hline( buffer: &mut Vec< u32 >, w: usize, h: usize, scale: f32, sx: u32, ex: u32, y: u32, col: u32 ) {
		// :TODO: clip line

		let mut y = ( y as f32 / scale ) as usize;
		let mut sx = ( sx as f32 / scale ) as usize;
		let mut ex = ( ex as f32 / scale ) as usize;
		if y >= h {
			y = h-1;
		}
		if sx >= w {
			sx = w-1;
		}
		if ex >= w {
			ex = w-1;
		}
		let col = col >> 8; // :HACK: get rid of alpha
		for x in sx..=ex {
			let p = w * y + x;
			buffer[ p ] = col;
		}
	}

	fn draw_vline( buffer: &mut Vec< u32 >, w: usize, h: usize, scale: f32, x: u32, sy: u32, ey: u32, col: u32 ) {
		// :TODO: clip line
		let x = ( x as f32 / scale ) as usize;
		let sy = ( sy as f32 / scale ) as usize;
		let ey = ( ey as f32 / scale ) as usize;
		let col = col >> 8; // :HACK: get rid of alpha
		for y in sy..=ey {
			let p = w * y + x;
			buffer[ p ] = col;
		}
	}

	fn draw_frame( buffer: &mut Vec< u32 >, w: usize, h: usize, scale: f32, x: u32, y: u32, fw: u32, fh: u32, col: u32 ) {
		AtlasPreviewer::draw_hline( buffer, w, h, scale, x, x+fw, y, col );
		AtlasPreviewer::draw_hline( buffer, w, h, scale, x, x+fw, y+fh, col );
		AtlasPreviewer::draw_vline( buffer, w, h, scale, x, y, y+fh, col );
		AtlasPreviewer::draw_vline( buffer, w, h, scale, x+fw, y, y+fh, col );
	}

	pub fn preview(
		input: &str
	) -> Result<u32,OmError>{

		let start_time = SystemTime::now();
		let mut scale = 1.0;
		let mut frame_col: u32 = 0xa020a0ff;

		let atlases = Atlas::all_for_template( &input );
//		println!("{:?}", atlases );

		let n = atlases.len() as u32;
		if n == 0 {
			Err(OmError::Generic("No matching atlas found.".to_string()))
		} else {
			let mut prev_active_atlas = 0xffffffff;
			let mut active_atlas = 0;

			// display
			let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
			let mut img_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

			let mut window = Window::new(
				"Test - ESC to exit",
				WIDTH,
				HEIGHT,
				WindowOptions::default(),
			)
			.unwrap_or_else(|e| {
				panic!("{}", e);
			});

    		window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

			while window.is_open() && !window.is_key_down(Key::Escape) {
				let now = SystemTime::now();
				let time = now.duration_since(start_time).unwrap().as_millis();
				let time = time as u128 as f32 / 1000.0;
				if prev_active_atlas != active_atlas {
					match atlases.get( active_atlas ) {
						None => {},
						Some( a ) => {
							println!("New active atlas {:?}", a );
							prev_active_atlas = active_atlas;
							match &a.image {
								None => {
									// :TODO: clear buffer
								},
								Some( i ) => {
									// :TODO: calculate best scale
									scale = 0.5;

									// assumption texture is square
									let is = i.dimensions().0 as f32;
									scale = is / 1024.0;
									AtlasPreviewer::blit( &mut img_buffer, WIDTH, HEIGHT, scale, &i );
								},
							}							
						}						
					}
				}
//				let frame_col = ( frame_col as f32 * ( time * 0.1 ).sin() ) as u32;
				let m = 0.5 + 0.5 * ( time * 1.5 ).sin();
//				println!("m {:?}", m );
//				let frame_col = AtlasPreviewer::mixRgba( 0xffffffff, 0x000000ff, m );
				let frame_col = AtlasPreviewer::mixRgba( 0xffffffff, 0x802080ff, m );
				match atlases.get( active_atlas ) {
					None => {},
					Some( a ) => {
						buffer.copy_from_slice( &img_buffer );
						for e in &a.entries {
							AtlasPreviewer::draw_frame( &mut buffer, WIDTH, HEIGHT, scale, e.x, e.y, e.width, e.height, frame_col );
						}
					},
				}

			// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
			window
				.update_with_buffer(&buffer, WIDTH, HEIGHT)
				.unwrap();
			}

			Ok( n )	
		}
	}
}
