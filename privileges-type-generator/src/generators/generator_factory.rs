use super::{language::Language, rust_generator::RustGenerator, typescript_generator::TypeScriptGenerator};

pub fn generator_factory(language: Language) -> Result<Box<dyn super::Generator>, ()> {
    match language {
        Language::Rust => Ok(Box::new(RustGenerator {})),
        Language::TypeScript => Ok(Box::new(TypeScriptGenerator {})),
        _ => Err(())
    }
}