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

    pub fn parse_command(&self, command: String) -> Option<CliCommand>{
        if self.config.is_none(){
            return None;
        }

        let mut split_command = command.split_whitespace();
        let mut base_command = CliCommand::new("default".to_string());
        let mut cur_command = &mut base_command;
        let mut cur_config: &ConfCommand = &ConfCommand::placeholder();

        let mut param = split_command.next(); 
        while param.is_some(){
            if param.is_none(){
                return Some(base_command);
            }

            let param_val = param.unwrap();

            if cur_command.name == "default"{
                if is_flag(param_val.to_string()){
                    println!("Skipping default flags");
                    param = split_command.next();
                    continue;
                }
                else{
                    if self.config.as_ref().unwrap().get_command(&param_val.to_string()).is_none(){
                        println!("No command \"{}\" in \"{}\"", param.unwrap(), base_command.to_string());
                        return None;
                    }
                    cur_command.subcommand = Some(Box::new(CliCommand::new(param_val.to_string())));
                    cur_command = cur_command.subcommand.as_mut().unwrap();
                    cur_config = self.config.as_ref().unwrap().get_command(&param_val.to_string()).unwrap();
                    param = split_command.next();
                }
            }

            else if cur_config.description == "placeholder command"{
                println!("Error while parsing command. Config placeholder found");
                return None;
            }

            else if is_flag(param_val.to_string()){
                let conf_flag = cur_config.get_flag(&param_val.to_string());
                if conf_flag.is_none(){
                    println!("Command \"{}\" does not contain flag \"{}\"", base_command.to_string(), param_val);
                    return None;
                }
                
                let mut flag = CliFlag::new(param_val.to_string());

                for i in 0..conf_flag.unwrap().params.len(){
                    param = split_command.next();

                    if param.is_none(){
                        println!("No param \"{}\" for flag \"{:?}\" in command \"{}\"", conf_flag.unwrap().params[i].name, conf_flag.unwrap().names, base_command.to_string());
                        return None;
                    }

                    if !is_type(param.unwrap().to_string(), &conf_flag.unwrap().params[i].param_type){
                        println!("Wrong type for param \"{}\". Expected type: {:?}", conf_flag.unwrap().params[i].name, conf_flag.unwrap().flag_type);
                        return None;
                    }

                    let cli_param = CliParam::new(conf_flag.unwrap().params[i].name.clone(), param.unwrap().to_string());
                    flag.add_param(cli_param);
                }
                    
                cur_command.add_flag(flag);
            }
            
            for i in 0..cur_config.params.len(){
                println!("{}", cur_config.params[i].name);
                if param.is_none(){
                    println!("No param \"{}\" for command \"{}\"", cur_config.params[i].name, base_command.to_string());
                    return None;
                }

                if !is_type(param.unwrap().to_string(), &cur_config.params[i].param_type){
                    println!("Wrong type for param \"{}\". Expected type: {:?}", cur_config.params[i].name, cur_config.params[i].name);
                    return None;
                }

                let cli_param = CliParam::new(cur_config.params[i].name.clone(), param.unwrap().to_string());
                cur_command.add_param(cli_param);

                param = split_command.next();
            }

            if param.is_some(){
                let new_config = cur_config.subcommands.get(param.unwrap());
                if new_config.is_some(){
                    cur_command.subcommand = Some(Box::new(CliCommand::new(param.unwrap().to_string())));
                    cur_command = cur_command.subcommand.as_mut().unwrap();
                    cur_config = new_config.unwrap();
                }
                else{
                    println!("No subcommand \"{}\" for command \"{}\"", param.unwrap(), base_command.to_string());
                    return None;
                }
            }

            param = split_command.next();
        }

        Some(base_command)
    }

    pub fn validate_command(&self, command: CliBuild) -> bool{
        false
    }
}
