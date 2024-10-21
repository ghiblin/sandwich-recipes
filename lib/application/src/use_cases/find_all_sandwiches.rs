use domain::entities::Sandwich;

#[derive(Debug)]
pub enum FindAllError {
    Unknown(String),
}

pub fn find_all_sandwiches<'a>(
    _name: &'a str,
    _ingredients: &'a Vec<&str>,
) -> Result<Vec<Sandwich>, FindAllError> {
    Ok(vec![])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_find_all_sandwiches() {
        let sand_list = find_all_sandwiches("", &vec![]).unwrap();

        assert_eq!(sand_list.len(), 0);
    }
}
