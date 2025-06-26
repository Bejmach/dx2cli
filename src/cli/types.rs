use crate::conf::types::*;
use std::{fmt};

#[derive(Debug)]
pub struct CliCommand{
    pub name: String,
    pub flags: Vec<CliFlag>,
    pub subcommand: Option<Box<CliCommand>>,
}

impl CliCommand{
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
            CliCommand::new(subcommand)
        ));
    }
    pub fn get_subcommand_ref(&self) -> &CliCommand{
        self.subcommand.as_ref().unwrap()
    }
    pub fn get_subcommand_mut_ref(&mut self) -> &mut CliCommand{
        self.subcommand.as_mut().unwrap()
    }
}

impl fmt::Display for CliCommand{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_formatter(cmd: &CliCommand, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result{
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
}
impl CliFlag{
    pub fn new(name: String) -> Self{
        Self { 
            name
        }
    }
}
impl fmt::Display for CliFlag{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
