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

use crate::{
    cli::Configuration,
    ops::{gen_program, init_program},
};

mod cli;
mod error;
mod ops;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new()?;
    // Go for the operation requested
    match config.command {
        cli::ExecutionCommand::Create => gen_program(&config),
        cli::ExecutionCommand::Init => init_program(&config),
    }
    Ok(())
}
