use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ingredient {
    name: String,
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}
