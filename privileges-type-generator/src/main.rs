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

    let mut privileges: Privileges = serde_yaml::from_reader(file).expect("Could not deserialize file");

    privileges.resources.values_mut().for_each(|values| {
        if let Some(index) = values.iter().position(|v| v == "default") {
            values.remove(index);
            values.append(&mut privileges.default.clone());
        }
    });

    let _ = create_privileges(privileges);

    Ok(())
}

fn create_privileges(privileges: Privileges) -> std::io::Result<()> {
    let path = "./src/privileges";
    let filename = "mod.rs";
    let _dir = std::fs::create_dir_all(&path);
    let mut file = std::fs::File::create(format!("{path}/{filename}"))?;

    for (resource, privileges_list) in privileges.resources.iter() {
        create_enum(&mut file, resource, privileges_list);
        create_deserializer(&mut file, resource, privileges_list);
    }

    Ok(())
}

fn capitalize(string: &String) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    
    chars.into_iter().collect()
}

fn write_line(file: &mut std::fs::File, indents: usize, content: String) {
    let indent = "\t";
    let _ = file.write(indent.repeat(indents).as_bytes());
    let _ = file.write(content.as_bytes());
    let _ = file.write(b",\n");
}

fn create_enum(file: &mut std::fs::File, resource: &String, privileges_list: &Vec<String>) {
    let resource_cap = capitalize(resource);
    let _ = file.write(b"#[derive(Debug)]\n");
    let head = format!("pub enum {resource_cap} {{\n");
    let _ = file.write(head.as_bytes());
    
    for privilege in privileges_list {
        let privilege_cap = capitalize(privilege);
        write_line(file, 1, privilege_cap)
    }

    let _ = file.write(b"}\n");
}

fn create_deserializer(file: &mut std::fs::File, resource: &String, privileges_list: &Vec<String>) {
    let resource_cap = capitalize(resource);
    let head = format!("impl std::str::FromStr for {resource_cap} {{\n");
    let _ = file.write(head.as_bytes());
    let _ = file.write(b"\ttype Err = ();\n");
    let _ = file.write(b"\tfn from_str(input: &str) -> Result<Self, Self::Err> {\n");
    let _ = file.write(b"\t\tmatch input {\n");
    for privilege in privileges_list {
        let privilege_cap = capitalize(privilege);
        let _ = file.write(format!("\t\t\t\"{privilege}\" => Ok({resource_cap}::{privilege_cap}),\n").as_bytes());
    }
    let _ = file.write(b"\t\t\t_ => Err(()),\n");
    let _ = file.write(b"\t\t}\n");
    let _ = file.write(b"\t}\n");
    let _ = file.write(b"}\n");
}