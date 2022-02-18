//! @brief utility functions

use crate::error::{CargoResult, ProgramError};
use cargo_toml::Manifest;
use dirs::home_dir;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, str};
use yaml_rust::YamlLoader;

// Constants
const SOLANA_INSTALL: &str = ".local/share/solana/install/active_release/version.yml";
const VER_STRING: &str = "channel";
const PROG_IDENTIFIER: &str = r"PROGNAME";

/// Load entry point template and substitute in program name
fn get_entry_point_resource(new_name: String) -> String {
    let in_str = str::from_utf8(include_bytes!("../resources/program/entry_point.rs")).unwrap();
    let re = Regex::new(PROG_IDENTIFIER).unwrap();
    str::from_utf8(re.replace_all(&in_str, new_name).as_bytes())
        .unwrap()
        .to_string()
}

fn get_error_resource() -> String {
    str::from_utf8(include_bytes!("../resources/program/state.rs"))
        .unwrap()
        .to_string()
}

fn get_instruction_resource() -> String {
    str::from_utf8(include_bytes!("../resources/program/instruction.rs"))
        .unwrap()
        .to_string()
}
fn get_process_resource() -> String {
    str::from_utf8(include_bytes!("../resources/program/process.rs"))
        .unwrap()
        .to_string()
}
fn get_state_resource() -> String {
    str::from_utf8(include_bytes!("../resources/program/state.rs"))
        .unwrap()
        .to_string()
}

fn get_lib_resource() -> String {
    str::from_utf8(include_bytes!("../resources/program/lib.rs"))
        .unwrap()
        .to_string()
}

pub fn get_program_resources(new_name: String) -> HashMap<&'static str, String> {
    let mut prog_resources = HashMap::<&str, String>::new();
    prog_resources.insert(
        "program/src/entry_point.rs",
        get_entry_point_resource(new_name),
    );
    prog_resources.insert("program/src/error.rs", get_error_resource());
    prog_resources.insert("program/src/instruction.rs", get_instruction_resource());
    prog_resources.insert("program/src/process.rs", get_process_resource());
    prog_resources.insert("program/src/state.rs", get_state_resource());
    prog_resources.insert("program/src/lib.rs", get_lib_resource());
    prog_resources
}

/// Locates the solana install, returns Option<active version string> if found
/// otherwise None
pub fn get_solana_installed_version() -> CargoResult<String> {
    match home_dir() {
        Some(p) => {
            let mut path = PathBuf::new();
            path.push(p);
            path.push(SOLANA_INSTALL);
            match path.exists() {
                true => {
                    let mut contents = String::new();
                    let mut file = File::open(path)?;
                    file.read_to_string(&mut contents)?;
                    let docs = YamlLoader::load_from_str(&contents)?;
                    let res = docs[0][VER_STRING].as_str().unwrap();
                    Ok(res[1..].to_string())
                }
                false => Err(ProgramError::SolanaNotFound(path.display().to_string())),
            }
        }
        None => Err(ProgramError::NoHomeFound),
    }
}

#[inline]
pub fn cargo_template_as_manifest() -> CargoResult<Manifest> {
    // Load template and substitute placeholders
    Ok(Manifest::from_str(
        &str::from_utf8(include_bytes!("../resources/program/prog.cargo.toml")).unwrap(),
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version_info_pass() {
        assert!(get_solana_installed_version().unwrap().len() > 0);
    }

    #[test]
    fn entry_point_pass() {
        println!("{:?}", get_entry_point_resource("foo".to_string()));
    }
}
