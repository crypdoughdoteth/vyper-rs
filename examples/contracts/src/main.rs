use std::error::Error;

use ethers::prelude::abigen;
mod venv; 
mod deploy; 
mod compile; 

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; 
    use crate::{deploy::deploy, compile::compile_and_generate_bindings, venv::venv_example};
    #[test]
    fn d() {
        tokio_test::block_on(async {
            deploy().await.unwrap();
        })
    }

    #[test]
    fn c() {
        compile_and_generate_bindings().unwrap();
    }

    #[test]
    fn v() {
        venv_example().unwrap();
    }
}