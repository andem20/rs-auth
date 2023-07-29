use super::{
    language::Language, rust_generator::RustGenerator, typescript_generator::TypeScriptGenerator, java_generator::JavaGenerator,
};

pub fn generator_factory(language: Language) -> Result<Box<dyn super::Generator>, ()> {
    match language {
        Language::Rust => Ok(Box::new(RustGenerator {})),
        Language::TypeScript => Ok(Box::new(TypeScriptGenerator {})),
        Language::Java => Ok(Box::new(JavaGenerator {})),
    }
}
