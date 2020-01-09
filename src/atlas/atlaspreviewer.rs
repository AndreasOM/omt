use crate::util::OmError;

use crate::atlas::Atlas;

pub struct AtlasPreviewer {

}

impl AtlasPreviewer {
	pub fn preview(
		input: &str
	) -> Result<u32,OmError>{

		let atlases = Atlas::all_for_template( &input );
		println!("{:?}", atlases );
		
		let n = atlases.len() as u32;
		if n == 0 {
			Err(OmError::Generic("No matching atlas found.".to_string()))
		} else {
			Ok( n )	
		}
	}
}
