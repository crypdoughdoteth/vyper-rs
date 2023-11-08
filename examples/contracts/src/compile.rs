use std::{path::PathBuf, error::Error};
use ethers::prelude::Abigen;
use vyper_rs::vyper::Vyper;

pub fn compile_and_generate_bindings() -> Result<(), Box<dyn Error>> {
    let cpath: PathBuf = PathBuf::from("../../multisig.vy");
    let abi: PathBuf = PathBuf::from("./my_abi.json");
    let contract = Vyper::new(cpath, abi);
    contract.gen_abi()?;
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

