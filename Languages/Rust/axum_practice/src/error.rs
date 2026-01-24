use std::error::Error;
use std::fmt;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_with::SerializeDisplay;
use strum::AsRefStr;

pub type Result<T> = core::result::Result<T, CustomErr>;

#[derive(Debug, Clone, Serialize, AsRefStr)]
#[serde(tag = "type", content = "data")]
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

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
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

    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // --Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // --Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // --fallback_service
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for CustomErr {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {:?}", "INTO_RES", self);

        // let status = self.status_code();
        // let msg = self.client_message();

        // (status, msg).into_response()
        // create a placeholder for axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);
        response
    }
}

//note: never ever pass through your server errors to the client, it is a big secret exposure
