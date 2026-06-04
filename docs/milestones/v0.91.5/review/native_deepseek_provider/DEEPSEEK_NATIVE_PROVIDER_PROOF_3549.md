# Native DeepSeek Provider Proof Packet `#3549`

## Metadata

- Issue: `#3549`
- Milestone: `v0.91.5`
- Surface: native hosted-provider adapter
- Provider family: `deepseek`
- Credential policy: operator environment only; no secret material recorded
- Live run status: `passed_operator_provided_key`

## Change Summary

`#3549` adds a first-class Rust-native DeepSeek provider path:

- `type: "deepseek"` is accepted by provider construction and provider
  substrate normalization.
- The native adapter targets DeepSeek's chat completions API by default:
  `https://api.deepseek.com/chat/completions`.
- The native adapter reads `DEEPSEEK_API_KEY` from the environment by default.
- `adl provider setup deepseek` now emits `type: "deepseek"` instead of the
  older `http:deepseek-chat` compatibility profile.
- `http:deepseek-chat` remains available only as a compatibility/profile lane
  for ADL-style completion gateways.
- Compatibility `http:*` profiles remain classified as `generic_http` in the
  provider substrate even when their endpoint names contain vendor-family
  strings.

## Contract Boundaries

- No credential values are written to tracked files.
- No native tool-calling claim is made for DeepSeek.
- No OpenRouter path is treated as native DeepSeek.
- Local/Ollama DeepSeek support is unchanged and remains a separate local model
  lane.
- Live provider proof requires the operator to provide `DEEPSEEK_API_KEY`.

## Focused Test Surfaces Added

The implementation adds focused Rust tests for:

- DeepSeek request construction against a local loopback JSON server.
- Bearer authentication header behavior without storing credential material.
- DeepSeek chat-completion response parsing.
- Missing `DEEPSEEK_API_KEY` failure classification.
- Missing message-content failure classification.
- Custom endpoint credential-safety rejection.
- Provider setup output using native `type: "deepseek"`.
- Provider substrate recognition of native `deepseek` as hosted HTTP.
- Provider substrate preservation of `http:deepseek-chat` as a compatibility
  lane rather than native DeepSeek.

## Validation Results

Focused Rust validation passed:

```bash
DEEPSEEK_API_KEY="$(tr -d '\n\r' < "$HOME/keys/deepseek.key")" \
  cargo test --manifest-path adl/Cargo.toml deepseek --lib
```

Result:

- `5` tests passed
- `0` failed
- Covered native DeepSeek request construction, missing credentials, bad
  response shape, native substrate routing, and compatibility-lane preservation

Focused provider setup validation passed:

```bash
cargo test --manifest-path adl/Cargo.toml provider_setup_supports_all_declared_families
```

Result:

- `cli::provider_cmd::tests::provider_setup_supports_all_declared_families`
  passed in both `src/main.rs` and `src/bin/adl_csdlc.rs` test targets
- `0` failures

Focused compatibility-lane validation passed:

```bash
cargo test --manifest-path adl/Cargo.toml \
  provider_substrate_keeps_http_deepseek_profile_as_compatibility_lane --lib
```

Result:

- `1` test passed
- `0` failed

Focused coverage-impact evidence passed:

```bash
cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov --workspace --all-features \
  --json --summary-only --output-path target/coverage-impact-summary.json -- provider
```

Result:

- provider-focused coverage slice passed across provider setup, provider
  profiles, provider substrate, native adapters, provider integration, and
  related provider proof surfaces
- `0` failures
- Summary artifact written to `adl/target/coverage-impact-summary.json`

## Live Run Disposition

Live DeepSeek API execution was run once with the operator-provided key from
`$HOME/keys/deepseek.key`. The key was loaded into process environment only and
was not printed or written to tracked files.

Command shape:

```bash
key="$(tr -d '\n\r' < "$HOME/keys/deepseek.key")"
curl -sS --fail-with-body https://api.deepseek.com/chat/completions \
  -H "Authorization: Bearer ${key}" \
  -H 'Content-Type: application/json' \
  -d '{"model":"deepseek-chat","messages":[{"role":"user","content":"Return exactly: ADL_DEEPSEEK_OK"}],"max_tokens":16,"stream":false}'
```

Observed non-secret result:

- Response text: `ADL_DEEPSEEK_OK`
- Reported served model: `deepseek-v4-flash`

Required live proof command shape, after operator credential setup:

```bash
export DEEPSEEK_API_KEY=...
adl provider setup deepseek --force
```

A stronger future proof can add a bounded demo wrapper that invokes the native
provider with a small prompt and records only status, model, HTTP status,
timestamp, prompt length, and output length.

## Non-Claims

- This packet does not claim DeepSeek tool-calling support.
- This packet does not claim benchmark quality or role aptitude.
- This packet does not claim the compatibility `http:deepseek-chat` profile is
  equivalent to the native DeepSeek API.
