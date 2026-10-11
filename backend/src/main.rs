use tokio::net::TcpListener;

use crate::error::CodeError;

use std::env;

mod error;
mod auth;
mod router;

#[tokio::main()]
async fn main() -> Result<(), CodeError> {

    if cfg!(debug_assertions) {
        dotenvy::dotenv()?;
    }

    let addr = env::var("ADDRESS")?;
    let listener = TcpListener::bind(addr).await?;

    let state = RouterState {  };

    let router = router::build_router(state);
    axum::serve(listener, router).await?;

    Ok(())
}


pub struct RouterState {

}