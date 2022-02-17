//! @brief execution operations

use crate::{cli::Configuration, utils::get_program_resources};

pub fn gen_program(config: &Configuration) {
    println!("{:?}", std::env::current_dir().unwrap());
    println!(
        "{}",
        toml::to_string(&config.program_manifest_template).unwrap()
    );
    for (location, file_text) in get_program_resources(config.progname.clone()) {
        println!("CREATING FILE: {}", location);
        println!("{}", file_text);
    }
}
pub fn init_program(config: &Configuration) {
    gen_program(config)
}
