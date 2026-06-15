# Provider setup: openrouter

This bundle gives you a local starting point for configuring the `openrouter` provider family.

Files:
- `provider.adl.yaml`: mergeable ADL provider/agent snippet
- `env.example`: local env template for your credential

Selected default model:
- `deepseek/deepseek-v4-flash`

Steps:
1. Copy `env.example` to a local untracked env file and put your real credential in `OPENROUTER_API_KEY`.
2. Leave `config.endpoint` unset unless you are testing against a trusted compatible endpoint.
3. Merge the provider/agent snippet into your workflow file.
4. Change `provider_model_id` to any trusted model ID supported by this provider family when you want a different task/model route.
5. Source your local env file before running ADL.

Important:
- This family uses ADL's Rust-native provider adapter for its vendor API.
- Leave `config.endpoint` unset for the default vendor endpoint unless you are testing against a trusted compatible endpoint.
- No secrets are stored by this command; the generated env file is only a local template.

Notes:
Use this for the Rust-native OpenRouter provider path. The default endpoint is OpenRouter's chat completions API; override config.endpoint only for tests or a trusted compatible endpoint. OpenRouter capability support is model-dependent, so record model-specific lane evidence rather than assuming gateway-wide tool or JSON support.
