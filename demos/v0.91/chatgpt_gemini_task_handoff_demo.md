# ChatGPT -> Gemini -> ChatGPT Task Handoff Demo

## Summary

This bounded `v0.91` demo advances the story from "the models can talk" to
"the models can collaborate on one explicit task."

`ChatGPT` issues one bounded request to `Gemini`, `Gemini` returns a
constrained result, and `ChatGPT` explicitly integrates that result in a final
turn.

## Scope Boundary

This demo proves:

- one explicit handoff
- one bounded response
- one explicit integration turn
- saved runtime, transcript, and invocation artifacts

It does **not** prove:

- general delegated execution authority
- tool-use autonomy
- multi-step task planning
- three-party coordination

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v091_chatgpt_gemini_task_handoff.sh
```

## What Runs

- local provider bridge:
  - `adl/tools/real_chatgpt_gemini_provider_adapter.py`
- runtime workflow:
  - `adl/examples/v0-91-chatgpt-gemini-task-handoff.adl.yaml`
- wrapper:
  - `adl/tools/demo_v091_chatgpt_gemini_task_handoff.sh`

## Primary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_task_handoff/transcript.md`
- `artifacts/v091/chatgpt_gemini_task_handoff/task_handoff_summary.json`
- `artifacts/v091/chatgpt_gemini_task_handoff/proof_note.md`

## Secondary Proof Surfaces

- `artifacts/v091/chatgpt_gemini_task_handoff/provider_invocations.json`
- `artifacts/v091/chatgpt_gemini_task_handoff/observatory_projection.json`
- `artifacts/v091/chatgpt_gemini_task_handoff/runtime/runs/v0-91-chatgpt-gemini-task-handoff/run_summary.json`
- `artifacts/v091/chatgpt_gemini_task_handoff/runtime/runs/v0-91-chatgpt-gemini-task-handoff/logs/trace_v1.json`

## Success Signal

The demo is successful when:

- the transcript contains a clear request, response, and integration turn
- `ChatGPT` and `Gemini` remain explicit in every turn
- the output contract is visible and actually honored
- `ChatGPT` names what it used from Gemini's reply
- the proof note stays honest about what remains unproven
