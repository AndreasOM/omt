use std::fs::File;
use std::io::{BufRead, BufReader };


pub struct FileHelper {

}

impl FileHelper {
	pub fn lines_in_file(filename: &str) -> Result<Vec<String>, &'static str> {
		let file = File::open(filename);

		let file = match file {
			Ok( p ) => p,
			Err( _e ) => return Err("Error reading file"),
		};

		let bufreader = BufReader::new(file);

		let mut lines: Vec<String> = Vec::new();
		for l in bufreader.lines() {
			let line = l.unwrap();
			lines.push( line );
		}

		Ok(lines)
	}
}
