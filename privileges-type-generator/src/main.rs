use std::str::FromStr;

use permission_type_transpiler::{generators::{Generator, language::Language, generator_factory::generator_factory}, structs::Privileges};

fn main() -> std::io::Result<()> {
    let path = "../privileges.yaml";
    let file = std::fs::File::open(&path)?;

    let mut privileges: Privileges = serde_yaml::from_reader(file).expect("Could not deserialize file");
    privileges.expand_defaults();

    let language = Language::from_str("Rust").expect("Not a valid language");

    let generator = generator_factory(language).expect("Could not initialize generator");

    let _ = create_privileges(privileges, generator);

    Ok(())
}

fn create_privileges(privileges: Privileges, generator: impl Generator) -> std::io::Result<()> {
    let path = "../privileges";
    let filename = "mod.rs";
    let _dir = std::fs::create_dir_all(&path);
    let mut file = std::fs::File::create(format!("{path}/{filename}"))?;

    for (resource, privileges_list) in privileges.resources.iter() {
        generator.create_enum(&mut file, resource, privileges_list);
        generator.create_deserializer(&mut file, resource, privileges_list);
    }

    Ok(())
}