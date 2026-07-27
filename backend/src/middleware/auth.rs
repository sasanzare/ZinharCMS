use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::jwt;
use crate::state::AppState;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub ver: i64,
    pub exp: i64,
    pub iat: i64,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::to_owned)
        .ok_or_else(|| AppError::Unauthorized("missing bearer token".to_owned()))?;

    let claims = jwt::verify_access_token(&token, &state.config)
        .map_err(|_| AppError::InvalidAccessToken)?;
    let claims = crate::services::sessions::validate_access_claims(&state.db, claims)
        .await
        .map_err(map_access_claim_validation_error)?;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

pub(crate) fn map_access_claim_validation_error(error: AppError) -> AppError {
    match error {
        AppError::Unauthorized(_) => AppError::InvalidAccessToken,
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::map_access_claim_validation_error;
    use crate::error::AppError;

    #[test]
    fn access_claim_validation_preserves_server_errors() {
        assert!(matches!(
            map_access_claim_validation_error(AppError::Unauthorized("stale token".to_owned())),
            AppError::InvalidAccessToken
        ));
        assert!(matches!(
            map_access_claim_validation_error(AppError::Internal(
                "database unavailable".to_owned()
            )),
            AppError::Internal(_)
        ));
    }
}
