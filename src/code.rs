use std::path::PathBuf;
use crate::failure::report;
use anyhow::{bail, Result};
use ash_core::prelude as ash;
use cashier::CashFile;

pub fn build(mut path: PathBuf) -> Result<()> {
    if !path.exists() {
        bail!("Path does not exist");
    }
    if path.is_file() {
        let src = ash::Source::from_file(path.clone())?;
        match ash::build(&src) {
            Ok(_) => {
                path.set_extension("cash");
                CashFile::from_file(path).expect("work during presentation").compile();
            }
            Err(errs) => errs.into_iter().for_each(|err| report::error(&src, err)),
        }
    } else {
        unimplemented!();
    }

    Ok(())
}
