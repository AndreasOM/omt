use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use glob::glob;
use regex::Regex;
use yaml_rust2::Yaml;
use yaml_rust2::YamlLoader;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum ParameterValue {
	NoValue,
	IntegerValue(i64),
	StringValue(String),
}

impl fmt::Display for ParameterValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParameterValue::NoValue => write!(f, "NOVALUE"),
			ParameterValue::IntegerValue(i) => write!(f, "{}", i),
			ParameterValue::StringValue(s) => write!(f, "\"{}\"", s),
		}
		//		write!(f, "FUU")
	}
}

struct ToolRun {
	tool:       String,
	command:    String,
	output:     String,
	input:      Vec<String>,
	parameters: HashMap<String, ParameterValue>,
	cmd_line:   String,
}

impl ToolRun {
	fn new(
		tool: &str,
		command: &str,
		output: &str,
		input: &Vec<String>,
		parameters: &HashMap<String, ParameterValue>,
		cmd_line: &str,
	) -> ToolRun {
		ToolRun {
			tool:       tool.to_string(),
			command:    command.to_string(),
			output:     output.to_string(),
			input:      input.clone(),
			parameters: parameters.clone(),
			cmd_line:   cmd_line.to_string(),
		}
	}
	fn run(&self, asset_builder: &AssetBuilder) -> Result<u32, &'static str> {
		let tool = self.tool.as_str();
		match tool {
			"" => Ok(0),
			"noop" => {
				println!("NOOP -> Do nothing");
				Ok(0)
			},
			"$asset" => {
				println!("$asset command found");
				asset_builder.tool_asset(&self)
			},
			_tool => asset_builder.tool_call_external(&self),
		}
	}
}

pub struct AssetBuilder {
	content_directory: String,
	data_directory:    String,
	_temp_directory:   String,
	_archive:          String,
	_paklist:          String,
	dry_run:           bool,
}

impl AssetBuilder {
	pub fn new(
		content_directory: &str,
		data_directory: &str,
		temp_directory: &str,
		archive: &str,
		paklist: &str,
		dry_run: &bool,
	) -> AssetBuilder {
		AssetBuilder {
			content_directory: content_directory.to_string(),
			data_directory:    data_directory.to_string(),
			_temp_directory:   temp_directory.to_string(),
			_archive:          archive.to_string(),
			_paklist:          paklist.to_string(),
			dry_run:           *dry_run,
		}
	}

