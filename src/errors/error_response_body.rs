use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponseBody<'a> {
    timestamp: String,
    status: u16,
    error: &'a str,
    message: &'a str,
    path: &'a str,
}

impl<'a> ErrorResponseBody<'a> {
    pub fn new(status: u16, error: &'a str, message: &'a str, path: &'a str) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string();

        ErrorResponseBody {
            timestamp,
            status,
            error,
            message,
            path,
        }
    }

    pub fn forbidden(path: &'a str) -> Self {
        ErrorResponseBody::new(403, "Forbidden", "Access Denied", path)
    }

    pub fn unauthorized(path: &'a str) -> Self {
        ErrorResponseBody::new(401, "Unauthorized", "Access Denied", path)
    }

    pub fn internal_server_error(path: &'a str, msg: &'a str) -> Self {
        ErrorResponseBody::new(500, "Internal Server Error", msg, path)
    }

    pub fn not_found(path: &'a str, msg: &'a str) -> Self {
        ErrorResponseBody::new(404, "Not Found", msg, path)
    }

    pub fn bad_request(path: &'a str, msg: &'a str) -> Self {
        ErrorResponseBody::new(400, "Bad Request", msg, path)
    }
}