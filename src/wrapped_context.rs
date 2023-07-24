use crate::opts::Opts;
use color_eyre::eyre::{bail, Context as EyreContext, ContextCompat, Result};
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

#[derive(Debug, PartialEq, Eq)]
pub enum SupportedType {
	Json,
	Toml,
	Yaml,
}

impl WrappedContext {
	pub fn new(opts: Opts) -> Self {
		Self { context: Context::new(), opts }
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn append_json(&mut self, str: &str) -> Result<()> {
		debug!("Appending json");
		let json = str.parse::<serde_json::Value>().context("failed to parse JSON")?;
		let object = json.as_object().context("JSON value must be an object")?;

		for (k, v) in object.iter() {
			self.handle_collision("json", k, v);
		}

		Ok(())
	}

	pub fn append_toml(&mut self, str: &str) -> Result<()> {
		debug!("Appending toml");
		let value = str.parse::<toml::Value>().context("failed to parse TOML")?;
		let table = value.as_table().context("TOML value must be a table")?;

		for (k, v) in table.iter() {
			self.handle_collision("toml", k, v);
		}

		Ok(())
	}

	pub fn append_yaml(&mut self, str: &str) -> Result<()> {
		debug!("Appending yaml");
		let value: serde_yaml::Value = serde_yaml::from_str(str).context("failed to parse YAML")?;
		let mapping = value.as_mapping().context("YAML value must be a mapping")?;

		for (k, v) in mapping.iter() {
			let k = k.as_str().context("YAML mapping's key must be a string")?;
			self.handle_collision("yaml", k, v);
		}

		Ok(())
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
				eprintln!("Collision detected when adding {k:?}={v:?} from {from}");
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

	pub fn get_type(str: &str) -> Option<SupportedType> {
		if let Ok(v) = str.parse::<serde_json::Value>() {
			if v.as_object().is_some() {
				return Some(SupportedType::Json);
			} else {
				debug!("Found json but not an Object")
			}
		} else {
			debug!("not json");
		}

		if let Ok(v) = str.parse::<toml::Value>() {
			if v.as_table().is_some() {
				return Some(SupportedType::Toml);
			} else {
				debug!("Found toml but not a table")
			}
		} else {
			debug!("not toml");
		}

		if let Ok(v) = serde_yaml::from_str::<serde_yaml::Value>(str) {
			if v.as_mapping().is_some() {
				return Some(SupportedType::Yaml);
			} else {
				debug!("Found yaml but not a mapping")
			}
		} else {
			debug!("not yaml");
		}

		None
	}

	pub fn create_context(&mut self) -> Result<()> {
		if (self.opts.env || self.opts.env_only) && self.opts.env_first {
			info!("Appending env to context first, env-key: {:?}", self.opts.env_key);
			self.append_env();
		}

		if self.opts.stdin {
			let stdin = io::stdin();
			let mut stdin = stdin.lock();
			let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
			stdin.read_to_end(&mut buf).context("failed to read stdin")?;
			let input = String::from_utf8(buf.to_vec()).context("invalid UTF8 string")?;

			match Self::get_type(&input) {
				Some(SupportedType::Json) if !input.is_empty() => self.append_json(&input),
				Some(SupportedType::Toml) if !input.is_empty() => self.append_toml(&input),
				Some(SupportedType::Yaml) if !input.is_empty() => self.append_yaml(&input),
				_ => Ok(()),
			}
			.context("failed to append stdin to context")?;
		} else if let Some(context_file) = &self.opts.context {
			let input = fs::read_to_string(context_file).context("failed to read context file")?;

			match context_file.extension() {
				Some(ext) if ext == "json" => self.append_json(&input),
				Some(ext) if ext == "toml" => self.append_toml(&input),
				Some(ext) if ext == "yaml" || ext == "yml" => self.append_yaml(&input),
				ext => bail!("extension not supported: {ext:?}"),
			}
			.context("failed to append file to context")?;
		}

		if (self.opts.env || self.opts.env_only) && !self.opts.env_first {
			info!("Appending env to context, env-key: {:?}", self.opts.env_key);
			self.append_env();
		}

		Ok(())
	}
}

#[cfg(test)]
mod test_context {
	use super::*;

	#[test]
	fn test_get_type_json() {
		let data = json!({
			"name": "John Doe",
			"age": 43u8,
			"phones": [
				"+44 1234567",
				"+44 2345678"
			]
		})
		.to_string();

		assert!(WrappedContext::get_type(&data) == Some(SupportedType::Json));
	}

	#[test]
	fn test_get_type_toml() {
		let data = r##"
        name = "John"
		age = 42
    	"##
		.to_string();

		assert!(WrappedContext::get_type(&data) == Some(SupportedType::Toml));
	}

	#[test]
	fn test_get_type_yaml() {
		let data = r##"name: "Bob"
ag: 42"##
			.to_string();
		assert!(WrappedContext::get_type(&data) == Some(SupportedType::Yaml));
	}

	#[test]
	fn test_get_type_na() {
		let data = r##"
        foobar
    	"##
		.to_string();

		assert!(WrappedContext::get_type(&data).is_none());
	}
}
