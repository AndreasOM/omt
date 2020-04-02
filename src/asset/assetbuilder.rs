use glob::glob;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

#[derive(Clone,Hash,Eq,PartialEq,Debug)]
enum ParameterValue {
	NoValue,
	IntegerValue(i64),
	StringValue(String),
}

impl fmt::Display for ParameterValue {
	fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			ParameterValue::NoValue => write!(f, "NOVALUE"),
			ParameterValue::IntegerValue( i ) => write!(f, "{}", i),
			ParameterValue::StringValue( s ) => write!(f, "\"{}\"", s ),

		}
//		write!(f, "FUU")
	}
}

struct ToolRun {
	tool: String,
	command: String,
	output: String,
	input: Vec<String>,
	parameters: HashMap<String,ParameterValue>,
	cmd_line: String,
}

impl ToolRun {
	fn new(
		tool: &str,
		command: &str,
		output: &str,
		input: &Vec<String>,
		parameters: &HashMap<String,ParameterValue>,
		cmd_line: &str,
	) -> ToolRun
	{
		ToolRun {
			tool: tool.to_string(),
			command: command.to_string(),
			output: output.to_string(),
			input: input.clone(),
			parameters: parameters.clone(),
			cmd_line: cmd_line.to_string(),
		}
	}
}

pub struct AssetBuilder{
	content_directory: String,
	data_directory: String,
	temp_directory: String,
	archive: String,
	paklist: String,
}

impl AssetBuilder{
	pub fn new(
		content_directory: &str,
		data_directory: &str,
		temp_directory: &str,
		archive: &str,
		paklist: &str,
	) -> AssetBuilder {
		AssetBuilder {
			content_directory: content_directory.to_string(),
			data_directory:    data_directory.to_string(),
			temp_directory:    temp_directory.to_string(),
			archive:           archive.to_string(),
			paklist:           paklist.to_string(),
		}
	}


	fn tool_asset(
		&self,
		tool_run: &ToolRun,
	)
	-> Result<u32,&'static str> {
		match tool_run.command.as_ref() {
			"" => {
				println!("NO command for asset tool" );
				Err("NO command for asset tool" )
			},
			"dump" => {
				println!("command.  : {:?}", tool_run.command );
				println!("output    : {:?}", tool_run.output );
				println!("input     : {:?}", tool_run.input );
				println!("parameters: {:?}", tool_run.parameters );
				Ok(0)
			},
			"copy" => {
				let mut number_of_assets_updated = 0;
				for source in &tool_run.input {

//				let source = tool_run.input[0].clone();
					let source_filename = Path::new( source ).file_name().unwrap().to_str().unwrap();//_or(OsStr::new("")).to_str();
					let output = self.replace_placeholders( &tool_run, &source_filename );
					println!("OUTPUT {:?}", output );
					let dest = format!("{}/{}", self.data_directory, output );
					match fs::copy( &source, &dest ) {
						Ok( bytes ) => {
							println!("📁 🔧 ✅ Copied {:?} bytes from {:?} to {:?}", bytes, &source, &dest);
							number_of_assets_updated += 1;
						},
						Err( e ) => {
							println!("📁 🔧 ‼️ Error: Copying from {:?} to {:?}", &source, &dest);
							return Err( "Error while copying" );
						},
					}
				}
				Ok( number_of_assets_updated )
			},
			cmd => {
				println!("Unhandled asset tool command: {:?}", cmd );
				Err( "Unhandled asset tool command" )
			},
		}
	}

	fn replace_placeholders(
		&self,
		tool_run: &ToolRun,
		input: &str
		)
	-> String {
		let output = input.clone();
		let re = Regex::new(r"\$\{(.*?)\}").unwrap();

		let output = re.replace_all(
			&output,
			|c: &regex::Captures| {
				let placeholder = c.get(1).map_or( "", |m| m.as_str() );
				println!("Found {:?}", placeholder );
				match placeholder {
					"" => "".to_string(),
					"tool" => tool_run.tool.clone(),
					"command" => tool_run.command.clone(),
					"output" => format!("\"{}\"", tool_run.output).clone(),
					"input" => {
						tool_run.input.iter().map( |s|
							format!("\t\"{}\"", s).clone()
						).collect::<Vec<_>>().join(" ").to_string()
					},
					"input:basename" => {
						let input = if tool_run.input.len() > 0 {
							tool_run.input[ 0 ].clone()
						} else {
							"".to_string()
						};
						match Path::new( &input ).file_name() {
							Some( basename ) => basename.to_os_string().into_string().unwrap_or( "".to_string() ),
							None => "".to_string(),
						} //.unwrap_or("".to_os_string()).to_string()
					},
					"data_directory" => self.data_directory.clone(),
					param => {
						println!("{:?}", tool_run.parameters.get( param ) );
						tool_run.parameters.get( param ).unwrap_or( &ParameterValue::NoValue ).to_string()
					},
				}
			}
		);

		output.to_string()
	}

	fn tool_call_external(
		&self,
		tool_run: &ToolRun,
	)
	-> Result<u32,&'static str> {
		let cmd_line = self.replace_placeholders( &tool_run, &tool_run.cmd_line );
		let cmd_line = self.replace_placeholders( &tool_run, &cmd_line );
		println!("Calling\n{}\tfor {:#?}", cmd_line, tool_run.input );
