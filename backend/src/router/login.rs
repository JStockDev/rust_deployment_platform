use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use openid::Options;
use rand::{
    RngExt, SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};
use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{RouterState, error::CodeError};

pub async fn route(
    State(state): State<Arc<RouterState>>,
    jar: CookieJar,
) -> Result<Response, CodeError> {
    // Do checks later to see if JWT already exists
    let user_id = Uuid::new_v4().to_string();

    let mut rng = StdRng::try_from_rng(&mut rand::rng())?;
    let random_state: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();

    let user_session = Cookie::new("session_id", user_id.clone());
    state.redis.clone().set_ex(user_id, &random_state, 15).await?;

    let auth_url = state.openid_client.auth_url(&Options {
        scope: Some("openid profile email".to_string()),
        state: Some(random_state),
        ..Default::default()
    });

    Ok((
        StatusCode::FOUND,
        jar.add(user_session),
        Redirect::to(auth_url.as_str()),
    )
        .into_response())
}
