use crate::modules::database::entity::user_details::ActiveModel as UserDetailsActiveModel;
use crate::modules::database::entity::users::{self, ActiveModel as UserActiveModel};
use crate::modules::utils::json::check_json_payload;
use crate::modules::utils::response::{send_error, send_success};
use crate::modules::utils::security::hash_password;
use ntex::web;
use ntex::web::error::JsonPayloadError;
use ntex::web::types::{Json, State};
use sea_orm::TransactionTrait;
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

    // Start transaction
    let txn = match db.get_ref().begin().await {
        Ok(txn) => txn,
        Err(_) => {
            return send_error(
                500,
                "db_error",
                "Failed to start transaction",
                Option::<()>::None,
            );
        }
    };

    // Insert new user
    let new_user = UserActiveModel {
        email: Set(data.email.clone()),
        password: Set(hash_password(&data.password)),
        ..Default::default()
    };

    let inserted_user = match new_user.insert(&txn).await {
        Ok(user) => user,
        Err(_) => {
            let _ = txn.rollback().await;
            return send_error(
                500,
                "insert_failed",
                "Failed to create user",
                Option::<()>::None,
            );
        }
    };

    // Insert user details
    let new_details = UserDetailsActiveModel {
        user_id: Set(inserted_user.id),
        first_name: Set(data.first_name.clone()),
        last_name: Set(data.last_name.clone()),
        ..Default::default()
    };

    if (new_details.insert(&txn).await).is_err() {
        let _ = txn.rollback().await;
        return send_error(
            500,
            "insert_failed",
            "Failed to create user details",
            Option::<()>::None,
        );
    }

    let _ = txn.commit().await;

    send_success(
        "User created successfully",
        serde_json::json!({ "id": inserted_user.id }),
    )
}
