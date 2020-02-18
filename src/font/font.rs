use crate::util::OmError;

use std::fs::File;
use std::io::{BufReader, Read, Write};

use image::{ DynamicImage, ImageFormat, GenericImage, GenericImageView };
use rusttype::{point, FontCollection, PositionedGlyph, Scale};

//#[derive(Debug)]
struct Glyph {
	pub codepoint:	u8,
	width:		u32,
	height:		u32,
	pub image:	DynamicImage,
}

impl std::fmt::Debug for Glyph {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::result::Result<(), std::fmt::Error > {
		f.debug_struct("Glyph")
			.field("codepoint", &self.codepoint)
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}

impl Glyph {
	pub fn new( codepoint: u8, width: u32, height: u32 ) -> Glyph {
		Glyph {
			codepoint: codepoint,
			width: width,
			height: height,
			image: image::DynamicImage::new_rgba8( width, height ),
		}
	}
	pub fn set_pixel( &mut self, x: i32, y:i32, v: f32 ) {
		if x < 0 || y < 0 {
			return;
		}
		let x = x as u32;
		let y = y as u32;
		if x >= self.width || y >= self.height {
			return;
		}
//		println!("set_pixel {:?}, {:?}, {:?}", x, y, v );
		let v = ( v * 255.0 ) as u8;
		let pixel = image::Rgba([v, v, v, v]);
		self.image.put_pixel( x, y, pixel );
	}
}

#[derive(Debug)]
pub struct Font {
	glyphs: Vec<Glyph>,
}

impl Font {
// 		match Font::create( &output, texsize, size, &input ) {

	fn new( ) -> Font {
		Font {
			glyphs: Vec::new(),
		}
	}
	fn add_glyph( &mut self, glyph: Glyph ) {
		self.glyphs.push( glyph );
	}

	pub fn create(
		output: &str, texsize: u32, size: u32, input: &Vec<&str> 
	) -> Result<u32, OmError>{
		// load ttf
		// :TODO: load all input fonts!
		let mut f = match File::open(input[ 0 ]) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};

//		let mut bufreader = BufReader::new( f );
		let mut buffer = Vec::new();

	    // read the whole file
	    f.read_to_end(&mut buffer).unwrap();//_or_else( return Err(OmError::Generic( "Error reading font file".to_string() )));

		let collection = FontCollection::from_bytes(&buffer[..] as &[u8]).unwrap_or_else(|e| {
	        panic!("error constructing a FontCollection from bytes: {}", e);
    	});

		let font = collection
		        .into_font() // only succeeds if collection consists of one font
		        .unwrap_or_else(|e| {
		            panic!("error turning FontCollection into a Font: {}", e);
		        });

		let scale = Scale::uniform(size as f32);
		let start = point(0.0, 0.0 /*+ v_metrics.ascent*/ );

		let mut the_font = Font::new();
		for c in 0..128u8 {
			let g = font.glyph( c as char );
			let data = g.get_data();
			println!("{:?} -> {:#?}", c, data );

			// :HACK: :TODO: rasterize after positioning into final image
			let ch = format!("{}", c as char );
//			println!("ch: >{:?}<", ch );
			let layout = font.layout( &ch, scale, start);
			for pg in layout {
				match pg.pixel_bounding_box() {
					None => {},
					Some( bb ) => {
						println!("bb {:?}", bb );
						let w = bb.width() as u32;
						let h = bb.height() as u32;
						let mut glyph = Glyph::new( c, w, h );
						pg.draw(|x, y, v| {
//							glyph.set_pixel( ( bb.min.x+x as i32 ) , ( bb.min.y+y as i32 ), v );
							glyph.set_pixel( ( x as i32 ) , ( y as i32 ), v );
						});
						the_font.add_glyph( glyph );
						break;
					}
				}
			}
		}

//		println!("the font: {:?}", the_font );

		for g in the_font.glyphs {
			let filename = format!("test_glyph_{:?}.png", g.codepoint );
			g.image.save_with_format(filename, ImageFormat::PNG);
		}

		Err( OmError::NotImplemented( "Font::create Not implemented".to_string() ) )
	}
}
