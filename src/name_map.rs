use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

type Crc32 = u32;

#[derive(Debug, Default)]
pub struct Entry {
	crc:  Crc32,
	name: String,
}

impl core::fmt::Display for Entry {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "{:#010x} {}", self.crc, self.name)
	}
}

impl Entry {
	pub fn new(crc: Crc32, name: String) -> Self {
		Self { crc, name }
	}
}

#[derive(Debug, Default)]
pub struct NameMap {
	entries: HashMap<Crc32, Entry>,
	dirty:   bool,
}

const NAME_LEN: usize = 250;

impl NameMap {
	pub fn load_or_create(filename: &str) -> anyhow::Result<Self> {
		// :TODO:
		let mut nm: NameMap = Default::default();

		let file = File::open(filename);

		let file = match file {
			Ok(p) => p,
			Err(_e) => anyhow::bail!("Error reading file"),
		};

		let mut bufreader = BufReader::new(file);

		// read header
		// check magic
		let magic = [0x4fu8, 0x4d, 0x4e, 0x41];
		for m in &magic {
			let b = bufreader.read_u8().unwrap_or(0);
			if b != *m {
				anyhow::bail!("Broken magic");
			}
		}
		let v = bufreader.read_u8().unwrap_or(0);
		if v != 1 {
			anyhow::bail!("Wrong version");
		}

		let flags = bufreader.read_u8().unwrap_or(0);
		if flags != 0 {
			anyhow::bail!(":TODO: Flags not implemented");
		}

		for _reserved in 0..2 {
			let r = bufreader.read_u8().unwrap_or(0);
			if r != 0 {
				anyhow::bail!(":TODO: Reserved field not zero");
			}
		}

		let number_of_names = bufreader.read_u32::<LittleEndian>().unwrap_or(0);
		println!("Reading {:?} names from files", number_of_names);

		for _e in 0..number_of_names {
			let crc = bufreader.read_u32::<LittleEndian>().unwrap_or(0);
			let len = bufreader.read_u16::<LittleEndian>().unwrap_or(0) as usize;
			let mut buf = Vec::with_capacity(len);
			buf.resize(len, 0);
			bufreader.read_exact(&mut buf)?;
			let name = core::str::from_utf8(&buf)?;
			nm.insert(crc, name.to_string());
		}

		nm.clear_dirty();

		Ok(nm)
	}

	pub fn save(&self, filename: &str) -> anyhow::Result<()> {
		let file = File::create(filename);
		// :TODO: rethink error handling
		let mut file = match file {
			Ok(p) => p,
			Err(_e) => anyhow::bail!("Error writing file"),
		};

		let flags = 0u8;
		file.write_all(&[
			0x4f, 0x4d, 0x4e, 0x41,  // magic header
			1,     // version
			flags, // flags
			0, 0, // reserved
		])
		.unwrap();

		let number_of_entries = self.entries.len() as u32;
		file.write_u32::<LittleEndian>(number_of_entries).unwrap();

		for entry in self.entries.values() {
			// crc, pos, size all as LittleEndian u32
			file.write_u32::<LittleEndian>(entry.crc)?;
			let l = entry.name.len();
			let (l, _pad) = if l > NAME_LEN {
				println!("Warning: {} clipped to {} characters", entry.name, NAME_LEN);
				(NAME_LEN, 0)
			} else {
				(l, NAME_LEN - l)
			};
			file.write_u16::<LittleEndian>(l as u16)?;
			file.write_all(&entry.name.as_bytes()[0..l])?;
			// Note: we decided not to pad, and use the pascal style length prefix instead
			/*
			for _ in 0..pad {
				file.write_u8(0)?;
			}
			*/
		}

		// self.dirty = false;
		Ok(())
	}

	pub fn insert(&mut self, crc: Crc32, name: String) {
		let _values = self.entries.entry(crc).or_insert_with(|| {
			self.dirty = true;
			Entry::new(crc, name)
		});
	}

	pub fn get_name(&self, crc: Crc32) -> Option<&String> {
		self.entries.get(&crc).map(|e| &e.name)
	}

	pub fn dirty(&self) -> bool {
		self.dirty
	}

	pub fn clear_dirty(&mut self) {
		self.dirty = false
	}
}

impl core::fmt::Display for NameMap {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		for e in self.entries.values() {
			write!(f, "{}\n", e)?;
		}
		Ok(())
	}
}
