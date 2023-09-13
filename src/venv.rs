use crate::vyper::{Evm, Vyper};
use anyhow::bail;
use serde_json::{to_writer_pretty, Value};
use std::{
    fs::File,
    path::Path,
    process::Command,
};
/// Default state on construction of this type.
/// Can transition to `Initialized` or `Skip`.
pub struct NotInitialized;

/// Venv was activiated using the `init()` method.
/// Can call `ivyper_venv` to install the vyper compiler into the venv.
/// Can call `try_ready()` to check if vyper is installed and transition to `Ready`.
pub struct Initialized;

/// Declined to activate a venv
/// can call `ivyper_pip()` to install vyper globablly
pub struct Skip;
/// Vyper was installed successfully into venv or already exists.
pub struct Ready;
/// Vyper was successfully installed globally or already exists.
pub struct Complete;
///  
/// Manages versions of the vyper compiler with a venv or globally.
/// This state machine represents all valid states for the venv and vyper compiler.
/// While this works with and without a venv, it is strongly recommended to use a venv.
/// Creating a venv with this program is simple, just call `new()` to construct the type and
/// `init()` to create the venv (if not already created). If you created a venv, then you can call
/// the installation method ivyper_venv. Otherwise, this step can be skipped in favor of
/// transitioning to the ready state immediately. Under Venv<Ready> are methods that are executed
/// inside your venv. This is not true for the Vyper module. 
///   
///  States:
///
///      NotInitialized:
///
///          Methods:
///
///              new
///
///              init
///
///              skip
///
///      Initialized:
///
///          Methods:
///
///              ivyper_venv
///
///              try_ready
///
///      Skip:
///
///          Methods:
///
///              ivyper_pip
///
///              try_ready
///
///      Ready:
///
///         Methods: 
///             
///             compile
///         
///             compile_ver
///
///             abi
///
///             abi_json
///     
///     Complete
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
    /// If the venv exists, then we activate it
    /// Otherwise, we need to create one
    /// Platform agnostic by matching aginst the target OS
    /// One for Bash one for CMD  
    /// Once the Venv is created, we activate it too
    pub fn init(self) -> anyhow::Result<Venv<Initialized>> {
        match Path::new("./venv").exists() {
            true => Ok(Venv {
                state: std::marker::PhantomData::<Initialized>,
            }),
            false => {
                if cfg!(target_os = "windows") {
                    let a = Command::new("cmd").arg("mkdir").arg("venv").output()?;
                    if !a.status.success() {
                        bail!("{}", String::from_utf8_lossy(&a.stderr).to_string());
                    }
                } else {
                    let a = Command::new("sh").arg("mkdir").arg("venv").output()?;
                    if !a.status.success() {
                        bail!("{}", String::from_utf8_lossy(&a.stderr).to_string());
                    }
                };
                let b = Command::new("python")
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
                    let c = Command::new("sh")
                        .arg("cd")
                        .arg("./venv/scripts")
                        .arg("&&")
                        .arg("activate.bat")
                        .arg("&&")
                        .arg("cd")
                        .arg("../../")
                        .arg("&&")
                        .arg("pip")
                        .arg("install")
                        .arg(format!("vyper=={}", version))
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("Version {} of Vyper has been installed", version);
                } else {
                    let c = Command::new("sh")
                        .arg("cd")
                        .arg("./venv/scripts")
                        .arg("&&")
                        .arg("activate.bat")
                        .arg("&&")
                        .arg("cd")
                        .arg("../../")
                        .arg("&&")
                        .arg("pip")
                        .arg("install")
                        .arg(format!("vyper=={}", version))
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("Version {} of Vyper has been installed", version);
                }
                return Ok(Venv {
                    state: std::marker::PhantomData::<Ready>,
                });
            }
            None => {
                if cfg!(target_os = "windows") {
                    let c = Command::new("cmd")
                        .arg("cd")
                        .arg("./venv/scripts")
                        .arg("&&")
                        .arg("activate.bat")
                        .arg("&&")
                        .arg("cd")
                        .arg("../../")
                        .arg("&&")
                        .arg("pip")
                        .arg("install")
                        .arg("vyper")
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("The latest version of vyper has been installed");
                } else {
                    let c = Command::new("sh")
                        .arg("cd")
                        .arg("./venv/scripts")
                        .arg("&&")
                        .arg("activate.bat")
                        .arg("&&")
                        .arg("cd")
                        .arg("../../")
                        .arg("&&")
                        .arg("pip")
                        .arg("install")
                        .arg("vyper")
                        .output()?;
                    if !c.status.success() {
                        bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                    }
                    println!("The latest version of vyper has been installed");
                }
                return Ok(Venv {
                    state: std::marker::PhantomData::<Ready>,
                });
            }
        }
    }

    pub fn try_ready(self) -> anyhow::Result<Venv<Ready>> {
        match Vyper::exists() {
            true => Ok(Venv {
                state: std::marker::PhantomData::<Ready>,
            }),
            false => {
                bail!("Vyper not installed")
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
                let c = Command::new("pip")
                    .arg("install")
                    .arg(format!("vyper=={}", version))
                    .output()?;
                if !c.status.success() {
                    bail!("{}", String::from_utf8_lossy(&c.stderr).to_string());
                }
                println!("Version {} of Vyper has been installed", version);
            }
            None => {
                let c = Command::new("pip").arg("install").arg("vyper").output()?;
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
    pub fn compile(contract: &mut Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("cmd")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate.bat")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg(&contract.path_to_code)
                .output()?;

            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{:?}", bytecode);
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("sh")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg(&contract.path_to_code)
                .output()?;

            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{:?}", bytecode);
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn compile_blueprint(contract: &mut Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("cmd")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate.bat")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg("-f")
                .arg("blueprint_bytecode")
                .arg(&contract.path_to_code)
                .output()?;

            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{:?}", bytecode);
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("sh")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg("-f")
                .arg("blueprint_bytecode")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{:?}", bytecode);
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn compile_ver(contract: &mut Vyper, ver: Evm) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("cmd")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate.bat")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg(&contract.path_to_code)
                .arg("--evm-version")
                .arg(ver.to_string())
                .output()?;

            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{:?}", bytecode);
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("sh")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg(&contract.path_to_code)
                .arg("--evm-version")
                .arg(ver.to_string())
                .output()?;
            if output.status.success() {
                let bytecode = String::from_utf8_lossy(&output.stdout).to_string();
                println!("{:?}", bytecode);
                contract.bytecode = Some(bytecode);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn abi(contract: &Vyper) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            let output = Command::new("cmd")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate.bat")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(&output.stdout))?
                    .to_string();
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("sh")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(&output.stdout))?
                    .to_string();
                let file = File::create(&contract.abi)?;
                to_writer_pretty(file, &json)?;
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        Ok(())
    }

    pub fn abi_json(contract: &Vyper) -> anyhow::Result<Value> {
        if cfg!(target_os = "windows") {
            let output = Command::new("cmd")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate.bat")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(&output.stdout))?;
                return Ok(json);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        } else {
            let output = Command::new("sh")
                .arg("cd")
                .arg("./venv/scripts")
                .arg("&&")
                .arg("activate")
                .arg("&&")
                .arg("cd")
                .arg("../../")
                .arg("&&")
                .arg("vyper")
                .arg("-f")
                .arg("abi")
                .arg(&contract.path_to_code)
                .output()?;
            if output.status.success() {
                let json = serde_json::from_str::<Value>(&String::from_utf8_lossy(&output.stdout))?;
                return Ok(json);
            } else {
                bail!(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
    }
    
}
