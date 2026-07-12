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
- `bedrock`
- `openrouter`
- `z_ai`
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
- `openai`, `anthropic`, `deepseek`, `openrouter`, `bedrock`, and `z_ai` now use Rust-native provider adapters by default:
  - `type: "openai"` targets the OpenAI Responses API unless `config.endpoint` is explicitly overridden
  - `type: "anthropic"` targets the Anthropic Messages API unless `config.endpoint` is explicitly overridden
  - `type: "deepseek"` targets the DeepSeek chat completions API unless `config.endpoint` is explicitly overridden
  - `type: "openrouter"` targets the OpenRouter chat completions API unless `config.endpoint` is explicitly overridden
  - `type: "bedrock"` targets AWS Bedrock Runtime through the AWS SDK and defaults to `ADL_AWS_PROFILE=agent-logic-admin`
  - `type: "z_ai"` targets the Z.ai/Zhipu OpenAI-compatible chat completions API unless `config.endpoint` is explicitly overridden
- ADL's bounded HTTP provider expects a completion-style contract:
  - request JSON with `{"prompt": "..."}`
  - response JSON with `{"output": "..."}`
- raw vendor-native endpoints may need an adapter or compatibility gateway if
  they do not expose that exact contract directly; this applies to HTTP/profile
  families such as `chatgpt`, `claude`, `gemini`, and `http`
- provider-family demos should keep setup instructions here and keep family-specific
  runtime proof steps in their own wrapper surfaces

Example:

```bash
adl provider setup chatgpt
adl provider setup claude
adl provider setup openai --out ./.adl/provider-setup/openai
adl provider setup deepseek
adl provider setup bedrock
adl provider setup z_ai
```

AWS Bedrock native note:
- `adl provider setup bedrock` emits `type: "bedrock"`, defaults to `profile: "agent-logic-admin"` and `region: "us-west-2"`, and uses AWS SDK credential resolution rather than bearer-token auth
- ADL AWS work must use the Agent Logic business profile unless the operator explicitly authorizes a bounded personal-account diagnostic
- the initial provider mini-sprint target is `amazon.nova-lite-v1:0`; `amazon.nova-pro-v1:0` is the secondary target when access and cost posture allow

DeepSeek native note:
- `adl provider setup deepseek` emits `type: "deepseek"`, reads `DEEPSEEK_API_KEY`, and uses `https://api.deepseek.com/chat/completions` by default
- the older `http:deepseek-chat` profile remains a compatibility surface for ADL-style completion gateways; it is not the native DeepSeek API path

Z.ai native note:
- `adl provider setup z_ai` emits `type: "z_ai"`, reads `ZAI_API_KEY`, and uses `https://open.bigmodel.cn/api/paas/v4/chat/completions` by default
- the first built-in Z.ai profile is `z_ai:glm-5`, which maps to provider model id `glm-5` for the provider mini-sprint UTS route `hosted:adl-z-ai:glm-5`

Loopback demo note:
- the `v0.87.1` bounded HTTP family demo uses `http://127.0.0.1:8787/complete` with a dummy bearer token as a local proof path for the ADL completion contract

ChatGPT demo note:
- the `v0.87.1` ChatGPT family demo uses the `chatgpt:gpt-5.4` profile plus a local bounded completion adapter on `http://127.0.0.1:8787/complete`; it proves the current setup/profile surface, not a raw vendor-native endpoint

Claude family note:
- the first-class Claude setup surface uses `claude:claude-3-7-sonnet` plus the same bounded ADL completion contract; it is distinct from the generic `anthropic` compatibility setup so Claude can be referenced symmetrically with ChatGPT in multi-agent workflows

Fable 5 UTS acceptance note:
- use `adl/tools/run_fable5_uts_acceptance.sh` to run Claude Fable 5 through the ADL provider adapter and the sibling UTS benchmark runner
- use `docs/tooling/CALL_CLAUDE_FABLE_5.md` for a bounded diagnostic or review call with retained provider artifacts
- the script writes an ad-hoc selector for `hosted:adl-anthropic:claude-fable-5`, injects a portable `max_output_tokens` budget through `adl/tools/adl_provider_adapter_with_budget.py`, runs the UTS deterministic self-check, runs an optional hosted probe, and then runs the `regular,uts_only` lanes
- provide credentials with `--key-file "$HOME/keys/claude2.key"` or an already-set `ANTHROPIC_API_KEY`; the script must not print or retain the key value
- proof for issue #5044 is recorded in `docs/milestones/v0.91.7/review/provider/FABLE5_UTS_ACCEPTANCE_5044.md`

Live multi-agent demo note:
- `adl/tools/demo_v0871_real_multi_agent_discussion.sh` is the operator-run live-provider companion to the deterministic multi-agent demo
- `adl/tools/test_demo_v0871_real_multi_agent_discussion.sh` preserves local ergonomics by exiting successfully when operator credentials are unavailable, but it now emits an explicit machine-readable non-proving skip disposition instead of implying live-provider proof
- it reads `OPENAI_API_KEY` and `ANTHROPIC_API_KEY` from the environment when set, or from explicit operator-selected key-file overrides when provided
- it starts a local adapter that bridges ADL's current `{"prompt": "..."} -> {"output": "..."}` HTTP contract to vendor-native OpenAI and Anthropic APIs
- generated artifacts record provider family/model/status metadata only; they must not include secret values or raw credential headers
- a real D13L proof claim requires the credentialed path to complete and write the invocation/transcript artifacts named in the demo matrix
