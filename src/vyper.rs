//! This is the main module of the crate. Uses the global installation of Vyper.

use crate::{
    utils::{self, get_contracts_in_dir},
    vyper_errors::VyperErrors,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_writer_pretty, Value};
use std::{
    borrow::BorrowMut,
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    thread,
};
use tokio::task::JoinHandle;

/// Represents important information about a Vyper contract. ABI doesn't need to point to an
/// existing file since it can just be generated using `gen_abi()`. If the ABI already exists at the given path, you can use serde_json to retrieve it from a file.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vyper<'a> {
    pub path_to_code: &'a Path,
    pub bytecode: Option<String>,
    pub abi: PathBuf,
    pub venv: Option<&'a Path>,
}

impl<'a> Display for Vyper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nRoot path: {:?}, \nContract Bytecode: {:?}, \nContract Abi: {:?}",
            self.path_to_code, self.bytecode, self.abi
        )
    }
}

impl<'a> Vyper<'a> {
    /// Constructor function that takes in the path to your vyper contract
    pub fn new(path: &'a Path) -> Self {
        let np = path.with_extension("json");
        Self {
            path_to_code: path,
            bytecode: None,
            abi: np,
            venv: None,
        }
    }

    pub fn with_abi(root: &'a Path, abi_path: PathBuf) -> Self {
        Self {
            path_to_code: root,
            bytecode: None,
            abi: abi_path,
            venv: None,
        }
    }

    pub fn with_venv(path: &'a Path, venv: &'a Path) -> Vyper<'a> {
        let abi = path.with_extension("json");

