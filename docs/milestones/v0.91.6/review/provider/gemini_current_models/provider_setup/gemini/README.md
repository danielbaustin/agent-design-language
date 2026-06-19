# Provider setup: gemini

This bundle gives you a local starting point for configuring the `gemini` provider family.

Files:
- `provider.adl.yaml`: mergeable ADL provider/agent snippet
- `env.example`: local env template for your credential

Selected default model:
- `gemini-2.0-flash`

Steps:
1. Copy `env.example` to a local untracked env file and put your real credential in `GEMINI_API_KEY`.
2. Set `config.endpoint` in `provider.adl.yaml` to a real ADL-compatible completion endpoint.
3. Merge the provider/agent snippet into your workflow file.
4. Change `provider_model_id` to any trusted model ID supported by this provider family when you want a different task/model route.
5. Source your local env file before running ADL.

Important:
- ADL's bounded HTTP provider expects a completion-style HTTP contract: request body with `{"prompt": ...}`, response body with `{"output": ...}`.
- Raw vendor-native endpoints may require a compatibility gateway or adapter if they do not expose that contract directly.
- No secrets are stored by this command; the generated env file is only a local template.

Notes:
Use this with an ADL-compatible HTTP endpoint that fronts Gemini-compatible models. The generated env file is local-only and should not be committed.
