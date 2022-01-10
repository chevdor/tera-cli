mod opts;
mod template;
mod wrapped_context;

use crate::template::Template;
use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use log::{debug, info, trace};
use opts::*;
use std::{fs::canonicalize, fs::File, io::Write, string::String};
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
	let mut include = opts.include;
	let mut path = canonicalize(&opts.template).unwrap();

	if opts.include_path.is_some() {
		include = true;
		path = canonicalize(opts.include_path.as_ref().unwrap()).unwrap();
	}

	let mut wrapped_context = wrapped_context::WrappedContext::new(opts);
	wrapped_context.create_context();

	let context: &Context = wrapped_context.context();
	trace!("context:\n{:#?}", context);

	let rendered;

	if include {
		let mut dir = path.to_str().unwrap();

		if path.is_file() {
			dir = path.parent().unwrap().to_str().unwrap();
		}

		let glob = dir.to_owned() + "/**/*";

		let mut tera = match Tera::new(&glob) {
			Ok(t) => t,
			Err(e) => {
				println!("Parsing error(s): {}", e);
				::std::process::exit(1);
			}
		};

		if !autoescape {
			tera.autoescape_on(vec![]);
		}

		rendered = tera.render_str(&template, context).unwrap();
	} else {
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
