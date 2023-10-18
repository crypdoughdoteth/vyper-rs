use crate::vyper::{Evm, Vyper, Vypers};
use anyhow::bail;
use serde_json::{to_writer_pretty, Value};
use std::{error::Error, fs::File, path::Path, process::Command, sync::Arc};
/// Default state on construction of this type.
/// Can transition to `Initialized` or `Skip`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct NotInitialized;
/// Venv was activiated using the `init()` method.
/// Can call `ivyper_venv` to install the vyper compiler into the venv.
/// Can call `try_ready()` to check if vyper is installed and transition to `Ready`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Initialized;

/// Declined to activate a venv
/// can call `ivyper_pip()` to install vyper globablly with pip3
/// can call try_ready() to check if vyper is installed and transition to Complete
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Skip;
/// Vyper was installed successfully into venv or already exists.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Ready;
/// Vyper was successfully installed globally or already exists.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Complete;
///  
/// Manages versions of the vyper compiler with a venv or globally.
/// This state machine represents all valid states for the venv and vyper compiler.
/// While this works with and without a venv, it is strongly recommended to use a venv.
/// Creating a venv with this program is simple, just call `new()` to construct the type and
/// `init()` to create the venv (if not already created). If you called init(), then you can call
/// the installation method ivyper_venv(). There is an optional argument that takes in the desired
/// compiler version. You may call try_ready() if the compiler is already installed in your venv.
/// Otherwise, this step be skipped with the skip() method in order to install vyper globally or use a preexisting installation.
/// The accompanying installation method is called ivyper_pip() and try_ready() is also available in this namespace.
/// Both of these methods under Venv<Skip> return a Complete state. When this is reached, you may know with certainty
/// that a version of Vyper is installed globally with pip and you can safely use the methods in
/// the Vyper module. Likewise, when the state is Venv<Ready>, you
/// may use the namespace to access methods for use inside the venv. Methods inside the Venv<Ready>
/// namespace are mostly equivalent to the ones in the Vyper module, thus you can rely on the
/// documentation for these methods inside the Venv module.
///   
//  States:
//
//      NotInitialized:
//
//          Methods:
//
//              new
//
//              init
//
//              skip
//
//      Initialized:
//
//          Methods:
//
//              ivyper_venv
//
//              try_ready
//
//      Skip:
//
//          Methods:
//
//              ivyper_pip
//
//              try_ready
//
//      Ready:
//
//         Methods:
//
//             compile
//
//             compile_ver
//
//             abi
//
//             abi_json
//
//     Complete
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Venv<State = NotInitialized> {
    state: std::marker::PhantomData<State>,
}

impl Default for Venv<NotInitialized> {
    fn default() -> Self {
        Self {
            state: std::marker::PhantomData::<NotInitialized>,
        }
    }
}

impl Venv<NotInitialized> {
    /// Constructs the Venv type with PhantomData
    pub fn new() -> Venv<NotInitialized> {
        Self {
            state: std::marker::PhantomData::<NotInitialized>,
        }
    }

    /// Init will check whether or not a venv was created by this program
    /// If it was not, we will create one
    pub fn init(self) -> anyhow::Result<Venv<Initialized>> {
        match Path::new("./venv").exists() {
            true => Ok(Venv {
                state: std::marker::PhantomData::<Initialized>,
            }),
            false => {
                let a = Command::new("mkdir").arg("venv").output()?;
                if !a.status.success() {
                    bail!("{}", String::from_utf8_lossy(&a.stderr).to_string());
                }

                let b = Command::new("python3")
                    .arg("-m")
                    .arg("venv")
                    .arg("./venv")
                    .output()?;
                if !b.status.success() {
                    bail!("{}", String::from_utf8_lossy(&b.stderr).to_string());
                }

                Ok(Venv {
                    state: std::marker::PhantomData::<Initialized>,
                })
            }
        }
    }
    /// For the psychopaths that decide to globally rawdog pip on their PC  
    pub fn skip(self) -> Venv<Skip> {
        Venv {
            state: std::marker::PhantomData::<Skip>,
        }
    }
}
impl Venv<Initialized> {
    /// Installs vyper into virtual environment
    /// Optional argument for the version of vyper to be installed
    pub fn ivyper_venv(self, ver: Option<&str>) -> anyhow::Result<Venv<Ready>> {
        match ver {
            Some(version) => {
                if cfg!(target_os = "windows") {
                    let c = Command::new("./venv/scripts/pip3")
                        .arg("install")
                        .arg(format!("vyper=={}", version))
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("Version {} of Vyper has been installed", version);
                } else {
                    let c = Command::new("./venv/bin/pip3")
                        .arg("install")
                        .arg(format!("vyper=={}", version))
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("Version {} of Vyper has been installed", version);
                }

                Ok(Venv {
                    state: std::marker::PhantomData::<Ready>,
                })
            }
            None => {
                if cfg!(target_os = "windows") {
                    let c = Command::new("./venv/scripts/pip3")
                        .arg("install")
                        .arg("vyper")
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("The latest version of vyper has been installed");
                } else {
                    let c = Command::new("./venv/bin/pip3")
                        .arg("install")
                        .arg("vyper")
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("The latest version of vyper has been installed");
                }
                Ok(Venv {
                    state: std::marker::PhantomData::<Ready>,
                })
            }
        }
    }
    /// Check to see if Vyper is installed in a Venv. If so, transition state to Ready and
    /// access to the methods of this namespace.
    pub fn try_ready(self) -> anyhow::Result<Venv<Ready>> {
        if cfg!(target_os = "windows") {
            match Path::new("./venv/scripts/vyper").exists() {
                true => Ok(Venv {
                    state: std::marker::PhantomData::<Ready>,
                }),
                false => {
                    bail!("Vyper was not installed in venv")
                }
            }
        } else {
            match Path::new("./venv/bin/vyper").exists() {
                true => Ok(Venv {
                    state: std::marker::PhantomData::<Ready>,
                }),
                false => {
                    bail!("Vyper was not installed in venv")
                }
            }
        }
    }
}

