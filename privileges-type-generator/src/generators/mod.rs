use std::{fs::File, io::Read};

use crate::structs::Privileges;

pub mod generator_factory;
pub mod language;
pub mod rust_generator;
pub mod typescript_generator;
pub mod java_generator;

pub trait Generator {
    fn generate_resource_privileges(
        &self,
        file: &mut File,
        resource: &String,
        privileges_list: &Vec<String>,
    );

    fn generate_pivileges_file(&self, privileges: Privileges, path: &str) -> std::io::Result<()> {
        let _dir = std::fs::create_dir_all(&path);
        let mut file = std::fs::File::create(format!("{path}/{}", self.get_filename()))?;
    
        for (resource, privileges_list) in privileges.resources.iter() {
            self.generate_resource_privileges(&mut file, &resource, &privileges_list);
        }
    
        Ok(())
    }

    fn load_template_file(&self, path: &str) -> String {
        let mut template_file = std::fs::File::open(path).expect("Template file does not exist.");
        let mut template_string = String::new();
        let _ = template_file.read_to_string(&mut template_string);
        template_string
    }

    fn get_filename(&self) -> &str;
}