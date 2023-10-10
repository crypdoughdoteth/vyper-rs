use crate::vyper_errors::CompilerError;
use anyhow::{bail, Result};
use itertools::izip;
use serde::{Deserialize, Serialize};
use serde_json::{to_writer_pretty, Value};
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command,
    sync::Arc,
};

/// Represents important information about a Vyper contract
#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize,
)]
pub struct Vyper {
    pub path_to_code: PathBuf,
    pub bytecode: Option<String>,
    pub abi: PathBuf,
}

impl Display for Vyper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\ncontract path: {}, \ncontract abi: {}, \ncontract bytecode: {:#?}",
            self.path_to_code.display(),
            self.abi.display(),
            self.bytecode
        )
    }
}

impl Vyper {
    /// Constructor function that takes in the path to your vyper contract and the _desired path/{name}.json_ for your ABI
    ///  
    pub fn new(path: PathBuf, abi_path: PathBuf) -> Self {
        Self {
            path_to_code: path,
            bytecode: None,
            abi: abi_path,
        }
    }
    /// checks whether vyper is in PATH and can be invoked by this library
    pub fn exists() -> bool {
        Command::new("vyper").arg("-h").output().is_ok()
    }

    /// check the version of the vyper compiler
    pub fn get_version() -> Result<String, Box<dyn Error>> {
        let out = Command::new("vyper").arg("--version").output()?;
        if !out.status.success() {
            return Err(Box::new(CompilerError::new(
                "Couldn't locate version info, installation does not exist".to_string(),
            )));
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    /// Compiles a vyper contract by invoking the vyper compiler, updates the ABI field in the Vyper struct
    pub fn compile(&mut self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper").arg(&self.path_to_code).output()?;
        if compiler_output.status.success() { 
            let mut out = String::from_utf8_lossy(&compiler_output.stdout).to_string();
            for _ in 0..2 {
                out.pop();
            }
            self.bytecode = Some(out);
            Ok(())
        } else {
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }

    pub fn compile_blueprint(&mut self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("blueprint_bytecode")
            .arg(&self.path_to_code)
            .output()?;
        if compiler_output.status.success() {
            
            let out = String::from_utf8_lossy(&compiler_output.stdout).to_string();
            self.bytecode = Some(out);
            Ok(())
        } else {
           
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }

    /// Compiles a vyper contract by invoking the vyper compiler, arg for specifying the EVM version to compile to
    pub fn compile_ver(&mut self, ver: Evm) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg(&self.path_to_code)
            .arg("--evm-version")
            .arg(ver.to_string())
            .output()?;

        if compiler_output.status.success() {
           
            let mut out = String::from_utf8_lossy(&compiler_output.stdout).to_string();
            for _ in 0..2 {
                out.pop();
            }
            self.bytecode = Some(out);
            Ok(())
        } else {
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// Generates the ABI and creates a file @ the abi path specified in the Vyper struct
    pub fn abi(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("abi")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                &compiler_output.stdout,
            ))?;
            let file = File::create(&self.abi)?;
            to_writer_pretty(file, &json)?;
            Ok(())
        } else { 
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }

    /// Generates the ABI and creates a file @ the abi path specified in the Vyper struct
    pub fn abi_json(&self) -> Result<Value, Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("abi")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                &compiler_output.stdout,
            ))?;
            Ok(json)
        } else { 
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }

