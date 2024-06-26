use anyhow::{bail, Result};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::OnceCell;
use tracing::info;

use crate::config::get_config;

static CACHED_KEYS: OnceCell<Vec<Jwk>> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub e: String,
    pub n: String,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    #[serde(rename = "cognito:username")]
    pub username: String,
}

pub async fn verify_claims(token: &str) -> Result<Claims> {
    let keys = keys().await?;

    let header = decode_header(token)?;
    let kid = match header.kid {
        Some(k) => k,
        None => bail!("token header has no kid"),
    };
    let key = match keys.iter().find(|&k| k.kid == kid) {
        Some(key) => key,
        None => bail!("none of the keys match token kid"),
    };

    info!(key = debug(key), "found appropriate key");

    let mut validation = Validation::new(Algorithm::RS256);
    let audience = &get_config().audience;
    validation.set_audience(&[audience]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_components(&key.n, &key.e)?,
        &validation,
    )?;

    Ok(token_data.claims)
}

async fn keys() -> Result<&'static Vec<Jwk>> {
    CACHED_KEYS.get_or_try_init(fetch_keys).await
}

async fn fetch_keys() -> Result<Vec<Jwk>> {
    let url = &get_config().jwks_url;
    info!(url, "fetching jwks");
    let client = reqwest::Client::builder().use_rustls_tls().build()?;
    let res = client.get(url).send().await?;

    let jwk_text = res.text().await?;

    let keys_value = match serde_json::from_str::<Value>(&jwk_text)? {
        Value::Object(mut obj) => match obj.get_mut("keys") {
            Some(val) => val.take(),
            None => bail!("no keys found in JWK JSON"),
        },
        _ => bail!("JWK is not a mapping for keys"),
    };

    let keys: Vec<Jwk> = serde_json::from_value(keys_value)?;
    Ok(keys)
}
