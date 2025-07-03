use rand::prelude::*;
use std::{
        fs::{self, File},
        io::Write
    };
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultConfig{
    pub key: String,
    pub current_config: String,
}

impl DefaultConfig{
    pub fn new(key: String) -> Self{
        Self {
            key,
            current_config: String::new(),
        }
    }
}

pub struct DefaultReader{
    defaults: DefaultConfig,
}

impl DefaultReader{
    pub fn new() -> Self{
        if !check_defaults(){
            let result = create_defaults();
            if result.is_err(){
                panic!("Error creating defaults");
            }
        }

        let defaults = read_defaults().expect("Error reading defaults");

        Self { defaults }
    }

    pub fn get_key(&self) -> &String{
        &self.defaults.key
    }
}

impl Default for DefaultReader{
    fn default() -> Self{
        if !check_defaults(){
            let result = create_defaults();
            if result.is_err(){
                panic!("Error creating defaults");
            }
        }

        let defaults = read_defaults().expect("Error reading defaults");

        Self { defaults }
    }
}

fn check_defaults() -> bool{
    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(".config/gracli/defaults.gra.yaml");
    config_path.exists()
}
    
fn create_defaults() -> std::io::Result<()>{
    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(".config/gracli/defaults.gra.yaml");
    
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&config_path)?;

    let  mut rng = rand::rng();
    let mut key_str = String::new();

    for _i in 0..32{
        key_str.push( rng.sample(rand::distr::Alphanumeric) as char );
    }

    let config = DefaultConfig::new(key_str);
    file.write_all(serde_yml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}

fn read_defaults() -> Option<DefaultConfig>{
    if !check_defaults() {
        return None;
    }

    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(".config/gracli/defaults.gra.yaml");
    
    let file_str = fs::read_to_string(config_path).unwrap();
        
    let config: DefaultConfig = serde_yml::from_str(&file_str).unwrap();

    Some(config)
}
