use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::atlas::Atlas;
use crate::atlas::AtlasEntry;
use crate::atlas::AtlasFitter;

#[derive(Debug, Default)]
pub struct AtlasSet {
	border:         u32,
	output:         Option<PathBuf>,
	target_size:    Option<u32>,
	maximum_size:   Option<u32>,
	reference_path: Option<PathBuf>,
	inputs:         Vec<PathBuf>,
	atlases:        Vec<Atlas>,
}

impl AtlasSet {
	pub fn refit(&mut self) -> anyhow::Result<u32> {
		let mut atlases = Vec::new();

		let mut entries = Vec::new();
		for i in self.inputs.iter() {
			println!("Analysing {:?}", i);
			let img = image::open(i).unwrap();

			let i_os_string = i.clone().into_os_string();
			let i_string = match i_os_string.to_str() {
				Some(s) => s,
				None => {
					anyhow::bail!("Error converting path to string")
					// panic!("Error converting path to string")
				},
			};

			let mut e = AtlasEntry::new(i_string, 0, 0);
			e.set_image(img);
			entries.push(e);
		}

		// sort entries by size
		entries.sort_by(
			|a, b| b.height.cmp(&a.height), // higher ones first
		);

		//let mut atlases: Vec<Atlas> = Vec::new();

		// something that takes a list of entries, and return a list of pages with those entries

		let mut atlas_fitter = AtlasFitter::new();

		for (idx, e) in entries.iter().enumerate() {
			atlas_fitter.add_entry(idx, e.width, e.height);
		}

		//		println!("atlas_fitter {:#?}", atlas_fitter);

		let size = self
			.target_size
			.expect("Pass size or use autofit() with maximum_size"); // :TODO: auto fit
		let pages = atlas_fitter.fit(size, self.border);
		//		println!("pages {:#?}", pages);

		// create atlases
		for p in &pages {
			let mut a = Atlas::new(size, self.border);
			for e in &p.entries {
				println!("{:#?}", e);
				let entry = &entries[e.id];
				let mut entry = entry.clone();
				entry.set_position(e.x, e.y);
				println!("{:#?}", entry);
				a.add_entry(entry);
			}
			//a.blit_entries(); // defer blitting
			atlases.push(a);
		}

		self.atlases = atlases;
		Ok(self.atlases.len() as u32)
	}

	pub fn autosize(&mut self) -> anyhow::Result<u32> {
		// brute force for now
		let mut size = 2;
		let maximum_size = self.maximum_size.unwrap_or(65536); // :TODO: decide on an actually reasonable limit
		let mut n;
		loop {
			self.target_size = Some(size);
			n = self.refit()?;
			if n == 1 {
				println!("✅ Using single atlas at size: {}", size);
				break;
			}
			let next_size = size * 2;
			if next_size > maximum_size {
				println!("🔶 Using maximum size: {}", maximum_size);
				break;
			}
			size = next_size;
		}

		self.target_size = Some(size);

		Ok(n)
	}

	pub fn save(&mut self, output: &Path, reference_path: Option<&Path>) -> anyhow::Result<u32> {
		for a in self.atlases.iter_mut() {
			a.blit_entries();
		}

		// write outputs
		let mut n = 0;
		let output_os_string = output.as_os_str();
		let output_string = match output_os_string.to_str() {
			Some(s) => s,
			None => anyhow::bail!("Error converting path to string"),
		};
		for a in self.atlases.iter() {
			//			println!("Atlas #{} {:?}", n, a );
			let outname = crate::atlas::atlas::simple_format_u32(&output_string, n); //format!(output, n);
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
			if let Some(rp) = &reference_path {
				let atlas_stem = Path::new(&atlasname)
					.file_stem()
					.unwrap()
					.to_str()
					.unwrap()
					.to_string();
				println!("Writing references for {} to {}", &atlas_stem, rp.display());
				for e in a.entries.iter() {
					//dbg!(&e);
					let stem = e.get_stem();
					//println!("{} in {}", &stem, &atlas_stem);
					let mut omtr = PathBuf::new();
					omtr.push(&rp);
					omtr.push(&stem);
					omtr.set_extension(&"omtr");
					//dbg!(&omtr);
					let mut f = match File::create(omtr) {
						Ok(f) => f,
						Err(_) => anyhow::bail!("io"),
					};
					write!(f, "{}\n", &atlas_stem).unwrap();
				}
			}
			n += 1;
		}

		Ok(n)
	}
	pub fn with_output(mut self, output: &Path) -> Self {
		self.output = Some(output.to_path_buf());
		self
	}
	pub fn with_target_size(mut self, target_size: u32) -> Self {
		self.target_size = Some(target_size);
		self
	}
	pub fn target_size(&self) -> &Option<u32> {
		&self.target_size
	}
	pub fn with_border(mut self, border: u32) -> Self {
		self.border = border;
		self
	}
	pub fn with_reference_path(mut self, reference_path: &Path) -> Self {
		self.reference_path = Some(reference_path.to_path_buf());
		self
	}
	pub fn with_inputs(mut self, inputs: Vec<&Path>) -> Self {
		for input in inputs.iter() {
			// println!("{:?}", &input);
			self.inputs.push(input.to_path_buf());
		}
		self
	}
	pub fn with_maximum_size(mut self, maximum_size: u32) -> Self {
		self.maximum_size = Some(maximum_size);
		self
	}
}