//		let output = Command::new("/bin/sh").args(&["-c", "echo", ""]).output();
//		let output = Command::new("/bin/sh").args(&["-c", "date", ""]).output();
		let output = Command::new("/bin/sh").args(&["-c", &cmd_line]).output();
		match output {
			Err(e) => Err("Error running external command"),
			Ok( output ) => {
				let stdout = String::from_utf8_lossy(&output.stdout);
				let stderr = String::from_utf8_lossy(&output.stderr);

				println!("stdout:\n{}", stdout );
				println!("stderr:\n{}", stderr );
				println!("return code: {}", output.status.code().unwrap_or(-255));
				
				let number_of_assets_updated = 1;
				Ok( number_of_assets_updated )
			},
		}
	}


	pub fn build (
		&self,
	)
	-> Result<u32,&'static str> {

		let mut number_of_assets_updated = 0;

		// if content_directory is an explicit file only run that // :TODO: and ends in .asset_config.yaml ?
		let config_glob = if( Path::new( &self.content_directory ).is_file() ) {
			format!( "{}", self.content_directory )
		} else {
			// find all asset_config.yaml
			format!( "{}/**/*.asset_config.yaml", self.content_directory )
		};

		let mut config_files = Vec::new();
		for config_file in glob( &config_glob ).expect("Failed glob pattern") {
			match config_file {
				Err(e) => return Err( "Error finding config" ),
				Ok(config_file) => {
//					println!("Config file: {:?}", config_file );
					config_files.push( config_file );
				},
			}
		}

		println!("Found {:?} config files", config_files.len() );

		for config_file in config_files {
			// read yaml
			println!("===\n{:?}", config_file );
			let mut file = File::open( &config_file ).expect( "Failed opening file" );
			let mut config = String::new();
			file.read_to_string(&mut config).expect( "Failed reading file" );
			let yaml = YamlLoader::load_from_str(&config).unwrap();

			let config_file_path = Path::new(&config_file);
			let asset_path = config_file_path.parent().unwrap_or( Path::new(".") );
			println!("Asset Path {:?}", asset_path );
			// parse yaml
//			println!("YAML: {:?}", yaml );
			for doc in yaml {
//				println!("---");
				let tool = doc["tool"].as_str().unwrap_or("");
				let command = doc["command"].as_str().unwrap_or("");
				let output = doc["output"].as_str().unwrap_or("");
				let cmd_line = doc["cmd_line"].as_str().unwrap_or("");
				let mut input = Vec::new();

				if doc["input"].is_array() {
					match doc["input"].as_vec() {
						None => {},
						Some(i) => {
//							println!("i: {:?}", i );
							for i in i {
								match i.as_str() {
									None => {},
									Some(s) => input.push( s.to_string() )
								}
							}
						},
					}
				} else {
					let i = doc["input"].as_str();
					match i {
						Some(i) => input.push( i.to_string() ),
						None => {},
					};
				}

//				println!("INPUT {:?}", input );
				let input_original = input.iter().map( |i| {
					format!( "{}", i )
				}).collect::<Vec<_>>();
				let input = input.iter().map( |i| {
					format!( "{}/{}", asset_path.display(), i )
				}).collect::<Vec<_>>();
//				println!("INPUT {:?}", input );
//				println!("INPUT_ORIGINAL {:?}", input_original );
//				return Ok(1);

//				let input = doc["input"].as_str();

				let mut parameters = HashMap::new();

				match doc["parameters"].as_hash() {
					None => {},
					Some(params) => {
						for (name, value) in params {
//							println!("name: {:?} -> {:?}", name, value );
							let value = match value {
								Yaml::Integer( v ) => ParameterValue::IntegerValue( *v ),
								Yaml::String( v ) => ParameterValue::StringValue( v.clone() ),
								x => {
									println!("Unhandled parameter value {:?}", x );
									ParameterValue::NoValue
								}
							};
							let name = match name.as_str() {
								Some( s ) => s.to_string(),
								x => { println!("Unhandled name type {:?}", x ); "".to_string() },
							};
							parameters.insert( name, value );
						}
					}
				};
/*
				println!("tool      : {:?}", tool );
				println!("command.  : {:?}", command );
				println!("output    : {:?}", output );
				println!("input     : {:?}", input );
				println!("parameters: {:?}", parameters );
*/
				let tool_run = ToolRun::new( &tool, &command, &output, &input, &parameters, &cmd_line );
				// call tool
				match tool {
					""			=> continue,
					"noop"		=> println!("NOOP -> Do nothing"),
					"$asset"	=> {
						println!("$asset command found");
						match self.tool_asset( &tool_run ) {
							Ok( n ) => {
								number_of_assets_updated += n;
							},
							Err( e ) => {

							}
						}
					}
					tool		=> {
						match self.tool_call_external( &tool_run ) {
							Ok( n ) => {
								number_of_assets_updated += n;
							},
							Err( e ) => {

							}							
						}
					},
				}
			}
		}

		Ok( number_of_assets_updated )
	}
}
