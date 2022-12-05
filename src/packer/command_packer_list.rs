use std::fs;

use crate::name_map::NameMap;
use crate::packer::command_packer::CommandPacker;
use crate::packer::packer::Archive;

#[derive(Debug, Default)]
pub struct CommandPackerList {
	input:    Option<String>,
	name_map: Option<String>,
}

impl CommandPackerList {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
}

impl CommandPacker for CommandPackerList {
	fn run(&mut self) -> anyhow::Result<u32> {
		let name_map = if let Some(name_map_file) = &self.name_map {
			let nm = NameMap::load_or_create(&name_map_file)?;
			println!("Loaded NameMap:\n{}", nm);
			Some(nm)
		} else {
			None
		};
		if let Some(input) = &self.input {
			let metadata = match fs::metadata(input) {
				Err(_err) => anyhow::bail!("Input not found"),
				Ok(md) => md,
			};

			if !metadata.is_file() {
				anyhow::bail!("Input is not a file");
			}

			let mut archive = Archive::create(&String::new());
			archive.give_name_map(name_map);

			match archive.load(input) {
				Err(e) => {
					anyhow::bail!("Error in load: {}", e);
				},
				Ok(_) => {},
			};

			let _name_map = archive.take_name_map();

			for e in archive.entries() {
				println!("{}", e);
			}
			Ok(archive.entries().len() as u32)
		} else {
			anyhow::bail!("No input given!")
		}
	}
	fn set_input(&mut self, input: &str) {
		self.input = Some(input.to_string());
	}
	fn set_name_map(&mut self, name_map: &str) {
		self.name_map = Some(name_map.to_string());
	}
}
