use crate::name_map::NameMap;
use crate::packer::command_packer::CommandPacker;
use crate::packer::packer::Archive;
use crate::packer::packer::Helper;

#[derive(Debug, Default)]
pub struct CommandPackerPack {
	output:     Option<String>,
	basepath:   Option<String>,
	paklist:    Option<String>,
	name_map:   Option<String>,
	names_only: bool,
}

impl CommandPackerPack {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
}

impl CommandPacker for CommandPackerPack {
	fn run(&mut self) -> anyhow::Result<u32> {
		let name_map = if let Some(name_map_file) = &self.name_map {
			let nm = NameMap::load_or_create(&name_map_file)?;
			println!("Loaded (or created) NameMap:\n{}", nm);
			Some(nm)
		} else {
			None
		};
		let mut archive = Archive::create(&String::new());
		archive.give_name_map(name_map);

		if let Some(paklist) = &self.paklist {
			for filename in Helper::filenames_in_file(&paklist).unwrap_or(Vec::new()) {
				// :TODO: add better error handling
				println!("{:?}", filename);
				archive.add_entry(&filename);
			}
		}

		// only save (updated) name map on success
		if let Some(mut name_map) = archive.take_name_map() {
			println!("{}", name_map);
			if name_map.dirty() {
				if let Some(name_map_file) = &self.name_map {
					name_map.save(name_map_file)?;
					name_map.clear_dirty();
				}
			} else {
				println!("No names changed or added. Not saving.");
			}
		}

		let _name_map = archive.take_name_map();
		if let Some(output) = &self.output {
			return archive.save(&output);
		}
		Ok(0)
	}
	fn set_basepath(&mut self, basepath: &str) {
		self.basepath = Some(basepath.to_string());
	}
	fn set_paklist(&mut self, paklist: &str) {
		self.paklist = Some(paklist.to_string());
	}
	fn set_output(&mut self, output: &str) {
		self.output = Some(output.to_string());
	}
	fn set_name_map(&mut self, name_map: &str) {
		self.name_map = Some(name_map.to_string());
	}
	fn set_names_only(&mut self, names_only: bool) {
		self.names_only = names_only;
	}
}
