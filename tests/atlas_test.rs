use std::path::PathBuf;

use omt::atlas::Atlas;

#[test]
fn simple_combine_works() {
	let test_dir = std::env::temp_dir();
	let test_dir = test_dir.join("omt-test");
	let test_dir = test_dir.join("atlas");

	println!("{:?}", &test_dir);
	std::fs::create_dir_all(&test_dir);
	let output = test_dir.join("simple");
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
