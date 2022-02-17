//! @brief Error enums
use thiserror::Error;
use yaml_rust::ScanError;

#[derive(Debug, Error)]
#[error("...")]
pub enum ProgramError {
    #[error("Can not resolve home directory for user")]
    NoHomeFound,
    #[error("Solana install not found in {0}")]
    SolanaNotFound(String),
    // From other modules
    ClapError(#[from] clap::Error),
    IoError(#[from] std::io::Error),
    CargoErrir(#[from] cargo_toml::Error),
    YamlError(#[from] ScanError),
}

pub type CargoResult<T> = std::result::Result<T, ProgramError>;
