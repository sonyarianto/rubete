use crate::modules::entity::users::{self, ActiveModel};
use crate::utils::json::check_json_payload;
use crate::utils::response::{send_error, send_success};
use crate::utils::security::hash_password;
use ntex::web;
use ntex::web::error::JsonPayloadError;
use ntex::web::types::{Json, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
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
    db: State<DatabaseConnection>,
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

    // Check if user already exists
    let existing = match users::Entity::find()
        .filter(users::Column::Email.eq(data.email.clone()))
        .one(db.get_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return send_error(500, "db_error", "Database error", Option::<()>::None);
        }
    };

    if existing.is_some() {
        return send_error(
            400,
            "user_exists",
            "User already exists",
            Option::<()>::None,
        );
    }

    // Insert new user
    let new_user = ActiveModel {
        email: Set(data.email.clone()),
        password: Set(hash_password(&data.password)),
        ..Default::default() // fill other fields (like created_at) automatically
    };

    let inserted_user = match new_user.insert(db.get_ref()).await {
        Ok(user) => user,
        Err(_) => {
            return send_error(
                500,
                "insert_failed",
                "Failed to create user",
                Option::<()>::None,
            );
        }
    };

    send_success(
        "User created successfully",
        serde_json::json!({ "id": inserted_user.id }),
    )
}
