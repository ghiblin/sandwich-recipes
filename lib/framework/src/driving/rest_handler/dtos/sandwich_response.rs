use domain::entities::{value_objects::SandwichType, Sandwich};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SandwichResponse {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichType,
}

impl From<Sandwich> for SandwichResponse {
    fn from(s: Sandwich) -> Self {
        Self {
            id: s
                .id()
                .value()
                .clone()
                .unwrap_or(String::from(""))
                .to_string(),
            name: s.name().value().to_string(),
            ingredients: s.ingredients().value().clone(),
            sandwich_type: s.sandwich_type().clone(),
        }
    }
}
