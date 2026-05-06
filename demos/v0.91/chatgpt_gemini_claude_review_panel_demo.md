# ChatGPT + Gemini + Claude Review Panel Demo

## Summary

This bounded `v0.91` demo turns the triad into a practical six-turn panel:

- `ChatGPT` moderates and synthesizes
- `Gemini` analyzes feasibility
- `Claude` critiques editorial quality

The panel reviews one small launch brief and records an explicit disposition.

## Scope Boundary

This demo proves:

- explicit differentiated panel roles
- at least two distinct viewpoints
- one saved synthesis or disposition
- replayable transcript, register, and trace artifacts

It does **not** prove:

- production-ready review authority
- broader review-packet infrastructure
- an external review service

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v091_chatgpt_gemini_claude_review_panel.sh
```

## What Runs

- local provider bridge:
  - `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py`
- runtime workflow:
  - `adl/examples/v0-91-chatgpt-gemini-claude-review-panel.adl.yaml`
- wrapper:
  - `adl/tools/demo_v091_chatgpt_gemini_claude_review_panel.sh`

## Primary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_claude_review_panel/transcript.md`
- `artifacts/v091/chatgpt_gemini_claude_review_panel/panel_register.json`
- `artifacts/v091/chatgpt_gemini_claude_review_panel/proof_note.md`

## Secondary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_claude_review_panel/provider_invocations.json`
- `artifacts/v091/chatgpt_gemini_claude_review_panel/observatory_projection.json`
- `artifacts/v091/chatgpt_gemini_claude_review_panel/runtime/runs/v0-91-chatgpt-gemini-claude-review-panel/run_summary.json`
- `artifacts/v091/chatgpt_gemini_claude_review_panel/runtime/runs/v0-91-chatgpt-gemini-claude-review-panel/logs/trace_v1.json`

## Success Signal

The demo is successful when:

- the transcript keeps the panel roles explicit
- Gemini and Claude produce distinct viewpoints
- ChatGPT records a clear disposition
- the proof note remains honest about what the panel does not prove
