use std::{collections::HashMap};

use crate::conf::types::*;

pub struct ConfHasher{
}

impl ConfHasher{
    #[allow(unused)]
    pub fn new() -> Self{
        Self {  }
    }
    
    #[allow(unused)]
    pub fn parse_conf(&self, config: Config) -> String{
        get_subcommand_str(&config.commands)
    }
}

fn get_command_string(command: &ConfCommand) -> String{
    let flag_str = get_flag_str(&command.flags);
    let param_str = get_param_str(&command.params);
    let subcommand_str = get_subcommand_str(&command.subcommands);

    format!("{flag_str}{param_str}{subcommand_str}")
}

fn get_param_str(params: &[ConfParam]) -> String{
    params.iter()
        .map(|p| {
            let name_string = format!("{}{}", p.name.chars().next().unwrap(), p.name.len());
            let param_type = p.param_type as u8;

            format!("{name_string}{param_type}")
        }).collect()
}

fn get_flag_str(flags: &[ConfFlag]) -> String{
    flags.iter()
        .map(|f| {
            let flag_num = f.flag_type as u8;
            let run_str = format!{"{}", f.run.len()};
            let param_str = get_param_str(&f.params);
            let name_str: String = f.names.iter()
                .map(|n| {
                    format!("{}{}", n.chars().last().unwrap(), n.len())
                }).collect();
            format!("{name_str}{flag_num}{param_str}{run_str}")
        }).collect()
}

fn get_subcommand_str(subcommands: &HashMap<String, ConfCommand>) -> String{
    let mut subcommad_map: Vec<_> = subcommands.iter().collect();
    subcommad_map.sort_by_key(|(k, _)| k.to_string());

    subcommad_map.into_iter()
        .map(|(k, v)| {
            let key_str = format!("{}{}", k.chars().next().unwrap(), k.len());
            let value_str = get_command_string(v);

            format!("{key_str}{value_str}")
        }).collect()
}