        Vyper {
            path_to_code: path,
            bytecode: None,
            abi,
            venv: Some(venv),
        }
    }

    pub fn with_venv_and_abi(path: &'a Path, venv: &'a Path, abi: PathBuf) -> Vyper<'a> {
        Vyper {
            path_to_code: path,
            bytecode: None,
            abi,
            venv: Some(venv),
        }
    }

    pub fn abi_mut(&mut self) -> &mut PathBuf {
        self.abi.borrow_mut()
    }

    pub fn abi_exists(&self) -> bool {
        self.abi.exists()
    }

    pub fn contract_exists(&self) -> bool {
        self.path_to_code.exists()
    }

    pub fn get_vyper(&self) -> String {
        if let Some(venv) = self.venv {
            if cfg!(target_os = "windows") {
                format!("{}/scripts/vyper", venv.to_string_lossy())
            } else {
                format!("{}/bin/vyper", venv.to_string_lossy())
            }
        } else {
            "vyper".to_owned()
        }
    }

    pub fn get_pip(&self) -> String {
        if let Some(venv) = self.venv {
            if cfg!(target_os = "windows") {
                format!("{}/scripts/pip3", venv.to_string_lossy())
            } else {
                format!("{}/bin/pip3", venv.to_string_lossy())
            }
        } else {
            "pip3".to_owned()
        }
    }

    pub fn exists(&self) -> bool {
        Command::new(self.get_vyper()).arg("-h").output().is_ok()
    }

    /// check the version of the vyper compiler
    pub fn get_version(&self) -> Result<String, VyperErrors> {
        let out = Command::new(self.get_vyper()).arg("--version").output()?;
        if !out.status.success() {
            Err(VyperErrors::CompilerError(
                "Couldn't locate version info, installation does not exist".to_string(),
            ))?
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    /// Compiles a vyper contract by invoking the vyper compiler, updates the ABI field in the Vyper struct
    pub fn compile(&mut self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg(self.path_to_code)
            .output()?;
        if compiler_output.status.success() {
            let mut out = String::from_utf8_lossy(&compiler_output.stdout).to_string();
            for _ in 0..1 {
                out.pop();
            }
            if !out.starts_with("0x") {
                self.bytecode = out.split(":").last().map(|s| s.to_owned());
            } else {
                self.bytecode = Some(out);
            }

            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }

    pub fn compile_blueprint(&mut self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("blueprint_bytecode")
            .arg(self.path_to_code)
            .output()?;
        if compiler_output.status.success() {
            let mut out = String::from_utf8_lossy(&compiler_output.stdout).to_string();
            for _ in 0..1 {
                out.pop();
            }
            if !out.starts_with("0x") {
                self.bytecode = out.split(":").last().map(|s| s.to_owned());
            } else {
                self.bytecode = Some(out);
            }
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }

    /// Compiles a vyper contract by invoking the vyper compiler, arg for specifying the EVM version to compile to
    pub fn compile_ver(&mut self, ver: &Evm) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg(self.path_to_code)
            .arg("--evm-version")
            .arg(ver.to_string())
            .output()?;

        if compiler_output.status.success() {
            let mut out = String::from_utf8_lossy(&compiler_output.stdout).to_string();
            for _ in 0..1 {
                out.pop();
            }
            if !out.starts_with("0x") {
                self.bytecode = out.split(":").last().map(|s| s.to_owned());
            } else {
                self.bytecode = Some(out);
            }
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// Generates the ABI and creates a file @ the abi path specified in the Vyper struct
    pub fn gen_abi(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("abi")
            .arg(self.path_to_code)
            .output()?;

        if compiler_output.status.success() {
            let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                &compiler_output.stdout,
            ))?;

            let file = File::create(&self.abi)?;

            to_writer_pretty(file, &json)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }

    /// Generates the ABI and creates a file @ the abi path specified in the Vyper struct
    pub fn get_abi(&self) -> Result<Value, VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("abi")
            .arg(self.path_to_code)
            .output()?;

        if compiler_output.status.success() {
            let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                &compiler_output.stdout,
            ))?;
            Ok(json)
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }

    /// Storage layout as JSON, saves it to a file
    pub fn storage_layout(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("layout")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                &compiler_output.stdout,
            ))?;
            let file = File::create("./storage_layout.json")?;
            to_writer_pretty(file, &json)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// AST of your contract as JSON, saves it to a file
    pub fn ast(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("ast")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                &compiler_output.stdout,
            ))?;
            let file: File = File::create("./ast.json")?;
            to_writer_pretty(file, &json)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// Generates an external interface for your vyper contract to be called with
    pub fn interface(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("external_interface")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;
        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./interface.vy")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// Generates the opcodes produced by your vyper contract, saves it as a text file
    pub fn opcodes(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("opcodes")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./opcodes.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// Generates the opcodes produced by your vyper contract at runtime, saves it as a text file
    pub fn opcodes_runtime(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("opcodes_runtime")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./opcodes_runtime.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// Natspec user documentation for vyper contract
    pub fn userdoc(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("userdoc")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;
        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./userdoc.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
    /// Natspec dev documentation for vyper contract
    pub fn devdoc(&self) -> Result<(), VyperErrors> {
        let compiler_output = Command::new(self.get_vyper())
            .arg("-f")
            .arg("devdoc")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;
        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./devdoc.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            Err(VyperErrors::CompilerError(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            ))?
        }
    }
}

/// Represents multiple vyper contract allocated on the stack, synchronous / blocking API for
/// multiple compilations with scoped threads
#[derive(Debug, Hash, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct VyperStack<'a>(pub &'a mut [Vyper<'a>]);

impl<'a> VyperStack<'a> {
    pub fn compile_many(&mut self) -> Result<(), VyperErrors> {
        thread::scope(|s| {
            for i in self.0.iter_mut() {
                s.spawn(|| -> Result<(), VyperErrors> {
                    i.compile()?;
                    Ok(())
                });
            }
        });

        Ok(())
    }

    pub fn compile_many_ver(&mut self, evm_version: &Evm) -> Result<(), VyperErrors> {
        thread::scope(|s| {
            for i in self.0.iter_mut() {
                s.spawn(|| -> Result<(), VyperErrors> {
                    i.compile_ver(evm_version)?;
                    Ok(())
                });
            }
        });

        Ok(())
    }

    pub fn gen_abi_many(&self) -> Result<(), VyperErrors> {
        thread::scope(|s| {
            for i in self.0.iter() {
                s.spawn(|| -> Result<(), VyperErrors> {
                    i.gen_abi()?;
                    Ok(())
                });
            }
        });

        Ok(())
    }
}

/// Represents multiple vyper contracts
#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize,
)]
pub struct Vypers {
    pub path_to_code: Vec<PathBuf>,
    pub bytecode: Option<Vec<String>>,
    pub abi: Vec<PathBuf>,
    pub venv: Option<PathBuf>,
}

impl Vypers {
    /// Constructor function that takes in the paths to your vyper contracts and the _desired paths/{names}.json for your ABIs
    pub fn with_all(
        paths: Vec<PathBuf>,
        abi_paths: Vec<PathBuf>,
        venv: Option<PathBuf>,
    ) -> Self {
        if paths.len() != abi_paths.len() {
            panic!("Mismatched Vector Lengths");
        }

        Self {
            path_to_code: paths,
            bytecode: None,
            abi: abi_paths,
            venv,
        }
    }

