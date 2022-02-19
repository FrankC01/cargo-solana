//! @brief execution operations

use cargo_toml::Workspace;

use crate::{
    cli::Configuration,
    error::{self, CargoResult, ProgramError},
    utils::get_program_resources,
};
use std::{
    env::set_current_dir,
    fs::{create_dir, remove_file, rename, File},
    io::Write,
};

/// Generates program artifacts
/// First program folder and set Cargo.toml
/// Then src folder and dump in files
fn create_program(config: &Configuration) -> CargoResult<()> {
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
            // Smooth the progname and load the resources
            let resource_map = get_program_resources(str::replace(&config.progname, "-", "_"));
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
    println!("Landing back in {}", current_dir.display());
    set_current_dir(current_dir)?;
    Ok(())
}

/// Creates the program and updates the existing Cargo.toml workspace
pub fn create_program_update_workspace(config: &mut Configuration) -> CargoResult<()> {
    // Get and hold onto
    let mut current_dir = std::env::current_dir()?;
    // Generate program artifacts
    match create_program(config) {
        Ok(_) => {
            let cargo = &mut config.init_manifest.clone().unwrap();
            let program = "program".to_string();
            match &mut cargo.workspace {
                // Update existing
                Some(workspace) => workspace.members.push(program),
                // Create a whole new one
                None => {
                    cargo.workspace = Some(Workspace {
                        members: vec![program],
                        default_members: vec![],
                        exclude: vec![],
                        metadata: None,
                        resolver: None,
                    })
                }
            }
            // Rename existing to recover if error
            match rename("./Cargo.toml", "./CargoSolana.bak") {
                Ok(_) => {
                    let cargo_text = toml::to_string(&cargo).unwrap();
                    current_dir.push("Cargo.toml");
                    let mut cargo = File::create(&current_dir)?;
                    cargo.write_all(cargo_text.as_bytes())?;
                    remove_file("./CargoSolana.bak").unwrap();
                    Ok(())
                }
                Err(e) => {
                    current_dir.push("program");
                    let _ = std::fs::remove_dir_all(current_dir);
                    Err(error::ProgramError::IoError(e))
                }
            }
        }
        // Clean up program
        Err(e) => {
            current_dir.push("program");
            let _ = std::fs::remove_dir_all(current_dir);
            Err(e)
        }
    }
}

/// Generate project then program artifacts
pub fn create_project_program(config: &Configuration) -> CargoResult<()> {
    let mut current_dir = std::env::current_dir()?;
    current_dir.push(config.progname.clone());
    if current_dir.exists() {
        Err(ProgramError::ProjectExistsError(config.progname.clone()))
    } else {
        // Create and change into project dir
        println!("Creating {}", current_dir.display());
        create_dir(&current_dir)?;
        set_current_dir(&current_dir)?;
        // Create the toml file
        current_dir.push("Cargo.toml");
        println!("  Creating {}", current_dir.display());
        let mut cargo = File::create(&current_dir)?;
        let cargo_text = toml::to_string(&config.project_manifest_template)?;
        cargo.write_all(cargo_text.as_bytes())?;
        // Pop filename
        current_dir.pop();
        // Create program in current dir
        println!("Creating program in {}", current_dir.display());
        create_program(config)?;
        // Pop to original and change there
        current_dir.pop();
        println!("Changing back to {} dir", current_dir.display());
        set_current_dir(&current_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use cargo_toml::Manifest;

    use crate::{
        cli::ExecutionCommand,
        utils::{build_program_manifest, project_template_as_manifest},
    };
    use std::fs::{remove_file, rename};

    #[test]
    fn test_create_program_pass() {
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
        assert!(create_program(&configuration).is_ok());
        println!("Removing {}", base_dir.display());
        std::fs::remove_dir_all(base_dir).unwrap();
    }

    #[test]
    fn test_create_program_update_workspace_pass() {
        let name = "foo";
        let progm = build_program_manifest(name.to_string()).unwrap();
        let projm = project_template_as_manifest().unwrap();
        let mut base_dir = std::env::current_dir().unwrap();
        base_dir.push("program");
        let exist_cargo = Some(Manifest::from_path("./Cargo.toml").unwrap());
        rename("./Cargo.toml", "./Cargo.bak").unwrap();

        let mut configuration = Configuration {
            progname: name.to_string(),
            command: ExecutionCommand::Init,
            init_manifest: exist_cargo,
            program_manifest_template: progm,
            project_manifest_template: projm,
        };
        assert!(create_program_update_workspace(&mut configuration).is_ok());
        println!("Removing {}", base_dir.display());
        std::fs::remove_dir_all(base_dir).unwrap();
        println!("Removing temporary Cargo update");
        remove_file("./Cargo.toml").unwrap();
        rename("./Cargo.bak", "./Cargo.toml").unwrap();
    }
    #[test]
    fn test_create_project_pass() {
        let name = "foo-bar";
        let progm = build_program_manifest(name.to_string()).unwrap();
        let projm = project_template_as_manifest().unwrap();
        let mut base_dir = std::env::current_dir().unwrap();
        base_dir.push("foo-bar");
        let configuration = Configuration {
            progname: name.to_string(),
            command: ExecutionCommand::Init,
            init_manifest: None,
            program_manifest_template: progm,
            project_manifest_template: projm,
        };
        assert!(create_project_program(&configuration).is_ok());
        println!("Removing {}", base_dir.display());
        std::fs::remove_dir_all(base_dir).unwrap();
    }
}
