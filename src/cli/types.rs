use std::{fmt};

#[derive(Debug)]
pub struct CliBuild{
    pub name: String,
    pub flags: Vec<CliFlag>,
    pub subcommand: Option<Box<CliBuild>>,
}

impl CliBuild{
    pub fn new(name: String) -> Self{
        Self{
            name,
            flags: Vec::new(),
            subcommand: None,
        }
    }
    pub fn insert_flag(&mut self, flag: String){
        self.flags.push(CliFlag::new(flag));
    }
    pub fn set_subcommand(&mut self, subcommand: String){
        self.subcommand = Some(Box::new(
            CliBuild::new(subcommand)
        ));
    }
    pub fn get_subcommand_ref(&self) -> Option<&CliBuild>{
        if self.subcommand.is_none(){
            return None;
        }

        Some(self.subcommand.as_ref().unwrap())
    }
    pub fn get_subcommand_mut_ref(&mut self) -> Option<&mut CliBuild>{
        if self.subcommand.is_none(){
            return None;
        }

        Some(self.subcommand.as_mut().unwrap())
    }
}

impl fmt::Display for CliBuild{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_formatter(cmd: &CliBuild, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result{
            let indend_str = " ".repeat(indent);
            writeln!(f, "Command{{\n")?;
            writeln!(f, "{}name: \"{}\"", indend_str, cmd.name)?;
            write!(f, "{}flags: [", indend_str)?;
            for i in 0..cmd.flags.len(){
                write!(f, "\"{}\"", cmd.flags[i])?;
                if i != cmd.flags.len()-1{
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
            match &cmd.subcommand {
                Some(sub) => {
                    write!(f, "{}subcommand: ", indend_str)?;
                    fmt_formatter(sub, f, indent+2)?;
                }
                None => {
                    writeln!(f, "{}subcommand: None", indend_str)?;
                }
            }
            writeln!(f, "{}}}", indend_str)
        }

        fmt_formatter(&self, f, 2)
    }
}

#[derive(Debug)]
pub struct CliFlag{
    pub name: String,
    pub params: Vec<CliParam>,
}
impl CliFlag{
    pub fn new(name: String) -> Self{
        Self { 
            name,
            params: Vec::new(),
        }
    }

    pub fn add_param(&mut self, param: CliParam){
        self.params.push(param);
    }
}
impl fmt::Display for CliFlag{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}



#[derive(Debug)]
pub struct CliCommand{
    pub name: String,
    pub subcommand: Option<Box<CliCommand>>,
    pub flags: Vec<CliFlag>,
    pub params: Vec<CliParam>,
}

impl CliCommand{
    pub fn new(name: String) -> Self{
        Self { 
            name,
            subcommand: None,
            flags: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn set_subcommand(&mut self, subcommand: CliCommand){
        self.subcommand = Some(Box::new(subcommand));
    }
    pub fn add_flag(&mut self, flag: CliFlag){
        self.flags.push(flag);
    }
    pub fn add_param(&mut self, params: CliParam){
        self.params.push(params);
    }

    pub fn to_string(&self) -> String{
        let mut return_str: String = String::new();
        let mut cur_command = self;
        return_str.push_str(&cur_command.name);
        while cur_command.subcommand.is_some(){
            return_str.push_str(" ");
            return_str.push_str(&cur_command.subcommand.as_ref().unwrap().name);
            cur_command = cur_command.subcommand.as_ref().unwrap()
        }

        return_str
    }
}

#[derive(Debug)]
pub struct CliParam{
    pub name: String,
    pub value: String,
}

impl CliParam{
    pub fn new(name: String, value: String) -> Self{
        Self {
            name,
            value,
        }
    }
}

pub fn is_flag(str: String) -> bool{
    str.len() != 0 && str.chars().nth(0).unwrap() == '-'
}
