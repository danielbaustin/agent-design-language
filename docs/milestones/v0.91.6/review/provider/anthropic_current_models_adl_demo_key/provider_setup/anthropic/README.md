# Provider setup: anthropic

This bundle gives you a local starting point for configuring the `anthropic` provider family.

Files:
- `provider.adl.yaml`: mergeable ADL provider/agent snippet
- `env.example`: local env template for your credential

Selected default model:
- `claude-haiku-4-5-20251001`

Steps:
1. Copy `env.example` to a local untracked env file and put your real credential in `ANTHROPIC_API_KEY`.
2. Leave `config.endpoint` unset unless you are testing against a trusted compatible endpoint.
3. Merge the provider/agent snippet into your workflow file.
4. Change `provider_model_id` to any trusted model ID supported by this provider family when you want a different task/model route.
5. Source your local env file before running ADL.

Important:
- This family uses ADL's Rust-native provider adapter for its vendor API.
- Leave `config.endpoint` unset for the default vendor endpoint unless you are testing against a trusted compatible endpoint.
- No secrets are stored by this command; the generated env file is only a local template.

Notes:
Use this for the Rust-native Anthropic provider path. The default endpoint is Anthropic's Messages API; override config.endpoint only for tests or a trusted compatible endpoint.
