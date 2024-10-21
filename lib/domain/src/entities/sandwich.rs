use core::fmt;

use serde::{Deserialize, Serialize};

use super::value_objects::{SandwichId, SandwichIngredients, SandwichName, SandwichType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sandwich {
    id: SandwichId,
    name: SandwichName,
    ingredients: SandwichIngredients,
    sandwich_type: SandwichType,
}

impl Sandwich {
    pub fn new(
        id: String,
        name: String,
        ingredients: Vec<String>,
        sandwich_type: SandwichType,
    ) -> Result<Self, String> {
        let sandwich_id = SandwichId::try_from(id)?;
        let sandwich_name = SandwichName::try_from(name)?;
        let sandwich_ingredients = SandwichIngredients::try_from(ingredients)?;

        Ok(Self {
            id: sandwich_id,
            name: sandwich_name,
            ingredients: sandwich_ingredients,
            sandwich_type,
        })
    }

    pub fn id(&self) -> &SandwichId {
        &self.id
    }

    pub fn name(&self) -> &SandwichName {
        &self.name
    }

    pub fn ingredients(&self) -> &SandwichIngredients {
        &self.ingredients
    }

    pub fn sandwich_type(&self) -> &SandwichType {
        &self.sandwich_type
    }
}

impl fmt::Display for Sandwich {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            match &self.id.value() {
                Some(s) => s,
                None => "",
            },
            &self.name.value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SANDWICH_ID: &str = "sand-id";
    const SANDWICH_NAME: &str = "Hot dog";

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = vec!["Wurst".to_string(), "Ketchup".to_string()];

        let hot_dog = Sandwich::new(
            SANDWICH_ID.to_string(),
            SANDWICH_NAME.to_string(),
            ingredients.clone(),
            SandwichType::Meat,
        )
        .unwrap();

        assert_eq!(hot_dog.id().value().as_ref().unwrap(), SANDWICH_ID);
        assert_eq!(hot_dog.name.value(), SANDWICH_NAME);

        assert_eq!(ingredients.len(), hot_dog.ingredients.value().len());

        for (i, exp_ingr) in ingredients.iter().enumerate() {
            assert_eq!(exp_ingr, &hot_dog.ingredients.value()[i]);
        }
    }

    #[test]
    fn should_fail_without_a_name_or_ingredients() {
        // without name
        let err_sandwich = Sandwich::new(
            "".to_string(),
            "".to_string(),
            vec!["Wurst".to_string(), "Ketchup".to_string()],
            SandwichType::Meat,
        );

        assert_eq!(err_sandwich.is_err(), true);
        assert_eq!(err_sandwich.unwrap_err(), "Any sandwich must have a name");

        // without at least 1 ingredient
        let err_sandwich = Sandwich::new(
            SANDWICH_ID.to_string(),
            SANDWICH_NAME.to_string(),
            vec![],
            SandwichType::Meat,
        );

        assert_eq!(err_sandwich.is_err(), true);
        assert_eq!(
            err_sandwich.unwrap_err(),
            "Any sandwich must have at least one ingredient"
        );
    }
}
