use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;

/// Command line utility for the tera templating engine. You need to provide a template using the tera syntax
/// as well as some data (various format are supported).
#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Location of the template.
	#[clap(short, long)]
	pub template: PathBuf,

	/// This flag tells the command to parse all templates found in the same
	/// path where the given template is located.
	#[clap(short, long)]
	pub include: bool,

	/// Location of the context data. This file can be of the following type:
	/// json | toml | yaml. If you prefer to pass the data as stdin, use `--stdin`
	#[clap(index = 1, required_unless_present_any = &["stdin", "env-only"], conflicts_with = "env-only")]
	pub context: Option<PathBuf>,

	/// The context data can be passed using stdin
	#[clap(short, long, conflicts_with_all = &["context", "env-only"], required_unless_present_any = &["context", "env-only"])]
	pub stdin: bool,

	/// If true, the current ENV will be appended to the data under the --env-key key
	#[clap(short, long)]
	pub env: bool,

	/// By default, if --env is set, the environment variables will be attached at the root of the context.
	/// This is convenient but may end up conflicting with your data. To prevent collisions, you can provide
	/// a custom key with this option.
	#[clap(long)]
	pub env_key: Option<String>,

	/// By default, the context is made of the data you pass and the ENV is applied
	/// afterwards. Setting this option will apply the ENV first. This is interesting
	/// if you prefer your data to override the ENV.
	#[clap(long, requires = "env")]
	pub env_first: bool,

	/// if you prefer your data to override the ENV.
	#[clap(long, requires = "env")]
	pub fail_on_collision: bool,
	/// If you want to solely use the ENV as context, you may pass
	/// this option. This will prevent an error about no context being passed
	/// to be raised.
	#[clap(long)]
	pub env_only: bool,

	/// Optional output file. If not passed, using stdout.
	#[clap(short, long)]
	pub out: Option<PathBuf>,

	/// Auto-escape rendered content. This is useful for HTML output.
	#[clap(long = "escape", short)]
	pub autoescape: bool,
}
