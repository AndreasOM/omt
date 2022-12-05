use std::fs;

use crate::name_map::NameMap;
use crate::packer::command_packer::CommandPacker;
use crate::packer::packer::Archive;

#[derive(Debug, Default)]
pub struct CommandPackerUnpack {
	input:      Option<String>,
	targetpath: Option<String>,
	name_map:   Option<String>,
	names_only: bool,
}

impl CommandPackerUnpack {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
}

impl CommandPacker for CommandPackerUnpack {
	fn run(&mut self) -> anyhow::Result<u32> {
		let name_map = if let Some(name_map_file) = &self.name_map {
			let nm = NameMap::load_or_create(&name_map_file)?;
			println!("Loaded NameMap:\n{}", nm);
			Some(nm)
		} else {
			None
		};
		let mut archive = Archive::create(&String::new());
		archive.give_name_map(name_map);

		if let Some(input) = &self.input {
			let metadata = match fs::metadata(input) {
				Err(_err) => anyhow::bail!("Input not found"),
				Ok(md) => md,
			};

			if !metadata.is_file() {
				anyhow::bail!("Input is not a file");
			}

			match archive.load(input) {
				Err(e) => {
					anyhow::bail!("Error in load: {}", e);
				},
				Ok(_) => {},
			};
		} else {
			anyhow::bail!("No input given!")
		}

		for e in archive.entries() {
			println!("{}", e);
		}
		if let Some(targetpath) = &self.targetpath {
			let metadata = match fs::metadata(targetpath) {
				Err(_err) => anyhow::bail!("Targetpath not found"), // :TODO: implement
				Ok(md) => md,
			};

			if !metadata.is_dir() {
				anyhow::bail!("Targetpath is not a directory");
			}
			archive.set_names_only(self.names_only);

			archive.unpack(targetpath).unwrap();
		}

		let _name_map = archive.take_name_map();
		Ok(archive.entries().len() as u32)
	}
	fn set_input(&mut self, input: &str) {
		self.input = Some(input.to_string());
	}
	fn set_name_map(&mut self, name_map: &str) {
		self.name_map = Some(name_map.to_string());
	}
	fn set_targetpath(&mut self, targetpath: &str) {
		self.targetpath = Some(targetpath.to_string());
	}
	fn set_names_only(&mut self, names_only: bool) {
		self.names_only = names_only;
	}
}