    pub fn new(paths: Vec<PathBuf>) -> Self {
        let np = paths.iter().map(|e| e.with_extension("json")).collect();
        Self {
            path_to_code: paths,
            bytecode: None,
            abi: np,
            venv: None,
        }
    }

    pub fn in_dir(path: PathBuf) -> Option<Vypers> {
        if let Ok(contracts) = get_contracts_in_dir(path) {
            Some(Vypers::new(contracts))
        } else {
            None
        }
    }

    pub async fn in_workspace(path: PathBuf) -> Option<Vypers> {
        if let Ok(contracts) = utils::scan_workspace(path).await {
            Some(Vypers::new(contracts))
        } else {
            None
        }
    }

    pub fn with_venv(paths: Vec<PathBuf>, venv: &Path) -> Self {
        let abis = paths.iter().map(|e| e.with_extension("json")).collect();

        Self {
            path_to_code: paths,
            bytecode: None,
            abi: abis,
            venv: Some(venv.to_path_buf()),
        }
    }

    pub fn set_venv(mut self, venv: PathBuf) -> Vypers {
        self.venv = Some(venv);
        self
    }
    pub fn get_vyper(&self) -> String {
        if let Some(venv) = &self.venv {
            if cfg!(target_os = "windows") {
                format!("{}/scripts/vyper", venv.to_string_lossy())
            } else {
                format!("{}/bin/vyper", venv.to_string_lossy())
            }
        } else {
            "vyper".to_owned()
        }
    }

    pub fn get_pip(&self) -> String {
        if let Some(venv) = &self.venv {
            if cfg!(target_os = "windows") {
                format!("{}/scripts/pip3", venv.to_string_lossy())
            } else {
                format!("{}/bin/pip3", venv.to_string_lossy())
            }
        } else {
            "pip3".to_owned()
        }
    }

