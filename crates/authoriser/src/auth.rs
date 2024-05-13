use anyhow::Result;
use aws_lambda_events::apigw::ApiGatewayV2CustomAuthorizerIamPolicyResponse;
use aws_lambda_events::event::apigw::ApiGatewayCustomAuthorizerPolicy;
use aws_lambda_events::event::iam::IamPolicyStatement;
use aws_lambda_events::iam::IamPolicyEffect;
use serde::{Deserialize, Serialize};

use crate::decode::{verify_claims, Claims};

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    username: String,
}

pub async fn authorise(
    token: &str,
) -> Result<ApiGatewayV2CustomAuthorizerIamPolicyResponse<Context>> {
    let claims = verify_claims(token).await?;
    let response = generate_allow(claims);

    Ok(response)
}

fn generate_allow(claims: Claims) -> ApiGatewayV2CustomAuthorizerIamPolicyResponse<Context> {
    let context = Context {
        username: claims.username,
    };

    let statement = IamPolicyStatement {
        action: vec!["execute-api:Invoke".to_string()],
        effect: IamPolicyEffect::Allow,
        resource: vec!["*".to_string()],
        condition: None,
    };
    let policy_document = ApiGatewayCustomAuthorizerPolicy {
        version: Some("2012-10-17".to_owned()),
        statement: vec![statement],
    };

    ApiGatewayV2CustomAuthorizerIamPolicyResponse {
        principal_id: None,
        policy_document,
        context,
    }
}