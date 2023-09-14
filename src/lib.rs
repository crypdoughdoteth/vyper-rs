//! This is the documentation for the Vyper-rs crate.
//! Vyper-rs is a library to interact with the vyper compiler and manage versions with a venv.
//! Our goal is to connect Vyper with the robust tooling and infrastructure for the Solidity ecosystem written in Rust.
use crate::vyper::Vyper;
use serde_json::{to_writer_pretty, Value};
use std::{error::Error, fmt::Display, fs::File, path::Path, process::Command};
pub mod vyper;
pub mod vyper_errors;
use itertools::izip;
pub mod utils;
pub mod venv;

#[cfg(test)]
mod test {
    use super::*;
    use crate::venv::Venv;
    use crate::vyper::{Evm, Vypers};
    use crate::vyper_errors::CompilerError;

    #[test]
    fn basic() {
        let path = Path::new("./multisig.vy");
        let abi_path = Path::new("./abi.json");
        let mut vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.compile().unwrap();
        vyper_contract.abi().unwrap();
    }

    #[test]
    fn compile_version() {
        let path = Path::new("./multisig.vy");
        let abi_path = Path::new("./abi.json");
        let mut vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.compile_ver(Evm::Shanghai).unwrap();
    }

    #[test]
    fn concurrent_compilation_vers() {
        tokio_test::block_on(async {
            let path: &Path = Path::new("./multisig.vy");
            let path2: &Path = Path::new("./multisig.vy");
            let path3: &Path = Path::new("./multisig.vy");
            let path4: &Path = Path::new("./multisig.vy");
            let abi: &Path = Path::new("./abi1.json");
            let abi2: &Path = Path::new("./abi2.json");
            let abi3: &Path = Path::new("./abi3.json");
            let abi4: &Path = Path::new("./abi4.json");
            let mut vyper_contracts =
                Vypers::new(vec![path, path2, path3, path4], vec![abi, abi2, abi3, abi4]);
            vyper_contracts.compile_ver(Evm::Shanghai).await.unwrap();
            assert!(!vyper_contracts.bytecode.is_none());
        })
    }

    #[test]
    fn concurrent_compilation() {
        tokio_test::block_on(async {
            let path: &Path = Path::new("./multisig.vy");
            let path2: &Path = Path::new("./multisig.vy");
            let path3: &Path = Path::new("./multisig.vy");
            let path4: &Path = Path::new("./multisig.vy");
            let abi: &Path = Path::new("./abi1.json");
            let abi2: &Path = Path::new("./abi2.json");
            let abi3: &Path = Path::new("./abi3.json");
            let abi4: &Path = Path::new("./abi4.json");
            let mut vyper_contracts =
                Vypers::new(vec![path, path2, path3, path4], vec![abi, abi2, abi3, abi4]);
            vyper_contracts.compile().await.unwrap();
            assert!(!vyper_contracts.bytecode.is_none());
        })
    }

    #[test]
    fn interface() {
        let path = Path::new("./multisig.vy");
        let abi_path: &Path = Path::new("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.interface().unwrap();
    }

    #[test]
    fn storage() {
        let path = Path::new("./multisig.vy");
        let abi_path = Path::new("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.storage_layout().unwrap();
    }

    #[test]
    fn opcodes() {
        let path = Path::new("./multisig.vy");
        let abi_path = Path::new("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.opcodes().unwrap();
    }

    #[test]
    fn ast() {
        let path = Path::new("./multisig.vy");
        let abi_path = Path::new("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.ast().unwrap();
    }

    #[test]
    fn bp() {
        let path = Path::new("./multisig.vy");
        let abi_path = Path::new("./abi.json");
        let mut vyper_contract = Vyper::new(path, abi_path);
        Vyper::compile_blueprint(&mut vyper_contract).unwrap();
        println!("{}", vyper_contract.bytecode.unwrap());
    }

    #[test]
    fn exists() {
        assert_eq!(true, Vyper::exists())
    }

    #[test]
    fn parse_bp() {
        let case1 = b"\xFE\x71\x00\x00";
        let case2 = b"\xFE\x71\x01\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x00";
        let case3 = b"\xFE\x71\x02\x01\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x00";
        // println!("{:?}", utils::parse_blueprint(case1).unwrap());
        // println!("{:?}", utils::parse_blueprint(case2).unwrap());
        // println!("{:?}", utils::parse_blueprint(case3).unwrap());
        let (a, b, c) = utils::parse_blueprint(case1).unwrap();
        assert_eq!((0u8, None, vec![0]), (a, b, c));
        let (a2, b2, c2) = utils::parse_blueprint(case2).unwrap();
        assert_eq!(
            (0u8, Some(vec![255, 255, 255, 255, 255, 255, 255]), vec![0]),
            (a2, b2, c2)
        );
        let (a3, b3, c3) = utils::parse_blueprint(case3).unwrap();
        assert_eq!(
            (
                0u8,
                Some(vec![
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
                ]),
                vec![0]
            ),
            (a3, b3, c3)
        );
    }
}
