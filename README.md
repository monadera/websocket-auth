# Amazon WebSocket API Gateway authoriser in Rust

This is the accompanying code repository for the blog post
[Amazon WebSocket API Gateway authoriser in Rust](https://monadera.com/blog/ws-gateway-authoriser/).

It contains the Rust code for a Lambda authoriser compatible
with AWS WebSocket API in API Gateway. The `infrastructure`
folder contains the example CDK code to deploy an API Gateway
along with two Lambda functions - the authoriser and a dummy handler.
