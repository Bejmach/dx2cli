use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub commands: std::collections::HashMap<String, ConfCommand>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfCommand{
    pub name: String,
    pub description: String,
    pub flags: Option<Vec<ConfFlag>>,
    pub subcommands: Option<std::collections::HashMap<String, ConfCommand>>,
    pub params: Option<Vec<ConfParam>>,
    pub run: Option<String>,
}

impl ConfCommand{
    pub fn has_flag(&self, flag: &String) -> Option<&ConfFlag>{
        for i in 0..self.flags.as_ref().unwrap().len(){
            if self.flags.as_ref().unwrap().get(i).unwrap().flag.contains(flag){
                return self.flags.as_ref().unwrap().get(i);
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FlagType{
    Modify,
    Overwrite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfFlag{
    pub flag: Vec<String>,
    pub flag_type: FlagType,
    pub description: String,
    pub param: Option<ConfParam>,
    pub run: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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



