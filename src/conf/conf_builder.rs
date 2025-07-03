use crate::conf::types::*;

pub struct ConfBuilder{
    pub commands: std::collections::HashMap<String, ConfCommand>,
}

impl ConfBuilder{
    pub fn from_file(lua_path: String) -> Self{
        let mut commands = commands_from_file(&parse_yaml_path(lua_path));
        import_commands(&mut commands);

        Self { commands }
    }
}

pub fn parse_yaml_path(path: String) -> String{
    path.replace(".", "/")
}

pub fn import_commands(commands: &mut std::collections::HashMap<String, ConfCommand>){
    for value in commands.values_mut(){
        if !value.import.is_empty(){
            for i in 0..value.import.len(){
                let imported_commands = commands_from_file(&parse_yaml_path(value.import[i].clone()));
                value.extend(imported_commands);
            }
        }

        import_commands(&mut value.subcommands);
    }
}
