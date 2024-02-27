//! This is the documentation for the Vyper-rs crate.
//! Vyper-rs is a library to interact with the vyper compiler and manage versions with a venv.
//! Our goal is to connect Vyper with the robust tooling and infrastructure for the Solidity ecosystem written in Rust and become the standard compiler interface.

pub mod macros;
pub mod utils;
pub mod venv;
pub mod vyper;
pub mod vyper_errors;

#[cfg(test)]
mod test {
    use self::{vyper::VyperStack, vyper_errors::VyperErrors};

    use super::*;
    use crate::{
        utils::Blueprint,
        vyper::{Evm, Vyper, Vypers},
    };
    use std::path::{Path, PathBuf};

    #[test]
    fn basic() {
        let path = PathBuf::from("./multisig.vy");
        let mut vyper_contract = Vyper::new(&path);
        vyper_contract.compile().unwrap();
        vyper_contract.gen_abi().unwrap();
        assert!(vyper_contract.bytecode.unwrap().starts_with("0x"));
    }

    #[test]
    fn compile_version() {
        let path = PathBuf::from("./multisig.vy");
        let mut vyper_contract = Vyper::new(&path);
        vyper_contract.compile_ver(&Evm::Shanghai).unwrap();
    }

    #[test]
    fn concurrent_compilation_vers() {
        tokio_test::block_on(async {
            let path: PathBuf = PathBuf::from("./multisig.vy");
            let path2: PathBuf = PathBuf::from("./multisig.vy");
            let path3: PathBuf = PathBuf::from("./multisig.vy");
            let path4: PathBuf = PathBuf::from("./multisig.vy");
            let mut vyper_contracts = Vypers::new(vec![path, path2, path3, path4]);
            vyper_contracts
                .compile_many_ver(Evm::Shanghai)
                .await
                .unwrap();
            assert!(!vyper_contracts.bytecode.is_none());
        })
    }

    #[test]
    fn concurrent_compilation() {
        tokio_test::block_on(async {
            let path: PathBuf = PathBuf::from("./multisig.vy");
            let path2: PathBuf = PathBuf::from("./multisig.vy");
            let path3: PathBuf = PathBuf::from("./multisig.vy");
            let path4: PathBuf = PathBuf::from("./multisig.vy");
            let mut vyper_contracts = Vypers::new(vec![path, path2, path3, path4]);
            vyper_contracts.compile_many().await.unwrap();
            assert!(!vyper_contracts.bytecode.is_none());
        })
    }

    #[test]
    fn interface() {
        let path = PathBuf::from("./multisig.vy");
        let vyper_contract = Vyper::new(&path);
        vyper_contract.interface().unwrap();
    }

    #[test]
    fn storage() {
        let path = PathBuf::from("./multisig.vy");
        let vyper_contract = Vyper::new(&path);
        vyper_contract.storage_layout().unwrap();
    }

    #[test]
    fn opcodes() {
        let path = PathBuf::from("./multisig.vy");
        let vyper_contract = Vyper::new(&path);
        vyper_contract.opcodes().unwrap();
    }

    #[test]
    fn ast() {
        let path = PathBuf::from("./multisig.vy");
        let vyper_contract = Vyper::new(&path);
        vyper_contract.ast().unwrap();
    }

    #[test]
    fn bp() {
        let path = PathBuf::from("./multisig.vy");
        let mut vyper_contract = Vyper::new(&path);
        vyper_contract.compile_blueprint().unwrap();
    }

    #[test]
    fn exists() {
        assert_eq!(true, Vyper::exists(&Vyper::new(Path::new("./multisig.vy"))))
    }

