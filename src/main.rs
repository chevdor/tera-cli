mod opts;
mod template;
mod wrapped_context;

use crate::template::Template;
use clap::{crate_name, crate_version, Clap};
use env_logger::Env;
use log::{debug, info, trace};
use opts::*;
use std::{fs::File, io::Write, string::String};
use tera::{Context, Tera};

fn main() -> Result<(), String> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	debug!("opts:\n{:#?}", opts);

	let template = Template::load(&opts.template).expect("Failed reading the template");
	trace!("template:\n{}", template);

	let autoescape = opts.autoescape;
	let output = opts.out.to_owned();
	let include = opts.include;

	let mut wrapped_context = wrapped_context::WrappedContext::new(opts);
	wrapped_context.create_context();

	let context: &Context = wrapped_context.context();
	trace!("context:\n{:#?}", context);

	let mut rendered = String::from("");

	if !include {
		rendered = Tera::one_off(&template, context, autoescape).unwrap();
	}

	println!("{}", rendered);

	if let Some(out_file) = output {
		debug!("Saving to {}", out_file.display());
		let mut file = File::create(out_file).expect("Failed opening output file");
		return file.write_all(rendered.as_bytes()).map_err(|e| e.to_string());
	}

	Ok(())
}
