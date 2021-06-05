mod context;
mod opts;

use clap::{crate_name, crate_version, Clap};
use env_logger::Env;
use log::info;
use opts::*;
use tera::{Context, Tera};

use crate::context::get_context;

fn main() -> Result<(), String> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();

	let context: Context = get_context(opts.context, opts.env).unwrap();
	let template = "PLACEHOLDER".to_string();
	let rendered = Tera::one_off(&template, &context, opts.autoescape).unwrap();

	println!("Result: {}", rendered);
	Ok(())
}
