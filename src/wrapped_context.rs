use crate::opts::Opts;
use log::{debug, info, trace, warn};
use serde_json::{self, json};
use std::{
	collections::HashMap,
	env,
	fmt::Debug,
	fs,
	io::{self, Read},
};
use tera::Context;

const _1KB: usize = 1024;
const BUFFER_SIZE: usize = 8 * _1KB;

#[derive(Debug)]
pub struct WrappedContext {
	context: Context,
	opts: Opts,
}

impl WrappedContext {
	pub fn new(opts: Opts) -> Self {
		Self { context: Context::new(), opts }
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn append_json(&mut self, str: &str) {
		debug!("Appending json");
		let json = str.parse::<serde_json::Value>().expect("Parse json");
		let object = json.as_object().unwrap();

		for (k, v) in object.iter() {
			self.handle_collision("json", k, v);
		}
	}

	pub fn append_toml(&mut self, str: &str) {
		debug!("Appending toml");
		let value = str.parse::<toml::Value>().unwrap();
		let table = value.as_table().unwrap();

		for (k, v) in table.iter() {
			self.handle_collision("toml", k, v);
		}
	}

	pub fn append_yaml(&mut self, str: &str) {
		debug!("Appending yaml");
		let value: serde_yaml::Value = serde_yaml::from_str(str).unwrap();
		let mapping = value.as_mapping().unwrap();

		for (k, v) in mapping.iter() {
			let k = k.as_str().unwrap();
			self.handle_collision("yaml", k, v);
		}
	}

	fn handle_collision<K, V>(&mut self, from: &str, k: K, v: V)
	where
		K: Debug + AsRef<str>,
		V: Debug + serde::Serialize,
	{
		trace!("key: {:?}", k);
		let exist = self.context.get(k.as_ref());
		if let Some(current) = exist {
			warn!("Key '{}' is being overwritten by the ENV", k.as_ref());
			warn!("  - Current value: {:?}", current);
			warn!("  - New value    : {:?}", v);

			if self.opts.fail_on_collision {
				eprintln!("Collision detected when adding {:?}={:?} from {}", k, v, from);
				std::process::exit(1);
			}
		}
		self.context.insert(k.as_ref(), &v);
	}

	pub fn append_env(&mut self) {
		debug!("Appending env");
		let env_vars = env::vars().collect::<HashMap<String, String>>();

		if let Some(key) = &self.opts.env_key {
			let env_json = json!(env_vars);
			self.context.insert(key, &env_json);
		} else {
			for (k, v) in env_vars.iter() {
				self.handle_collision("env", k, v);
			}
		}
	}

	pub fn create_context(&mut self) {
		if self.opts.env && self.opts.env_first {
			info!("Appending env to context first, env-key: {:?}", self.opts.env_key);
			self.append_env();
		}

		if self.opts.stdin {
			let stdin = io::stdin();
			let mut stdin = stdin.lock();
			let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
			let res = stdin.read_to_end(&mut buf).map_err(|e| e.to_string());
			if res.is_err() || res.unwrap() == 0 {
				panic!("if you provide stdin, make sure it is not empty"); // TODO: do we care actually ?
			}
			let input = String::from_utf8(buf.to_vec()).unwrap();

			// TODO: here we could check the type and append accordingly
			// match Self::get_type(&input) {
			// 	json && input.len() > 0 => ...
			// 	toml && input.len() > 0 => ...
			// 	yaml && input.len() > 0 => ...
			// 	_ => {}
			// }
			self.append_json(&input);
		} else {
			// here we know that we have a Path since --stdin is not passed
			let context_file = self.opts.context.as_ref().unwrap();
			let input = fs::read_to_string(context_file).unwrap();

			match context_file.extension() {
				Some(ext) if ext == "json" => self.append_json(&input),
				Some(ext) if ext == "toml" => self.append_toml(&input),
				Some(ext) if ext == "yaml" => self.append_yaml(&input),
				ext => {
					panic!("Extension not supported: {:?}", ext)
				}
			};
		};

		if self.opts.env && !self.opts.env_first {
			info!("Appending env to context, env-key: {:?}", self.opts.env_key);
			self.append_env();
		}
	}
}
