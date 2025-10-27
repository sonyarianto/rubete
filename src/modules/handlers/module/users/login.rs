use crate::modules::database::entity::user_details::{self, Entity as UserDetailsEntity};
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

#[derive(Serialize)]
struct Claims {
    sub: i32,
    user_id: i32,
    email: String,
    exp: usize,
    iat: usize,
    jti: String,
}

fn generate_access_token(user_id: i32, email: &str) -> Result<String, String> {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{EncodingKey, Header, encode};
    use std::env;

    // Get JWT secret from environment
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;

    // Get expiration minutes from env, default to 15 if not set or invalid
    let expire_minutes = env::var("ACCESS_TOKEN_EXPIRE_MINUTES")
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(15);

    // Generate expiration timestamp
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expire_minutes))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        user_id,
        email: email.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
        jti: uuid::Uuid::new_v4().to_string(),
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err("Failed to generate token".to_string()),
    }
}

fn generate_refresh_token(user_id: i32, email: &str) -> Result<String, String> {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{EncodingKey, Header, encode};
    use std::env;

    // Get JWT secret from environment
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;

    // Generate expiration timestamp (7 days)
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
        jti: uuid::Uuid::new_v4().to_string(),
        user_id,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err("Failed to generate token".to_string()),
    }
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

    let access_token = match generate_access_token(user.id, &user.email) {
        Ok(token) => token,
        Err(msg) => {
            return send_error(500, "token_error", &msg, Option::<()>::None);
        }
    };

    // Generate refresh token (long-lived, 7 days)
    let _refresh_token = match generate_refresh_token(user.id, &user.email) {
        Ok(token) => token,
        Err(msg) => {
            return send_error(500, "token_error", &msg, Option::<()>::None);
        }
    };

    let session_mode =
        std::env::var("SESSION_MODE").unwrap_or_else(|_| "jwt_stateless".to_string());

    // If session mode is "jwt_server_stateful", store JTI in UserSession table
    if session_mode == "jwt_server_stateful" {
        // Here you would typically store the JTI in the database associated with the user
        // For brevity, this part is omitted
        // TODO: Implement storing JTI in UserSession table
    }

    // Create cookie called refresh_token with HttpOnly and Secure flags
    // Note: In a real application, you would set this cookie in the HTTP response headers
    // For brevity, this part is omitted
    // TODO: Set refresh_token cookie in response, maybe using ntex-session?

    send_success(
        "Login successful",
        serde_json::json!({ "id": user.id, "email": user.email, "first_name": details.first_name, "last_name": details.last_name, "access_token": access_token }),
    )
}
