use crate::code;
use anyhow::Result;
use argh::FromArgs;
use std::path::PathBuf;

/// All commands
#[derive(FromArgs, Debug)]
struct TopLevel {
    #[argh(subcommand)]
    nested: CliOptions,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CliOptions {
    Run(RunOptions),
    Build(BuildOptions),
}

/// Builds provided file or project
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
pub struct BuildOptions {
    /// path to file or project
    #[argh(option, default = "std::env::current_dir().unwrap()")]
    pub path: PathBuf,
}

/// Runs provided file or project
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
pub struct RunOptions {
    /// path to file or project
    #[argh(option, default = "std::env::current_dir().unwrap()")]
    pub path: PathBuf,
}

pub fn init() -> Result<()> {
    let top_level: TopLevel = argh::from_env();
    match top_level.nested {
        CliOptions::Run(options) => code::build(options.path)?,
        CliOptions::Build(options) => code::build(options.path)?,
    }

    Ok(())
}
