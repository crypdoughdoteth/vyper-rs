//! Utilities offered by the crate.

use std::{
    // error::Error,
    env,
    fs::{self, read_dir, File, ReadDir},
    io::Error,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{bail, Result};
use tokio::task::JoinHandle;

/// Parses the ERC-5202 bytecode container format for indexing blueprint contracts.
///
/// "A blueprint contract MUST use the preamble 0xFE71<version bits><length encoding bits>. 6 bits are allocated to the version, and 2 bits to the length encoding. The first version begins at 0 (0b000000), and versions increment by 1. The value 0b11 for <length encoding bits> is reserved. In the case that the length bits are 0b11, the third byte is considered a continuation byte (that is, the version requires multiple bytes to encode). The exact encoding of a multi-byte version is left to a future ERC.
/// A blueprint contract MUST contain at least one byte of initcode.
/// A blueprint contract MAY insert any bytes (data or code) between the version byte(s) and the initcode. If such variable length data is used, the preamble must be 0xFE71<version bits><length encoding bits><length bytes><data>. The <length encoding bits> represent a number between 0 and 2 (inclusive) describing how many bytes <length bytes> takes, and <length bytes> is the big-endian encoding of the number of bytes that <data> takes."
///
/// "ERC-5202: Blueprint contract format," Ethereum Improvement Proposals, no. 5202, June 2022. [Online serial].
/// Available: https://eips.ethereum.org/EIPS/eip-5202.

pub fn parse_blueprint(bytecode: &[u8]) -> Result<(u8, Option<Vec<u8>>, Vec<u8>)> {
    if bytecode.is_empty() {
        bail!("Empty Bytecode");
    }
    if &bytecode[0..2] != b"\xFE\x71" {
        bail!("Not a blueprint!");
    }

    let erc_version = (&bytecode[2] & 0b11111100) >> 2;
    let n_length_bytes = &bytecode[2] & 0b11;

    if n_length_bytes == 0b11 {
        bail!("Reserved bits are set");
    }

    let size_temp = bytecode[3..(3 + n_length_bytes as usize)].to_vec();
    let data_length = match size_temp.len() {
        0 => 0,
        _ => {
            let size: String = hex::encode(&size_temp);
            match u32::from_str_radix(&size, size_temp.len() as u32 * 8u32) {
                Ok(num) => num,
                Err(e) => bail!(e),
            }
        }
    };

    let preamble_data: Option<Vec<u8>> = match data_length {
        0 => None,
        _ => {
            let data_start = 3 + n_length_bytes as usize;
            Some(bytecode[data_start..data_start + data_length as usize].to_vec())
        }
    };

    let initcode =
        bytecode[3 + n_length_bytes as usize + data_length as usize..].to_vec();
    match initcode.is_empty() {
        true => {
            bail!("Empty Initcode!")
        }
        false => Ok((erc_version, preamble_data, initcode)),
    }
}

pub async fn scan_workspace(root: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let cwd = root.clone();
    let h1 = tokio::spawn(async move { get_contracts_in_dir(cwd) });
    let hh_ape = root.join("/contracts");
    let h2 = tokio::spawn(async move { get_contracts_in_dir(hh_ape) });
    let foundry = root.join("/src");
    let h3 = tokio::spawn(async move { get_contracts_in_dir(foundry) });
    let mut res = Vec::new();
    for i in [h1, h2, h3].into_iter() {
        let result = match i.await {
            Ok(Ok(x)) => x,
            _ => Vec::new(),
        };
        res.push(result)
    }
    Ok(res.into_iter().flatten().collect::<Vec<PathBuf>>())
}

pub fn get_contracts_in_dir(dir: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let files = read_dir(dir)?;
    let contracts = files.into_iter().try_fold(
        Vec::new(),
        |mut acc, x| -> Result<Vec<PathBuf>, Error> {
            let file = x?;
            if file.path().ends_with(".vy") {
                acc.push(file.path())
            }
            Ok(acc)
        },
    )?;
    Ok(contracts)
}
