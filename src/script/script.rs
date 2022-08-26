use std::fs;

use rlua::Lua;

use crate::util::OmError;

pub struct Script {}

impl Script {
	pub fn build(input: &str, _mode: &str, output: &str) -> Result<u32, OmError> {
		let data = match fs::read_to_string(input) {
			Ok(data) => data,
			Err(e) => return Err(OmError::Generic(e.to_string())),
		};

		let lua = Lua::new();

		let r = lua.context(|lua_ctx| match lua_ctx.load(&data).into_function() {
			Ok(_) => Ok(0),
			Err(e) => Err(e),
		});

		//		println!( "{:?}", r );

		match r {
			Ok(_f) => {
				println!("Writing lua to {}", output);
				fs::write(output, data).expect("// Unable to write file");
				return Ok(1);
			},
			Err(_e) => {
				return Err(OmError::Generic("Error in script".to_string()));
			},
		};
	}
}
