use std::{collections::HashMap, env, fs};

use serde::{Serialize, Deserialize};

static EXPECTED_FORMATS: &[&str] = &[".yaml", ".yml", ".gra.yaml", ".gra.yml"];

pub fn commands_from_file(path: &str) -> std::collections::HashMap<String, ConfCommand>{
    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(format!(".config/gracli"));

    let correct_path = verify_format(&config_path, path);
    if correct_path.is_empty(){
        println!("{}", config_path.to_str().unwrap());
        panic!("No file with the provided name found");
    }

    let config_str = fs::read_to_string(correct_path).expect(&format!("No config in location: \"{path}\", exist"));
    let commands: std::collections::HashMap<String, ConfCommand> = serde_yml::from_str(&config_str).expect("Could not parse config");

    commands
}

pub fn verify_format(path: &std::path::Path, sufix: &str) -> String{
    for form in EXPECTED_FORMATS{
        let mut expected_path = path.to_path_buf();
        expected_path.push(format!("{sufix}{form}"));

        println!("{}: {}", expected_path.to_str().unwrap(), expected_path.exists());

        if expected_path.exists(){
            return expected_path.to_str().unwrap().to_string();
        }
    }
    String::new()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfCommand{
    pub description: String,

    #[serde(default = "default_subcommand_map")]
    pub subcommands: std::collections::HashMap<String, ConfCommand>,

    #[serde(default = "default_subcommand_import")]
    pub import: Vec<String>,

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
            import: Vec::new(),
            flags: Vec::new(), 
            params: Vec::new(),
            run: "".to_string(),
        }
    }

    pub fn extend(&mut self, commands: HashMap<String, ConfCommand>){
        self.subcommands.extend(commands);
    }
}

fn default_subcommand_import() -> Vec<String>{
    Vec::new()
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



