use crate::utils::json::check_json_payload;
use crate::utils::response::send_error;
use ntex::web;
use ntex::web::error::JsonPayloadError;
use ntex::web::types::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 1, message = "first_name is required"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "last_name is required"))]
    pub last_name: String,
}

#[web::post("/users")]
pub async fn create_user(
    payload: Result<Json<CreateUserRequest>, JsonPayloadError>,
) -> impl web::Responder {
    // Handle JSON parsing errors
    let data = match check_json_payload(payload) {
        Ok(v) => v,
        Err(resp) => return resp,
    };

    // Run validation when JSON was parsed successfully
    if let Err(errors) = data.validate() {
        return send_error(422, "validation_error", "Validation failed", Some(errors));
    }

    // Normal success path
    web::HttpResponse::Ok().json(&data)
}
