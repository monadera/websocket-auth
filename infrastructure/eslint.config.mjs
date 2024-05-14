import pluginJs from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

export default [
  {
    languageOptions: { globals: globals.browser },
    rules: {
      "@typescript-eslint/ban-types": [
        "error",
        {
          types: {
            Function: false, // CDK uses this a lot
          },
          extendDefaults: true,
        },
      ],
    },
  },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  {
    ignores: ["**/*.js"],
  },
];
