use ethers::{
    abi::Address,
    contract::ContractFactory,
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Bytes,
};

use std::{
    convert::TryFrom, error::Error, fs::File, path::PathBuf, str::FromStr, sync::Arc,
    time::Duration,
};
use vyper_rs::vyper::Vyper;

pub async fn deploy() -> Result<(), Box<dyn Error>> {
    let cpath: PathBuf = PathBuf::from("../../multisig.vy");
    let mut contract = Vyper::new(&cpath);
    contract.compile()?;
    contract.gen_abi()?;
    let anvil = Anvil::new().spawn();

    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    let provider = Provider::<Http>::try_from(anvil.endpoint())?
        .interval(Duration::from_millis(10u64));

    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
    let client = Arc::new(client);

    let factory = ContractFactory::new(
        ethers::abi::Contract::load(File::open(contract.abi)?)?,
        Bytes::from_str(&contract.bytecode.unwrap())?,
        client,
    );
    println!("{:#?}", factory);
    let owner: Vec<Address> =
        vec!["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse::<Address>()?];
    let _ = factory.deploy(owner)?.send().await?;
    println!("success!");
    Ok(())
}
