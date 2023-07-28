use std::{io::Write, fs::File};

use crate::{util::{string_util, file_util}, structs::Privileges};

pub struct TypeScriptGenerator {}

impl TypeScriptGenerator {
    fn create_enum(&self, file: &mut std::fs::File, resource: &String, privileges_list: &Vec<String>) {
        let resource_cap = string_util::capitalize(resource);
        let head = format!("enum {resource_cap} {{\n");
        let _ = file.write(head.as_bytes());
        
        for privilege in privileges_list {
            let privilege_cap = string_util::capitalize(privilege);
            file_util::write_line(file, 1, privilege_cap)
        }
    
        let _ = file.write(b"}\n");
    }
}

impl super::Generator for TypeScriptGenerator {
    fn generate_resource_privileges(&self, file: &mut File, resource: &String, privileges_list: &Vec<String>) {
        todo!()
    }

    fn get_filename(&self) -> &str {
        "enum.ts"
    }
}