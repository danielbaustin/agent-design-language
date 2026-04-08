# Provider Setup

`adl provider setup <family>` generates a local, untracked setup bundle for a
remote provider family.

Current supported families:
- `chatgpt`
- `openai`
- `anthropic`
- `gemini`
- `deepseek`
- `http`

Default output location:
- `.adl/provider-setup/<family>/`

Generated files:
- `provider.adl.yaml`
- `.env.example`
- `README.md`

The generated bundle is intentionally local-only:
- no secrets are stored by the command
- `.env.example` is a template, not a credential store
- users are expected to copy/fill a local env file and source it before running ADL

Important transport note:
- ADL's bounded HTTP provider expects a completion-style contract:
  - request JSON with `{"prompt": "..."}`
  - response JSON with `{"output": "..."}`
- raw vendor-native endpoints may need an adapter or compatibility gateway if
  they do not expose that exact contract directly

Example:

```bash
adl provider setup chatgpt
adl provider setup openai --out ./.adl/provider-setup/openai
```
