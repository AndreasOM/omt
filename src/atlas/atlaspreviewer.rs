use crate::util::OmError;

use crate::atlas::Atlas;
use crate::gfx::DrawBuffer;

use image::{ DynamicImage, ImageFormat, GenericImage, GenericImageView };
use minifb::{Key, Window, WindowOptions};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AtlasPreviewer {

}

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const GRID_SIZE: usize = 64;
impl AtlasPreviewer {

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
			let mut img_draw_buffer = DrawBuffer::new( WIDTH as u32, HEIGHT as u32);
			let mut draw_buffer = DrawBuffer::new( WIDTH as u32, HEIGHT as u32);

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
									img_draw_buffer.set_scale( scale );
									img_draw_buffer.blit_image( &i );
								},
							}							
						}						
					}
				}

//				let frame_col = ( frame_col as f32 * ( time * 0.1 ).sin() ) as u32;
				let m = 0.5 + 0.5 * ( time * 1.5 ).sin();
//				println!("m {:?}", m );
//				let frame_col = AtlasPreviewer::mixRgba( 0xffffffff, 0x000000ff, m );
				let frame_col = DrawBuffer::mixRgba( 0xffffffff, 0x802080ff, m );
				match atlases.get( active_atlas ) {
					None => {},
					Some( a ) => {
						draw_buffer.copy_from_draw_buffer( &img_draw_buffer );
						for e in &a.entries {
							draw_buffer.draw_frame( e.x, e.y, e.width, e.height, frame_col );
						}
					},
				}
			// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
			window
//				.update_with_buffer(&buffer, WIDTH, HEIGHT)
				.update_with_buffer(draw_buffer.get_data(), draw_buffer.get_width() as usize, draw_buffer.get_height() as usize )
				.unwrap();
			}

			Ok( n )	
		}
	}
}
