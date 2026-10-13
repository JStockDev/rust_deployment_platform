use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use openid::Token;
use redis::AsyncTypedCommands;

use crate::{RouterState, error::CodeError};

pub async fn route(
    State(state): State<Arc<RouterState>>,
    Query(params): Query<HashMap<String, String>>,
    jar: CookieJar,
) -> Result<Response, CodeError> {
    let mut redis = state.redis.clone();

    let returned_state = params.get("state").ok_or(CodeError::DecodeError)?;
    let code = params.get("code").ok_or(CodeError::DecodeError)?;

    let user_session = jar.get("session_id").ok_or(CodeError::DecodeError)?.clone();

    let stored_state = redis
        .get(user_session.value())
        .await?
        .ok_or(CodeError::Unauthorised)?;

    if &stored_state != returned_state {
        return Err(CodeError::Unauthorised);
    }
    
    redis.del(user_session.value()).await?;
    let jar = jar.remove(user_session);

    let token: Token = state
        .openid_client
        .request_token(code)
        .await
        .map_err(|_| CodeError::DecodeError)?
        .into();

    if let Some(mut token) = token.id_token {
        state
            .openid_client
            .decode_token(&mut token)
            .map_err(|_| CodeError::DecodeError)?;

        state
            .openid_client
            .validate_token(&token, None, None)
            .map_err(|_| CodeError::Unauthorised)?;
    } else {
        return Err(CodeError::Unauthorised);
    }

    Ok((StatusCode::OK, jar).into_response())
}
