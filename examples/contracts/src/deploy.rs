use ethers::{
    contract::ContractFactory,
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer}, abi::Address, types::Bytes,
};

use vyper_rs::vyper::Vyper;
use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration, error::Error, fs::File, str::FromStr};


pub async fn deploy() -> Result<(), Box<dyn Error>> {
 
    let cpath: &Path = Path::new("../../multisig.vy");
    let abi: &Path = Path::new("./my_abi.json");
    let mut contract = Vyper::new(cpath, abi);
    contract.compile()?;
    contract.abi()?;
    let anvil = Anvil::new().spawn();

    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
    let client = Arc::new(client);

    let factory = ContractFactory::new(ethers::abi::Contract::load(File::open(contract.abi)?)?, Bytes::from_str(&contract.bytecode.unwrap())?, client);
    println!("{:#?}", factory);
    let owner: Vec<Address> = vec!["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse::<Address>()?];
    let _ = factory.deploy(owner)?.send().await?;
    println!("success!");    
    Ok(())

}

