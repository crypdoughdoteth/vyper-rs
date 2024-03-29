use ethers::prelude::Abigen;
use std::{error::Error, path::PathBuf};
use vyper_rs::vyper::Vyper;

pub fn compile_and_generate_bindings() -> Result<(), Box<dyn Error>> {
    let cpath: PathBuf = PathBuf::from("../../multisig.vy");
    let contract = Vyper::new(&cpath);
    contract.gen_abi()?;
    println!("Generating bindings for {contract}\n");

    let _bindings =
        Abigen::new("MyContract", contract.abi.to_string_lossy().to_string())?
            .generate()?
            .write_to_file("./MyContract.rs")?;
    Ok(())
}
