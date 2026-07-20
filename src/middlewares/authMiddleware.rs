use actix_web::{
    Error, HttpMessage,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    http::header,
    middleware::Next,
    web,
};

use crate::{
    auth::jwt::verify_token, repositories::userRepository::fetch_user_by_id, state::AppState,
};

pub async fn auth_middleware(
    mut req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let Some(auth_header) = req.headers().get(header::AUTHORIZATION) else {
        return Err(ErrorUnauthorized("Unauthorized"));
    };

    let Ok(auth_header) = auth_header.to_str() else {
        return Err(ErrorUnauthorized("Unauthorized"));
    };

    let Some(token) = auth_header.strip_prefix("Bearer ") else {
        return Err(ErrorUnauthorized("Unauthorized"));
    };

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorInternalServerError("AppState not configured"))?;

    let claims = verify_token(token, &state.config.jwt_secret)
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

    let user = fetch_user_by_id(&state.pool, claims.sub)
        .await
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

    req.extensions_mut().insert(user);

    next.call(req).await
}
