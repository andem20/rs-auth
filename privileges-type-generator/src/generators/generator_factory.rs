use super::{language::Language, rust_generator::RustGenerator};

pub fn generator_factory(language: Language) -> Result<impl super::Generator, ()> {
    match language {
        Language::Rust => Ok(RustGenerator {}),
        _ => Err(())
    }
}