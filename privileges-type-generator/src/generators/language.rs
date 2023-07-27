use std::str::FromStr;

pub enum Language {
    Rust,
    TypeScript,
    Java,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::TypeScript),
            "java" => Ok(Language::Java),
            _ => Err(())
        }
    }
}