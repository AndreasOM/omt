use std::path::PathBuf;

mod test {
	mod Atlas {
		use std::path::PathBuf;

		use omt::atlas::Atlas;
		use omt::atlas::AtlasSet;

		fn temp_output(name: &str) -> (PathBuf, PathBuf) {
			let test_dir = std::env::temp_dir();
			let test_dir = test_dir.join("omt-test");
			let test_dir = test_dir.join("atlas");

			std::fs::create_dir_all(&test_dir).unwrap();
			let output = test_dir.join(name);
			eprintln!("{:?}", &output);
			(test_dir, output)
		}
		#[test]
		fn simple_combine_works() {
			/*
			let test_dir = std::env::temp_dir();
			let test_dir = test_dir.join("omt-test");
			let test_dir = test_dir.join("atlas");

			println!("{:?}", &test_dir);
			std::fs::create_dir_all(&test_dir);
			let output = test_dir.join("simple");
			*/
			let (test_dir, output) = temp_output("simple");
			let size = 128;
			let border = 0;
			let reference_path = Some(&test_dir);
			let mut input = Vec::new();
			input.push("Data/64x64_red.png");
			input.push("Data/64x64_green.png");
			input.push("Data/64x64_blue.png");

			let input = input.iter().map(|i| i.into()).collect();

			let r = Atlas::combine(
				&output,
				size,
				border,
				//&input.iter().map(String::as_str).collect(),
				&input,
				reference_path,
			);

			println!("{:?}", &output);
		}
		#[test]
		fn v2_simple_combine_works() -> anyhow::Result<()> {
			let (test_dir, output) = temp_output("v2-simple-%d");

			let mut atlas_set = AtlasSet::default()
				.with_output(&output)
				.with_target_size(128)
				.with_border(0)
				.with_reference_path(&test_dir)
				.with_inputs(
					[
						PathBuf::from("Data/64x64_red.png").as_path(),
						PathBuf::from("Data/64x64_green.png").as_path(),
						PathBuf::from("Data/64x64_blue.png").as_path(),
					]
					.to_vec(),
				);
			let l = atlas_set.refit()?;
			assert_eq!(1, l);

			let r = atlas_set.save(&output, Some(&test_dir))?;
			assert_eq!(1, r);

			Ok(())
		}
	}
}
