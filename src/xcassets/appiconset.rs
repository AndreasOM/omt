use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug,Deserialize,Serialize)]
pub struct ImageEntry {
	pub filename: Option<String>,
	pub idiom: String,
	pub scale: String,
	pub size: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AppIconSetInfo {
	pub author: String,
	pub version: u32,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AppIconSet {
	pub images: Vec<ImageEntry>,
	pub info: AppIconSetInfo,
}

#[derive(Debug)]
pub enum AppIconSetError {
	LoadError,
	SaveError,
	ParseError,
}

impl AppIconSet {
	pub fn load(
		filename: &str
	) -> Result<AppIconSet, AppIconSetError> {
		let json = match fs::read_to_string(&filename) {
			Ok( j ) => j,
			Err( _ ) => return Err( AppIconSetError::LoadError ),
		};

		let appIconSet: AppIconSet = match serde_json::from_str( &json ) {
			Ok( ais ) => ais,
			Err( e ) => {
				println!("ERROR: serde_json::from_str failed with {:?}", e);
				println!("json was: {:?}", json );
				return Err( AppIconSetError::ParseError )
			},
		};

//		println!("{:#?}", appIconSet);

		Ok( appIconSet )
	}

	pub fn save( 
		&self,
		filename: &str
	) -> Result<u32, AppIconSetError> {
		let json = match serde_json::to_string_pretty(&self) {
			Ok( j ) => j,
			Err( e ) => {
				println!("ERROR: serde_json::to_string failed with {:?}", e);
				println!("object was: {:?}", self );
				return Err( AppIconSetError::ParseError )				
			}
		};

		match fs::write( filename, json ) {
			Ok(_) => return Ok( 0 ),
			Err( e ) => {
				println!("ERROR: writing json to file {:?}", e);
				return Err( AppIconSetError::SaveError );				
			},
		};
	}
}