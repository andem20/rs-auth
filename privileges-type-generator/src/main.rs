use std::{collections::HashMap, io::Write};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Privileges {
    default: Vec<String>,
    resources: HashMap<String, Vec<String>>
}


fn main() -> std::io::Result<()> {
    let path = "../privileges.yaml";
    let file = std::fs::File::open(&path)?;

    let privileges: Privileges = serde_yaml::from_reader(file).expect("Could not deserialize file");

    let _ = create_privileges(privileges);

    Ok(())
}

fn create_privileges(privileges: Privileges) -> std::io::Result<()> {
    let path = "../privileges";
    let filename = "mod.rs";
    let _dir = std::fs::create_dir_all(&path);
    let mut file = std::fs::File::create(format!("{path}/{filename}"))?;

    for (resource, privileges_list) in privileges.resources.iter() {
        let resource_cap = capitalize(resource);

        let head = format!("pub enum {} {{\n", resource_cap);
        let _ = file.write(head.as_bytes());
        
        for privilege in privileges_list {
            if privilege == "default" {
                for default_privilege in privileges.default.iter() {
                    let default_privilege_cap = capitalize(default_privilege);
                    let _ = file.write(b"\t");
                    let _ = file.write(default_privilege_cap.as_bytes());
                    let _ = file.write(b",\n");
                }

                continue;
            }

            let privilege_cap = capitalize(privilege);
            let _ = file.write(b"\t");
            let _ = file.write(privilege_cap.as_bytes());
            let _ = file.write(b",\n");
        }

        let _ = file.write(b"}\n");
    }

    Ok(())
}

fn capitalize(string: &String) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    
    chars.into_iter().collect()
}
