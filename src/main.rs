pub mod cli;
pub mod conf;

use cli::command_builder::*;
use conf::types::*;

fn main(){
    let command = "--debug set wallpaper /path/to/file.png";

    let config: Config = Config::from_file("debug_data/config.yaml").unwrap();

    let builder = CommandBuilder::new(Some(config));

    let parsed_command = builder.parse_command(command.to_string());

    println!("{:#?}", parsed_command);
}
