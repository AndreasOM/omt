use crate::util::OmError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use image::{ DynamicImage, ImageFormat, GenericImage, GenericImageView };
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(Clone)]
pub struct Entry{
	filename:	String,
	pub image:		Option<DynamicImage>,
	pub x:			u32,
	pub y:			u32,
	pub width:		u32,
	pub height:		u32,
}

impl std::fmt::Debug for Entry {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::result::Result<(), std::fmt::Error > {
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
	fn new( filename: &str, width: u32, height: u32 ) -> Entry {
		Entry{
			filename: 	filename.to_string(),
			image: 		None,
			x:			0,
			y:			0,
			width: 		width,
			height: 	height,
		}
	}

	fn set_image( &mut self, image: DynamicImage ) {
		self.width  = image.dimensions().0;
		self.height = image.dimensions().1;
		self.image = Some( image );
	}

	fn set_position( &mut self, x: u32, y: u32 ) {
		self.x = x;
		self.y = y;
	}
	fn get_basename( &self ) -> String {
		let basename = Path::new(&self.filename).file_name().unwrap().to_str().unwrap();
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
	fn get_matrix( &self, size: u32 ) -> [f32;6] {
		// :TODO: cleanup please
		let sx = self.x as f32 / size as f32;
		let sy = self.y as f32 / size as f32;
//		let ex = ( self.x + self.width ) as f32 / size as f32;
//		let ey = ( self.y + self.height ) as f32 / size as f32;
		let scale_x = self.width as f32 / size as f32; //ex - sx;
		let scale_y = self.height as f32 / size as f32; //ey - sy;
		[
			scale_x, 0.0, sx,
			0.0, scale_y, sy,
		]
	}	
}

fn simple_format_u32( f: &str, n: u32 ) -> String {
	let s = f.clone();
	let re = Regex::new(r"(%d)").unwrap();

//	println!("simple_format_u32 {:?} with {:?}", s, re );
	let s = re.replace_all(
		&s,
		|c: &regex::Captures| {
			let placeholder = c.get(1).map_or( "", |m| m.as_str() );
//			println!("Found {:?}", placeholder );
			match placeholder {
				"" => "".to_string(),
				"%d" => n.to_string(),
				x => {
					println!("simple_format_u32 got {:?}", x);
					x.to_string()
				},
			}
		}
	);

	s.to_string()
}
#[derive(Debug)]
struct Row {
	y: u32,			// start of row
	width: u32,
	height: u32,
	end_x: u32, 	// current end of row
}

impl Row {
	fn new( y: u32, width: u32, height: u32 ) -> Row {
		Row {
			y: y,
			width: width,
			height: height,
			end_x: 0
		}
	}

	fn would_fit( &self, w: u32, h: u32 ) -> bool {
		if self.height >= h {
			let available_space = self.width - self.end_x;
			if available_space >= w {
				true
			} else {
				// not enough space
//				println!("Row {:?} not enough space for {:?}", self, w );
				false
			}

		} else {
			// not high enough
//			println!("Row {:?} not high enough for {:?}", self, h );
			false
		}
	}
}


//#[derive()]
pub struct Atlas {
	size: u32,
	border: u32,
	pub entries: Vec<Entry>,
	pub image: Option<DynamicImage>,
	rows: Vec<Row>,
	used_height: u32,
	atlas_filename: Option<String>,
	image_filename: Option<String>,
}

impl std::fmt::Debug for Atlas {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::result::Result<(), std::fmt::Error > {
		f.debug_struct("Atlas")
			.field("size", &self.size)
			.field("border", &self.border)
			.field("entries", &self.entries)
//			.field("image", if self.image.is_some() {"YES"} else {"NO"} )
			.finish()
	}
}

impl Atlas {
	fn new( size: u32, border: u32 ) -> Atlas {
		Atlas {
			size: size,
			border: border,
			entries: Vec::new(),
			image: Some( image::DynamicImage::new_rgba8(size, size) ),
			rows: Vec::new(),
			used_height: 0,
			atlas_filename: None,
			image_filename: None,
		}
	}

