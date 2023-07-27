use std::io::Write;

use crate::util::{string_util, file_util};

pub struct RustGenerator {}

impl super::Generator for RustGenerator {
    fn create_enum(&self, file: &mut std::fs::File, resource: &String, privileges_list: &Vec<String>) {
        let resource_cap = string_util::capitalize(resource);
        let _ = file.write(b"#[derive(Debug)]\n");
        let head = format!("pub enum {resource_cap} {{\n");
        let _ = file.write(head.as_bytes());
        
        for privilege in privileges_list {
            let privilege_cap = string_util::capitalize(privilege);
            file_util::write_line(file, 1, privilege_cap)
        }
    
        let _ = file.write(b"}\n");
    }
    
    fn create_deserializer(&self, file: &mut std::fs::File, resource: &String, privileges_list: &Vec<String>) {
        let resource_cap = string_util::capitalize(resource);
        let head = format!("impl std::str::FromStr for {resource_cap} {{\n");
        let _ = file.write(head.as_bytes());
        let _ = file.write(b"\ttype Err = ();\n");
        let _ = file.write(b"\tfn from_str(input: &str) -> Result<Self, Self::Err> {\n");
        let _ = file.write(b"\t\tmatch input {\n");
        for privilege in privileges_list {
            let privilege_cap = string_util::capitalize(privilege);
            let _ = file.write(format!("\t\t\t\"{privilege}\" => Ok({resource_cap}::{privilege_cap}),\n").as_bytes());
        }
        let _ = file.write(b"\t\t\t_ => Err(()),\n");
        let _ = file.write(b"\t\t}\n");
        let _ = file.write(b"\t}\n");
        let _ = file.write(b"}\n");
    }
}