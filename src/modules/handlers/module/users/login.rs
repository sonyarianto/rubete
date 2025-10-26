use crate::modules::database::entity::user_details::{self, Entity as UserDetailsEntity};
use crate::modules::database::entity::users::{self, Entity as UsersEntity};
use crate::modules::utils::json::check_json_payload;
use crate::modules::utils::response::{send_error, send_success};
use crate::modules::utils::security::verify_password;
use jsonwebtoken::{EncodingKey, Header, encode};
use ntex::web;
use ntex::web::error::JsonPayloadError;
use ntex::web::types::{Json, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: i32,
    email: String,
    exp: usize,
}

const JWT_SECRET: &[u8] = b"your_secret_key"; // Replace with a secure key

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

    // Fetch user details
    let details = match UserDetailsEntity::find()
        .filter(user_details::Column::UserId.eq(user.id))
        .one(db.get_ref())
        .await
    {
        Ok(Some(details)) => details,
        _ => {
            return send_error(
                500,
                "db_error",
                "Failed to fetch user details",
                Option::<()>::None,
            );
        }
    };

    // Get expiration minutes from env, default to 15 if not set or invalid
    let expire_minutes = env::var("ACCESS_TOKEN_EXPIRE_MINUTES")
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(15);

    // Generate JWT token, access token is short-lived, only 15 minutes
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(expire_minutes))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp: expiration,
    };

    let access_token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ) {
        Ok(t) => t,
        Err(_) => {
            return send_error(
                500,
                "token_error",
                "Failed to generate token",
                Option::<()>::None,
            );
        }
    };

    send_success(
        "Login successful",
        serde_json::json!({ "id": user.id, "email": user.email, "first_name": details.first_name, "last_name": details.last_name, "access_token": access_token }),
    )
}
