use std::fs::File;
use std::io::{BufReader, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat};
use om_fork_distance_field::DistanceFieldExt;
use rusttype::{point, Font as RTFont, Scale};

use crate::atlas::AtlasFitter;

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
	pub codepoint: u8,
	pub width:     u32,
	pub height:    u32,
	pub x:         u32,
	pub y:         u32,
	pub advance:   u16,
	pub y_offset:  f32,
	pub matrix:    [f32; 6],
}
/*
impl std::fmt::Debug for Glyph {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::result::Result<(), std::fmt::Error > {
		f.debug_struct("Glyph")
			.field("codepoint", &self.codepoint)
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}
*/
impl Glyph {
	pub fn new(codepoint: u8, width: u32, height: u32) -> Glyph {
		Glyph {
			codepoint: codepoint,
			width:     width,
			height:    height,
			x:         0,
			y:         0,
			advance:   0,
			y_offset:  0.0,
			matrix:    [0.0; 6],
		}
	}
	fn recalc_matrix(&mut self, texsize: u32) {
		let sx = self.x as f32 / texsize as f32;
		let sy = self.y as f32 / texsize as f32;
		let scale_x = self.width as f32 / texsize as f32;
		let scale_y = self.height as f32 / texsize as f32;
		self.matrix = [scale_x, 0.0, sx, 0.0, scale_y, sy];
	}
	fn recalc_from_matrix(&mut self, texsize: u32) {
		self.width = (self.matrix[0 * 3 + 0] * texsize as f32).trunc() as u32;
		self.height = (self.matrix[1 * 3 + 1] * texsize as f32).trunc() as u32;
		self.x = (self.matrix[0 * 3 + 2] * texsize as f32).trunc() as u32;
		self.y = (self.matrix[1 * 3 + 2] * texsize as f32).trunc() as u32;
	}
}

pub struct Font {
	pub glyphs: Vec<Glyph>,
	texsize:    u32,
	size:       u32,
	border:     u32,
	pub image:  DynamicImage,
}

impl std::fmt::Debug for Font {
	// :TODO:
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		f.debug_struct("Glyph")
			.field("glyphs", &self.glyphs)
			//			.field("width", &self.width)
			//			.field("height", &self.height)
			.finish()
	}
}

impl Font {
	// 		match Font::create( &output, texsize, size, &input ) {

	fn new(texsize: u32, size: u32, border: u32) -> Font {
		Font {
			glyphs:  Vec::new(),
			texsize: texsize,
			size:    size,
			border:  border,
			image:   image::DynamicImage::new_rgba8(texsize, texsize),
		}
	}

	fn recalc_matrix(&mut self, texsize: u32) {
		for g in &mut self.glyphs {
			g.recalc_matrix(texsize);
		}
	}

	fn recalc_from_matrix(&mut self, texsize: u32) {
		for g in &mut self.glyphs {
			g.recalc_from_matrix(texsize);
		}
	}

	fn load_omfont_v2(&mut self, filename: &str) -> anyhow::Result<u32> {
		let f = match File::open(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};

		let mut bufreader = BufReader::new(f);
		let chunk_magic = [0x4fu8, 0x4d, 0x46, 0x4f, 0x4e, 0x54];
		for m in &chunk_magic {
			let b = bufreader.read_u8().unwrap_or(0);
			if b != *m {
				anyhow::bail!("Broken chunk magic");
			}
		}
		let version = bufreader.read_u32::<LittleEndian>().unwrap_or(0);
		if version != 2 {
			anyhow::bail!("Unsupported version");
		}

		self.size = bufreader.read_u16::<LittleEndian>().unwrap_or(0) as u32;
		let count = bufreader.read_u16::<LittleEndian>().unwrap_or(0);

		let mut codepoints = Vec::new();

		for _c in 0..count {
			let codepoint = bufreader.read_u32::<LittleEndian>().unwrap_or(0);
			codepoints.push(codepoint);
		}

		for c in 0..count {
			let codepoint = codepoints[c as usize];
			let mut glyph = Glyph::new(codepoint as u8, 0, 0);
			for m in &mut glyph.matrix {
				*m = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
			}
			glyph.advance = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0) as u16;
			glyph.y_offset = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);

