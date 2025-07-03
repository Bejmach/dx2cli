use crate::conf::conf_builder::ConfBuilder;

pub mod cli;
pub mod conf;
pub mod security;
pub mod default;

fn main(){
    let conf_builder = ConfBuilder::from_file("config".to_string());

    println!("{:#?}", conf_builder.commands);
}
