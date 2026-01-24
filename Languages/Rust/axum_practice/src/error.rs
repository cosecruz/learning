use std::error::Error;
use std::fmt;

use axum::http::StatusCode;
use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, CustomErr>;

#[derive(Debug, Clone)]
pub enum CustomErr {
    LoginFail,

    // -- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailUnauthorized,
    AuthFailCtxNotInRequestExt,

    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },
}

impl fmt::Display for CustomErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CustomErr {}

impl CustomErr {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AuthFailNoAuthTokenCookie => StatusCode::UNAUTHORIZED,
            Self::AuthFailTokenWrongFormat => StatusCode::UNAUTHORIZED,
            Self::AuthFailUnauthorized => StatusCode::UNAUTHORIZED,
            Self::AuthFailCtxNotInRequestExt => StatusCode::UNAUTHORIZED,

            Self::TicketDeleteFailIdNotFound { .. } => StatusCode::NOT_FOUND,

            Self::LoginFail => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn client_message(&self) -> &'static str {
        match self {
            Self::AuthFailNoAuthTokenCookie => "missing auth token",
            Self::AuthFailTokenWrongFormat => "invalid auth token",
            Self::AuthFailUnauthorized => "unauthorized",

            _ => "internal server error",
        }
    }
}

impl IntoResponse for CustomErr {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {:?}", "INTO_RES", self);

        let status = self.status_code();
        let msg = self.client_message();

        (status, msg).into_response()
    }
}

//note: never ever pass through your server errors to the client, it is a big secret exposure
