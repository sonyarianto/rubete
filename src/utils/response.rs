use ntex::web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl<T> SuccessResponse<T>
where
    T: Serialize,
{
    pub fn new(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data,
        }
    }
}

pub fn send_success<T: serde::Serialize>(message: impl Into<String>, data: T) -> HttpResponse {
    HttpResponse::Ok().json(&SuccessResponse::new(message, data))
}
