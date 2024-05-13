mod auth;
mod decode;

use anyhow::anyhow;
use aws_lambda_events::apigw::{
    ApiGatewayV2CustomAuthorizerIamPolicyResponse, ApiGatewayWebsocketProxyRequest,
};
use lambda_runtime::tracing::Level;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use tracing::error;

use crate::auth::{authorise, Context};

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

async fn func(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayV2CustomAuthorizerIamPolicyResponse<Context>, Error> {
    let (event, _context) = event.into_parts();
    match event.query_string_parameters.first("auth") {
        None => {
            error!("missing auth token in connection request");
            Err(anyhow!("missing auth token").into())
        }
        Some(token) => {
            let response = authorise(token).await.map_err(|err| {
                // TODO: expired tokens should return 401 rather than 500
                let error_message = err.to_string();
                error!(error_message, "failed to authenticate connection");
                err
            })?;
            Ok(response)
        }
    }
}