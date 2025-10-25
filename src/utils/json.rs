use crate::utils::response::send_error;
use ntex::web::{HttpResponse, error::JsonPayloadError, types::Json};

/// Generic helper for JSON payload extraction.
/// Returns parsed data or early HttpResponse with formatted error.
pub fn check_json_payload<T>(payload: Result<Json<T>, JsonPayloadError>) -> Result<T, HttpResponse>
where
    T: serde::de::DeserializeOwned,
{
    match payload {
        Ok(json) => Ok(json.into_inner()),
        Err(e) => {
            let message = format!("{}", e);
            Err(send_error::<()>(400, "invalid_payload", &message, None))
        }
    }
}
