//! @brief execution operations

use crate::{
    cli::Configuration,
    error::{CargoResult, ProgramError},
    utils::get_program_resources,
};
use std::{
    env::set_current_dir,
    fs::{create_dir, File},
    io::Write,
};

/// Generate program artifacts
/// First program folder and set Cargo.toml
/// Then src folder and dump in files
pub fn create_program(config: &Configuration, build_workspace: bool) -> CargoResult<()> {
    let mut current_dir = std::env::current_dir()?;
    println!("Starting in {}", current_dir.display());
    current_dir.push("program");
    println!("Testing existance {}", current_dir.display());
    if !current_dir.exists() {
        {
            // Generate the base directory and change into it
            println!("Building {}", current_dir.display());
            create_dir(&current_dir)?;
            set_current_dir(&current_dir)?;
            // Plop in the manifest
            println!("  Putting Cargo.toml");
            let cargo_text = toml::to_string(&config.program_manifest_template)?;
            current_dir.push("Cargo.toml");
            let mut cargo = File::create(&current_dir)?;
            cargo.write_all(cargo_text.as_bytes())?;
            current_dir.pop();
        }
        // Generate the src directory
        {
            current_dir.push("src");
            println!("Building {}", current_dir.display());
            create_dir(&current_dir)?;
            set_current_dir(&current_dir)?;
            let resource_map = get_program_resources(config.progname.clone());
            for (res_filename, res_file) in resource_map {
                println!("  Creating {}", res_filename);
                let mut src_file = File::create(res_filename)?;
                src_file.write_all(res_file.as_bytes())?;
            }
            current_dir.pop();
        }
    } else {
        return Err(ProgramError::ProgramExistsError);
    }
    current_dir.pop();
    println!("Current dir {}", current_dir.display());
    if build_workspace {
        println!(
            "Updating Cargo.toml with workspace in {}",
            current_dir.display()
        );
    } else {
        println!("Skipping workspace update");
    }
    println!("Landing back in {}", current_dir.display());
    set_current_dir(current_dir)?;
    Ok(())
}

/// Generate project then program artifacts
pub fn create_project_program(config: &Configuration) -> CargoResult<()> {
    let _current_dir = std::env::current_dir()?;
    println!(
        "{}",
        toml::to_string(&config.project_manifest_template).unwrap()
    );
    create_program(config, false)
}

#[cfg(test)]
mod tests {
    use crate::{
        cli::ExecutionCommand,
        utils::{build_program_manifest, project_template_as_manifest},
    };

    use super::*;

    #[test]
    fn test_init() {
        // Build the configuration file
        let name = "foo";
        let progm = build_program_manifest(name.to_string()).unwrap();
        let projm = project_template_as_manifest().unwrap();
        let mut base_dir = std::env::current_dir().unwrap();
        base_dir.push("program");
        let configuration = Configuration {
            progname: name.to_string(),
            command: ExecutionCommand::Init,
            init_manifest: None,
            program_manifest_template: progm,
            project_manifest_template: projm,
        };
        assert!(create_program(&configuration, false).is_ok());
        println!("Removing {}", base_dir.display());
        std::fs::remove_dir_all(base_dir).unwrap();
    }
}