    /// Storage layout as JSON, saves it to a file
    pub fn storage_layout(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
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
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// AST of your contract as JSON, saves it to a file
    pub fn ast(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
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
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// Generates an external interface for your vyper contract to be called with
    pub fn interface(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("external_interface")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;
        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./interface.vy")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// Generates the opcodes produced by your vyper contract, saves it as a text file
    pub fn opcodes(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("opcodes")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./opcodes.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// Generates the opcodes produced by your vyper contract at runtime, saves it as a text file
    pub fn opcodes_runtime(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("opcodes_runtime")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;

        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./opcodes_runtime.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// Natspec user documentation for vyper contract
    pub fn userdoc(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("userdoc")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;
        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./userdoc.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else {
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
    }
    /// Natspec dev documentation for vyper contract
    pub fn devdoc(&self) -> Result<(), Box<dyn Error>> {
        let compiler_output = Command::new("vyper")
            .arg("-f")
            .arg("devdoc")
            .arg(self.path_to_code.to_string_lossy().to_string())
            .output()?;
        if compiler_output.status.success() {
            let mut buffer = BufWriter::new(File::create("./devdoc.txt")?);
            buffer.write_all(&compiler_output.stdout)?;
            Ok(())
        } else { 
            return Err(Box::new(CompilerError::new(
                String::from_utf8_lossy(&compiler_output.stderr).to_string(),
            )));
        }
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
}

impl Vypers {
    /// Constructor function that takes in the paths to your vyper contracts and the _desired paths/{names}.json for your ABIs
    pub fn new(paths: Vec<PathBuf>, abi_paths: Vec<PathBuf>) -> Self {
        if paths.len() != abi_paths.len() {
            panic!("Mismatched Vector Lengths");
        }

        Self {
            path_to_code: paths,
            bytecode: None,
            abi: abi_paths,
        }
    }

    /// Compile multiple vyper contracts concurrently on new threads, updates the ABI field in Vypers
    pub async fn compile_many(&mut self) -> Result<(), Box<dyn Error>> {
        let path = Arc::new(self.path_to_code.clone());
        let mut out_vec: Vec<String> = Vec::with_capacity(self.path_to_code.len());
        let mut threads = vec![];
        for i in 0..self.path_to_code.len() {
            let paths = Arc::clone(&path);
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new("vyper").arg(&paths[i]).output()?;
                if compiler_output.status.success() {
                    let mut out =
                        String::from_utf8_lossy(&compiler_output.stdout).to_string();
                    for _ in 0..2 {
                        out.pop();
                    }
                    Ok(out)
                } else {
                    bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            let x = child_thread.await.unwrap()?;
            out_vec.push(x);
        }
        self.bytecode = Some(out_vec);
        Ok(())
    }

    /// Compile multiple vyper contracts concurrently on new threads, updates the ABI field in Vypers. `Ver` arg is for specifying EVM version to compile each contract to.
    pub async fn compile_many_ver(&mut self, ver: Evm) -> Result<(), Box<dyn Error>> {
        let path = Arc::new(self.path_to_code.clone());
        let mut out_vec: Vec<String> = Vec::with_capacity(self.path_to_code.len());
        let version = ver.to_string();
        let mut threads = vec![];
        for i in 0..self.path_to_code.len() {
            let paths = Arc::clone(&path);
            let cver = version.clone();
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new("vyper")
                    .arg(&paths[i])
                    .arg("--evm-version")
                    .arg(cver)
                    .output()?;
                if compiler_output.status.success() {
                    let mut out =
                        String::from_utf8_lossy(&compiler_output.stdout).to_string();
                    for _ in 0..2 {
                        out.pop();
                    }
                    Ok(out)
                } else {
                    bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            let x = child_thread.await.unwrap()?;
            out_vec.push(x);
        }
        self.bytecode = Some(out_vec);
        Ok(())
    }

    /// Exposes Into trait under this namespace
    pub fn into_vec(self) -> Vec<Vyper> {
        Vec::<Vyper>::from(self)
    }

    /// Generates ABIs for each vyper contract concurrently
    pub async fn abi_many(&self) -> Result<(), Box<dyn Error>> {
        let c_path = Arc::new(self.path_to_code.clone());
        let abi_path = Arc::new(self.abi.clone());
        let mut threads = vec![];
        for i in 0..self.path_to_code.len() {
            let c = Arc::clone(&c_path);
            let abi = Arc::clone(&abi_path);
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new("vyper")
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
                    bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                }
                Ok(())
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            child_thread.await.unwrap()?
        }
        Ok(())
    }

    pub async fn abi_json_many(&self) -> Result<Vec<Value>, Box<dyn Error>> {
        let c_path = Arc::new(self.path_to_code.clone());
        let mut threads = vec![];
        for i in 0..self.path_to_code.len() {
            let c = Arc::clone(&c_path);
            let cthread = tokio::spawn(async move {
                let compiler_output = Command::new("vyper")
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
                    bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
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

impl From<Vypers> for Vec<Vyper> {
    fn from(value: Vypers) -> Vec<Vyper> {
        let code = value.path_to_code;
        let bytes = value.bytecode;
        let abi = value.abi;
        let mut res: Vec<Vyper> = vec![];
        match bytes {
            Some(by) => {
                for (a, b, c) in izip!(code, by, abi) {
                    res.push(Vyper {
                        path_to_code: a,
                        bytecode: Some(b.to_owned()),
                        abi: c,
                    })
                }
            }
            None => {
                for (a, c) in izip!(code, abi) {
                    res.push(Vyper {
                        path_to_code: a,
                        bytecode: None,
                        abi: c,
                    })
                }
            }
        }
        res
    }
}

impl Into<Vypers> for Vec<Vyper> {
    fn into(self) -> Vypers {
        let mut paths = vec![];
        let mut bytes: Vec<String> = vec![];
        let mut abis = vec![];
        self.into_iter().for_each(|x| {
            paths.push(x.path_to_code);
            abis.push(x.abi);
            if let Some(b) = x.bytecode {
                bytes.push(b)
            }
        });
        if bytes.is_empty() {
            Vypers {
                path_to_code: paths,
                bytecode: None,
                abi: abis,
            }
        } else {
            Vypers {
                path_to_code: paths,
                bytecode: Some(bytes),
                abi: abis,
            }
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
