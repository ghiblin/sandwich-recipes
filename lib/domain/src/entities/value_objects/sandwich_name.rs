use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandwichName(String);

impl SandwichName {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for SandwichName {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            Err("Any sandwich must have a name")
        } else {
            Ok(Self(name))
        }
    }
}
