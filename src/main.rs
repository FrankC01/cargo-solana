#![warn(missing_docs)]
//! Solana Program Template
//!
//! Creates a new Solana Program project structure and installs boilerplate program code.
//!
//! Alternatively, Init a program folder and install boilerplate program code to existing project.
//!
//! Commands:
//!
//! `cargo solana --help`</p>
//! `cargo solana create -n <name>`</p>
//! `cargo solana init -n <name>`</p>
//!

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
