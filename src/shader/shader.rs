use std::fs;

use shader_crusher::ShaderCrusher;

pub struct Shader {}

impl Shader {
	pub fn build(
		input: &str,
		_mode: &str, // :TODO:
		output: &str,
	) -> anyhow::Result<u32> {
		let mut sc = ShaderCrusher::new();
		let data = match fs::read_to_string(input) {
			Ok(data) => data,
			Err(e) => anyhow::bail!(e),
		};

		sc.set_input(&data);
		// :TODO: use correct mode
		sc.crush();
		// :TODO: check return value
		if sc.get_output().len() > 0 {
			// :TODO: use correct mode
			fs::write(output, data).expect("// Unable to write file");
			//			fs::write(output, sc.get_output()).expect("// Unable to write file");
			return Ok(1);
		} else {
			anyhow::bail!("Error in shader");
		}
	}
}