	fn tool_asset(&self, tool_run: &ToolRun) -> Result<u32, &'static str> {
		match tool_run.command.as_ref() {
			"" => {
				println!("NO command for asset tool");
				Err("NO command for asset tool")
			},
			"dump" => {
				println!("command.  : {:?}", tool_run.command);
				println!("output    : {:?}", tool_run.output);
				println!("input     : {:?}", tool_run.input);
				println!("parameters: {:?}", tool_run.parameters);
				Ok(0)
			},
			"copy" => {
				let mut number_of_assets_updated = 0;
				for source in &tool_run.input {
					//				let source = tool_run.input[0].clone();
					let source_filename = Path::new(source).file_name().unwrap().to_str().unwrap(); //_or(OsStr::new("")).to_str();
					let output = self.replace_placeholders(&tool_run, &source_filename);
					println!("OUTPUT {:?}", output);
					let dest = format!("{}/{}", self.data_directory, output);
					if self.dry_run {
						println!("🌵 Dry Run: Would copy from {:?} to {:?}", &source, &dest);
					} else {
						match fs::copy(&source, &dest) {
							Ok(bytes) => {
								println!(
									"📁 🔧 ✅ Copied {:?} bytes from {:?} to {:?}",
									bytes, &source, &dest
								);
								number_of_assets_updated += 1;
							},
							Err(_e) => {
								println!(
									"📁 🔧 ‼️ Error: Copying from {:?} to {:?}",
									&source, &dest
								);
								return Err("Error while copying");
							},
						}
					}
				}
				Ok(number_of_assets_updated)
			},
			cmd => {
				println!("Unhandled asset tool command: {:?}", cmd);
				Err("Unhandled asset tool command")
			},
		}
	}

	fn replace_placeholders(&self, tool_run: &ToolRun, input: &str) -> String {
		let output = input.clone();
		//		let re = Regex::new(r"\$\{((.*?)(\s*)(.*)?)\}").unwrap();
		let re = Regex::new(r"\$\{(.*?)\}").unwrap();

		let output = re.replace_all(
			// :TODO: the whole body of this needs a massive refactoring
			&output,
			|c: &regex::Captures| {
				// println!("{:#?}", &c);
				let placeholder_full = c.get(1).map_or("", |m| m.as_str());
				let placeholder = placeholder_full.split(" ").next().map_or("", |m| m);
				println!("Found {:?}", placeholder);
				match placeholder {
					"" => "".to_string(),
					"tool" => tool_run.tool.clone(),
					"command" => tool_run.command.clone(),
					"output" => format!("{}", tool_run.output).clone(),
					"input" => {
						tool_run
							.input
							.iter()
							.map(|s|
							// format!("\t\"{}\"", s).clone()
							// format!(" \"{}\"", s).clone()
							format!("{} ", s).clone())
							.collect::<Vec<_>>()
							.join(" ")
							.to_string()
					},
					"input:basename" => {
						let input = if tool_run.input.len() > 0 {
							tool_run.input[0].clone()
						} else {
							"".to_string()
						};
						match Path::new(&input).file_name() {
							Some(basename) => {
								let remove_ext =
									placeholder_full.split(" ").nth(1).map_or("", |m| m);
								println!("remove: {}", &remove_ext);

								let basename = basename
									.to_os_string()
									.into_string()
									.unwrap_or("".to_string());
								basename
									.strip_suffix(remove_ext)
									.map_or(basename.clone(), |m| m.to_string())
							},
							None => "".to_string(),
						} //.unwrap_or("".to_os_string()).to_string()
					},
					"data_directory" => self.data_directory.clone(),
					param => {
						// extended handling

						println!("{:?}", tool_run.parameters.get(param));
						tool_run
							.parameters
							.get(param)
							.unwrap_or(&ParameterValue::NoValue)
							.to_string()
					},
				}
			},
		);

		output.to_string()
	}

	fn tool_call_external(&self, tool_run: &ToolRun) -> Result<u32, &'static str> {
		let cmd_line = self.replace_placeholders(&tool_run, &tool_run.cmd_line);
		let cmd_line = self.replace_placeholders(&tool_run, &cmd_line);
		println!("Calling\n{}\tfor {:#?}", cmd_line, tool_run.input);
		//		let output = Command::new("/bin/sh").args(&["-c", "echo", ""]).output();
		//		let output = Command::new("/bin/sh").args(&["-c", "date", ""]).output();
		if self.dry_run {
			println!("🌵 Dry Run: >{}<", &cmd_line);
			Ok(0)
		} else {
			dbg!(&cmd_line);
			let mut cmd_line = cmd_line.split(' ');
			let cmd = cmd_line.next().unwrap_or("");
			let args: Vec<&str> = cmd_line.filter(|n| !n.trim().is_empty()).collect();
			dbg!(&cmd);
			dbg!(&args);
			let output = Command::new(cmd).args(&args).output();
			//		let output = Command::new("/bin/sh").args(&["-c", &cmd_line]).output();
			match output {
				Err(e) => {
					println!("Error running external program: {}", &e);
					Err("Error running external command")
				},
				Ok(output) => {
					let stdout = String::from_utf8_lossy(&output.stdout);
					let stderr = String::from_utf8_lossy(&output.stderr);

					println!("stdout:\n{}", stdout);
					println!("stderr:\n{}", stderr);
					println!("return code: {}", output.status.code().unwrap_or(-255));

					let number_of_assets_updated = 1;
					Ok(number_of_assets_updated)
				},
			}
		}
	}

	pub fn build(&self) -> Result<u32, &'static str> {
		let mut number_of_assets_updated = 0;

		// if content_directory is an explicit file only run that // :TODO: and ends in .asset_config.yaml ?
		let config_glob = if Path::new(&self.content_directory).is_file() {
			format!("{}", self.content_directory)
		} else {
			// find all asset_config.yaml
			format!("{}/**/*.asset_config.yaml", self.content_directory)
		};

		let mut config_files = Vec::new();
		for config_file in glob(&config_glob).expect("Failed glob pattern") {
			match config_file {
				Err(_e) => return Err("Error finding config"),
				Ok(config_file) => {
					//					println!("Config file: {:?}", config_file );
					config_files.push(config_file);
				},
			}
		}

		println!("Found {:?} config files", config_files.len());

		for config_file in config_files {
			// read yaml
			println!("===\n{:?}", config_file);
			let mut file = File::open(&config_file).expect("Failed opening file");
			let mut config = String::new();
			file.read_to_string(&mut config)
				.expect("Failed reading file");
			if config.len() == 0 {
				return Err("Empty config");
			}
			let yaml = match YamlLoader::load_from_str(&config) {
				//.unwrap();
				Err(_e) => return Err("Broken config"),
				Ok(yaml) => yaml,
			};
			println!("{:?}", yaml);

			let config_file_path = Path::new(&config_file);
			let asset_path = config_file_path.parent().unwrap_or(Path::new("."));
			println!("Asset Path {:?}", asset_path);
			// parse yaml
			//			println!("YAML: {:?}", yaml );
			for doc in yaml {
				//				println!("---");
				let tool = doc["tool"].as_str().unwrap_or("");
				let command = doc["command"].as_str().unwrap_or("");
				let output = doc["output"].as_str().unwrap_or("");
				let cmd_line = doc["cmd_line"].as_str().unwrap_or("");
				//				dbg!(&cmd_line);
				//				todo!("cmd_line");
				let mut input = Vec::new();

				let combine_inputs = doc["combine-inputs"].as_bool().unwrap_or(false);

				if doc["input"].is_array() {
					match doc["input"].as_vec() {
						None => {},
						Some(i) => {
							//							println!("i: {:?}", i );
							for i in i {
								match i.as_str() {
									None => {},
									Some(s) => input.push(s.to_string()),
								}
							}
						},
					}
				} else {
					let i = doc["input"].as_str();
					match i {
						Some(i) => input.push(i.to_string()),
						None => {},
					};
				}

				//				println!("INPUT {:?}", input );
				let _input_original = input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>();
				let input = input
					.iter()
					.map(|i| format!("{}/{}", asset_path.display(), i))
					.collect::<Vec<_>>();
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
								Yaml::Integer(v) => ParameterValue::IntegerValue(*v),
								Yaml::String(v) => ParameterValue::StringValue(v.clone()),
								x => {
									println!("Unhandled parameter value {:?}", x);
									ParameterValue::NoValue
								},
							};
							let name = match name.as_str() {
								Some(s) => s.to_string(),
								x => {
									println!("Unhandled name type {:?}", x);
									"".to_string()
								},
							};
							parameters.insert(name, value);
						}
					},
				};
				/*
								println!("tool      : {:?}", tool );
								println!("command.  : {:?}", command );
								println!("output    : {:?}", output );
								println!("input     : {:?}", input );
								println!("parameters: {:?}", parameters );
				*/
				// expand input
				let mut expanded_input = Vec::new();
				for i in input.iter() {
					//					expanded_input.push( i.clone() );
					for exp in glob(&i).expect("Failed glob pattern") {
						match exp {
							Err(_e) => return Err("Error globing input"),
							Ok(e) => {
								//								println!("e: {:#?}", e);
								match e.into_os_string().into_string() {
									Err(_e) => return Err("Error globing input"),
									Ok(e) => {
										//										println!("e: {:#?}", e);
										expanded_input.push(e);
									},
								}
							},
						}
					}
				}

				//				println!("expanded_input: {:#?}", expanded_input );

				// run once, or multiple times
				if combine_inputs {
					let tool_run = ToolRun::new(
						&tool,
						&command,
						&output,
						&expanded_input,
						&parameters,
						&cmd_line,
					);
					// call tool
					match tool_run.run(&self) {
						Ok(n) => {
							number_of_assets_updated += n;
						},
						Err(_e) => {},
					}
				} else {
					for input in expanded_input.iter() {
						let mut single_input = Vec::new();
						single_input.push(input.clone());
						let tool_run = ToolRun::new(
							&tool,
							&command,
							&output,
							&single_input,
							&parameters,
							&cmd_line,
						);
						// call tool
						match tool_run.run(&self) {
							Ok(n) => {
								number_of_assets_updated += n;
							},
							Err(_e) => {},
						}
					}
				}
			}
		}

		Ok(number_of_assets_updated)
	}
}
