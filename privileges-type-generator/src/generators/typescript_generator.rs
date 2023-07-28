use std::{io::{Write, Read}, fs::File};

use crate::{util::string_util, constants::template_constants::{RESOURCE_WILDCARD, RESOURCE_VALUES_WILDCARD}};

pub struct TypeScriptGenerator {}

impl super::Generator for TypeScriptGenerator {
    fn generate_resource_privileges(&self, file: &mut File, resource: &String, privileges_list: &Vec<String>) {
        let resource_cap = string_util::capitalize(resource);

        let mut template_file = std::fs::File::open("./templates/typescript.ts.template").expect("Template file does not exist.");
        let mut template_string = String::new();
        let _ = template_file.read_to_string(&mut template_string);

        let template_string = template_string.replace(&RESOURCE_WILDCARD, &resource_cap);

        let mut resource_values = String::new();
        
        for privilege in privileges_list {
            let privilege_cap = string_util::capitalize(privilege);
            resource_values.push_str(&format!("\t{privilege_cap} = '{privilege}',\n"));
        }

        let template_string = template_string.replace(&RESOURCE_VALUES_WILDCARD, &resource_values);

        let _ = file.write(template_string.as_bytes());
    }

    fn get_filename(&self) -> &str {
        "enum.ts"
    }
}