use std::{error::Error, path::Path};
use vyper_rs::venv::Venv;

pub fn venv_example() -> Result<(), Box<dyn Error>> {
    let mut contract = Venv::default()
        .init()?
        .ivyper_venv(None)?
        .vyper(Path::new("../../multisig.vy"));
    contract.compile()?;
    Ok(())
}
