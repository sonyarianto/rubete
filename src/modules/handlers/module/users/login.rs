use crate::modules::database::entity::users::{self, Entity as UsersEntity};
use crate::modules::utils::json::check_json_payload;
use crate::modules::utils::response::{send_error, send_success};
use crate::modules::utils::security::verify_password;
use ntex::web;
use ntex::web::error::JsonPayloadError;
use ntex::web::types::{Json, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

#[web::post("/login")]
pub async fn login_user(
    payload: Result<Json<LoginUserRequest>, JsonPayloadError>,
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

    // Find user by email
    let user = match UsersEntity::find()
        .filter(users::Column::Email.eq(data.email.clone()))
        .one(db.get_ref())
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return send_error(
                401,
                "invalid_credentials",
                "Invalid email or password",
                Option::<()>::None,
            );
        }
        Err(_) => {
            return send_error(500, "db_error", "Database error", Option::<()>::None);
        }
    };

    // Verify password
    if !verify_password(&data.password, &user.password) {
        return send_error(
            401,
            "invalid_credentials",
            "Invalid email or password",
            Option::<()>::None,
        );
    }

    // You may want to generate a token here (e.g., JWT), but for now just return success
    send_success(
        "Login successful",
        serde_json::json!({ "id": user.id, "email": user.email }),
    )
}
