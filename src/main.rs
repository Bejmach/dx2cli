pub mod cli;
pub mod conf;
pub mod security;
pub mod default;

use cli::command_builder::*;
use conf::types::*;

use crate::security::{conf_hasher::ConfHasher, key_reader::KeyReader};

fn main(){
    let command = "--debug set wallpaper /path/to/file.png";

    let config: Config = Config::from_file("debug_data/config.yaml").unwrap();

    println!("{:#?}", config);

    let builder = CommandBuilder::new(Some(config));

    let parsed_command = builder.parse_command(command.to_string());

    println!("{:#?}", parsed_command);

    let key_reader = KeyReader::default();

    println!("{}", key_reader.get_key());

    let hasher = ConfHasher::new();

    let config: Config = Config::from_file("debug_data/config.yaml").unwrap();

    println!("{}", hasher.parse_conf(config));
}