	fn new_from_atlas( atlasname: &str, size: u32 ) -> Atlas {
		let mut a = Atlas {
			size: size,
			border: 0,
			entries: Vec::new(),
			image: None,
			rows: Vec::new(),
			used_height: 0,
			atlas_filename: Some( atlasname.to_string() ),
			image_filename: None,
		};

		match a.load_atlas( &atlasname, a.size ) {
			Err( e ) => println!("{:?}", e ),
			_ => {},
		};

		a
	}

	fn blit( dest: &mut image::DynamicImage, source: &image::DynamicImage, start_x: u32, start_y: u32 ) {
		let w = source.dimensions().0;
		let h = source.dimensions().1;

		for y in 0..h {
			for x in 0..w {
				let dx = start_x + x;
				let dy = start_y + y;

				let pixel = source.get_pixel( x, y );
				dest.put_pixel( dx, dy, pixel );
			}
		}
	}
	fn add_row( &mut self, height: u32 ) -> Option<usize> {
		if height <= ( self.size - self.used_height ) {
			let row = Row::new( self.used_height, self.size, height );
			self.used_height += height;
			let row_index = self.rows.len();
//			println!("Created row #{:?} at {:?}. {:?} used now.", row_index, row.y, self.used_height );
			self.rows.push( row );
			Some( row_index )
		} else {
//			println!("Can not create row with {:?} height, {:?} used of {:?}", height, self.used_height, self.size );
			None
		}
	}
	fn add_entry_to_row_with_index( &mut self, entry: &Entry, row_index: usize ) -> bool {
		match self.rows.get_mut( row_index ) {
			None => false,	// give up, should never happen
			Some( row ) => {
//				println!("Got row {:?}", row );
				if row.would_fit( entry.width, entry.height  ) {
					// add it
					let mut e = entry.clone();
					// blitting
					let x = row.end_x;
					let y = row.y;
					match &mut self.image {
						None => {},
						Some( di ) => {
							Atlas::blit( di, &e.image.unwrap(), x, y );
						},
					}
					row.end_x += e.width;
					e.image = None;	// cleanup, data not needed anymore
					e.set_position( x, y );
					self.entries.push(
						e
					);
					true
				} else {
//					println!("Row {:?} would not fit {:?}", row, entry );
					false
				}
			}
		}
	}
	/* would prefer pass through with ownership transfer and return, but need more rust knowledge
	fn add_entry( &mut self, entry: &Entry ) -> Result<usize, Entry> {
		Err( *entry )
	*/
	fn add_entry( &mut self, entry: &Entry ) -> bool {
		let h = entry.height;

		if self.size < entry.width || self.size < entry.height {
			false
		} else {
			// find row
			let mut candidates = Vec::new();

			for ri in 0..self.rows.len() {
				let r = &self.rows[ ri ];
				if r.would_fit( entry.width, entry.height ) {
//					println!("Row {:?} would fit {:?}", r, entry );
					if r.height < 2*entry.height {	// do not waste too much space, "2" is purely guessed
						candidates.push( ri );
					}
				}
			}

			if candidates.len() > 0 {
				// find best candidate
				let best_candidate_index = 0;	// :TODO: actually find best candidate
				/*
				for ci in 0..candidates.len() {
					//
				}
				*/
//				println!("Got candidate rows. Using best one {:?}", candidates[ best_candidate_index ] );
				self.add_entry_to_row_with_index( entry, candidates[ best_candidate_index ] )
			} else {
				// or create new row
//				println!("No candidate row found creating new one. {:?}", self);
				match self.add_row( h ) {
					None				=> false,													// give up
					Some( row_index )	=> self.add_entry_to_row_with_index( entry, row_index ),
				}
			}
		} 
	}

	fn save_png( &self, filename: &str ) -> Result< u32, OmError > {
//		Err( OmError::NotImplemented("Atlas::save_png".to_string()))
		match self.image.as_ref().unwrap().save_with_format(filename, ImageFormat::PNG) {
			_ => Ok( 0 )
		}
	}

