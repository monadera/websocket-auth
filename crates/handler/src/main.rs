use anyhow::{Result};
use aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::tracing::Level;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Serialize;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .json()
        .init();
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: i32,
}

async fn func(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> std::result::Result<Response, Error> {
    let (event, _context) = event.into_parts();
    info!(event = debug(event), "handling event");

    Ok(Response { status_code: 200 })
}