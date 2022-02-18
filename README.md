# cargo-solana

Cargo tool to generate boilerplate Solana program to save time in basic boilerplate setup.

Program creation behaviors:
* If creating a project, `git` is assumed and a repo initialized
* If initializing just a program, VCS is ignored
* Program
    * entry point includes starter unit test framework for BPF testing and debugging
    * Program account serialization / deserialization implemented using Pack trait assuming non-variable data
    * Account structure
        * Includes initialization flag (u8) and verification
        * Includes version field (u8) for future change management. See COOKBOOK REF
    * `msg!` sprinkled throughout. Recommend removing these as you go as they consume Compute Units
    * Addition dependencies
        * `borsh` : For serialize/deserialize
        * `thiserror` : Simplify custom error


## Install
`cargo-solana` is not on crates, you need to build/deploy from repo

```bash
git clone REPO
cd REPO
cargo install cargo-solana --path .
```

## Help
`cargo solana --help`

## Scenarios
1. You want to create a whole new project that contains the framework for Solana program

`cargo solana create -n <PROJECT_NAME>`

Generates the following in:
```bash
    PROJECT_NAME
    Cargo.toml
    program
        Cargo.toml # Adds PROJECT_NAME as the program name
        src
            entry_point.rs
            error.rs
            instruction.rs
            process.rs
            state.rs
```

2. You just want to add a program to an existing project

`cargo solana init -n <PROGRAM_NAME>`

Adds only the program folder and contents:
```bash
    EXISTING_PROJECT_NAME
    Cargo.toml # Program folder added to `workspace` members
    program
        Cargo.toml
        src
            entry_point.rs
            error.rs
            instruction.rs
            process.rs
            state.rs
```