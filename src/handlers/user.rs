use crate::utils::response::send_error;
use ntex::web;
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
pub async fn create_user(payload: Json<CreateUserRequest>) -> impl web::Responder {
    let data = payload.into_inner();

    if let Err(errors) = data.validate() {
        return send_error(400, "VALIDATION_ERROR", "Validation failed", Some(errors));
    }

    web::HttpResponse::Ok().json(&data)
}
