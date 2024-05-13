#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import "source-map-support/register";

import { WebSocketAPIStack } from "../lib/stack";

const app = new cdk.App();

new WebSocketAPIStack(app, "ApiStack", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION,
  },
});
