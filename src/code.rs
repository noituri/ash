use crate::cli::RunOptions;
use crate::failure::report;
use ash_core::prelude as sv;
use std::error::Error;

pub fn run(options: RunOptions) -> Result<(), Box<dyn Error>> {
    if options.path.is_file() {
        let src = sv::Source::from_file(options.path)?;
        let result = sv::run(&src);
        if let Err(errs) = result {
            errs.into_iter().for_each(|err| report::error(&src, err));
        }
    } else {
        unimplemented!();
    }

    Ok(())
}
