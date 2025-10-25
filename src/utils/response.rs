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

#[derive(Serialize)]
pub struct ErrorResponse<T>
where
    T: Serialize,
{
    pub code: String,
    pub success: bool,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<T>,
}

impl<T> ErrorResponse<T>
where
    T: Serialize,
{
    pub fn new(code: impl Into<String>, message: impl Into<String>, details: Option<T>) -> Self {
        Self {
            code: code.into(),
            success: false,
            message: message.into(),
            details,
        }
    }
}

pub fn send_success<T: serde::Serialize>(message: impl Into<String>, data: T) -> HttpResponse {
    HttpResponse::Ok().json(&SuccessResponse::new(message, data))
}

pub fn send_error<T: Serialize>(
    status: u16,
    code: impl Into<String>,
    message: impl Into<String>,
    details: Option<T>,
) -> HttpResponse {
    HttpResponse::build(ntex::http::StatusCode::from_u16(status).unwrap())
        .json(&ErrorResponse::new(code, message, details))
}
