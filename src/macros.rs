//! The Macro module contains powerful macros used to enhance the developer experience. Some of
//! these macros are structural in the sense that they construct essential types. The
//! vyper!, vypers!, and venv! macros fall into this category. Furthermore, there are
//! macros such as compile! and abi! that make common tasks simple. Both compile! and abi! have
//! numerous match arms and some shared keywords.

/// The `vyper!` macro is used to construct the Vyper type without the boilerplate. If there is
/// more than one pair of literals passed into the macro, the macro will return a Vec<Vyper>.
///
/// Input: any length sequence of paired string literals (one for the contract, one for the abi (or desired path)).
///
/// ```rust
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::PathBuf;
///  fn try_me() {
///     let _: Vyper = vyper!("./multisig.vy");
///     let _: Vypers = vyper!("./multisig.vy", "./multisig.vy");
///  }
///  ```
#[macro_export]
macro_rules! vyper {

    ($p1: expr) => {
       Vyper::new(Path::new($p1))
    };
    ($($p1: expr),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1);
                contracts.push(v);
            )+
            Vypers::from(contracts)
        }
    };
}

/// The `compile!` macro is used to compile one more more Vyper contracts.
///
/// Input: any length sequence of paired string literals (one for the contract, one for the abi (or desired path)).
///
/// Keywords: venv.
///
/// venv - compile contract using an instance of the Vyper compiler inside a venv.
///
/// ```rust
///  use vyper_rs::venv::*;
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::PathBuf;
///  async fn try_me() {
///     let _: Vyper = compile!(venv "./multisig.vy");
///     let _: Vyper = compile!("./multisig.vy");
///     let _: Vypers = compile!(venv "./multisig.vy", "./multisig.vy");
///     let _: Vypers = compile!("./multisig.vy", "./multisig.vy");
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
            let mut contracts = Venv::default().init()?.ivyper_venv(None)?.vypers(contracts);
            contracts.compile_many().await?;
            contracts 
        }
    };
}

/// The `abi!` macro is used to compile one more more Vyper contracts and get or generate the ABI.
///
/// Input: any length sequence of paired string literals (one for the contract, one for the abi (or desired path)).
///
/// Keywords: paris, venv, get.
///
/// paris - compile contract for the Paris version of the EVM.
///
/// venv - compile contract using an instance of the Vyper compiler inside a venv.
///
/// get - instead of generating the ABI as a file, return it as JSON.
///
/// These keywords can be combined with one another just like with `compile!`.
/// ```rust
///  use vyper_rs::venv::*;
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::PathBuf;
///  use serde_json::Value;
/// async fn try_me() {
///     let _: Vyper = abi!("./multisig.vy", "./abi.json");
///     let _: (Vyper, Value) = abi!(get "./multisig.vy", "./abi.json");
///     let _: (Vyper, Venv<Ready>) = abi!(venv "./multisig.vy", "./abi.json");
///     let _: (Vyper, Value, Venv<Ready>) = abi!(venv get "./multisig.vy", "./abi.json");
///     let _: (Vyper, Value, Venv<Ready>) = abi!(venv get paris "./multisig.vy", "./abi.json");
///     let _: Vypers = abi!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");   
///     let _: (Vypers, Vec<Value>, Venv<Ready>) =  abi!(venv get "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
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
/// fn try_me() {
///     let _: Venv<Ready> = venv!();
///     let _: Venv<Ready> = venv!("0.3.10");
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
        Venv::default()
            .init()?
            .ivyper_venv(Some(version))?
    }};
}
