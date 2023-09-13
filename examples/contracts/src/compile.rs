use std::{path::Path, error::Error};
use ethers::prelude::Abigen;
use vyper_rs::vyper::Vyper;

pub fn compile_and_generate_bindings() -> Result<(), Box<dyn Error>> {
    let cpath: &Path = Path::new("../../multisig.vy");
    let abi: &Path = Path::new("./my_abi.json");
    let contract = Vyper::new(cpath, abi);
    contract.abi()?;
    println!("Generating bindings for {contract}\n");

    let _bindings = Abigen::new(
            "MyContract",
            contract.abi
                .to_string_lossy()
                .to_string()
            )?
            .generate()?
            .write_to_file("./MyContract.rs")?;
    Ok(())
}

