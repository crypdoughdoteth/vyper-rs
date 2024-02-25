//! Manages versions of the vyper compiler with a venv or globally.
//! This state machine represents all valid states for the venv and vyper compiler.
//! While this works with and without a venv, it is strongly recommended to use a venv.
//! Creating a venv with this program is simple, just call `new()` to construct the type and
//! `init()` to create the venv (if not already created). If you called init(), then you can call
//! the installation method ivyper_venv(). There is an optional argument that takes in the desired
//! compiler version. You may call try_ready() if the compiler is already installed in your venv.
//! Otherwise, this step be skipped with the skip() method in order to install vyper globally or use a preexisting installation.
//! The accompanying installation method is called ivyper_pip() and try_ready() is also available in this namespace.
//! Both of these methods under Venv<Skip> return a Complete state. When this is reached, you may know with certainty
//! that a version of Vyper is installed globally with pip and you can safely use the methods in
//! the Vyper module. Likewise, when the state is Venv<Ready>, you
//! may use the namespace to access methods for use inside the venv. Methods inside the Venv<Ready>
//! namespace are mostly equivalent to the ones in the Vyper module, thus you can rely on the
//! documentation for these methods inside the Venv module.
use crate::vyper::{Vyper, Vypers};
use anyhow::bail;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
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

/// Venv is the primary namespace of the module. Its methods are split between various states
/// represented by individual structs. The main documentation for the module is here. Functions
/// accessible under the Ready state are documented under the Vyper module with the same naming
/// conventions.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Venv<'a, State = NotInitialized> {
    venv_path: &'a Path,
    state: std::marker::PhantomData<State>,
}

impl<'a> Default for Venv<'a, NotInitialized> {
    fn default() -> Self {
        Self {
            venv_path: Path::new("./venv"),
            state: std::marker::PhantomData::<NotInitialized>,
        }
    }
}

impl<'a> Venv<'a, NotInitialized> {
    /// Constructs the Venv type with PhantomData
    pub fn new(venv_path: &'a Path) -> Venv<'a, NotInitialized> {
        Self {
            venv_path,
            state: std::marker::PhantomData::<NotInitialized>,
        }
    }

    /// Init will check whether or not a venv was created by this program
    /// If it was not, we will create one
    pub fn init(self) -> anyhow::Result<Venv<'a, Initialized>> {
        match self.venv_path.exists() {
            true => Ok(Venv {
                venv_path: self.venv_path,
                state: std::marker::PhantomData::<Initialized>,
            }),
            false => {
                let a = Command::new("mkdir").arg(self.venv_path).output()?;
                if !a.status.success() {
                    bail!("{}", String::from_utf8_lossy(&a.stderr).to_string());
                }

                let b = Command::new("python3")
                    .arg("-m")
                    .arg("venv")
                    .arg(self.venv_path)
                    .output()?;
                if !b.status.success() {
                    bail!("{}", String::from_utf8_lossy(&b.stderr).to_string());
                }

                Ok(Venv {
                    venv_path: self.venv_path,
                    state: std::marker::PhantomData::<Initialized>,
                })
            }
        }
    }
    /// For the psychopaths that decide to globally rawdog pip on their PC  
    fn skip() -> Venv<'a, Skip> {
        Venv {
            venv_path: Path::new("./venv"),
            state: std::marker::PhantomData::<Skip>,
        }
    }
}
impl<'a> Venv<'a, Initialized> {
    /// Installs vyper into virtual environment
    /// Optional argument for the version of vyper to be installed
    pub fn ivyper_venv(self, ver: Option<&'a str>) -> anyhow::Result<Venv<'a, Ready>> {
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
                    venv_path: self.venv_path,
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
                    venv_path: self.venv_path,
                    state: std::marker::PhantomData::<Ready>,
                })
            }
        }
    }
    /// Check to see if Vyper is installed in a Venv. If so, transition state to Ready and
    /// access to the methods of this namespace.
    pub fn try_ready(self) -> anyhow::Result<Venv<'a, Ready>> {
        if cfg!(target_os = "windows") {
            match Path::new("./venv/scripts/vyper").exists() {
                true => Ok(Venv {
                    venv_path: self.venv_path,
                    state: std::marker::PhantomData::<Ready>,
                }),
                false => {
                    bail!("Vyper was not installed in venv")
                }
            }
        } else {
            match Path::new("./venv/bin/vyper").exists() {
                true => Ok(Venv {
                    venv_path: self.venv_path,
                    state: std::marker::PhantomData::<Ready>,
                }),
                false => {
                    bail!("Vyper was not installed in venv")
                }
            }
        }
    }
}

impl<'a> Venv<'a, Skip> {
    /// Installs vyper compiler globally, without the protection of a venv
    /// Optional argument for the version of vyper to be installed
    pub fn ivyper_pip(self, ver: Option<&'a str>) -> anyhow::Result<Venv<Complete>> {
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
            venv_path: self.venv_path,
            state: std::marker::PhantomData::<Complete>,
        })
    }

    /// checks whether vyper is in PATH and can be invoked by this library
    pub fn global_exists() -> bool {
        Command::new("vyper").arg("-h").output().is_ok()
    }

    /// Transition to Complete if the Vyper compiler is installed globally
    pub fn try_ready(self) -> anyhow::Result<Venv<'a, Complete>> {
        match Self::global_exists() {
            true => Ok(Venv {
                venv_path: self.venv_path,
                state: std::marker::PhantomData::<Complete>,
            }),
            false => {
                bail!("Vyper not installed")
            }
        }
    }
}

impl<'a> Venv<'a, Complete> {
    fn vyper(self, path_to_contract: &'a Path) -> Vyper<'a> {
        Vyper::new(path_to_contract)
    }

    fn vypers(self, paths: Vec<PathBuf>) -> Vypers {
        Vypers::new(paths)
    }

    fn vyper_with_abi(self, path: &'a Path, abi: PathBuf) -> Vyper<'a> {
        Vyper::with_abi(path, abi)
    }

    fn vypers_from_dir(self, path: PathBuf) -> Option<Vypers> {
        Vypers::in_dir(path)
    }

    async fn vypers_from_workspace(self, path: PathBuf) -> Option<Vypers> {
        Vypers::in_workspace(path).await
    }
}

impl<'a> Venv<'a, Ready> {
    fn vyper(self, path_to_contract: &'a Path) -> Vyper<'a> {
        Vyper::with_venv(path_to_contract, self.venv_path)
    }

    fn vypers(self, paths: Vec<PathBuf>) -> Vypers {
        Vypers::with_venv(paths, self.venv_path)
    }

    fn vyper_with_abi(self, path: &'a Path, abi: PathBuf) -> Vyper<'a> {
        Vyper::with_venv_and_abi(path, self.venv_path, abi)
    }

    fn vypers_from_dir(self, path: PathBuf) -> Option<Vypers> {
        let vyps = Vypers::in_dir(path);
        let ret = vyps.map(|e| e.set_venv(self.venv_path.to_path_buf()));
        ret
    }

    async fn vypers_from_workspace(self, path: PathBuf) -> Option<Vypers> {
        let vyps = Vypers::in_workspace(path).await;
        let ret = vyps.map(|e| e.set_venv(self.venv_path.to_path_buf()));
        ret
    }
}
