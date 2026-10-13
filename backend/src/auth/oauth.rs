use std::env;

use openid::{Client, Discovered, StandardClaims};
use reqwest::{Client as HttpClient, Url};

use crate::error::CodeError;

pub async fn build_client(
    http_client: HttpClient,
) -> Result<Client<Discovered, StandardClaims>, CodeError> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let issuer_url = env::var("ISSUER_URL")?;
    let redirect_url = env::var("REDIRECT_URL")?;

    let oauth_client: Client<Discovered, StandardClaims> = Client::discover_with_client(
        http_client,
        client_id,
        client_secret,
        redirect_url,
        Url::parse(&issuer_url)?,
    )
    .await?;

    Ok(oauth_client)
}
