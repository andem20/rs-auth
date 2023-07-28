use std::str::FromStr;

use permission_type_transpiler::{
    generators::{generator_factory::generator_factory, language::Language, Generator},
    structs::Privileges,
};

fn main() -> std::io::Result<()> {
    let path = "../privileges.yaml";
    let file = std::fs::File::open(&path)?;

    let mut privileges: Privileges = serde_yaml::from_reader(file).expect("Could not deserialize file.");
    privileges.expand_defaults();

    let selected_language = Language::from_str("typescript").expect("Not a valid language.");
    let generator = generator_factory(selected_language).expect("Could not initialize generator.");

    generate_pivileges_file(privileges, generator)?;

    Ok(())
}

fn generate_pivileges_file(privileges: Privileges, generator: Box<dyn Generator>,) -> std::io::Result<()> {
    let path = "../privileges";
    let _dir = std::fs::create_dir_all(&path);
    let mut file = std::fs::File::create(format!("{path}/{}", generator.get_filename()))?;

    for (resource, privileges_list) in privileges.resources.iter() {
        generator.generate_resource_privileges(&mut file, &resource, &privileges_list);
    }

    Ok(())
}