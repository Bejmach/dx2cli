use std::{collections::HashMap, env, fs};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub commands: std::collections::HashMap<String, ConfCommand>,
}

impl Config{
    pub fn from_file(name: &str) -> Option<Self>{
        let mut config_path = env::home_dir().expect("Failed to get home directory");
        config_path.push(format!(".config/gracli/{name}.yaml"));

        let config_str = fs::read_to_string(config_path).expect(&format!("No config with name: \"{name}\", exist"));
        let config = serde_yml::from_str(&config_str);
        if config.is_ok(){
            return Some(config.unwrap());
        }
        else{
            println!("Parse error: {}", config.err().unwrap());
            return None;
        }
    }
    pub fn get_command(&self, command: &String) -> Option<&ConfCommand>{
        self.commands.get(command)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfCommand{
    pub description: String,

    #[serde(default = "default_subcommand_map")]
    pub subcommands: std::collections::HashMap<String, ConfCommand>,

    #[serde(default = "default_flag_vec")]
    pub flags: Vec<ConfFlag>,
    
    #[serde(default = "default_param_vec")]
    pub params: Vec<ConfParam>,

    #[serde(default = "default_run")]
    pub run: String,
}

impl ConfCommand{
    pub fn get_flag(&self, flag: &String) -> Option<&ConfFlag>{
        for i in 0..self.flags.len(){
            if self.flags.get(i).unwrap().names.contains(flag){
                return self.flags.get(i);
            }
        }
        None
    }
    pub fn get_subcommand(&self, command: &String) -> Option<&ConfCommand>{
        self.subcommands.get(command)
    }

    pub fn placeholder() -> Self{
        Self { 
            description: "placeholder command".to_string(), 
            subcommands: HashMap::new(), 
            flags: Vec::new(), 
            params: Vec::new(),
            run: "".to_string(),
        }
    }
}

fn default_subcommand_map() -> std::collections::HashMap<String, ConfCommand>{
    std::collections::HashMap::new()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlagType{
    Modify,
    Overwrite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfFlag{
    pub names: Vec<String>,
    pub description: String,

    #[serde(default = "default_flag_id")]
    pub id: String,

    #[serde(default = "default_flag_type")]
    pub flag_type: FlagType,

    #[serde(default = "default_param_vec")]
    pub params: Vec<ConfParam>,

    #[serde(default = "default_run")]
    pub run: String,
}

fn default_flag_id() -> String{
    String::new()
}

fn default_flag_type() -> FlagType{
    FlagType::Modify
}

fn default_param_vec() -> Vec<ConfParam>{
    Vec::new()
}
fn default_flag_vec() -> Vec<ConfFlag>{
    Vec::new()
}

fn default_run() -> String{
    "".to_string()
}

impl ConfFlag{
    pub fn new(names: Vec<String>, description: String) -> Self{
        ConfFlag { 
            names,
            description, 
            id: String::new(),
            flag_type: FlagType::Modify,
            params: Vec::new(),
            run: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParamType{
    Int,
    String,
    Bool,
    Float,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfParam{
    pub name: String,

    #[serde(default = "default_param_type")]
    pub param_type: ParamType,
}

fn default_param_type() -> ParamType{
    ParamType::String
}

pub fn is_type(str: String, param_type: &ParamType) -> bool{
    match param_type {
        ParamType::Int => {str.parse::<i32>().is_ok()},
        ParamType::Float => {str.parse::<f32>().is_ok()},
        ParamType::String => true,
        ParamType::Bool => {str == "true" || str == "false" || str == "0" || str == "1"}
    }
}



