use actix_web::web::Json;
use application::{helpers::string_vec_to_vec_str, use_cases::CreateError};

use super::{
    dtos::{CreateSandwichRequest, SandwichResponse},
    errors::ApiError,
    helpers::respond_json,
    validate,
};

/// create sandwich recipes
pub async fn create_sandwich(
    request: Json<CreateSandwichRequest>,
) -> Result<Json<SandwichResponse>, ApiError> {
    validate(&request)?;

    let result = application::use_cases::create_sandwich(
        &request.name,
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        &request.sandwich_type,
    );

    result
        .map(|v| respond_json(SandwichResponse::from(v)))
        .map_err(|e| match e {
            CreateError::InvalidData(m) => ApiError::InvalidData(m),
            CreateError::Conflict(m) => ApiError::Conflict(m),
            CreateError::Unknown(m) => ApiError::Unknown(m),
        })?
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };

    use domain::entities::Sandwich;

    use crate::{
        driving::rest_handler::dtos::{CreateSandwichRequest, SandwichResponse},
        tests::test_utils::shared::{
            assert_on_ingredients, stub_ingredients, stub_sandwich, SANDWICH_NAME, SANDWICH_TYPE,
        },
    };

    #[actix_web::test]
    async fn should_create_a_sandwich() {
        let create_req = CreateSandwichRequest {
            name: SANDWICH_NAME.to_string(),
            ingredients: stub_ingredients(),
            sandwich_type: SANDWICH_TYPE,
        };

        // init service
        let app = test::init_service(App::new().route("/", web::post().to(create_sandwich))).await;

        // Create request
        let req = TestRequest::post().set_json(create_req);

        // Execute request
        let resp = test::call_and_read_body_json(&app, req.to_request()).await;

        // validate response
        assert_on_sandwich_response(&resp, &stub_sandwich(false));
    }

    fn assert_on_sandwich_response(actual: &SandwichResponse, expected: &Sandwich) {
        assert_eq!(&actual.name, expected.name().value());
        assert_on_ingredients(&actual.ingredients, expected.ingredients().value());
    }
}
