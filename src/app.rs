use std::env::Args;

use clap::AppSettings;
use structopt::StructOpt;

use crate::subcommand::params::CoreParams;
use crate::subcommand::{replace, insert};

pub fn parse_and_execute(args: Args) -> Result<(), String> {
	let matches = CoreParams::clap()
		.name(env!("CARGO_PKG_NAME"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.version(env!("CARGO_PKG_VERSION"))
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::ArgsNegateSubcommands)
		.setting(AppSettings::SubcommandsNegateReqs)
		.get_matches_from(args);
	let cli_args = CoreParams::from_clap(&matches);
	match cli_args {
		CoreParams::Replace(param) => {
			replace::run(param)
		}
		CoreParams::Insert(param) => {
			insert::run(param)
		}
		_ => {
			panic!("Can't process")
		}
	}
}