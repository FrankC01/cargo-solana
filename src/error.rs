//! ProgramError enum
use thiserror::Error;
use yaml_rust::ScanError;

/// Error enum for runtime
#[derive(Debug, Error)]
#[error("...")]
pub enum ProgramError {
    #[error("Can not resolve home directory for user")]
    NoHomeFound,
    #[error("Solana install not found in {0}")]
    SolanaNotFound(String),
    #[error("Project file {0} exists")]
    ProjectExistsError(String),
    #[error("Program folder exists")]
    ProgramExistsError,
    // From other modules
    CargoError(#[from] cargo_toml::Error),
    ClapError(#[from] clap::Error),
    IoError(#[from] std::io::Error),
    TomlError(#[from] toml::ser::Error),
    YamlError(#[from] ScanError),
}

pub type CargoResult<T> = std::result::Result<T, ProgramError>;
