use std::{io::Write, fs::File};

use crate::{util::string_util, constants::template_constants::{RESOURCE_WILDCARD, RESOURCE_VALUES_WILDCARD, ENTRIES_WILDCARD}};

pub struct RustGenerator {}

impl super::Generator for RustGenerator {
    fn generate_resource_privileges(&self, file: &mut File, resource: &String, privileges_list: &Vec<String>) {
        let resource_cap = string_util::capitalize(resource);

        let template_string = self.load_template_file("./templates/rust.rs.template");
        let template_string = template_string.replace(&RESOURCE_WILDCARD, &resource_cap);

        let mut entries = String::new();
        let mut resource_values = String::new();
        
        for privilege in privileges_list {
            let privilege_cap = string_util::capitalize(privilege);
            resource_values.push_str(&format!("\t{privilege_cap},\n"));
            entries.push_str(&format!("\t\t\t{privilege_cap} => Ok({resource_cap}::{privilege_cap}),\n"));
        }

        let template_string = template_string.replace(&RESOURCE_VALUES_WILDCARD, &resource_values);
        let template_string = template_string.replace(&ENTRIES_WILDCARD, &entries);

        let _ = file.write(template_string.as_bytes());
    }

    fn get_filename(&self) -> &str {
        "mod.rs"
    }
}