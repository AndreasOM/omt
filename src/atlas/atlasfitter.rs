#[derive(Debug, Copy, Clone)]
pub struct Entry {
	id:     usize,
	width:  u32,
	height: u32,
}

impl Entry {
	pub fn new(id: usize, width: u32, height: u32) -> Entry {
		Entry {
			id:     id,
			width:  width,
			height: height,
		}
	}
}

#[derive(Debug)]
pub struct EntryWithPosition {
	pub id:     usize,
	pub width:  u32,
	pub height: u32,
	pub x:      u32,
	pub y:      u32,
}

impl EntryWithPosition {
	pub fn new_from_entry(entry: &Entry) -> EntryWithPosition {
		EntryWithPosition {
			id:     entry.id,
			width:  entry.width,
			height: entry.height,
			x:      0,
			y:      0,
		}
	}

	pub fn set_position(&mut self, x: u32, y: u32) {
		self.x = x;
		self.y = y;
	}
}

#[derive(Debug)]
struct Row {
	y:      u32, // start of row
	width:  u32,
	height: u32,
	end_x:  u32, // current end of row
}

impl Row {
	fn new(y: u32, width: u32, height: u32) -> Row {
		Row {
			y:      y,
			width:  width,
			height: height,
			end_x:  0,
		}
	}

	fn would_fit(&self, w: u32, h: u32) -> bool {
		if self.height >= h {
			let available_space = self.width - self.end_x;
			if available_space >= w {
				true
			} else {
				// not enough space
				//				println!("Row {:?} not enough space for {:?}", self, w );
				false
			}
		} else {
			// not high enough
			//			println!("Row {:?} not high enough for {:?}", self, h );
			false
		}
	}
}

#[derive(Debug, Default)]
pub struct Page {
	size:        u32,
	//	border:      u32,
	pub entries: Vec<EntryWithPosition>,
	rows:        Vec<Row>,
	used_height: u32,
}

impl Page {
	pub fn new(size: u32, _border: u32) -> Page {
		Page {
			size: size,
			//			border:      border,
			..Default::default()
		}
	}
	fn add_row(&mut self, height: u32) -> Option<usize> {
		if height <= (self.size - self.used_height) {
			let row = Row::new(self.used_height, self.size, height);
			self.used_height += height;
			let row_index = self.rows.len();
			//			println!("Created row #{:?} at {:?}. {:?} used now.", row_index, row.y, self.used_height );
			self.rows.push(row);
			Some(row_index)
		} else {
			//			println!("Can not create row with {:?} height, {:?} used of {:?}", height, self.used_height, self.size );
			None
		}
	}
	fn fit_entry_to_row_with_index(&mut self, entry: &Entry, row_index: usize) -> bool {
		match self.rows.get_mut(row_index) {
			None => false, // give up, should never happen
			Some(row) => {
				//				println!("Got row {:?}", row );
				if row.would_fit(entry.width, entry.height) {
					// add it
					let mut e = EntryWithPosition::new_from_entry(entry);
					// blitting
					let x = row.end_x;
					let y = row.y;
					row.end_x += e.width;
					e.set_position(x, y);
					self.entries.push(e);
					true
				} else {
					//					println!("Row {:?} would not fit {:?}", row, entry );
					false
				}
			},
		}
	}
	fn fit_entry(&mut self, entry: &Entry) -> bool {
		let h = entry.height;

		if self.size < entry.width || self.size < entry.height {
			false
		} else {
			// find row
			let mut candidates = Vec::new();

			for ri in 0..self.rows.len() {
				let r = &self.rows[ri];
				if r.would_fit(entry.width, entry.height) {
					//					println!("Row {:?} would fit {:?}", r, entry );
					if r.height < 2 * entry.height {
						// do not waste too much space, "2" is purely guessed
						candidates.push(ri);
					}
				}
			}

			if candidates.len() > 0 {
				// find best candidate
				let best_candidate_index = 0; // :TODO: actually find best candidate
							  /*
							  for ci in 0..candidates.len() {
								  //
							  }
							  */
				//				println!("Got candidate rows. Using best one {:?}", candidates[ best_candidate_index ] );
				self.fit_entry_to_row_with_index(entry, candidates[best_candidate_index])
			} else {
				// or create new row
				//				println!("No candidate row found creating new one. {:?}", self);
				match self.add_row(h) {
					None => false, // give up
					Some(row_index) => self.fit_entry_to_row_with_index(entry, row_index),
				}
			}
		}
	}
}

#[derive(Debug)]
pub struct AtlasFitter {
	entries: Vec<Entry>,
}

impl AtlasFitter {
	pub fn new() -> AtlasFitter {
		AtlasFitter {
			entries: Vec::new(),
		}
	}
	pub fn add_entry(&mut self, id: usize, width: u32, height: u32) {
		let e = Entry::new(id, width, height);
		self.entries.push(e);
	}

	pub fn fit(&self, size: u32, border: u32) -> Vec<Page> {
		let mut pages: Vec<Page> = Vec::new();

		for e in &self.entries {
			let mut did_fit = false;
			for p in &mut pages {
				if p.fit_entry(&e) {
					did_fit = true;
					break;
				}
			}
			if !did_fit {
				let mut p = Page::new(size, border);
				if !p.fit_entry(&e) {
					println!("‼️ Image doesn't fit into empty page {:?}", e);
					//					anyhow::bail!("‼️ Image doesn't fit into empty page");
				}
				pages.push(p);
			}
		}
		pages
	}
}