    /// Compile multiple vyper contracts concurrently on new threads, updates the ABI field in Vypers
    pub async fn compile_many(&mut self) -> Result<(), VyperErrors> {
        let path = Arc::new(self.path_to_code.clone());
        let mut out_vec: Vec<String> = Vec::with_capacity(self.path_to_code.len());
        let mut threads: Vec<JoinHandle<Result<String, VyperErrors>>> = vec![];
        let vy: Arc<String> = Arc::new(self.get_vyper());
        for i in 0..self.path_to_code.len() {
            let paths = Arc::clone(&path);
            let bin = Arc::clone(&vy);
            let cthread = tokio::spawn(async move {
                let compiler_output =
                    Command::new(bin.as_str()).arg(&paths[i]).output()?;
                if compiler_output.status.success() {
                    let mut out =
                        String::from_utf8_lossy(&compiler_output.stdout).to_string();

                    for _ in 0..1 {
                        out.pop();
                    }
                    if !out.starts_with("0x") {
                        if let Some(e) = out.split(":").last() {
                            Ok(e.to_owned())
                        } else {
                            Err(VyperErrors::StringParsingError)
                        }
                    } else {
                       Ok(out) 
                    }
                } else {
                    Err(VyperErrors::CompilerError(
                        String::from_utf8_lossy(&compiler_output.stderr).to_string(),
                    ))?
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            let x = child_thread.await??;
            out_vec.push(x);
        }
        self.bytecode = Some(out_vec);
        Ok(())
    }

    /// Compile multiple vyper contracts concurrently on new threads, updates the ABI field in Vypers. `Ver` arg is for specifying EVM version to compile each contract to.
    pub async fn compile_many_ver(&mut self, ver: Evm) -> Result<(), VyperErrors> {
        let path = Arc::new(self.path_to_code.clone());
        let vy = Arc::new(self.get_vyper());
        let mut out_vec: Vec<String> = Vec::with_capacity(self.path_to_code.len());
        let version = ver.to_string();
        let mut threads: Vec<JoinHandle<Result<String, VyperErrors>>> = vec![];
        for i in 0..self.path_to_code.len() {
            let paths = Arc::clone(&path);
            let bin = Arc::clone(&vy);
            let cver = version.clone();
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new(bin.as_str())
                    .arg(&paths[i])
                    .arg("--evm-version")
                    .arg(cver)
                    .output()?;
                if compiler_output.status.success() {
                    let mut out =
                        String::from_utf8_lossy(&compiler_output.stdout).to_string();
                    for _ in 0..1 {
                        out.pop();
                    }
                    if !out.starts_with("0x") {
                        if let Some(e) = out.split(":").last() {
                            Ok(e.to_owned())
                        } else {
                            Err(VyperErrors::StringParsingError)
                        }
                    } else {
                       Ok(out) 
                    }
                } else {
                    Err(VyperErrors::CompilerError(
                        String::from_utf8_lossy(&compiler_output.stderr).to_string(),
                    ))?
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            let x = child_thread.await??;
            out_vec.push(x);
        }
        self.bytecode = Some(out_vec);
        Ok(())
    }

    /// Generates ABIs for each vyper contract concurrently
    pub async fn gen_abi_many(&mut self) -> Result<(), VyperErrors> {
        let abi_path = Arc::new(self.abi.clone());
        let vy = Arc::new(self.get_vyper());
        let c_path = Arc::new(self.path_to_code.clone());
        let mut threads: Vec<JoinHandle<Result<(), VyperErrors>>> = vec![];
        for i in 0..c_path.len() {
            let c = Arc::clone(&c_path);
            let abi = Arc::clone(&abi_path);
            let bin = Arc::clone(&vy);
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new(bin.as_str())
                    .arg("-f")
                    .arg("abi")
                    .arg(&c[i])
                    .output()?;
                if compiler_output.status.success() {
                    let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                        &compiler_output.stdout,
                    ))?;
                    let file = File::create(&abi[i])?;
                    to_writer_pretty(file, &json)?;
                } else {
                    Err(VyperErrors::CompilerError(
                        String::from_utf8_lossy(&compiler_output.stderr).to_string(),
                    ))?
                }
                Ok(())
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            child_thread.await??
        }
        Ok(())
    }

    pub async fn get_abi_many(&self) -> Result<Vec<Value>, VyperErrors> {
        let c_path = Arc::new(self.path_to_code.clone());
        let mut threads: Vec<JoinHandle<Result<Value, VyperErrors>>> = vec![];
        let vy = Arc::new(self.get_vyper());
        for i in 0..self.path_to_code.len() {
            let c = Arc::clone(&c_path);
            let bin = Arc::clone(&vy);
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new(bin.as_str())
                    .arg("-f")
                    .arg("abi")
                    .arg(&c[i])
                    .output()?;
                if compiler_output.status.success() {
                    let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                        &compiler_output.stdout,
                    ))?;
                    Ok(json)
                } else {
                    Err(VyperErrors::CompilerError(
                        String::from_utf8_lossy(&compiler_output.stderr).to_string(),
                    ))?
                }
            });
            threads.push(cthread);
        }
        let mut res_vec = Vec::new();
        for child_thread in threads {
            res_vec.push(child_thread.await??);
        }
        Ok(res_vec)
    }
}

impl<'a> From<Vec<Vyper<'a>>> for Vypers {
    fn from(value: Vec<Vyper>) -> Vypers {
        let mut paths = vec![];
        let mut abis = vec![];
        let mut venv: Option<&Path> = None;

        value.into_iter().for_each(|x| {
            paths.push(x.path_to_code.to_path_buf());
            abis.push(x.abi);
            venv = x.venv;
        });

        match venv {
            Some(v) => Vypers::with_venv(paths, v),
            None => Vypers::new(paths),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Evm {
    Byzantium,
    Constantinople,
    Petersberg,
    Istanbul,
    Berlin,
    Paris,
    Shanghai,
    Cancun,
    Atlantis,
    Agharta,
}

impl Display for Evm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Evm::Byzantium => write!(f, "{}", "byzantium".to_owned()),
            Evm::Constantinople => write!(f, "{}", "constantinople".to_owned()),
            Evm::Petersberg => write!(f, "{}", "petersberg".to_owned()),
            Evm::Istanbul => write!(f, "{}", "istanbul".to_owned()),
            Evm::Berlin => write!(f, "{}", "berlin".to_owned()),
            Evm::Paris => write!(f, "{}", "paris".to_owned()),
            Evm::Shanghai => write!(f, "{}", "shanghai".to_owned()),
            Evm::Cancun => write!(f, "{}", "cancun".to_owned()),
            Evm::Atlantis => write!(f, "{}", "atlantis".to_owned()),
            Evm::Agharta => write!(f, "{}", "agharta".to_owned()),
        }
    }
}
