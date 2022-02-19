//! @brief command line functions

use crate::utils::{build_program_manifest, project_template_as_manifest};

use {
    cargo_toml::Manifest,
    clap::{command, AppSettings, Arg, Command},
    std::{env, str},
};

// Constants
const SOLANA_NAME: &str = "solana";

#[derive(Debug)]
pub enum ExecutionCommand {
    Create,
    Init,
}

#[derive(Debug)]
pub struct Configuration {
    pub progname: String,
    pub command: ExecutionCommand,
    pub init_manifest: Option<Manifest>,
    pub program_manifest_template: Manifest,
    pub project_manifest_template: Manifest,
}

impl Configuration {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // If run normally, the args passed are 'cargo-solana', '<args>'.  However, if run as
        // a cargo subcommand (i.e. cargo solana <target dir>), then cargo injects a new arg:
        // 'cargo-solana', 'solana', '<args>'.  We need to filter this extra arg out.
        //
        // This yields the situation where if the binary receives args of 'cargo-solana', 'solana'
        // then it might be a valid call (not a cargo subcommand - the user entered
        // 'cargo-solana solana' meaning to create a target dir called 'solana') or it might be an
        // invalid call (the user entered 'cargo solana' with no target dir specified).  The latter
        // case is assumed as being more likely.
        let mut vargs = Vec::<String>::new();
        for a in env::args() {
            match a.as_str() {
                SOLANA_NAME => {}
                _ => vargs.push(a),
            }
        }
        // Parse command line
        let mut cmdline = build_command_line_parser();
        let matches = cmdline.try_get_matches_from_mut(vargs);
        let config = match matches {
            Err(e) => e.exit(),
            _ => {
                let cmd_match = matches?;
                let (cmd, name, manifest) = match cmd_match.subcommand() {
                    Some(("create", s)) => (
                        ExecutionCommand::Create,
                        s.value_of("projprogname").unwrap(),
                        None,
                    ),
                    Some(("init", s)) => (
                        ExecutionCommand::Init,
                        s.value_of("progname").unwrap(),
                        Some(Manifest::from_path("./Cargo.toml")?),
                    ),
                    _ => unreachable!(),
                };

                // Complete configuration with
                // Preformatted program manifest
                // Project manifest
                Configuration {
                    command: cmd,
                    init_manifest: manifest,
                    program_manifest_template: build_program_manifest(name.to_string())?,
                    project_manifest_template: project_template_as_manifest()?,
                    progname: name.to_string(),
                }
            }
        };
        Ok(config)
    }
}

/// Builds command line argument parser
fn build_command_line_parser() -> Command<'static> {
    command!()
        .arg_required_else_help(true)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .propagate_version(true)
        /* Create a new Solana project
            'project'
                Cargo.toml (with workspace member == program)
                'program'
                    src
                        entry_point.ts
                        error.rs
                        instruction.rs
                        process.rs
                    tests
                        lib.rs
                    Cargo.toml (includes standard Solana dependencies and dev dependencies)
        */
        .subcommand(
            Command::new("create")
                .about("Create new Solana Program project")
                .arg(
                    Arg::new("projprogname")
                        .long("project-program-name")
                        .short('n')
                        .required(true)
                        .takes_value(true)
                        .help("Project's Program name (required)"),
                ),
        )
        /* Initialize a new Solana program folder in existing folder
            existingproject
                Cargo.toml (add workspace member or workspace altogether)
                progname
                    src
                        entry_point.ts
                        error.rs
                        instruction.rs
                        process.rs
                    tests
                        lib.rs
                    Cargo.toml (includes standard Solana dependencies and dev dependencies)
        */
        .subcommand(
            Command::new("init")
                .about("Add Solana program to current folder")
                .arg(
                    Arg::new("progname")
                        .long("program-name")
                        .short('n')
                        .required(true)
                        .takes_value(true)
                        .help("Program name"),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cmdline_help_pass() {
        let args = vec!["cargo-solana", "--help"];
        let mut cmdline = build_command_line_parser();
        let matches = cmdline.try_get_matches_from_mut(args);
        assert!(matches.is_err());
        match matches {
            Err(e) => assert_eq!(e.kind(), clap::ErrorKind::DisplayHelp),
            _ => panic!(),
        }
    }
    #[test]
    fn base_cmdline_version_pass() {
        let args = vec!["cargo-solana", "--version"];
        let mut cmdline = build_command_line_parser();
        let matches = cmdline.try_get_matches_from_mut(args);
        assert!(matches.is_err());
        match matches {
            Err(e) => assert_eq!(e.kind(), clap::ErrorKind::DisplayVersion),
            _ => panic!(),
        }
    }
    #[test]
    fn base_cmdline_create_fail() {
        let args = vec!["cargo-solana", "create"];
        let mut cmdline = build_command_line_parser();
        let matches = cmdline.try_get_matches_from_mut(args);
        assert!(matches.is_err());
        match matches {
            Err(e) => assert_eq!(e.kind(), clap::ErrorKind::MissingRequiredArgument),
            _ => panic!(),
        }
    }
    #[test]
    fn base_cmdline_init_fail() {
        let args = vec!["cargo-solana", "init"];
        let mut cmdline = build_command_line_parser();
        let matches = cmdline.try_get_matches_from_mut(args);
        assert!(matches.is_err());
        match matches {
            Err(e) => assert_eq!(e.kind(), clap::ErrorKind::MissingRequiredArgument),
            _ => panic!(),
        }
    }
    #[test]
    fn cargo_read_pass() {
        let man = Manifest::from_path("./Cargo.toml");
        assert!(man.is_ok());
    }
}
