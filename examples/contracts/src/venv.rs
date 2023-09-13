use std::{path::Path, error::Error};
use vyper_rs::{vyper::Vyper, venv::Venv};

pub fn venv_example() -> Result<(), Box<dyn Error>> {
    let cpath: &Path = Path::new("../../multisig.vy");
    let abi: &Path = Path::new("./my_abi.json");
    Venv::new()
        .init()?
        .ivyper_venv(None)?; 
    let mut contract = Vyper::new(cpath, abi);
    contract.compile()?;
    Ok(())
}