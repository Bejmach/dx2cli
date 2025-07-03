use std::{collections::HashMap, env, fs::{self, File}, io::Write};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

use crate::{conf::types::*, default::config::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfSignature{
    key: String,
}

pub struct ConfHasher{
}

impl ConfHasher{
    #[allow(unused)]
    pub fn new() -> Self{
        Self {  }
    }
    
    #[allow(unused)]
    pub fn parse_conf(&self, commands: &std::collections::HashMap<String, ConfCommand>) -> String{
        get_subcommand_str(commands)
    }

    /*pub fn verify_signature(&self, name: &String) -> bool{
        let config = Config::from_file(name).expect("No cofig file found");
        let signature = self.read_signature(name).expect("No signature exist");

        let defaults = DefaultReader::new();

        let config_hash = encrypt_string(self.parse_conf(config), defaults.get_key().to_string());

        config_hash == signature.key
    }
    pub fn create_signature(&self, name: &str){
        let config = Config::from_file(name).expect("No cofig file found");

        let config_parsed = self.parse_conf(config);
        let defaults = DefaultReader::new();

        let config_hash = encrypt_string(config_parsed, defaults.get_key().to_string());

        let conf_sig = ConfSignature{key: config_hash};

        let config_yaml = serde_yml::to_string(&conf_sig).expect("Could not parse config to string");

        let signature_str = format!("# hash generated automatically usign graCli, any change will cause the config to stop working, and will require to verify the file again\n{config_yaml}");

        let mut sig_path = env::home_dir().expect("Failed to get home directory");
        sig_path.push(format!(".config/gracli/.{name}.sig.yaml"));

        let mut file = File::create(&sig_path).expect("Could not create file");

        file.write_all(signature_str.as_bytes()).expect("Could not write config to file");
    }*/

    pub fn signature_exists(&self, name: &String) -> bool{
        let mut sig_path = env::home_dir().expect("Failed to get home directory");
        sig_path.push(format!(".config/gracli/.{name}.sig.yaml"));

        sig_path.exists()
    }
    fn read_signature(&self, name: &String) -> Option<ConfSignature>{
        if !self.signature_exists(name) {
            return None;
        }

        let mut sig_path = env::home_dir().expect("Failed to get home directory");
        sig_path.push(format!(".config/gracli/.{name}.sig.yaml"));
    
        let file_str = fs::read_to_string(sig_path).unwrap();
        
        let signature: ConfSignature = serde_yml::from_str(&file_str).unwrap();

        Some(signature)
    }
}

pub fn encrypt_string(str: String, salt: String) -> String{
    let combined_string = format!("{str}{salt}");

    let mut hasher = Sha256::new();

    hasher.update(combined_string);

    let result = hasher.finalize();
    hex::encode(result)
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
