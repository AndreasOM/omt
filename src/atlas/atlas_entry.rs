use std::path::Path;

use image::DynamicImage;
use image::GenericImageView;
use regex::Regex;

#[derive(Clone)]
pub struct AtlasEntry {
	pub filename: String,
	pub image:    Option<DynamicImage>,
	pub x:        u32,
	pub y:        u32,
	pub width:    u32,
	pub height:   u32,
}

impl std::fmt::Debug for AtlasEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		f.debug_struct("AtlasEntry")
			.field("filename", &self.filename)
			//			.field("image", if self.image.is_some() {"YES"} else {"NO"} )
			.field("x", &self.x)
			.field("y", &self.y)
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}
impl AtlasEntry {
	pub fn new(filename: &str, width: u32, height: u32) -> AtlasEntry {
		AtlasEntry {
			filename: filename.to_string(),
			image:    None,
			x:        0,
			y:        0,
			width:    width,
			height:   height,
		}
	}

	pub fn set_image(&mut self, image: DynamicImage) {
		self.width = image.dimensions().0;
		self.height = image.dimensions().1;
		self.image = Some(image);
	}

	pub fn set_position(&mut self, x: u32, y: u32) {
		self.x = x;
		self.y = y;
	}
	pub fn get_basename(&self) -> String {
		let basename = Path::new(&self.filename)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap();
		basename.to_string()
	}
	pub fn get_stem(&self) -> String {
		let stem = Path::new(&self.filename)
			.file_stem()
			.unwrap()
			.to_str()
			.unwrap();
		stem.to_string()
	}
	/*
			sx = s[ 0 ].to_f/size
			sy = s[ 1 ].to_f/size
			ex = e[ 0 ].to_f/size
			ey = e[ 1 ].to_f/size

			t.scaleX = (ex-sx)
			t.scaleY = (ey-sy)

			def getMatrix
				# :HACK: :TODO: add rotation
				[
					@scaleX	, 0.0		, @x,
					0.0		, @scaleY	, @y
				]
			end
	*/
	pub fn get_matrix(&self, size: u32) -> [f32; 6] {
		// :TODO: cleanup please
		let sx = self.x as f32 / size as f32;
		let sy = self.y as f32 / size as f32;
		//		let ex = ( self.x + self.width ) as f32 / size as f32;
		//		let ey = ( self.y + self.height ) as f32 / size as f32;
		let scale_x = self.width as f32 / size as f32; //ex - sx;
		let scale_y = self.height as f32 / size as f32; //ey - sy;
		[scale_x, 0.0, sx, 0.0, scale_y, sy]
	}
}
