use crate::cli::RunOptions;
use crate::failure::report;
use anyhow::{bail, Result};
use ash_core::prelude as ash;
use ash_vm::prelude::*;

pub fn run(options: RunOptions) -> Result<()> {
    if !options.path.exists() {
        bail!("Path does not exist");
    }
    if options.path.is_file() {
        let src = ash::Source::from_file(options.path)?;
        if let Err(errs) = ash::build(&src) {
            errs.into_iter().for_each(|err| report::error(&src, err));
        }
    } else {
        unimplemented!();
    }

    Ok(())
}