	fn load_atlas( &mut self, filename: &str, size: u32 ) -> Result< u32, OmError > {
		let f = match File::open(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};

		let mut bufreader = BufReader::new( f );
		let magic = match bufreader.read_u16::<LittleEndian>() { //.unwrap_or( 0xffff );
			Ok( m ) => m,
			x => {
				println!("{:?}", x);
				return Err(OmError::Generic("reading from buffer".to_string()))
			},
		};
		if magic != 0x4f53 {
			println!("Got magic {:?} from {:?}", magic, bufreader);
			return Err( OmError::Generic("Broken file magic".to_string() ) );
		}
		let v = bufreader.read_u16::<LittleEndian>().unwrap_or( 0 );
		if v != 1 {
			return Err( OmError::Generic("Wrong version".to_string() ) );
		}
		let chunk_magic = [ 0x4fu8, 0x4d, 0x41, 0x54, 0x4c, 0x41, 0x53, ];
		for m in &chunk_magic {
			let b = bufreader.read_u8().unwrap_or( 0 );
			if b != *m {
				return Err( OmError::Generic("Broken chunk magic".to_string() ) );
			}
		}
		let flags = bufreader.read_u8().unwrap_or( 0 );
		if flags != 'S' as u8 {
			return Err( OmError::Generic( ":TODO: compression not implemented".to_string() ) );
		}
		let chunk_version = [ 0x01u8, 0x00, 0x00, 0x00 ];
		for m in &chunk_version {
			let b = bufreader.read_u8().unwrap_or( 0 );
			if b != *m {
				return Err( OmError::Generic("Broken chunk version".to_string() ) );
			}
		}
		let entry_count = bufreader.read_u16::<LittleEndian>( ).unwrap_or( 0 );

		println!("Got {:?} entries", entry_count );

		for _ei in 0..entry_count {
			let mut name_buffer = [0u8;128];
			bufreader.read_exact(&mut name_buffer).unwrap();
			let mut name = String::from_utf8(name_buffer.to_vec()).unwrap();
			let first_zero = name.find( "\u{0}" ).unwrap_or( name.len() );
			name.truncate( first_zero );
			let mut matrix_buffer = [0f32;6];
			for m in &mut matrix_buffer {
				*m = bufreader.read_f32::<LittleEndian>().unwrap_or( 0.0 );
			}

			let w = ( matrix_buffer[ 0*3 + 0 ] * size as f32 ).trunc() as u32;
			let h = ( matrix_buffer[ 1*3 + 1 ] * size as f32 ).trunc() as u32;
			let mut e = Entry::new( &name, w, h );
			let x = ( matrix_buffer[ 0*3 + 2 ] * size as f32 ).trunc() as u32;
			let y = ( matrix_buffer[ 1*3 + 2 ] * size as f32 ).trunc() as u32;
			e.set_position( x, y );
			self.entries.push( e );
		}
		Ok( 0 )
	}

	// :TODO: support compression
	fn save_atlas( &self, filename: &str ) -> Result< u32, OmError > {
		let mut f = match File::create(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};
		f.write_u16::<LittleEndian>( 0x4f53 ).unwrap();
		f.write_u16::<LittleEndian>( 0x0001 ).unwrap();
		let compress = 'S';
		f.write_all(&[
			0x4f, 0x4d, 0x41, 0x54, 0x4c, 0x41, 0x53,	// OMATLAS
			compress as u8,
			0x01, 0x00, 0x00, 0x00,

		]).unwrap();
		f.write_u16::<LittleEndian>( self.entries.len() as u16 ).unwrap();
		for e in &self.entries {
			let n = e.get_basename();
			let mut c = 0;
			for nn in n.as_bytes() {
				f.write_u8( *nn ).unwrap();
				c += 1;
			}
			while c < 128 {
				f.write_u8( 0 ).unwrap();
				c += 1;
			}
			let m = e.get_matrix( self.size );
//			println!("Matrix {:?}", m );
			for mm in &m {
				f.write_f32::<LittleEndian>( *mm ).unwrap();
			}
		}
		Ok( 0 )
	}

	fn save_map( &self, filename: &str ) -> Result< u32, OmError > {
		let mut f = match File::create(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};

//		println!("{:?}", self );

		for e in &self.entries {
//			println!("{:?}", e );
			// overlay-00-title-square.png:0,0-2048,1536
			let basename = e.get_basename();
			let l = format!("{}:{},{}-{},{}\n", basename, e.x, e.y, e.x+e.width, e.y+e.height);
//			println!("{}", l);
			write!( f, "{}", l ).unwrap();
		}
		Ok( 0 )
	}

