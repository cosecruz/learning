use std::error::Error;
use std::fmt;

use axum::http::StatusCode;
use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, CustomErr>;

#[derive(Debug)]
pub enum CustomErr {
    LoginFail,

    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },
}

impl fmt::Display for CustomErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CustomErr {}

impl IntoResponse for CustomErr {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        //note: never ever pass through your server errors to the client, it is a big secret exposure
        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled_client_error").into_response()
    }
}
