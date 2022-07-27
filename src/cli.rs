use std::path::PathBuf;
use argh::FromArgs;
use crate::code;

/// All commands
#[derive(FromArgs, Debug)]
struct TopLevel {
    #[argh(subcommand)]
    nested: CliOptions
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CliOptions {
    Run(RunOptions)
}

/// Runs provided file or project
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
pub struct RunOptions {
    /// path to file or project
    #[argh(option, default = "std::env::current_dir().unwrap()")]
    path: PathBuf,
}

pub fn init() {
    let top_level: TopLevel = argh::from_env();
    dbg!(&top_level);
    match top_level.nested {
        CliOptions::Run(options) => code::run(options)
    }
}