    #[test]
    fn parse_bp() {
        let case1 = b"\xFE\x71\x00\x00";
        let case2 = b"\xFE\x71\x01\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x00";
        let case3 = b"\xFE\x71\x02\x01\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x00";
        {
            let Blueprint {
                erc_version,
                preamble_data,
                initcode,
            } = utils::parse_blueprint(case1).unwrap();
            assert_eq!((0u8, None, vec![0]), (erc_version, preamble_data, initcode));
        }
        {
            let Blueprint {
                erc_version,
                preamble_data,
                initcode,
            } = utils::parse_blueprint(case2).unwrap();
            assert_eq!(
                (0u8, Some(vec![255, 255, 255, 255, 255, 255, 255]), vec![0]),
                (erc_version, preamble_data, initcode)
            );
        }
        {
            let Blueprint {
                erc_version,
                preamble_data,
                initcode,
            } = utils::parse_blueprint(case3).unwrap();
            assert_eq!(
                (
                    0u8,
                    Some(vec![
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255
                    ]),
                    vec![0]
                ),
                (erc_version, preamble_data, initcode)
            );
        }
    }
    use crate::venv::{Ready, Venv};
    #[test]
    fn venv_test() {
        let mut contract = Venv::default()
            .init()
            .unwrap()
            .ivyper_venv(None)
            .unwrap()
            .vyper(Path::new("./multisig.vy"));
        contract.compile().unwrap();
    }

    #[test]
    fn version_detect() {
        Vyper::get_version(&Vyper::new(Path::new("./multisig.vy"))).unwrap();
    }

    #[test]
    fn vyper_macro_test() {
        let c = vyper!("./multisig.vy");
        let c_assertion = Vyper::new(Path::new("./multisig.vy"));
        assert_eq!(c, c_assertion);
        let c2_assertion = Vypers::from(vec![
            Vyper::new(Path::new("./multisig.vy")),
            Vyper::new(Path::new("./multisig.vy")),
        ]);
        let c2 = vyper!("./multisig.vy", "./multisig.vy");
        assert_eq!(c2, c2_assertion);
    }

    #[test]
    fn vypers_macro_test() {
        let vys_assertion = Vypers::new(vec![
            PathBuf::from("./multisig.vy"),
            PathBuf::from("./multisig.vy"),
        ]);
        let vys = vyper!("./multisig.vy", "./multisig.vy");
        assert_eq!(vys, vys_assertion);
    }

    #[test]
    fn compile_macro_test() -> Result<(), VyperErrors> {
        let mut contract_assertion = vyper!("./multisig.vy");
        contract_assertion.compile().unwrap();
        let contract = compile!("./multisig.vy");
        assert_eq!(contract, contract_assertion);
        Ok(())
    }

    #[tokio::test]
    async fn compile_mt_macro_test() -> Result<(), VyperErrors> {
        let mut vys_assertion = vyper!("./multisig.vy", "./multisig.vy");
        vys_assertion.compile_many().await.unwrap();
        let vys = compile!("./multisig.vy", "./multisig.vy");
        assert_eq!(vys, vys_assertion);
        Ok(())
    }

    #[test]
    fn compabijson_macro_test() -> Result<(), VyperErrors> {
        let c_assertion = compile!("./multisig.vy");
        let abi = c_assertion.get_abi().unwrap();
        let c = abi!("./multisig.vy");
        assert_eq!(c, abi);
        Ok(())
    }

    #[test]
    fn compabijson_mt_macro_test() -> Result<(), VyperErrors> {
        tokio_test::block_on(async {
            let vys_assertion = compile!("./multisig.vy", "./multisig.vy");
            let abis = vys_assertion.get_abi_many().await.unwrap();
            let vys = abi!("./multisig.vy", "./multisig.vy");
            assert_eq!(vys, abis);
            Ok(())
        })
    }

    #[test]
    fn venv_macro_test() -> Result<(), VyperErrors> {
        let _: Venv<Ready> = venv!();
        let _: Venv<Ready> = venv!("0.3.10");
        Ok(())
    }

    #[test]
    fn test_stack_mt() -> Result<(), VyperErrors> {
        let mut stack = [
            Vyper::new(Path::new("./multsig.vy")),
            Vyper::new(Path::new("./multsig.vy")),
        ];
        let mut contracts = VyperStack(&mut stack);
        contracts.gen_abi_many()?;
        contracts.compile_many()?;
        Ok(())
    }
}
