use std::fs::File;

pub mod rust_generator;
pub mod language;
pub mod generator_factory;
pub mod typescript_generator;

pub trait Generator {
    fn generate_resource_privileges(
        &self, 
        file: &mut File, 
        resource: &String, 
        privileges_list: &Vec<String>
    );
    
    fn get_filename(&self) -> &str;
}