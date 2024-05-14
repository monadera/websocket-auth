import * as cdk from "aws-cdk-lib";
import { CfnOutput, Stack } from "aws-cdk-lib";
import { WebSocketApi, WebSocketStage } from "aws-cdk-lib/aws-apigatewayv2";
import { WebSocketLambdaAuthorizer } from "aws-cdk-lib/aws-apigatewayv2-authorizers";
import { WebSocketLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import {
  Architecture,
  Code,
  Function,
  IFunction,
  Runtime,
} from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import * as child from "node:child_process";
import * as path from "node:path";

import { getConfig } from "./config";

interface RustFunctionProps {
  binName: string;
  environmentVariables?: { [key: string]: string };
}

export class WebSocketAPIStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const { userPoolId, userPoolClientId } = getConfig();

    const messageHandler = this.createRustFunction({ binName: "handler" });
    const authorizer = this.buildAuthorizer(userPoolId, userPoolClientId);
    const { url } = this.buildApi(authorizer, messageHandler);

    new CfnOutput(this, "ApiUrl", {
      value: url,
      description: "The URL of the WebSocket API.",
    });
  }

  private buildApi(authorizer: WebSocketLambdaAuthorizer, handler: IFunction) {
    const api = new WebSocketApi(this, "Api", {
      apiName: "SubscriptionsApi",
      connectRouteOptions: {
        authorizer,
        integration: new WebSocketLambdaIntegration(
          "ConnectIntegration",
          handler
        ),
      },
      disconnectRouteOptions: {
        integration: new WebSocketLambdaIntegration(
          "DisconnectIntegration",
          handler
        ),
      },
      defaultRouteOptions: {
        integration: new WebSocketLambdaIntegration(
          "DefaultIntegration",
          handler
        ),
      },
    });
    const stage = new WebSocketStage(this, "Stage", {
      webSocketApi: api,
      stageName: "default",
      autoDeploy: true,
    });

    return { api, callbackUrl: stage.callbackUrl, url: stage.url };
  }

  private buildAuthorizer(userPoolId: string, userPoolClientId: string) {
    const region = Stack.of(this).region;
    const jwksUrl = `https://cognito-idp.${region}.amazonaws.com/${userPoolId}/.well-known/jwks.json`;
    const authHandler = this.createRustFunction({
      binName: "authoriser",
      environmentVariables: {
        AUDIENCE: userPoolClientId,
        JWKS_URL: jwksUrl,
      },
    });

    return new WebSocketLambdaAuthorizer("Authorizer", authHandler, {
      identitySource: ["route.request.querystring.auth"],
    });
  }

  private createRustFunction({
    binName,
    environmentVariables,
  }: RustFunctionProps) {
    child.execSync("cargo lambda build -p authoriser --release --arm64", {
      cwd: "../",
    });
    const code = Code.fromAsset(
      path.join(__dirname, "..", "..", "target/lambda/", binName)
    );

    return new Function(this, binName, {
      code,
      architecture: Architecture.ARM_64,
      runtime: Runtime.PROVIDED_AL2023,
      handler: "does-not-matter",
      environment: environmentVariables,
    });
  }
}
