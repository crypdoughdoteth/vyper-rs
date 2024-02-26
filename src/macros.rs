//! The Macro module contains powerful macros used to enhance the developer experience.

/// The `vyper!` macro is used to construct the Vyper type without the boilerplate. If there is
/// more than one pair of literals passed into the macro, the macro will return a Vypers.
///
/// Input: any length sequence of expressions that evaluate to a Path.
///
/// ```rust
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::{PathBuf, Path};
///  use vyper_rs::vyper_errors::VyperErrors;
///  fn try_me() -> Result<(), VyperErrors> {
///     let _: Vyper = vyper!("./multisig.vy");
///     let _: Vypers = vyper!("./multisig.vy", "./multisig.vy");
///     Ok(())
///  }
///  ```
#[macro_export]
macro_rules! vyper {

    ($p1: expr) => {
       Vyper::new(Path::new($p1))
    };
    ($($p1: expr),+) => {
        {
            let mut contracts: Vec<PathBuf> = vec![];
            $(
                let v = PathBuf::from($p1);
                contracts.push(v);
            )+
            Vypers::new(contracts)
        }
    };
}

/// The `compile!` macro is used to compile one more more Vyper contracts.
///
/// Input: any length sequence of expressions that evaluate to a Path.
///
/// Keywords: venv.
///
/// venv - compile contract using an instance of the Vyper compiler inside a venv.
///
/// ```rust
///  use vyper_rs::venv::*;
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::{Path, PathBuf};
///  use vyper_rs::vyper_errors::VyperErrors;
///  async fn try_me() -> Result<(), VyperErrors> {
///     let _: Vyper = compile!(venv "./multisig.vy");
///     let _: Vyper = compile!("./multisig.vy");
///     let _: Vypers = compile!(venv "./multisig.vy", "./multisig.vy");
///     let _: Vypers = compile!("./multisig.vy", "./multisig.vy");
///     Ok(())
///  }
///  ```
#[macro_export]
macro_rules! compile {
    // classic, simple
    ($p1: expr) => {
        {
            let mut vy: Vyper = vyper!($p1);
            vy.compile()?;
            vy
        }
    };
    // a twist on the classic: compile inside a venv with keyword venv
    (venv $p1: expr) => {
        {
            let mut contract = Venv::default()
                .init()?
                .ivyper_venv(None)?
                .vyper(Path::new("../../multisig.vy"));
           contract.compile()?;
           contract
        }
    };
    // compile many
    ($($p1: expr),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1);
                contracts.push(v);
            )+
            let mut cs: Vypers = Vypers::from(contracts);
            cs.compile_many().await?;
            cs
        }
    };
    // compile many in venv
    (venv $($p1: expr),+) => {
        {
            let mut paths: Vec<PathBuf> = vec![];
            $(
                let v = PathBuf::from($p1);
                paths.push(v);
            )+
            let mut contracts = Venv::default().init()?.ivyper_venv(None)?.vypers(paths);
            contracts.compile_many().await?;
            contracts
        }
    };
}

/// The `abi!` macro is used to compile one more more Vyper contracts and get or generate the ABI.
///
/// Input: any length sequence of expressions that evaluate to a Path.
///
/// Keywords: paris, venv, get.
///
/// venv - compile contract using an instance of the Vyper compiler inside a venv.
///
/// ```rust
///  use vyper_rs::venv::*;
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use vyper_rs::vyper_errors::VyperErrors;
///  use std::path::{PathBuf, Path};
///  use serde_json::Value;
/// async fn try_me() -> Result<(), VyperErrors> {
///     let _: Value = abi!("./multisig.vy");
///     let _: Value = abi!(venv "./multisig.vy");
///     let _: Vec<Value> = abi!("./multisig.vy", "./multisig.vy");   
///     let _: Vec<Value> = abi!(venv "./multisig.vy", "./multisig.vy");   
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! abi {
    // OG matcher
    // return the ABI as json instead of creating a file
    ($p1: expr) => {
        {
            let c: Vyper = compile!($p1);
            c.get_abi()?
        }
    };
    // return the ABI as json instead of creating a file
    (venv $p1: expr) => {
        {
            let mut c: Vyper = compile!(venv $p1);
            c.get_abi()?
        }
    };
    // return many ABIs as json
    ($($p1: expr),+) => {
        {
            let mut paths: Vec<PathBuf> = vec![];
            $(
                let v = PathBuf::from($p1);
                paths.push(v);
            )+
            let cs: Vypers = Vypers::new(paths);
            cs.get_abi_many().await?
        }
    };
    // venv version of many
    (venv $($p1: expr),+) => {
        {
            let mut p: Vec<PathBuf> = vec![];
            $(
                let v = PathBuf::from($p1);
                p.push(v);
            )+
            let mut contracts = Venv::default().init()?.ivyper_venv(None)?.vypers(p);
            contracts.get_abi_many().await?
        }
    };
}
/// The `venv!` macro creates a virtual environment with the latest version of the vyper compiler installed.
/// Optionally, you can pass the desired version of the Vyper compiler you want to install, i.e
/// "0.3.10", as a &str.
///```rust
///
/// use vyper_rs::venv::*;
/// use vyper_rs::*;
/// use vyper_rs::vyper_errors::VyperErrors;
///
/// fn try_me() -> Result<(), VyperErrors> {
///     let _:  Venv<Ready> = venv!();
///     let _: Venv<Ready> = venv!("0.3.10");
///     Ok(())
/// }
///
///```
#[macro_export]
macro_rules! venv {
    () => {{
        Venv::default().init()?.ivyper_venv(None)?
    }};
    ($ver: literal) => {{
        let version: &str = $ver;
        Venv::default().init()?.ivyper_venv(Some(version))?
    }};
}
