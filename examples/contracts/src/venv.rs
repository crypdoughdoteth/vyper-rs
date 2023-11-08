use std::{path::PathBuf, error::Error};
use vyper_rs::{vyper::Vyper, venv::Venv};

pub fn venv_example() -> Result<(), Box<dyn Error>> {
    let cpath: PathBuf = PathBuf::from("../../multisig.vy");
    let abi: PathBuf = PathBuf::from("./my_abi.json");
    let venv = Venv::new()
        .init()?
        .ivyper_venv(None)?; 
    let mut contract = Vyper::new(cpath, abi);
    venv.compile(&mut contract)?;
    Ok(())
}
