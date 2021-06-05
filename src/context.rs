use std::{
	fs::File,
	io::{self, Read},
	path::PathBuf,
};
use tera::Context;

const _1KB: usize = 1024;
const BUFFER_SIZE: usize = 8 * _1KB;

pub fn get_context(file: Option<PathBuf>, env: bool) -> io::Result<Context> {
	if env {
		todo!("load env context");
	}

	let _buffer: Vec<u8> = match file {
		Some(filename) => {
			let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
			let mut f = File::open(filename)?;
			f.read_to_end(&mut buf)?;
			buf
		}
		None => {
			let stdin = io::stdin();
			let mut stdin = stdin.lock();

			let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
			stdin.read_to_end(&mut buf)?;

			// if buf.len() == 0 || buf == &[10] {
			//     app_clone.print_help().expect("Failed showing help");
			//     println!("\n🤚 If you pass data using stdin, it must not be empty!");
			//     exit(1);
			// }

			buf.to_vec()
		}
	};

	let context = Context::new();
	Ok(context)
}
