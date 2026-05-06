# ChatGPT + Gemini Direct Conversation Demo

## Summary

This bounded `v0.91` demo runs a four-turn direct conversation workflow
through the real ADL runtime using explicit `ChatGPT` and `Gemini` participant
identities and live provider calls.

The proof shape is intentionally narrow:

- two named agents
- four explicit sequential turns
- one bounded stop rule
- saved transcript, run summary, trace, and proof note

## Scope Boundary

This demo proves a bounded direct conversation loop with:

- explicit participant identity
- ordered turn metadata
- saved-state handoff between turns
- reviewer-facing proof artifacts

It does **not** claim:

- general provider federation
- unrestricted multi-agent autonomy
- production-ready routing or hardening
- broader triad or review-panel behavior

The runtime still uses a small local adapter boundary, but that adapter calls
the real OpenAI and Gemini APIs with operator-managed credentials instead of a
mock provider shim. The current default model choices are the top-tier text
models for each family: `gpt-5.5` and `gemini-3.1-pro-preview`.

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh
```

## What Runs

- local provider shim:
  - `adl/tools/real_chatgpt_gemini_provider_adapter.py`
- runtime workflow:
  - `adl/examples/v0-91-chatgpt-gemini-direct-conversation.adl.yaml`
- wrapper:
  - `adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh`

## Primary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_direct_conversation/transcript.md`
- `artifacts/v091/chatgpt_gemini_direct_conversation/proof_note.md`
- `artifacts/v091/chatgpt_gemini_direct_conversation/provider_invocations.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/runtime/runs/v0-91-chatgpt-gemini-direct-conversation/run_summary.json`

## Secondary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_direct_conversation/runtime/runs/v0-91-chatgpt-gemini-direct-conversation/logs/trace_v1.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/transcript_contract.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/demo_manifest.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/run_log.txt`
- `artifacts/v091/chatgpt_gemini_direct_conversation/provider_adapter.log`

## Success Signal

The demo is successful when:

- the transcript contains four explicit turns
- `ChatGPT` and `Gemini` are explicit and distinguishable in every turn
- the stop rule is explicit and visible in the proof surfaces
- provider invocations show one live OpenAI lane and one live Gemini lane
- the transcript and proof note stay honest about what the demo does not prove
