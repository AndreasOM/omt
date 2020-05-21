use crate::util::OmError;

use shader_crusher::ShaderCrusher;

use std::fs;

pub struct Shader {

}

impl Shader {

	pub fn build(
		input: &str,
		_mode: &str,		// :TODO:
		output: &str
	) -> Result<u32, OmError>{

		let mut sc = ShaderCrusher::new();
		let data = match fs::read_to_string(input) {
			Ok( data ) => data,
			Err( e ) => return Err(OmError::Generic(e.to_string())),
		};

		sc.set_input( &data );
		// :TODO: use correct mode
		sc.crush();
		// :TODO: check return value
		if sc.get_output().len() > 0 {
			// :TODO: use correct mode
			fs::write(output, data).expect("// Unable to write file");
//			fs::write(output, sc.get_output()).expect("// Unable to write file");
			return Ok( 1 );
		} else {
			return Err(OmError::Generic("Error in shader".to_string()));
		}

	}
}

