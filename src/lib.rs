//! This is the documentation for the Vyper-rs crate.
//! Vyper-rs is a library to interact with the vyper compiler and manage versions with a venv.
//! Our goal is to connect Vyper with the robust tooling and infrastructure for the Solidity ecosystem written in Rust.
pub mod macros;
pub mod utils;
pub mod venv;
pub mod vyper;
pub mod vyper_errors;

#[cfg(test)]
mod test {
    use super::*;
    use crate::vyper::{Evm, Vyper, Vypers};
    use std::path::PathBuf;

    #[test]
    fn basic() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path = PathBuf::from("./abi.json");
        let mut vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.compile().unwrap();
        vyper_contract.gen_abi().unwrap();
    }

    #[test]
    fn compile_version() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path = PathBuf::from("./abi.json");
        let mut vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.compile_ver(Evm::Shanghai).unwrap();
    }

    #[test]
    fn concurrent_compilation_vers() {
        tokio_test::block_on(async {
            let path: PathBuf = PathBuf::from("./multisig.vy");
            let path2: PathBuf = PathBuf::from("./multisig.vy");
            let path3: PathBuf = PathBuf::from("./multisig.vy");
            let path4: PathBuf = PathBuf::from("./multisig.vy");
            let abi: PathBuf = PathBuf::from("./abi1.json");
            let abi2: PathBuf = PathBuf::from("./abi2.json");
            let abi3: PathBuf = PathBuf::from("./abi3.json");
            let abi4: PathBuf = PathBuf::from("./abi4.json");
            let mut vyper_contracts =
                Vypers::new(vec![path, path2, path3, path4], vec![abi, abi2, abi3, abi4]);
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
            let abi: PathBuf = PathBuf::from("./abi1.json");
            let abi2: PathBuf = PathBuf::from("./abi2.json");
            let abi3: PathBuf = PathBuf::from("./abi3.json");
            let abi4: PathBuf = PathBuf::from("./abi4.json");
            let mut vyper_contracts =
                Vypers::new(vec![path, path2, path3, path4], vec![abi, abi2, abi3, abi4]);
            vyper_contracts.compile_many().await.unwrap();
            assert!(!vyper_contracts.bytecode.is_none());
        })
    }

    #[test]
    fn interface() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path: PathBuf = PathBuf::from("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.interface().unwrap();
    }

    #[test]
    fn storage() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path = PathBuf::from("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.storage_layout().unwrap();
    }

    #[test]
    fn opcodes() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path = PathBuf::from("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.opcodes().unwrap();
    }

    #[test]
    fn ast() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path = PathBuf::from("./abi.json");
        let vyper_contract = Vyper::new(path, abi_path);
        vyper_contract.ast().unwrap();
    }

    #[test]
    fn bp() {
        let path = PathBuf::from("./multisig.vy");
        let abi_path = PathBuf::from("./abi.json");
        let mut vyper_contract = Vyper::new(path, abi_path);
        Vyper::compile_blueprint(&mut vyper_contract).unwrap();
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
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255
                ]),
                vec![0]
            ),
            (a3, b3, c3)
        );
    }
    use crate::venv::{Ready, Venv};
    #[test]
    fn venv_test() {
        let venv = Venv::new().init().unwrap().ivyper_venv(None).unwrap();
        let mut contract =
            Vyper::new(PathBuf::from("./multisig.vy"), PathBuf::from("./abi.json"));
        venv.compile(&mut contract).unwrap();
    }

    #[test]
    fn version_detect() {
        Vyper::get_version().unwrap();
    }

    #[test]
    fn vyper_macro_test() {
        let c = vyper!("./multisig.vy", "./abi.json");
        let c_assertion =
            Vyper::new(PathBuf::from("./multisig.vy"), PathBuf::from("./abi.json"));
        assert_eq!(c, c_assertion);
        let c2_assertion = vec![
            Vyper::new(PathBuf::from("./multisig.vy"), PathBuf::from("./abi.json")),
            Vyper::new(PathBuf::from("./multisig.vy"), PathBuf::from("./abi.json")),
        ];
        let c2 = vyper!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
        assert_eq!(c2, c2_assertion);
    }

    #[test]
    fn vypers_macro_test() {
        let vys_assertion = Vypers::new(
            vec![
                PathBuf::from("./multisig.vy"),
                PathBuf::from("./multisig.vy"),
            ],
            vec![PathBuf::from("./abi.json"), PathBuf::from("./abi.json")],
        );
        let vys = vypers!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
        assert_eq!(vys, vys_assertion);
    }

    #[test]
    fn compile_macro_test() {
        let mut contract_assertion = vyper!("./multisig.vy", "./abi.json");
        contract_assertion.compile().unwrap();
        let contract = compile!("./multisig.vy", "./abi.json");
        assert_eq!(contract, contract_assertion);
    }

    #[test]
    fn compile_mt_macro_test() {
        tokio_test::block_on(async {
            let mut vys_assertion =
                vypers!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            vys_assertion.compile_many().await.unwrap();
            let vys =
                compile!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            assert_eq!(vys, vys_assertion);
        })
    }

    #[test]
    fn compabi_macro_test() {
        let c_assertion = compile!("./multisig.vy", "./abi.json");
        c_assertion.gen_abi().unwrap();
        let c = abi!("./multisig.vy", "./abi.json");
        assert_eq!(c, c_assertion);
    }

    #[test]
    fn compabi_mt_macro_test() {
        tokio_test::block_on(async {
            let vys_assertion =
                compile!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            vys_assertion.get_abi_many().await.unwrap();
            let vys = abi!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            assert_eq!(vys, vys_assertion);
        })
    }

    #[test]
    fn compabijson_macro_test() {
        let c_assertion = compile!("./multisig.vy", "./abi.json");
        let abi = c_assertion.get_abi().unwrap();
        let c = abi!(get "./multisig.vy", "./abi.json");
        assert_eq!(c, (c_assertion, abi));
    }

    #[test]
    fn compabijson_mt_macro_test() {
        tokio_test::block_on(async {
            let vys_assertion =
                compile!("./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let abis = vys_assertion.get_abi_many().await.unwrap();
            let vys =
                abi!(get "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            assert_eq!(vys, (vys_assertion, abis));
        })
    }

    #[test]
    fn venv_macro_test() {
        let _: Venv<Ready> = venv!();
        let _: Venv<Ready> = venv!("0.3.10");
    }

    #[test]
    fn venv_compile_macro_test() {
        let (_, _): (Vyper, Venv<Ready>) = compile!(venv "./multisig.vy", "./abi.json");
        let _: Vyper = compile!(paris "./multisig.vy", "./abi.json");
        let (_, _): (Vyper, Venv<Ready>) =
            compile!(venv paris "./multisig.vy", "./abi.json");
        tokio_test::block_on(async {
            let _ = compile!(venv "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = compile!(paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = compile!(venv paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
        })
    }
    #[test]
    fn more_abi_tests() {
        tokio_test::block_on(async {
            let _ =
                abi!(venv "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = abi!(venv get "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ =
                abi!(paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = abi!(venv paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = abi!(get paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = abi!(venv get paris "./multisig.vy", "./abi.json", "./multisig.vy", "./abi.json");
            let _ = abi!(venv "./multisig.vy", "./abi.json");
            let _ = abi!(venv get "./multisig.vy", "./abi.json");
            let _ = abi!(paris "./multisig.vy", "./abi.json");
            let _ = abi!(venv paris "./multisig.vy", "./abi.json");
            let _ = abi!(get paris "./multisig.vy", "./abi.json");
            let _ = abi!(venv get paris "./multisig.vy", "./abi.json");
        });
    }
}
