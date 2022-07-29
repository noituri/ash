use crate::cli::RunOptions;
use std::error::Error;
use svalinn_core::prelude as sv;

pub fn run(options: RunOptions) -> Result<(), Box<dyn Error>> {
    if options.path.is_file() {
        sv::run(sv::Source::from_file(options.path)?);
    } else {
        unimplemented!();
    }

    Ok(())
}
