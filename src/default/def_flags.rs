use crate::conf::types::*;


pub struct DefFlags{
    flags: Vec<ConfFlag>,
}

impl Default for DefFlags{
    fn default() -> Self {
        let mut flags: Vec<ConfFlag> = Vec::new();

        let remember_flag = ConfFlag::new(vec!["-r".to_string(), "--remember".to_string()], "Remember the command".to_string());
        let default_flag = ConfFlag::new(vec!["-d".to_string(), "--default".to_string()], "Run default commands".to_string());

        flags.push(remember_flag);
        flags.push(default_flag);

        Self {
            flags
        }
    } 
}

impl DefFlags{
    #[allow(unused)]
    fn has_flag(&self, flag: String) -> bool{
        for i in 0..self.flags.len(){
            if self.flags[i].names.contains(&flag){
                return true;
            }
        }
        false
    }
}
