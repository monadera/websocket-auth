use aws_lambda_events::apigw::ApiGatewayV2CustomAuthorizerIamPolicyResponse;
use aws_lambda_events::event::apigw::ApiGatewayCustomAuthorizerPolicy;
use aws_lambda_events::event::iam::IamPolicyStatement;
use aws_lambda_events::iam::IamPolicyEffect;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::decode::verify_claims;

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    username: String,
}

pub async fn authorise(token: &str) -> ApiGatewayV2CustomAuthorizerIamPolicyResponse<Context> {
    match verify_claims(token).await {
        Ok(claims) => {
            let context = Context {
                username: claims.username,
            };

            ApiGatewayV2CustomAuthorizerIamPolicyResponse {
                principal_id: None,
                policy_document: generate_policy(IamPolicyEffect::Allow),
                context,
            }
        }
        Err(err) => {
            error!(error = debug(err), "failed to authenticate connection");
            generate_deny_response()
        }
    }
}

pub fn generate_deny_response() -> ApiGatewayV2CustomAuthorizerIamPolicyResponse<Context> {
    let context = Context {
        username: "".to_string(),
    };

    ApiGatewayV2CustomAuthorizerIamPolicyResponse {
        principal_id: None,
        policy_document: generate_policy(IamPolicyEffect::Deny),
        context,
    }
}

fn generate_policy(effect: IamPolicyEffect) -> ApiGatewayCustomAuthorizerPolicy {
    let statement = IamPolicyStatement {
        action: vec!["execute-api:Invoke".to_string()],
        effect,
        resource: vec!["*".to_string()],
        condition: None,
    };
    ApiGatewayCustomAuthorizerPolicy {
        version: Some("2012-10-17".to_owned()),
        statement: vec![statement],
    }
}
