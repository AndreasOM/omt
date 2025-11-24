use std::fs;

use mlua::Lua;

pub struct Script {}

impl Script {
	pub fn build(input: &str, _mode: &str, output: &str) -> anyhow::Result<u32> {
		let data = match fs::read_to_string(input) {
			Ok(data) => data,
			Err(e) => anyhow::bail!(e),
		};

		let lua = Lua::new();

		let r = match lua.load(&data).into_function() {
			Ok(_) => Ok(0),
			Err(e) => Err(e),
		};

		//		println!( "{:?}", r );

		match r {
			Ok(_f) => {
				println!("Writing lua to {}", output);
				fs::write(output, data).expect("// Unable to write file");
				return Ok(1);
			},
			Err(_e) => {
				anyhow::bail!("Error in script");
			},
		};
	}
}
