use crate::{entities::Sandwich, value_objects::SandwichType};

#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    NotFound,
    Conflict(String),
}

// this is my port / use case
pub fn update_sandwich<'a>(
    id: &'a str,
    name: &'a str,
    ingredients: &'a Vec<&str>,
    sandwich_type: &SandwichType,
) -> Result<Sandwich, UpdateError> {
    if id.is_empty() {
        return Err(UpdateError::InvalidData(String::from(
            "Cannot update without a target id",
        )));
    }

    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let sandwich = Sandwich::new(
        String::from(id),
        name.to_string(),
        ingredients,
        sandwich_type.clone(),
    )
    .map_err(|e| UpdateError::InvalidData(e))?;

    Ok(sandwich)
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::{str_vec_to_vec_string, string_vec_to_vec_str},
        tests::test_utils::shared::{
            assert_on_ingredients, stub_ingredients, CHEESEBURGER_NAME, SANDWICH_ID, SANDWICH_TYPE,
        },
    };

    use super::*;

    #[test]
    fn should_update_an_existing_sandwich() {
        let ingredients = stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);

        let cheeseburger =
            update_sandwich(SANDWICH_ID, CHEESEBURGER_NAME, &ingredients, &SANDWICH_TYPE).unwrap();

        assert_eq!(cheeseburger.name().value(), CHEESEBURGER_NAME);

        let expected_ingr = str_vec_to_vec_string(ingredients);
        assert_on_ingredients(&expected_ingr, cheeseburger.ingredients().value());
    }
}
