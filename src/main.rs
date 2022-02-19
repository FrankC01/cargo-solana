//! Solana Program Template
//!
//! Creates new Solana Program project structure with optional CLI and Node constructs as well
//!
//! Commands:
//!
//! `cargo soltempl program <name>`
//!
//! `cargo soltempl cli <name>`
//!
//! `cargo soltempl full <name>`

use cli::Configuration;
use ops::{create_program_update_workspace, create_project_program};

// Modules
mod cli;
mod error;
mod ops;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new()?;
    // Go for the operation requested
    match config.command {
        cli::ExecutionCommand::Create => create_project_program(&config)?,
        cli::ExecutionCommand::Init => create_program_update_workspace(&mut config)?,
    }
    Ok(())
}
