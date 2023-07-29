use std::str::FromStr;

use permission_type_transpiler::{
    generators::{generator_factory::generator_factory, language::Language},
    structs::Privileges,
};

fn main() -> std::io::Result<()> {
    let path = "../privileges.yaml";
    let file = std::fs::File::open(&path)?;

    let mut privileges: Privileges = serde_yaml::from_reader(file).expect("Could not deserialize file.");
    privileges.expand_defaults();

    let selected_language = Language::from_str("rust").expect("Not a valid language.");
    let generator = generator_factory(selected_language).expect("Could not initialize generator.");

    let path = "../privileges";

    generator.generate_pivileges_file(privileges, path)?;

    Ok(())
}