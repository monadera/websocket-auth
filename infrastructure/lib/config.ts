import * as dotenv from "dotenv";

dotenv.config();

export function getConfig() {
  const userPoolId = process.env.USER_POOL_ID ?? "";
  const userPoolClientId = process.env.USER_POOL_CLIENT_ID ?? "";

  return {
    userPoolId,
    userPoolClientId,
  };
}
