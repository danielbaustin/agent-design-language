# ChatGPT + Gemini + Claude Triad Conversation Demo

## Summary

This bounded `v0.91` demo is the first shared three-party conversation proof in
the multi-agent wave.

`ChatGPT`, `Gemini`, and `Claude` all contribute attributable turns in one
saved exchange, with explicit turn order and a bounded stop rule.

## Scope Boundary

This demo proves:

- one shared triad conversation
- explicit participant identity
- explicit turn ordering
- replayable transcript, trace, and invocation artifacts

It does **not** prove:

- review-panel synthesis quality
- general N-party federation
- autonomous coordination beyond the saved turns

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v091_chatgpt_gemini_claude_triad_conversation.sh
```

## What Runs

- local provider bridge:
  - `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py`
- runtime workflow:
  - `adl/examples/v0-91-chatgpt-gemini-claude-triad-conversation.adl.yaml`
- wrapper:
  - `adl/tools/demo_v091_chatgpt_gemini_claude_triad_conversation.sh`

## Primary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_claude_triad_conversation/transcript.md`
- `artifacts/v091/chatgpt_gemini_claude_triad_conversation/proof_note.md`
- `artifacts/v091/chatgpt_gemini_claude_triad_conversation/provider_invocations.json`

## Secondary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_claude_triad_conversation/observatory_projection.json`
- `artifacts/v091/chatgpt_gemini_claude_triad_conversation/runtime/runs/v0-91-chatgpt-gemini-claude-triad-conversation/run_summary.json`
- `artifacts/v091/chatgpt_gemini_claude_triad_conversation/runtime/runs/v0-91-chatgpt-gemini-claude-triad-conversation/logs/trace_v1.json`

## Success Signal

The demo is successful when:

- all three participants appear explicitly in one saved exchange
- the turn order is easy to follow from the artifact alone
- the stop rule is bounded and visible
- the proof note stays honest about what the triad does not prove