			self.glyphs.push(glyph);
		}

		Ok(0)
	}

	fn load_omfont(&mut self, filename: &str) -> anyhow::Result<u32> {
		let f = match File::open(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};

		let mut bufreader = BufReader::new(f);
		let magic = match bufreader.read_u32::<LittleEndian>() {
			//.unwrap_or( 0xffff );
			Ok(m) => m,
			x => {
				println!("{:?}", x);
				anyhow::bail!("reading from buffer");
			},
		};
		if magic != 0x4e464d4f {
			println!("Got magic {:#08x} from {:?}", magic, bufreader);
			anyhow::bail!("Broken file magic");
		}

		let _height = bufreader.read_u32::<LittleEndian>().unwrap_or(0);

		let mut tex_coords = [0f32; 4 * 2];
		let mut v_pos = [0f32; 4 * 3];

		for codepoint in 0..128u8 {
			for i in 0..4 {
				tex_coords[i * 2 + 0] = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
				tex_coords[i * 2 + 1] = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
				v_pos[i * 2 + 0] = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
				v_pos[i * 2 + 1] = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
				v_pos[i * 2 + 2] = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
			}
			//			println!("{:#?}", tex_coords );
			let tex_l = tex_coords[0 * 2 + 0];
			let tex_t = tex_coords[0 * 2 + 1];
			let tex_r = tex_coords[3 * 2 + 0];
			let tex_b = tex_coords[3 * 2 + 1];

			let tex_w = (tex_r - tex_l) * self.texsize as f32;
			let tex_h = (tex_b - tex_t) * self.texsize as f32;

			//			println!("{:?} {:?}", tex_w, tex_h );
			let mut g = Glyph::new(codepoint, tex_w as u32, tex_h as u32);
			g.x = (tex_l * self.texsize as f32) as u32;
			g.y = (tex_t * self.texsize as f32) as u32;
			self.glyphs.push(g);
			//			let left_tx = bufreader.read_f32::<LittleEndian>().unwrap_or( 0.0 );
		}
		for codepoint in 0..128u8 {
			let advance = bufreader.read_u16::<LittleEndian>().unwrap_or(0xff);
			self.glyphs[codepoint as usize].advance = advance as u16;
		}
		Ok(0)
	}

	fn new_from_omfont(fontname: &str, texsize: u32) -> Font {
		let size = 40;
		let border = 0;

		let mut f = Font {
			glyphs:  Vec::new(),
			texsize: texsize,
			size:    size,
			border:  border,
			image:   image::DynamicImage::new_rgba8(texsize, texsize),
		};
		match f.load_omfont_v2(fontname) {
			Ok(_) => {
				// calculate x, y, width, height
				f.recalc_from_matrix(texsize);
				//				println!("{:#?}", f );
			},
			Err(_) => {
				match f.load_omfont(fontname) {
					// for the moment we don't care
					Ok(_) => {},
					Err(_) => {},
				};
			},
		};

		f
	}
	fn add_glyph(&mut self, glyph: Glyph) {
		self.glyphs.push(glyph);
	}

	fn fit_glyphs(&mut self) -> bool {
		let mut atlas_fitter = AtlasFitter::new();

		for (idx, e) in self.glyphs.iter().enumerate() {
			atlas_fitter.add_entry(idx, e.width, e.height);
		}

		let pages = atlas_fitter.fit(self.texsize, self.border);

		if pages.len() > 1 {
			println!("Need {} pages to fit glyphs", pages.len());
			return false;
		}
		for p in &pages {
			for e in &p.entries {
				let mut glyph = &mut self.glyphs[e.id];
				glyph.x = e.x;
				glyph.y = e.y;
			}
		}
		true
	}

	pub fn set_pixel(&mut self, x: i32, y: i32, v: f32) {
		if x < 0 || y < 0 {
			return;
		}
		let x = x as u32;
		let y = y as u32;
		if x >= self.texsize || y >= self.texsize {
			return;
		}
		//		println!("set_pixel {:?}, {:?}, {:?}", x, y, v );
		let v = (v * 255.0) as u8;
		let pixel = image::Rgba([v, v, v, v]);
		self.image.put_pixel(x, y, pixel);
	}

	fn blit_image(&mut self, gx: u32, gy: u32, img: &image::DynamicImage) {
		let w = img.dimensions().0;
		let h = img.dimensions().1;

		for y in 0..h {
			for x in 0..w {
				let p = img.get_pixel(x, y);
				let tx = gx + x;
				let ty = gy + y;
				//				println!("{}, {} + {}, {} = {}, {}", gx, gy, h, w, tx, ty );
				self.image.put_pixel(tx, ty, p);
			}
		}
	}

	fn blit_glyphs(
		&mut self,
		font: rusttype::Font,
		distancefield_scale: u16,
		distancefield_max_distance: u16,
	) -> bool {
		let scale_factor = match distancefield_scale {
			0 => 1,
			1 => 1,
			f => f as u32,
		};

		let scale = Scale::uniform((self.size * scale_factor) as f32);
		let start = point(0.0, 0.0 /*+ v_metrics.ascent*/);

		let glyphs = self.glyphs.clone(); // needed to avoid borrow problem below :(
		for g in glyphs {
			let ch = format!("{}", g.codepoint as char);
			//			println!("Blitting {:?}", ch );
			let layout = font.layout(&ch, scale, start);
			for pg in layout {
				match pg.pixel_bounding_box() {
					None => {},
					Some(bb) => {
						let w = bb.width() as u32 + 2 * self.border * scale_factor;
						let h = bb.height() as u32 + 2 * self.border * scale_factor;
						let mut glyph_image = image::DynamicImage::new_rgba8(w, h);
						pg.draw(|x, y, v| {
							let v = (v * 255.0) as u8;
							let pixel = image::Rgba([v, v, v, v]);
							glyph_image.put_pixel(
								x + self.border * scale_factor,
								y + self.border * scale_factor,
								pixel,
							);
						});

						//
						if distancefield_scale >= 1 {
							// downscale
							let downscale_factor = 1.0 / distancefield_scale as f32;
							let w = (w as f32 * downscale_factor) as u32;
							let h = (h as f32 * downscale_factor) as u32;
							let max_distance =
								distancefield_max_distance as u32 * scale_factor as u32;

							let distance_field = glyph_image.grayscale().distance_field(
								om_fork_distance_field::Options {
									size: (w, h),
									max_distance: max_distance as u16,
									..Default::default()
								},
							);
							glyph_image = image::DynamicImage::new_rgba8(w, h);
							for y in 0..h {
								for x in 0..w {
									let pixel = distance_field.get_pixel(x, y);
									let luma = pixel[0];
									let r = luma;
									let g = luma;
									let b = luma;
									let a = luma;

									let rgba = image::Rgba([r, g, b, a]);
									glyph_image.put_pixel(x, y, rgba);
								}
							}
						}
						self.blit_image(g.x, g.y, &glyph_image);

						break;
					},
				}
			}
		}

		true
	}

	pub fn load(name: &str) -> anyhow::Result<Font> {
		let pngname = format!("{}.png", name);
		let img = image::open(&pngname).unwrap();
		if img.dimensions().0 != img.dimensions().1 {
			println!(
				"Error: Non-square texture for font found with dimensions {:?}",
				img.dimensions()
			);
			anyhow::bail!("Error: Non-square texture for font");
		}

		let texsize = img.dimensions().0;

		let fontname = format!("{}.omfont", name);
		let mut font = Font::new_from_omfont(&fontname, texsize);

		font.image = img;

		//		anyhow::bail!( "Font::load not implemented")
		Ok(font)
	}
	fn save_omfont_v2(&self, filename: &str) -> anyhow::Result<u32> {
		let mut f = match File::create(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};
		f.write_all(&[
			0x4f, 0x4d, 0x46, 0x4f, 0x4e, 0x54, // OMFONT
			0x2, 0x00, 0x00, 0x00, // u32 version
		])
		.unwrap();

		f.write_u16::<LittleEndian>(self.size as u16).unwrap();
		f.write_u16::<LittleEndian>(self.glyphs.len() as u16)
			.unwrap();

		for g in &self.glyphs {
			f.write_u32::<LittleEndian>(g.codepoint as u32).unwrap();
		}

		for g in &self.glyphs {
			let m = g.matrix;
			for mm in &m {
				f.write_f32::<LittleEndian>(*mm).unwrap();
			}
			f.write_f32::<LittleEndian>(g.advance as f32).unwrap();
			f.write_f32::<LittleEndian>(g.y_offset as f32).unwrap();
		}

		Ok(1)
	}

	#[allow(dead_code)]
	fn save_omfont(&self, filename: &str) -> anyhow::Result<u32> {
		let mut f = match File::create(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};
		f.write_all(&[
			0x4f, 0x4d, 0x46, 0x4e, // OMFN
		])
		.unwrap();

		f.write_u32::<LittleEndian>(self.size).unwrap();

		if self.glyphs.len() != 128 {
			println!("Wrong number of glyphs {} expected 128", self.glyphs.len());
			anyhow::bail!("Wrong number of glyphs");
		}

		for g in &self.glyphs {
			let tex_x = (g.x as f32) / (self.texsize as f32);
			let tex_y = (g.y as f32) / (self.texsize as f32);
			let tex_w = (g.width as f32) / (self.texsize as f32);
			let tex_h = (g.height as f32) / (self.texsize as f32);

			//			let _tex_s = self.texsize as f32;

			let v_top = g.height as f32; // t
			let v_bot = 0.0; // t - h

			// upper left
			f.write_f32::<LittleEndian>(tex_x).unwrap();
			f.write_f32::<LittleEndian>(tex_y).unwrap();
			f.write_f32::<LittleEndian>(0.0).unwrap();
			f.write_f32::<LittleEndian>(v_top).unwrap();
			f.write_f32::<LittleEndian>(0.0).unwrap();

			// lower left
			f.write_f32::<LittleEndian>(tex_x).unwrap();
			f.write_f32::<LittleEndian>(tex_y + tex_h).unwrap();
			f.write_f32::<LittleEndian>(0.0).unwrap();
			f.write_f32::<LittleEndian>(v_bot).unwrap();
			f.write_f32::<LittleEndian>(0.0).unwrap();

			// upper right
			f.write_f32::<LittleEndian>(tex_x + tex_w).unwrap();
			f.write_f32::<LittleEndian>(tex_y).unwrap();
			f.write_f32::<LittleEndian>(g.width as f32).unwrap();
			f.write_f32::<LittleEndian>(v_top as f32).unwrap();
			f.write_f32::<LittleEndian>(0.0).unwrap();

			// lower right
			f.write_f32::<LittleEndian>(tex_x + tex_w).unwrap();
			f.write_f32::<LittleEndian>(tex_y + tex_h).unwrap();
			f.write_f32::<LittleEndian>(g.width as f32).unwrap();
			f.write_f32::<LittleEndian>(v_bot).unwrap();
			f.write_f32::<LittleEndian>(0.0).unwrap();
		}

		for g in &self.glyphs {
			// advance
			f.write_u16::<LittleEndian>(g.advance as u16).unwrap();
		}

		Ok(1)
	}

	pub fn create(
		output: &str,
		texsize: u32,
		size: u32,
		border: u32,
		distancefield_scale: u16,
		distancefield_max_distance: u16,
		input: &Vec<&str>,
	) -> anyhow::Result<u32> {
		// load ttf
		// :TODO: load all input fonts!
		let mut f = match File::open(input[0]) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};

		let mut buffer = Vec::new();

		// read the whole file
		f.read_to_end(&mut buffer).unwrap(); //_or_else( anyhow::bail!( "Error reading font file");

		let font = RTFont::try_from_bytes(&buffer[..] as &[u8]).unwrap_or_else(|| {
			panic!("error constructing a Font from bytes");
		});
		/*
		let collection = FontCollection::from_bytes(&buffer[..] as &[u8]).unwrap_or_else(|e| {
			panic!("error constructing a FontCollection from bytes: {}", e);
		});

		let font = collection
			.into_font() // only succeeds if collection consists of one font
			.unwrap_or_else(|e| {
				panic!("error turning FontCollection into a Font: {}", e);
			});
		*/
		let scale = Scale::uniform(size as f32); // :TODO: rusttype's understanding of size is different to our's!
		let start = point(0.0, 0.0 /*+ v_metrics.ascent*/);

		let mut the_font = Font::new(texsize, size, border);
		let mut cnt = 0;
		for c in 0..128u8 {
			cnt += 1;
			let codepoint = c;
			//			let codepoint = 0x30;	// :HACK:
			//			let g = font.glyph( codepoint as char );
			//			let data = g.get_data();
			//			println!("{:?} -> {:#?}", c, data );

			// :HACK: :TODO: rasterize after positioning into final image
			let ch = format!("{}#", codepoint as char);
			//			println!("ch: >{:?}<", ch );
			let layout = font.layout(&ch, scale, start);
			let mut maybe_glyph: Option<Glyph> = None;
			for pg in layout {
				let pos = pg.position();
				match pg.pixel_bounding_box() {
					None => {
						let glyph = Glyph::new(codepoint, 0, 0);
						the_font.add_glyph(glyph);
					},
					Some(bb) => {
						//						println!("bb {:?}", bb );
						match maybe_glyph {
							None => {
								//								println!("{} -> {:?}", ch, bb );
								let h = bb.height() as u32 + 2 * border;
								let w = bb.width() as u32 + 2 * border;
								let mut glyph = Glyph::new(codepoint, w, h);
								let y_offset = bb.max.y as f32;
								//								println!("\t{} -> {}", ch, y_offset );
								glyph.y_offset = y_offset / texsize as f32;
								maybe_glyph = Some(glyph);
							},
							Some(mut glyph) => {
								// second character!
								//								println!("{} -> {:?}", ch, bb );
								//								println!("Pos {:?}", pos.x );
								// :TODO: use advance_width
								glyph.advance = pos.x as u16;
								//								let y_offset = 0.5 * glyph.height as f32;
								the_font.add_glyph(glyph);
								break;
							},
						}
					},
				}
			}
		}
		println!("CNT {:?}", cnt);

		if !the_font.fit_glyphs() {
			anyhow::bail!("Failed to fit glyphs into texture");
		}
		if !the_font.blit_glyphs(font, distancefield_scale, distancefield_max_distance) {
			anyhow::bail!("Failed to blitting glyphs into texture");
		}
		the_font.recalc_matrix(texsize);
		//		println!("the font: {:#?}", the_font );

		let filename = format!("{}.png", output);
		println!("Writing texture to {}", filename);
		the_font
			.image
			.save_with_format(filename, ImageFormat::Png)
			.unwrap();

		let filename = format!("{}.omfont", output);
		println!("Writing font data to {}", filename);
		//		match the_font.save_omfont( &filename ) {
		match the_font.save_omfont_v2(&filename) {
			Ok(_) => {},
			Err(e) => {
				println!("Error writing font data {:?}", e);
			},
		}

		Ok(0)
	}
}
