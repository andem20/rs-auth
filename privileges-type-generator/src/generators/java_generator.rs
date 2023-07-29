use std::{io::Write, fs::File};

use crate::{util::string_util, constants::template_constants::{RESOURCE_WILDCARD, RESOURCE_VALUES_WILDCARD}};

pub struct JavaGenerator {}

impl super::Generator for JavaGenerator {
    fn generate_resource_privileges(&self, file: &mut File, resource: &String, privileges_list: &Vec<String>) {
        let resource_cap = string_util::capitalize(resource);

        let template_string = self.load_template_file("./templates/java.java.template");
        let template_string = template_string.replace(&RESOURCE_WILDCARD, &resource_cap);

        let mut resource_values = String::new();
        
        for privilege in privileges_list {
            let privilege_cap = string_util::capitalize(privilege);
            resource_values.push_str(&format!("\t\t{privilege_cap},\n"));
        }

        let template_string = template_string.replace(&RESOURCE_VALUES_WILDCARD, &resource_values);

        let _ = file.write(template_string.as_bytes());
    }

    fn get_filename(&self) -> &str {
        "Privileges.java"
    }

    fn generate_pivileges_file(&self, privileges: crate::structs::Privileges, path: &str) -> std::io::Result<()> {
        let _dir = std::fs::create_dir_all(&path);
        let mut file = std::fs::File::create(format!("{path}/{}", self.get_filename()))?;

        file.write(b"public class Privileges {\n")?;
    
        for (resource, privileges_list) in privileges.resources.iter() {
            self.generate_resource_privileges(&mut file, &resource, &privileges_list);
        }

        file.write(b"}")?;
    
        Ok(())
    }
}