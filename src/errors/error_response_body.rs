use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponseBody {
    timestamp: String,
    status: u16,
    error: String,
    message: String,
    path: String,
}

// TODO: timestamp format isn't exactly like the target-application -> fix this at some point
impl ErrorResponseBody {
    pub fn new(status: u16, error: String, message: String, path: String) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let timestamp = now.to_rfc3339();

        ErrorResponseBody {
            timestamp,
            status,
            error,
            message,
            path,
        }
    }

    pub fn forbidden(path: String) -> Self {
        ErrorResponseBody::new(403, "Forbidden".to_string(), "Access Denied".to_string(), path)
    }

    pub fn internal_server_error(path: String) -> Self {
        ErrorResponseBody::new(500, "Internal Server Error".to_string(), "Something went wrong".to_string(), path)
    }

    pub fn not_found(path: String, msg: String) -> Self {
        ErrorResponseBody::new(404, "Not Found".to_string(), msg, path)
    }
}