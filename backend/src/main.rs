use openid::Client as OpenIDClient;
use rand::{SeedableRng, rngs::{StdRng, SysRng}};
use reqwest::Client as HttpClient;
use std::env;
use tokio::net::TcpListener;
use redis::{Client as RedisClient, aio::MultiplexedConnection};

use crate::{auth::oauth, error::CodeError};

mod auth;
mod error;
mod router;

#[tokio::main()]
async fn main() -> Result<(), CodeError> {
    if cfg!(debug_assertions) {
        dotenvy::dotenv()?;
    }

    let addr = env::var("ADDRESS")?;
    let listener = TcpListener::bind(addr).await?;

    
    let redis = RedisClient::open(env::var("REDIS_ADDRESS")?)?;
    let redis_con = redis.get_multiplexed_async_connection().await?;

    let http_client = HttpClient::new();
    let openid_client = oauth::build_client(http_client.clone()).await?;

    let base_address = env::var("BASE_ADDRESS")?;
    let state = RouterState {
        redis: redis_con,
        base_address,
        http_client,
        openid_client,
    };

    let router = router::build_router(state);
    axum::serve(listener, router).await?;

    Ok(())
}

pub struct RouterState {
    redis: MultiplexedConnection,
    base_address: String,
    http_client: HttpClient,
    openid_client: OpenIDClient,
}
