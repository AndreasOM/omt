use std::path::Path;

use anyhow::Result;

pub struct ColorMapper {}

impl ColorMapper {
	pub fn process(
		source_pal_path: &Path,
		target_pal_path: &Path,
		input_path: &Path,
		output_path: &Path,
	) -> Result<()> {
		// Load the source palette
		let _source_pal = image::open(source_pal_path)?;

		// Load the target palette
		let _target_pal = image::open(target_pal_path)?;

		// Load the input image
		let input_image = image::open(input_path)?;

		// For now, just write the input image to output without any conversion
		input_image.save(output_path)?;

		Ok(())
	}
}
