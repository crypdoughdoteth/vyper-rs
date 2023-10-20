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
///     let _: Vyper = vyper!("./multisig.vy", "./abi.json");
///     let _: Vec<Vyper> = vyper!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
///  }
///  ```
#[macro_export]
macro_rules! vyper {
    ($p1: literal, $p2: literal) => {
        Vyper {path_to_code: PathBuf::from($p1), bytecode: None, abi: PathBuf::from($p2)}
    };
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            contracts
        }
    };
}

/// The `vypers!` macro is used to construct the Vypers type without the boilerplate.
///
/// Input: any length sequence of paired string literals (one for the contract, one for the abi (or desired path)).
///
/// ```rust
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::PathBuf;
///  fn try_me() {
///     let _: Vypers = vypers!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
///  }
///  ```
#[macro_export]
macro_rules! vypers{
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let cs: Vypers = contracts.into();
            cs
        }
    };
}
/// The `compile!` macro is used to compile one more more Vyper contracts.
///
/// Input: any length sequence of paired string literals (one for the contract, one for the abi (or desired path)).
///
/// Keywords: paris, venv.
///
/// paris - compile contract for the Paris version of the EVM.
///
/// venv - compile contract using an instance of the Vyper compiler inside a venv.
///
/// These keywords can even be used together!
///
/// ```rust
///  use vyper_rs::venv::*;
///  use vyper_rs::vyper::*;
///  use vyper_rs::*;
///  use std::path::PathBuf;
///  async fn try_me() {
///     let _: (Vypers, Venv<Ready>) = compile!(venv paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
///     let _: (Vyper, Venv<Ready>) = compile!(venv "./multisig.vy", "./abi.json");
///     let _: Vyper = compile!("./multisig.vy", "./abi.json");
///     let _: Vyper = compile!(paris "./multisig.vy", "./abi.json");
///  }
///  ```
#[macro_export]
macro_rules! compile {
    // classic, simple
    ($p1: literal, $p2: literal) => {
        {
            let mut vy: Vyper = vyper!($p1, $p2);
            vy.compile().unwrap();
            vy
        }
    };
    // a twist on the classic: compile inside a venv with keyword venv
    (venv $p1: literal, $p2: literal) => {
        {
            let venv: Venv<Ready> = venv!();
            let mut vy: Vyper = vyper!($p1, $p2);
            venv.compile(&mut vy).unwrap();
            (vy, venv)
        }
    };
    // EVM Target: Paris
    (paris $p1: literal, $p2: literal) => {
        {
            let mut vy: Vyper = vyper!($p1, $p2);
            vy.compile_ver(Evm::Paris).unwrap();
            vy
        }
    };
    // compile to paris inside venv
    (venv paris $p1: literal, $p2: literal) => {
        {
            let venv: Venv<Ready> = venv!();
            let mut vy: Vyper = vyper!($p1, $p2);
            venv.compile_ver(&mut vy, Evm::Paris).unwrap();
            (vy, venv)
        }
    };
    // compile many
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many().await.unwrap();
            cs
        }
    };
    // compile many in venv
    (venv $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many(&mut cs).await.unwrap();
            (cs, venv)
        }
    };
   // Compile many for Paris
    (paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many_ver(Evm::Paris).await.unwrap();
            cs
        }
    };
    // compile many for paris inside venv
    (venv paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many_ver(&mut cs, Evm::Paris).await.unwrap();
            (cs, venv)
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
    ($p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!($p1, $p2);
            c.gen_abi().unwrap();
            c
        }
    };
    // OG matcher with a venv
    (venv $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv $p1, $p2);
            v.gen_abi(&c).unwrap();
            (c, v)
        }
    };
    // return the ABI as json instead of creating a file
    (get $p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!($p1, $p2);
            let abi = c.get_abi().unwrap();
            (c, abi)
        }
    };
    // returns the ABI as JSON, Vypers struct, and Venv
    (venv get $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv $p1, $p2);
            let abi = v.get_abi(&c).unwrap();
            (c, abi, v)
        }
    };
    // gen abi, compile for paris
    (paris $p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!(paris $p1, $p2);
            c.gen_abi().unwrap();
            c
        }
    };
    // return abi in json form, compile for paris
    (get paris $p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!(paris $p1, $p2);
            let abi = c.get_abi().unwrap();
            (c, abi)
        }
    };
    // gen abi, compile for paris
    (venv paris $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv paris $p1, $p2);
            v.gen_abi(&c).unwrap();
            (c, v)
        }
    };
    // returns contract, abi in JSON form, venv
    (venv get paris $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv paris $p1, $p2);
            let abi = v.get_abi(&c).unwrap();
            (c, abi, v)
        }
    };

    // Generate many ABIs
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many().await.unwrap();
            cs.gen_abi_many().await.unwrap();
            cs
        }
    };
    // return many ABIs as json
    (get $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many().await.unwrap();
            let abis = cs.get_abi_many().await.unwrap();
            (cs, abis)
        }
    };
    // venv version of many
    (venv $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many(&mut cs).await.unwrap();
            venv.gen_abi_many(&cs).await.unwrap();
            (cs, venv)
        }
    };
    // return many ABIs w/ venv
    (venv get $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many(&mut cs).await.unwrap();
            let abi = venv.get_abi_many(&cs).await.unwrap();
            (cs, abi, venv)
        }
    };
    // gen abis compiled for Paris hard fork
    (paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many_ver(Evm::Paris).await.unwrap();
            cs.gen_abi_many().await.unwrap();
            cs
        }
    };
    // gen abis compiled for Paris hard fork w/ venv
    (venv paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many_ver(&mut cs, Evm::Paris).await.unwrap();
            venv.gen_abi_many(&cs).await.unwrap();
            (cs, venv)
        }
    };
    // return many abis compiled for paris
    (get paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many_ver(Evm::Paris).await.unwrap();
            let abi = cs.get_abi_many().await.unwrap();
            (cs, abi)
        }
    };
    // return many abis compiled for paris w/ venv
    (venv get paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many_ver(&mut cs, Evm::Paris).await.unwrap();
            let abi = venv.get_abi_many(&cs).await.unwrap();
            (cs, abi, venv)
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
        Venv::new().init().unwrap().ivyper_venv(None).unwrap()
    }};
    ($ver: literal) => {{
        let version: &str = $ver;
        Venv::new()
            .init()
            .unwrap()
            .ivyper_venv(Some(version))
            .unwrap()
    }};
}
