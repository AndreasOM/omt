
use image::{ DynamicImage, GenericImageView };

// const GRID_SIZE: u32 = 64;

// :TODO: remove grid or refactor

pub struct DrawBuffer {
	width: u32,
	height: u32,
	data: Vec<u32>,
	scale: f32,
}

impl DrawBuffer {
	pub fn new( w: u32, h: u32 ) -> DrawBuffer {
		DrawBuffer {
			width: w,
			height: h,
			data: vec![0; ( w*h ) as usize],
			scale: 1.0,
		}
	}

	pub fn get_scale( &self ) -> f32 {
		self.scale
	}
	pub fn set_scale( &mut self, scale: f32 ) {
		self.scale = scale;
	}

	pub fn copy_from_draw_buffer( &mut self, other: &DrawBuffer ) {
		// :TODO: verify sizes
		self.data.copy_from_slice( other.get_data() );
	}

	pub fn fill_with_grid( &mut self, size: u32, col_a: u32, col_b: u32 ) {
		let h = self.height; //image.dimensions().0;
		let w = self.width; //image.dimensions().1;
		let mut pos = 0;
		for y in 0..h {
			for x in 0..w {
				if ( x/size + y/size )%2 == 0 {
					self.data[ pos ] = col_a;
				} else {
					self.data[ pos ] = col_b;					
				}
				pos += 1;
			}
		}		
	}

	pub fn blit_image( &mut self, image: &DynamicImage) {
		let mut pos = 0;
		let h = self.height;
		let w = self.width;
		for y in 0..h {
			for x in 0..w {
				let sx = ( x as f32 * self.scale ).trunc() as u32;
				let sy = ( y as f32 * self.scale ).trunc() as u32;
				let pixel = image.get_pixel( sx, sy );
				let r = pixel[ 0 ] as u32;
				let g = pixel[ 1 ] as u32;
				let b = pixel[ 2 ] as u32;
				let a = pixel[ 3 ] as u32;
				let bg = self.data[ pos ];
				let fg: u32 = ( r << 16 )|( g << 8 )|( b << 0 );
				let rgb = DrawBuffer::mix_rgba( fg, bg, ( a as f32 )/255.0 );

				self.data[ pos ] = rgb;
				pos += 1;
			}
		}
	}

	pub fn draw_hline( &mut self, sx: u32, ex: u32, y: u32, col: u32 ) {
		// :TODO: clip line

		let w = self.width as usize;
		let h = self.height as usize;

		let mut y = ( y as f32 / self.scale ) as usize;
		let mut sx = ( sx as f32 / self.scale ) as usize;
		let mut ex = ( ex as f32 / self.scale ) as usize;
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
			self.data[ p ] = col;
		}
	}

	#[allow(dead_code)]
	fn draw_vline( &mut self, x: u32, sy: u32, ey: u32, col: u32 ) {
		// :TODO: clip line
		let w = self.width as usize;
		let h = self.height as usize;

		let mut x = ( x as f32 / self.scale ) as usize;
		let mut sy = ( sy as f32 / self.scale ) as usize;
		let mut ey = ( ey as f32 / self.scale ) as usize;
		if x >= w {
			x = w-1;
		}
		if sy >= h {
			sy = h-1;
		}
		if ey >= h {
			ey = h-1;
		}
		let col = col >> 8; // :HACK: get rid of alpha
		for y in sy..=ey {
			let p = w * y + x;
			self.data[ p ] = col;
		}
	}

	fn clamp_i32( v: i32, min: i32, max: i32 ) -> i32 {
		if v < min {
			min
		} else if v > max {
			max
		} else {
			v
		}
	}

	pub fn draw_filled_rectangle( &mut self, sx: i32, sy: i32, ex: i32, ey: i32, col: u32 ) {
		// :TODO: flip as needed
//		println!("Drawing rect {:?}, {:?} - {:?}, {:?}", sx, sy, ex, ey);
		let s = self.scale;
		let sx = ( ( sx as f32 )/s ).trunc() as i32;
		let sy = ( ( sy as f32 )/s ).trunc() as i32;
		let ex = ( ( ex as f32 )/s ).trunc() as i32;
		let ey = ( ( ey as f32 )/s ).trunc() as i32;

		let sx = DrawBuffer::clamp_i32( sx, 0, self.width as i32 ) as u32;
		let sy = DrawBuffer::clamp_i32( sy, 0, self.height as i32 ) as u32;
		let ex = DrawBuffer::clamp_i32( ex, 0, self.width as i32 ) as u32;
		let ey = DrawBuffer::clamp_i32( ey, 0, self.height as i32 ) as u32;

//		println!("Drawing clamped rect {:?}, {:?} - {:?}, {:?}", sx, sy, ex, ey);
		for y in sy..ey {
			for x in sx..ex {
				let offset = ( self.width * y + x ) as usize;
				self.data[ offset ] = col;
			}
		}
	}
	pub fn draw_frame( &mut self, x: i32, y: i32, fw: u32, fh: u32, col: u32, line_width: u32 ) {
		let hw = ( ( line_width as f32 )*0.5 ).trunc() as i32;
		let fw = fw as i32;
		let fh = fh as i32;

		// top
		self.draw_filled_rectangle( x-hw, y-hw, x+fw+hw, y+hw, col );
		// bottom
		self.draw_filled_rectangle( x-hw, y+fh-hw, x+fw+hw, y+fh+hw, col );
		// left
		self.draw_filled_rectangle( x-hw, y+hw, x+hw, y+fh-hw, col );
		// right
		self.draw_filled_rectangle( x+fw-hw, y+hw, x+fw+hw, y+fh-hw, col );
	}

	pub fn get_width( &self ) -> u32 {
		self.width
	}
	pub fn get_height( &self ) -> u32 {
		self.height
	}

	pub fn get_data( &self ) -> &Vec<u32> {
		&self.data
	}

	pub fn mix_rgba( a: u32, b: u32, f: f32 ) -> u32 {
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

	#[allow(dead_code)]
	fn mix_byte( a: u8, b: u8, f: u8 ) -> u8 {
		let f = ( f as f32 )/255.0;
		let fa = a as f32 * f;
		let fb = b as f32 * (1.0-f);

		( fa + fb ) as u8
	}

}

