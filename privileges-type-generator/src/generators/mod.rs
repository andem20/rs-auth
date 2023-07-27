pub mod rust_generator;
pub mod language;
pub mod generator_factory;

pub trait Generator {
    fn create_enum(
        &self,
        file: &mut std::fs::File, 
        resource: &String, 
        privileges_list: &Vec<String>
    );

    fn create_deserializer(
        &self,
        file: &mut std::fs::File, 
        resource: &String, 
        privileges_list: &Vec<String>
    );
}