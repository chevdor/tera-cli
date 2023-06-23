mod opts;
mod template;
mod wrapped_context;

use crate::template::Template;
use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use log::{debug, info, trace};
use opts::*;
use std::{fs::canonicalize, fs::File, io::Write, string::String};
use tera::{Context, Tera};

#[cfg(feature = "fluent")]
use fluent_templates::{ArcLoader, FluentLoader, LanguageIdentifier};

#[cfg(feature = "fluent")]
use std::env;

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

	#[cfg(feature = "fluent")]
	let locale: LanguageIdentifier = match opts.locale.to_owned() {
		Some(locale) => locale.parse(),
		None => "und".parse(),
	}
	.unwrap();

	#[cfg(feature = "fluent")]
	let locales_path = match opts.locales_path.to_owned() {
		Some(path) => path,
		None => "./locales".into(),
	};

	let mut wrapped_context = wrapped_context::WrappedContext::new(opts);
	wrapped_context.create_context();

	let context: &Context = wrapped_context.context();
	trace!("context:\n{:#?}", context);

	let mut tera: Tera;

	if include {
		let mut dir = path.to_str().unwrap();

		if path.is_file() {
			dir = path.parent().unwrap().to_str().unwrap();
		}

		let glob = dir.to_owned() + "/**/*";

		tera = match Tera::new(&glob) {
			Ok(t) => t,
			Err(e) => {
				println!("Parsing error(s): {e}");
				::std::process::exit(1);
			}
		};
	} else {
		tera = Tera::default();
	}

	if !autoescape {
		tera.autoescape_on(vec![])
	};

	#[cfg(feature = "fluent")]
	if cfg!(feature = "fluent") {
		let builder =
			ArcLoader::builder(&locales_path, locale.clone()).customize(|bundle| bundle.set_use_isolating(false));
		if let Ok(locale_loader) = builder.build() {
			let ftls = FluentLoader::new(locale_loader).with_default_lang(locale);
			tera.register_function("fluent", ftls)
		}
	};

	let rendered = tera.render_str(&template, context).unwrap();

	if let Some(out_file) = output {
		debug!("Saving to {}", out_file.display());
		let mut file = File::create(out_file).expect("Failed opening output file");
		file.write_all(rendered.as_bytes()).map_err(|e| e.to_string())
	} else {
		println!("{rendered}");
		Ok(())
	}
}
