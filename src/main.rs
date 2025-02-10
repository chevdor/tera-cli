#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]

mod opts;
mod template;
mod wrapped_context;

use crate::template::Template;
use clap::{crate_name, crate_version, Parser};
use color_eyre::eyre::{Context as EyreContext, ContextCompat, Result};
use env_logger::Env;
use log::{debug, info, trace};
use opts::*;
use std::{fs::canonicalize, fs::File, io::Write};
use tera::{Context, Tera};

#[cfg(feature = "fluent")]
use fluent_templates::{ArcLoader, FluentLoader, LanguageIdentifier};

#[cfg(feature = "fluent")]
use std::env;

fn main() -> Result<()> {
	color_eyre::install()?;

	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	info!("Running {} v{}", crate_name!(), crate_version!());

	let opts = Opts::parse();
	debug!("opts:\n{:#?}", opts);

	let template = Template::load(&opts.template).context("failed to read the template")?;
	trace!("template:\n{}", template);

	let autoescape = opts.autoescape;
	let output = opts.out.clone();
	let mut include = opts.include;
	let mut path = canonicalize(&opts.template).context("failed to get absolute path to `template`")?;

	if let Some(include_path) = &opts.include_path {
		include = true;
		path = canonicalize(include_path).context("failed to get absolute path to `include`")?;
	}

	#[cfg(feature = "fluent")]
	let locale: LanguageIdentifier = opts.locale.as_deref().unwrap_or("und").parse().context("failed to parse locale")?;

	#[cfg(feature = "fluent")]
	let locales_path = opts.locales_path.clone().unwrap_or_else(|| "./locales".into());

	let mut wrapped_context = wrapped_context::WrappedContext::new(opts);
	wrapped_context.create_context().context("failed to create context")?;

	let context: &Context = wrapped_context.context();
	trace!("context:\n{:#?}", context);

	let mut tera = if include {
		let dir =
			if path.is_file() { path.parent().context("failed to get parent directory")?.to_path_buf() } else { path };

		let glob = dir.join("**").join("*");

		Tera::new(glob.to_str().context("invalid UTF8 string")?)?
	} else {
		Tera::default()
	};

	if !autoescape {
		tera.autoescape_on(vec![])
	};

	#[cfg(feature = "fluent")]
	if cfg!(feature = "fluent") {
		let builder =
			ArcLoader::builder(&locales_path, locale.clone()).customize(|bundle| bundle.set_use_isolating(false));
		if let Ok(locale_loader) = builder.build() {
			let ftls = FluentLoader::new(locale_loader).with_default_lang(locale);
			tera.register_function("fluent", ftls);
		}
	};

	let rendered = tera.render_str(&template, context).context("failed to render")?;

	if let Some(out_file) = output {
		debug!("Saving to {}", out_file.display());
		let mut file = File::create(out_file).context("failed to open output file")?;
		file.write_all(rendered.as_bytes()).context("failed to write to output file")?;
	} else {
		println!("{rendered}");
	}

	Ok(())
}
