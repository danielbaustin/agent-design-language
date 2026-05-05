# ChatGPT + Gemini Direct Conversation Demo

## Summary

This bounded `v0.91` demo runs a four-turn direct conversation workflow
through the real ADL runtime using explicit `ChatGPT` and `Gemini` participant
identities.

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

The provider behavior is served through a deterministic local compatibility
shim so the proof stays runnable and reviewable without external account setup.

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh
```

## What Runs

- local provider shim:
  - `adl/tools/mock_chatgpt_gemini_direct_conversation_provider.py`
- runtime workflow:
  - `adl/examples/v0-91-chatgpt-gemini-direct-conversation.adl.yaml`
- wrapper:
  - `adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh`

## Primary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_direct_conversation/transcript.md`
- `artifacts/v091/chatgpt_gemini_direct_conversation/proof_note.md`
- `artifacts/v091/chatgpt_gemini_direct_conversation/runtime/runs/v0-91-chatgpt-gemini-direct-conversation/run_summary.json`

## Secondary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_direct_conversation/runtime/runs/v0-91-chatgpt-gemini-direct-conversation/logs/trace_v1.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/transcript_contract.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/demo_manifest.json`
- `artifacts/v091/chatgpt_gemini_direct_conversation/run_log.txt`
- `artifacts/v091/chatgpt_gemini_direct_conversation/provider_server.log`

## Success Signal

The demo is successful when:

- the transcript contains four explicit turns
- `ChatGPT` and `Gemini` are explicit and distinguishable in every turn
- the stop rule is explicit and visible in the proof surfaces
- the transcript and proof note stay honest about what the demo does not prove
