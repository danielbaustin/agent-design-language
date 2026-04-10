# Provider Setup

`adl provider setup <family>` generates a local, untracked setup bundle for a
remote provider family.

Current supported families:
- `chatgpt`
- `claude`
- `openai`
- `anthropic`
- `gemini`
- `deepseek`
- `http`

Related shared proof-surface docs:
- `docs/tooling/PROVIDER_DEMO_SURFACES.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`

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
- provider-family demos should keep setup instructions here and keep family-specific
  runtime proof steps in their own wrapper surfaces

Example:

```bash
adl provider setup chatgpt
adl provider setup claude
adl provider setup openai --out ./.adl/provider-setup/openai
```

Loopback demo note:
- the `v0.87.1` bounded HTTP family demo uses `http://127.0.0.1:8787/complete` with a dummy bearer token as a local proof path for the ADL completion contract

ChatGPT demo note:
- the `v0.87.1` ChatGPT family demo uses the `chatgpt:gpt-5.4` profile plus a local bounded completion adapter on `http://127.0.0.1:8787/complete`; it proves the current setup/profile surface, not a raw vendor-native endpoint

Claude family note:
- the first-class Claude setup surface uses `claude:claude-3-7-sonnet` plus the same bounded ADL completion contract; it is distinct from the generic `anthropic` compatibility setup so Claude can be referenced symmetrically with ChatGPT in multi-agent workflows

Live multi-agent demo note:
- `adl/tools/demo_v0871_real_multi_agent_discussion.sh` is the operator-run live-provider companion to the deterministic multi-agent demo
- it reads `OPENAI_API_KEY` and `ANTHROPIC_API_KEY` from the environment when set, otherwise from `$HOME/keys/openai.key` and `$HOME/keys/claude.key`
- it starts a local adapter that bridges ADL's current `{"prompt": "..."} -> {"output": "..."}` HTTP contract to vendor-native OpenAI and Anthropic APIs
- generated artifacts record provider family/model/status metadata only; they must not include secret values or raw credential headers
