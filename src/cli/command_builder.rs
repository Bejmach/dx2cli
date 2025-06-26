use crate::cli::types::*;
use crate::conf::types::*;

#[allow(dead_code)]
pub struct CommandBuilder{
    config: Option<Config>,
}

impl CommandBuilder{
    pub fn new(config: Option<Config>) -> Self{
        Self {
            config
        }
    }

    pub fn set_config(&mut self, config: Config){
        self.config = Some(config);
    }

    pub fn parse_command(&self, command: String) -> CliCommand{
        let mut split_command = command.split_whitespace();
        let command_len = command.split_whitespace().count();
        let mut base_command = CliCommand::new("start".to_string());
        let mut cur_command = &mut base_command;
        for _i in 0..command_len{
            let next_param = split_command.next();
            if next_param.is_none(){
                return base_command;
            }
            let param_str = next_param.unwrap().to_string();
            if param_str.len() == 0{
                continue;
            }

            if param_str.chars().nth(0).unwrap() == '-'{
                cur_command.insert_flag(param_str);
            }
            else{
                cur_command.set_subcommand(param_str);
                cur_command = cur_command.get_subcommand_mut_ref();
            }
        }

        base_command
    }

    pub fn validate_command(&self, command: CliCommand) -> bool{
        if self.config.is_none(){
            println!("Set config, before validating");
            return false;
        }
        let config: &Config = self.config.as_ref().unwrap();
        let mut cur_command = command.get_subcommand_ref();

        let mut conf_command = config.commands.get(&cur_command.name);

        loop {

            if conf_command.is_none(){
                return false;
            }

            for i in 0..cur_command.flags.len(){
                let flag = conf_command.unwrap().has_flag(&cur_command.flags.get(i).unwrap().name);
                if flag.is_none(){
                    return false;
                }
                if flag.unwrap().run.is_some(){
                    if flag.unwrap().param.is_none() && cur_command.subcommand.is_none(){
                        return true;
                    }
                    else if cur_command.subcommand.is_some() && 
                        cur_command.get_subcommand_ref().subcommand.is_none()
                        /* && flag.param.is_param(cur_command.get_subcommand_ref().name) */{
                        return true;
                    }
                    else {return false;}
                }
            }

            if conf_command.unwrap().params.is_some(){
                for _i in 0..conf_command.unwrap().params.as_ref().unwrap().len(){
                    if cur_command.subcommand.is_some()/* && conf_command.unwrap().params[i].is_param(cur_command.get_subcommand_ref().name) */{
                        cur_command = cur_command.get_subcommand_ref();
                    }
                    else {return false;}
                }
            }

            if cur_command.subcommand.is_none(){
                if conf_command.unwrap().run.is_some(){
                    break;
                }
                else {
                    return false;
                }
            }

            cur_command = cur_command.get_subcommand_ref();
            conf_command = conf_command.as_ref().unwrap().subcommands.as_ref().unwrap().get(&cur_command.name);

            if conf_command.is_none(){
                return false;
            }
        }

        return true;
    }
}
