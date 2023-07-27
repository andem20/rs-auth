use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Privileges {
    pub default: Vec<String>,
    pub resources: HashMap<String, Vec<String>>
}

impl Privileges {
    pub fn expand_defaults(&mut self) {
        self.resources.values_mut().for_each(|values| {
            if let Some(index) = values.iter().position(|v| v == "default") {
                values.remove(index);
                values.append(&mut self.default.clone());
            }
        });
    }
}