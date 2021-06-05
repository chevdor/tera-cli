use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;

/// Command line utility for the tera templating engine. You need to provide a template using the tera syntax
/// as well as some data (various format are supported).
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Location of the template.
	#[clap(short, long)]
	pub template: PathBuf,

	/// Location of the context data. Can be .json, .toml, .yaml|yml
	#[clap(index = 1)]
	pub context: Option<PathBuf>,

	/// If true, the current ENV will be appended to the data under the "env" key
	#[clap(short, long)]
	pub env: bool,

	/// Optional output file. If not passed, using stdout.
	#[clap(short, long)]
	pub out: Option<PathBuf>,

	/// Auto-escape rendered content
	#[clap(long = "escape", short)]
	pub autoescape: bool,
}
