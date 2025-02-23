use axum::{body::Bytes, http::StatusCode};

// Handler that immediately returns a `200 OK` response with a plain text
// body.
pub async fn string_handler() -> String {
    "Hello, World!".to_string()
}

// Handler that buffers the request body and returns it.
//
// This works because `Bytes` implements `FromRequest`
// and therefore can be used as an extractor.
//
// `String` and `StatusCode` both implement `IntoResponse` and
// therefore `Result<String, StatusCode>` also implements `IntoResponse`
pub async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
