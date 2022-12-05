pub trait CommandPacker {
	fn run(&mut self) -> anyhow::Result<u32> {
		Ok(0)
	}

	fn set_basepath(&mut self, _basepath: &str) {}
	fn set_paklist(&mut self, _paklist: &str) {}
	fn set_output(&mut self, _output: &str) {}
	fn set_input(&mut self, _input: &str) {}
	fn set_name_map(&mut self, _name_map: &str) {}
	fn set_targetpath(&mut self, _targetpath: &str) {}
	fn set_names_only(&mut self, _names_only: bool) {}
}