	pub fn hello() {
		println!("Atlas::hello()");
	}

	pub fn all_for_template( input: &str ) -> Vec< Atlas > {
		let mut result = Vec::new();
		let mut n = 0;
		loop {
			let inname = simple_format_u32( input, n ); //format!(output, n);
			let atlasname = format!("{}.atlas", inname );
			let pngname = format!("{}.png", inname );
			if !Path::new( &atlasname ).exists() {
				println!("{:?} not found. Stopping", atlasname );
				break;
			}
			if !Path::new( &pngname ).exists() {
				println!("{:?} not found. Stopping", pngname );
				break;
			}
			// load image, to get the size
			let img = image::open(&pngname).unwrap();
			if img.dimensions().0 != img.dimensions().1 {
				println!("Error: Non-square texture atlas found with dimensions {:?}", img.dimensions());
			}

			let size = img.dimensions().0;

			let mut a = Atlas::new_from_atlas( &atlasname, size );
			a.image_filename = Some( pngname.to_string() );
			a.image = Some( img );

			result.push( a );
			n += 1;
		}
		result
	}

	pub fn info(
		input: &str
	) -> Result<u32,OmError>{
		let atlases = Atlas::all_for_template( &input );

		for a in &atlases {
//			println!("Atlas {} {}", atlasname, pngname);
			match ( &a.atlas_filename, &a.image_filename ) {
				( None, None ) => {},
				( Some( n ), None ) => {
					println!("Atlas {}", n);
				},
				( Some( a ), Some( i ) ) => {
					println!("Atlas {} {}", a, i);
				},
				( None, Some( i ) ) => {
					println!("Atlas Image: {}", i);
				},
			}
			println!("\tSize  : {}", a.size);
			println!("\tBorder: {}", a.border);
			for e in &a.entries {
				println!("\t\t{:>5} x {:>5}  @  {:>5},{:>5}   | {}", e.width, e.height, e.x, e.y, e.filename );
			}
		}

		let n = atlases.len() as u32;
		if n == 0 {
			Err(OmError::Generic("No matching atlas found.".to_string()))
		} else {
			Ok( n )	
		}
	}

	pub fn combine(
		output: &str, size: u32, border: u32, input: &Vec<&str> 
	) -> Result<u32, OmError>{
		let mut entries = Vec::new();
		// collect inputs
		for i in input {
//			println!("Analysing {:?}", i );
			let img = image::open(i).unwrap();

			let mut e = Entry::new( i, 0, 0 );
			e.set_image( img );
			entries.push(e);
		}

		// sort entries by size
		entries.sort_by( |a, b|
			b.height.cmp( &a.height )	// higher ones first
		);


		let mut atlases: Vec<Atlas> = Vec::new();

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
					return Err(OmError::Generic("‼️ Image doesn't fit into empty atlas".to_string()));
				}
				atlases.push( a );				
			}
		}

		// write outputs
		let mut n = 0;
		for a in atlases {
//			println!("Atlas #{} {:?}", n, a );
			let outname = simple_format_u32( output, n ); //format!(output, n);
			let pngname = format!("{}.png", outname );
			let atlasname = format!("{}.atlas", outname );
			let mapname = format!("{}.map", outname );
			match a.save_png( &pngname ) {
				Ok( _bytes_written ) => {
//					println!("{:?} bytes written to image {}", bytes_written, pngname );
				},
				Err( e ) => {
					return Err( e );
				}
			}
			match a.save_atlas( &atlasname ) {
				Ok( _bytes_written ) => {
//					println!("{:?} bytes written to atlas {}", bytes_written, atlasname );
				},
				Err( e ) => {
					return Err( e );
				}
			}
			match a.save_map( &mapname ) {
				Ok( _bytes_written ) => {
//					println!("{:?} bytes written to map {}", bytes_written, atlasname );
				},
				Err( e ) => {
					return Err( e );
				}
			}
			n += 1;
		}

		Ok( n )
	}
}
