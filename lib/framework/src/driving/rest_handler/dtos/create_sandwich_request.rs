use domain::entities::value_objects::SandwichType;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateSandwichRequest {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichType,
}
