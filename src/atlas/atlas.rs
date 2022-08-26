use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat};
use regex::Regex;

use crate::atlas::AtlasFitter;

#[derive(Clone)]
pub struct Entry {
	filename:   String,
	pub image:  Option<DynamicImage>,
	pub x:      u32,
	pub y:      u32,
	pub width:  u32,
	pub height: u32,
}

impl std::fmt::Debug for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		f.debug_struct("Entry")
			.field("filename", &self.filename)
			//			.field("image", if self.image.is_some() {"YES"} else {"NO"} )
			.field("x", &self.x)
			.field("y", &self.y)
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}
impl Entry {
	fn new(filename: &str, width: u32, height: u32) -> Entry {
		Entry {
			filename: filename.to_string(),
			image:    None,
			x:        0,
			y:        0,
			width:    width,
			height:   height,
		}
	}

	fn set_image(&mut self, image: DynamicImage) {
		self.width = image.dimensions().0;
		self.height = image.dimensions().1;
		self.image = Some(image);
	}

	fn set_position(&mut self, x: u32, y: u32) {
		self.x = x;
		self.y = y;
	}
	fn get_basename(&self) -> String {
		let basename = Path::new(&self.filename)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap();
		basename.to_string()
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
	fn get_matrix(&self, size: u32) -> [f32; 6] {
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

fn simple_format_u32(f: &str, n: u32) -> String {
	let s = f.clone();
	let re = Regex::new(r"(%d)").unwrap();

	//	println!("simple_format_u32 {:?} with {:?}", s, re );
	let s = re.replace_all(&s, |c: &regex::Captures| {
		let placeholder = c.get(1).map_or("", |m| m.as_str());
		//			println!("Found {:?}", placeholder );
		match placeholder {
			"" => "".to_string(),
			"%d" => n.to_string(),
			x => {
				println!("simple_format_u32 got {:?}", x);
				x.to_string()
			},
		}
	});

	s.to_string()
}

//#[derive()]
pub struct Atlas {
	size:           u32,
	border:         u32,
	pub entries:    Vec<Entry>,
	pub image:      Option<DynamicImage>,
	atlas_filename: Option<String>,
	image_filename: Option<String>,
}

impl std::fmt::Debug for Atlas {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		f.debug_struct("Atlas")
			.field("size", &self.size)
			.field("border", &self.border)
			.field("entries", &self.entries)
			//			.field("image", if self.image.is_some() {"YES"} else {"NO"} )
			.finish()
	}
}

impl Atlas {
	fn new(size: u32, border: u32) -> Atlas {
		Atlas {
			size:           size,
			border:         border,
			entries:        Vec::new(),
			image:          Some(image::DynamicImage::new_rgba8(size, size)),
			atlas_filename: None,
			image_filename: None,
		}
	}

	pub fn add_entry(&mut self, entry: Entry) {
		self.entries.push(entry);
	}

	pub fn blit_entries(&mut self) {
		match &mut self.image {
			None => {},
			Some(di) => {
				for entry in &self.entries {
					match &entry.image {
						None => {},
						Some(image) => {
							Atlas::blit(di, &image, entry.x, entry.y);
						},
					}
				}
			},
		}
	}

	fn new_from_atlas(atlasname: &str, size: u32) -> Atlas {
		let mut a = Atlas {
			size:           size,
			border:         0,
			entries:        Vec::new(),
			image:          None,
			atlas_filename: Some(atlasname.to_string()),
			image_filename: None,
		};

		match a.load_atlas(&atlasname, a.size) {
			Err(e) => println!("{:?}", e),
			_ => {},
		};

		a
	}

	fn blit(
		dest: &mut image::DynamicImage,
		source: &image::DynamicImage,
		start_x: u32,
		start_y: u32,
	) {
		let w = source.dimensions().0;
		let h = source.dimensions().1;

		for y in 0..h {
			for x in 0..w {
				let dx = start_x + x;
				let dy = start_y + y;

				let pixel = source.get_pixel(x, y);
				dest.put_pixel(dx, dy, pixel);
			}
		}
	}

	fn save_png(&self, filename: &str) -> anyhow::Result<()> {
		match self
			.image
			.as_ref()
			.unwrap()
			.save_with_format(filename, ImageFormat::Png)
		{
			_ => Ok(()),
		}
	}

	fn load_atlas(&mut self, filename: &str, size: u32) -> anyhow::Result<()> {
		let f = match File::open(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};

		let mut bufreader = BufReader::new(f);
		let magic = match bufreader.read_u16::<LittleEndian>() {
			//.unwrap_or( 0xffff );
			Ok(m) => m,
			x => {
				println!("{:?}", x);
				anyhow::bail!("reading from buffer");
			},
		};
		if magic != 0x4f53 {
			println!("Got magic {:?} from {:?}", magic, bufreader);
			anyhow::bail!("Broken file magic");
		}
		let v = bufreader.read_u16::<LittleEndian>().unwrap_or(0);
		if v != 1 {
			anyhow::bail!("Wrong version");
		}
		let chunk_magic = [0x4fu8, 0x4d, 0x41, 0x54, 0x4c, 0x41, 0x53];
		for m in &chunk_magic {
			let b = bufreader.read_u8().unwrap_or(0);
			if b != *m {
				anyhow::bail!("Broken chunk magic");
			}
		}
		let flags = bufreader.read_u8().unwrap_or(0);
		if flags != 'S' as u8 {
			anyhow::bail!(":TODO: compression not implemented");
		}
		let chunk_version = [0x01u8, 0x00, 0x00, 0x00];
		for m in &chunk_version {
			let b = bufreader.read_u8().unwrap_or(0);
			if b != *m {
				anyhow::bail!("Broken chunk version");
			}
		}
		let entry_count = bufreader.read_u16::<LittleEndian>().unwrap_or(0);

		println!("Got {:?} entries", entry_count);

		for _ei in 0..entry_count {
			let mut name_buffer = [0u8; 128];
			bufreader.read_exact(&mut name_buffer).unwrap();
			let mut name = String::from_utf8(name_buffer.to_vec()).unwrap();
			let first_zero = name.find("\u{0}").unwrap_or(name.len());
			name.truncate(first_zero);
			let mut matrix_buffer = [0f32; 6];
			for m in &mut matrix_buffer {
				*m = bufreader.read_f32::<LittleEndian>().unwrap_or(0.0);
			}

			let w = (matrix_buffer[0 * 3 + 0] * size as f32).trunc() as u32;
			let h = (matrix_buffer[1 * 3 + 1] * size as f32).trunc() as u32;
			let mut e = Entry::new(&name, w, h);
			let x = (matrix_buffer[0 * 3 + 2] * size as f32).trunc() as u32;
			let y = (matrix_buffer[1 * 3 + 2] * size as f32).trunc() as u32;
			e.set_position(x, y);
			self.entries.push(e);
		}
		Ok(())
	}

	// :TODO: support compression
	fn save_atlas(&self, filename: &str) -> anyhow::Result<()> {
		let mut f = match File::create(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};
		f.write_u16::<LittleEndian>(0x4f53).unwrap();
		f.write_u16::<LittleEndian>(0x0001).unwrap();
		let compress = 'S';
		f.write_all(&[
			0x4f,
			0x4d,
			0x41,
			0x54,
			0x4c,
			0x41,
			0x53, // OMATLAS
			compress as u8,
			0x01,
			0x00,
			0x00,
			0x00,
		])
		.unwrap();
		f.write_u16::<LittleEndian>(self.entries.len() as u16)
			.unwrap();
		for e in &self.entries {
			let n = e.get_basename();
			let mut c = 0;
			for nn in n.as_bytes() {
				f.write_u8(*nn).unwrap();
				c += 1;
			}
			while c < 128 {
				f.write_u8(0).unwrap();
				c += 1;
			}
			let m = e.get_matrix(self.size);
			//			println!("Matrix {:?}", m );
			for mm in &m {
				f.write_f32::<LittleEndian>(*mm).unwrap();
			}
		}
		Ok(())
	}

	fn save_map(&self, filename: &str) -> anyhow::Result<()> {
		let mut f = match File::create(filename) {
			Ok(f) => f,
			Err(_) => anyhow::bail!("io"),
		};

		//		println!("{:?}", self );

		for e in &self.entries {
			//			println!("{:?}", e );
			// overlay-00-title-square.png:0,0-2048,1536
			let basename = e.get_basename();
			let l = format!(
				"{}:{},{}-{},{}\n",
				basename,
				e.x,
				e.y,
				e.x + e.width,
				e.y + e.height
			);
			//			println!("{}", l);
			write!(f, "{}", l).unwrap();
		}
		Ok(())
	}

	pub fn hello() {
		println!("Atlas::hello()");
	}

	pub fn all_for_template(input: &str) -> Vec<Atlas> {
		let mut result = Vec::new();
		let mut n = 0;
		loop {
			let inname = simple_format_u32(input, n); //format!(output, n);
			let atlasname = format!("{}.atlas", inname);
			let pngname = format!("{}.png", inname);
			if !Path::new(&atlasname).exists() {
				println!("{:?} not found. Stopping", atlasname);
				break;
			}
			if !Path::new(&pngname).exists() {
				println!("{:?} not found. Stopping", pngname);
				break;
			}
			// load image, to get the size
			let img = image::open(&pngname).unwrap();
			if img.dimensions().0 != img.dimensions().1 {
				println!(
					"Error: Non-square texture atlas found with dimensions {:?}",
					img.dimensions()
				);
			}

			let size = img.dimensions().0;

			let mut a = Atlas::new_from_atlas(&atlasname, size);
			a.image_filename = Some(pngname.to_string());
			a.image = Some(img);

			result.push(a);
			n += 1;
		}
		result
	}

	pub fn info(input: &str) -> anyhow::Result<u32> {
		let atlases = Atlas::all_for_template(&input);

		for a in &atlases {
			//			println!("Atlas {} {}", atlasname, pngname);
			match (&a.atlas_filename, &a.image_filename) {
				(None, None) => {},
				(Some(n), None) => {
					println!("Atlas {}", n);
				},
				(Some(a), Some(i)) => {
					println!("Atlas {} {}", a, i);
				},
				(None, Some(i)) => {
					println!("Atlas Image: {}", i);
				},
			}
			println!("\tSize  : {}", a.size);
			println!("\tBorder: {}", a.border);
			for e in &a.entries {
				println!(
					"\t\t{:>5} x {:>5}  @  {:>5},{:>5}   | {}",
					e.width, e.height, e.x, e.y, e.filename
				);
			}
		}

		let n = atlases.len() as u32;
		if n == 0 {
			anyhow::bail!("No matching atlas found.");
		} else {
			Ok(n)
		}
	}

	pub fn combine(output: &str, size: u32, border: u32, input: &Vec<&str>) -> anyhow::Result<u32> {
		let mut entries = Vec::new();
		// collect inputs
		for i in input {
			println!("Analysing {:?}", i);
			let img = image::open(i).unwrap();

			let mut e = Entry::new(i, 0, 0);
			e.set_image(img);
			entries.push(e);
		}

		// sort entries by size
		entries.sort_by(
			|a, b| b.height.cmp(&a.height), // higher ones first
		);

		let mut atlases: Vec<Atlas> = Vec::new();

		// something that takes a list of entries, and return a list of pages with those entries

		let mut atlas_fitter = AtlasFitter::new();

		for (idx, e) in entries.iter().enumerate() {
			atlas_fitter.add_entry(idx, e.width, e.height);
		}

		//		println!("atlas_fitter {:#?}", atlas_fitter);

		let pages = atlas_fitter.fit(size, border);
		//		println!("pages {:#?}", pages);

		// create atlases
		for p in &pages {
			let mut a = Atlas::new(size, border);
			for e in &p.entries {
				println!("{:#?}", e);
				let entry = &entries[e.id];
				let mut entry = entry.clone();
				entry.set_position(e.x, e.y);
				println!("{:#?}", entry);
				a.add_entry(entry);
			}
			a.blit_entries();
			atlases.push(a);
		}
		/*
		// combine outputs
		for e in entries.drain(..) {
			let mut did_fit = false;
			for a in &mut atlases {
				if a.add_entry( &e) {
					did_fit = true;
					break;
				}
			}
			if !did_fit {
				let mut a = Atlas::new( size, border );
				if !a.add_entry( &e ) {
					println!("‼️ Image doesn't fit into empty atlas {:?}", e );
					anyhow::bail!("‼️ Image doesn't fit into empty atlas");
				}
				atlases.push( a );
			}
		}
		*/
		// write outputs
		let mut n = 0;
		for a in atlases {
			//			println!("Atlas #{} {:?}", n, a );
			let outname = simple_format_u32(output, n); //format!(output, n);
			let pngname = format!("{}.png", outname);
			let atlasname = format!("{}.atlas", outname);
			let mapname = format!("{}.map", outname);
			match a.save_png(&pngname) {
				Ok(_bytes_written) => {
					//					println!("{:?} bytes written to image {}", bytes_written, pngname );
				},
				Err(e) => {
					println!("Error writing .png to {}", &pngname);
					return Err(e);
				},
			}
			match a.save_atlas(&atlasname) {
				Ok(_bytes_written) => {
					//					println!("{:?} bytes written to atlas {}", bytes_written, atlasname );
				},
				Err(e) => {
					println!("Error writing .atlas to {}", &atlasname);
					return Err(e);
				},
			}
			match a.save_map(&mapname) {
				Ok(_bytes_written) => {
					//					println!("{:?} bytes written to map {}", bytes_written, atlasname );
				},
				Err(e) => {
					println!("Error writing .map to {}", &mapname);
					return Err(e);
				},
			}
			n += 1;
		}

		Ok(n)
	}
}
