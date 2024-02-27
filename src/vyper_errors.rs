//! This module contains the main error type returned when there's some issue with the compiler in
//! the Vyper module.
use std::{error::Error, fmt::Display, io, num::ParseIntError};

#[derive(Debug)]
pub enum VyperErrors {
    IoError(io::Error),
    CompilerError(String),
    SerializationError(serde_json::Error),
    ConcurrencyError(tokio::task::JoinError),
    PipError(String),
    DirError(String),
    VenvError(String),
    BlueprintError(String),
    IntParseError(ParseIntError),
    StringParsingError,
}

impl Display for VyperErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VyperErrors::IoError(err) => {
                write!(f, "An error occured while using system IO: {}", err)
            }
            VyperErrors::SerializationError(s) => write!(
                f,
                "An error occurred while serializing or deserializing data: {}",
                s,
            ),
            VyperErrors::CompilerError(msg) => write!(f, "{}", msg),
            VyperErrors::PipError(msg) => write!(f, "{}", msg),
            VyperErrors::ConcurrencyError(je) => {
                write!(f, "Failed to join async tasks: {}", je)
            }
            VyperErrors::DirError(msg) => write!(f, "{}", msg),
            VyperErrors::VenvError(msg) => write!(f, "{}", msg),
            VyperErrors::BlueprintError(msg) => write!(f, "{}", msg),
            VyperErrors::IntParseError(e) => write!(f, "{}", e),
            VyperErrors::StringParsingError => write!(
                f,
                "An error occurred while parsing bytecode from vyper compiler output"
            ),
        }
    }
}

impl Error for VyperErrors {}

impl From<std::io::Error> for VyperErrors {
    fn from(value: std::io::Error) -> Self {
        VyperErrors::IoError(value)
    }
}

impl From<serde_json::Error> for VyperErrors {
    fn from(value: serde_json::Error) -> Self {
        VyperErrors::SerializationError(value)
    }
}

impl From<tokio::task::JoinError> for VyperErrors {
    fn from(value: tokio::task::JoinError) -> Self {
        VyperErrors::ConcurrencyError(value)
    }
}
