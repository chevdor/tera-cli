use std::{fs, io, path::PathBuf};

pub struct Template;

impl Template {
    pub fn load(path: &PathBuf) -> io::Result<String> {
        fs::read_to_string(path)
    }
}
