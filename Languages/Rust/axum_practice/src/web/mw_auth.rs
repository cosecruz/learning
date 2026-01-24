use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::{extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::CustomErr;
use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;

// FromRequestParts extracts Ctx from extensions (populated by mw_ctx_resolver)
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = CustomErr;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        println!("->> {:<12} - Ctx::from_request_parts", "EXTRACTOR");

        // Extract the Result<Ctx> that was stored by mw_ctx_resolver
        parts
            .extensions
            .get::<Result<Ctx, CustomErr>>()
            .ok_or(CustomErr::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

// region: ---Middleware ctx resolver
pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    let result_ctx = auth_token
        .ok_or(CustomErr::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
        .map(|(user_id, _exp, _sign)| Ctx::new(user_id));

    // Remove the cookie if something went wrong other than no cookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(CustomErr::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx_result in the request extension
    // This allows downstream handlers/middleware to extract it
    req.extensions_mut().insert(result_ctx);

    next.run(req).await
}
// endregion: ---Middleware ctx resolver

// region: ---Middleware require auth
// Simplified middleware - just logs that auth happened
pub async fn mw_require_auth(
    ctx: Ctx, // Extraction happens automatically!
    req: Request,
    next: Next,
) -> Response {
    println!(
        "->> {:<12} - mw_require_auth - user_id: {}",
        "MIDDLEWARE", ctx.user_id
    );
    next.run(req).await
}

// Alternative: Handle the error in middleware
pub async fn mw_require_auth_alt(
    ctx: Result<Ctx, CustomErr>,
    mut req: Request,
    next: Next,
) -> Response {
    println!("->> {:<12} - mw_require_auth_alt", "MIDDLEWARE");

    match ctx {
        Ok(ctx) => {
            // Insert into extensions so handlers can extract it too
            req.extensions_mut().insert(ctx);
            next.run(req).await
        }
        Err(e) => e.into_response(),
    }
}
// endregion: ---Middleware require auth

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> crate::Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(CustomErr::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| CustomErr::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
