pub mod cli;
pub mod conf;

use std::fs;

use cli::command_builder::*;
use conf::types::*;

fn main(){
    let config_str = fs::read_to_string("debug_data/config.yaml").unwrap();

    let command = "--debug set wallpaper /path/to/file.png";
    let mut builder: CommandBuilder = CommandBuilder::new(None);

    let parsed = builder.parse_command(command.to_string());

    println!("{}", parsed);
    println!("{}", config_str);

    let config: Config = serde_yml::from_str(&config_str).unwrap();

    println!("{:#?}", config);

    builder.set_config(config);

    println!("{}", builder.validate_command(parsed));
}
