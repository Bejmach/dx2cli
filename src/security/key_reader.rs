use rand::prelude::*;
use std::{
        fs::{self, File},
        io::Write
    };
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityConf{
    pub key: String,
}

impl SecurityConf{
    pub fn new(key: String) -> Self{
        Self { 
            key
        }
    }
}

pub struct KeyReader{
    security_conf: SecurityConf,
}

impl Default for KeyReader{
    fn default() -> Self{
        if !check_security() {
            let result = create_security();
            if result.is_err(){
                panic!("{}", result.err().unwrap());
            }
        }
        let security_conf = read_security().unwrap();
        
        Self { 
            security_conf,
        }
    }
}

impl KeyReader{
    #[allow(unused)]
    pub fn get_key(&self) -> &String{
        &self.security_conf.key
    }
}

fn check_security() -> bool{
    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(".config/gracli/security.gra.yaml");
    config_path.exists()
}
    
fn create_security() -> std::io::Result<()>{
    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(".config/gracli/security.gra.yaml");
    
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&config_path)?;

    let  mut rng = rand::rng();
    let mut key_str = String::new();

    for _i in 0..32{
        key_str.push( rng.sample(rand::distr::Alphanumeric) as char );
    }

    let config = SecurityConf::new(key_str);
    file.write_all(serde_yml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}

fn read_security() -> Option<SecurityConf>{
    if !check_security() {
        return None;
    }

    let mut config_path = env::home_dir().expect("Failed to get home directory");
    config_path.push(".config/gracli/security.gra.yaml");
    
    let file_str = fs::read_to_string(config_path).unwrap();
        
    let config: SecurityConf = serde_yml::from_str(&file_str).unwrap();

    Some(config)
}