impl Venv<Skip> {
    /// Installs vyper compiler globally, without the protection of a venv
    /// Optional argument for the version of vyper to be installed
    pub fn ivyper_pip(self, ver: Option<&str>) -> anyhow::Result<Venv<Complete>> {
        match ver {
            Some(version) => {
                let c = Command::new("pip3")
                    .arg("install")
                    .arg(format!("vyper=={}", version))
                    .output()?;
                if !c.status.success() {
                    bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                }
                println!("Version {} of Vyper has been installed", version);
            }
            None => {
                let c = Command::new("pip3").arg("install").arg("vyper").output()?;
                if !c.status.success() {
                    bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                }
                println!("The Latest Version of Vyper has been installed");
            }
        }
        Ok(Venv {
            state: std::marker::PhantomData::<Complete>,
        })
    }
    /// Transition to Complete if the Vyper compiler is installed globally
    pub fn try_ready(self) -> anyhow::Result<Venv<Complete>> {
        match Vyper::exists() {
            true => Ok(Venv {
                state: std::marker::PhantomData::<Complete>,
            }),
            false => {
                bail!("Vyper not installed")
            }
        }
    }
}

impl Venv<Ready> {
    /// check the version of the vyper compiler
    pub fn get_version(&self) -> anyhow::Result<String> {
        if cfg!(target_os = "windows") {
            let out = Command::new("./venv/scripts/vyper")
                .arg("--version")
                .output()?;
            if !out.status.success() {
                bail!("Couldn't locate version info");
            }
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            let out = Command::new("./venv/bin/vyper").arg("--version").output()?;
            if !out.status.success() {
                bail!("Couldn't locate version info");
            }
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
    }

    pub fn compile(&self, contract: &mut Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn compile_blueprints(&self, contract: &mut Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("blueprint_bytecode")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();

                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("blueprint_bytecode")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn compile_ver(&self, contract: &mut Vyper, ver: Evm) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg(&contract.path_to_code)
                .arg("--evm-version")
                .arg(ver.to_string())
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg(&contract.path_to_code)
                .arg("--evm-version")
                .arg(ver.to_string())
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn gen_abi(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?
                .to_string();
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?
                .to_string();
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn get_abi(&self, contract: &Vyper) -> anyhow::Result<Value> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                Ok(json)
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                Ok(json)
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
    }

    pub fn storage_layout(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("layout")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("layout")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn ast(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("ast")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("ast")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn interface(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("interface")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("interface")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn opcodes(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("opcodes")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("opcodes")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn opcodes_runtime(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("opcodes_runtime")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("opcodes_runtime")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn userdoc(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("userdoc")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("userdoc")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn devdoc(&self, contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("./venv/scripts/vyper")
                .arg("-f")
                .arg("devdoc")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("./venv/bin/vyper")
                .arg("-f")
                .arg("devdoc")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(
                    &output.stdout,
                ))?;
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub async fn compile_many(
        &self,
        contracts: &mut Vypers,
    ) -> Result<(), Box<dyn Error>> {
        let path = Arc::new(contracts.path_to_code.clone());
        let mut out_vec: Vec<String> = Vec::with_capacity(contracts.path_to_code.len());
        let mut threads = vec![];
        for i in 0..contracts.path_to_code.len() {
            let paths = Arc::clone(&path);
            let cthread = tokio::spawn(async move {
                if cfg!(target_os = "windows") {
                    let compiler_output = Command::new("./venv/scripts/vyper")
                        .arg(&paths[i])
                        .output()?;
                    if compiler_output.status.success() {
                        let out =
                            String::from_utf8_lossy(&compiler_output.stdout).to_string();
                        Ok(out)
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                } else {
                    let compiler_output =
                        Command::new("./venv/bin/vyper").arg(&paths[i]).output()?;
                    if compiler_output.status.success() {
                        let out =
                            String::from_utf8_lossy(&compiler_output.stdout).to_string();
                        Ok(out)
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            let x = child_thread.await.unwrap()?;
            out_vec.push(x);
        }
        contracts.bytecode = Some(out_vec);
        Ok(())
    }

    pub async fn compile_many_ver(
        &self,
        contracts: &mut Vypers,
        ver: Evm,
    ) -> Result<(), Box<dyn Error>> {
        let path = Arc::new(contracts.path_to_code.clone());
        let mut out_vec: Vec<String> = Vec::with_capacity(contracts.path_to_code.len());
        let mut threads = vec![];
        for i in 0..contracts.path_to_code.len() {
            let cver = ver.clone().to_string();
            let paths = Arc::clone(&path);
            let cthread = tokio::spawn(async move {
                if cfg!(target_os = "windows") {
                    let compiler_output = Command::new("./venv/scripts/vyper")
                        .arg(&paths[i])
                        .arg("--evm-version")
                        .arg(cver)
                        .output()?;
                    if compiler_output.status.success() {
                        let out =
                            String::from_utf8_lossy(&compiler_output.stdout).to_string();
                        Ok(out)
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                } else {
                    let compiler_output = Command::new("./venv/bin/vyper")
                        .arg(&paths[i])
                        .arg("--evm-version")
                        .arg(cver)
                        .output()?;
                    if compiler_output.status.success() {
                        let out =
                            String::from_utf8_lossy(&compiler_output.stdout).to_string();
                        Ok(out)
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            let x = child_thread.await.unwrap()?;
            out_vec.push(x);
        }
        contracts.bytecode = Some(out_vec);
        Ok(())
    }
    pub async fn gen_abi_many(&self, contracts: &Vypers) -> Result<(), Box<dyn Error>> {
        let c_path = Arc::new(contracts.path_to_code.clone());
        let abi_path = Arc::new(contracts.abi.clone());
        let mut threads = vec![];
        for i in 0..contracts.path_to_code.len() {
            let c = Arc::clone(&c_path);
            let abi = Arc::clone(&abi_path);
            let cthread = tokio::spawn(async move {
                if cfg!(target_os = "windows") {
                    let compiler_output = Command::new("./venv/scripts/vyper")
                        .arg("-f")
                        .arg("abi")
                        .arg(&c[i])
                        .output()?;
                    if compiler_output.status.success() {
                        let json = serde_json::from_str::<Value>(
                            &String::from_utf8_lossy(&compiler_output.stdout),
                        )?;
                        let file = File::create(&abi[i])?;
                        to_writer_pretty(file, &json)?;
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                    Ok(())
                } else {
                    let compiler_output = Command::new("./venv/bin/vyper")
                        .arg("-f")
                        .arg("abi")
                        .arg(&c[i])
                        .output()?;
                    if compiler_output.status.success() {
                        let json = serde_json::from_str::<Value>(
                            &String::from_utf8_lossy(&compiler_output.stdout),
                        )?;
                        let file = File::create(&abi[i])?;
                        to_writer_pretty(file, &json)?;
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                    Ok(())
                }
            });
            threads.push(cthread);
        }
        for child_thread in threads {
            child_thread.await.unwrap()?
        }
        Ok(())
    }

    pub async fn get_abi_many(
        &self,
        contracts: &Vypers,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        let c_path = Arc::new(contracts.path_to_code.clone());
        let mut threads = vec![];
        for i in 0..contracts.path_to_code.len() {
            let c = Arc::clone(&c_path);
            let cthread = tokio::spawn(async move {
                if cfg!(target_os = "windows") {
                    let compiler_output = Command::new("./venv/scripts/vyper")
                        .arg("-f")
                        .arg("abi")
                        .arg(&c[i])
                        .output()?;
                    if compiler_output.status.success() {
                        let json = serde_json::from_str::<Value>(
                            &String::from_utf8_lossy(&compiler_output.stdout),
                        )?;
                        Ok(json)
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
                } else {
                    let compiler_output = Command::new("./venv/bin/vyper")
                        .arg("-f")
                        .arg("abi")
                        .arg(&c[i])
                        .output()?;
                    if compiler_output.status.success() {
                        let json = serde_json::from_str::<Value>(
                            &String::from_utf8_lossy(&compiler_output.stdout),
                        )?;
                        Ok(json)
                    } else {
                        bail!(String::from_utf8_lossy(&compiler_output.stderr).to_string())
                    }
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
