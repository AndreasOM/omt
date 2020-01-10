
use image::{ DynamicImage, ImageFormat, GenericImage, GenericImageView };

const GRID_SIZE: u32 = 64;

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
		let h = self.height; //image.dimensions().0;
		let w = self.width; //image.dimensions().1;
		for y in 0..h {
			for x in 0..w {
				let sx = ( x as f32 * self.scale ).trunc() as u32;
				let sy = ( y as f32 * self.scale ).trunc() as u32;
				let pixel = image.get_pixel( sx, sy );
				let r = pixel[ 0 ] as u32;
				let g = pixel[ 1 ] as u32;
				let b = pixel[ 2 ] as u32;
				let a = pixel[ 3 ] as u32;
//				let bg = ( 0x20 + (( x/GRID_SIZE + y/GRID_SIZE )%2)*0xB0 ) as u8;
//				let bg = [bg,bg,bg];
				let bg = self.data[ pos ];
				let fg: u32 = ( r << 16 )|( g << 8 )|( b << 0 );
				let rgb = DrawBuffer::mixRgba( fg, bg, ( a as f32 )/255.0 );

//				let r = DrawBuffer::mixByte( r, bg[ 0 ], a ) as u32;
//				let g = DrawBuffer::mixByte( g, bg[ 1 ], a ) as u32;
//				let b = DrawBuffer::mixByte( b, bg[ 2 ], a ) as u32;

//				let rgb: u32 = ( r << 16 )|( g << 8 )|( b << 0 );
				self.data[ pos ] = rgb;
				pos += 1;
			}
		}
	}

	fn draw_hline( &mut self, sx: u32, ex: u32, y: u32, col: u32 ) {
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

	pub fn draw_frame( &mut self, x: u32, y: u32, fw: u32, fh: u32, col: u32 ) {
		self.draw_hline( x, x+fw, y, col );
		self.draw_hline( x, x+fw, y+fh, col );
		self.draw_vline( x, y, y+fh, col );
		self.draw_vline( x+fw, y, y+fh, col );
/*
		AtlasPreviewer::draw_hline( buffer, w, h, scale, x, x+fw, y, col );
		AtlasPreviewer::draw_hline( buffer, w, h, scale, x, x+fw, y+fh, col );
		AtlasPreviewer::draw_vline( buffer, w, h, scale, x, y, y+fh, col );
		AtlasPreviewer::draw_vline( buffer, w, h, scale, x+fw, y, y+fh, col );
*/
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
	pub fn mixRgba( a: u32, b: u32, f: f32 ) -> u32 {
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

	fn mixByte( a: u8, b: u8, f: u8 ) -> u8 {
		let f = ( f as f32 )/255.0;
		let fa = a as f32 * f;
		let fb = b as f32 * (1.0-f);

		( fa + fb ) as u8
	}

}

