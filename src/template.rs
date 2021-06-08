use std::{fs, io, path::Path};

pub struct Template;

impl Template {
	pub fn load(path: &Path) -> io::Result<String> {
		fs::read_to_string(path)
	}
}
