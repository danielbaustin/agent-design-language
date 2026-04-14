# Multi-Agent Transcript Artifact Contract

Status: bounded demo contract

This document defines the canonical reviewer-facing transcript artifact shape for
bounded multi-agent discussion demos such as the ChatGPT + Claude tea
discussion.

The contract is intentionally narrow. It does not define a general conversation
runtime, memory model, or autonomous agent society. It defines the proof surface
for one bounded ADL run.

## Canonical Artifact

The primary readable artifact is:

```text
transcript.md
```

The transcript is paired with:

```text
transcript_contract.json
```

Generated transcript artifacts are review surfaces, not source-of-truth runtime
inputs.

## Required Layout

A conforming transcript must contain:

- a top-level title
- a provenance statement saying it was assembled from runtime-written step outputs
- exactly one ordered section per declared turn
- a stable separator between turns
- a human-readable heading for each turn

The required heading set is contract-defined. Different bounded demos may use
different turn counts and speaker orders as long as they declare and validate
them explicitly.

## Required Companion Artifacts

A transcript proof surface is complete only when paired with:

- `demo_manifest.json`
- runtime `run_summary.json`
- runtime `logs/trace_v1.json`
- runtime-written per-turn output files

Some demos may also publish additional reviewer aids such as `synthesis.md`.

## Machine-Readable Contract

`transcript_contract.json` must use this object shape:

```json
{
  "schema_version": "multi_agent_discussion_transcript.v1",
  "transcript_path": "transcript.md",
  "turn_count": 20,
  "turns": [
    {
      "turn_id": "turn_01",
      "ordinal": 1,
      "speaker": "ChatGPT",
      "heading": "# Turn 1 - ChatGPT",
      "source_output": "out/discussion/01-chatgpt-opening.md"
    }
  ],
  "companion_artifacts": {
    "demo_manifest": "demo_manifest.json",
    "synthesis": "synthesis.md",
    "run_summary": "runtime/runs/v0-88-real-multi-agent-tea-discussion/run_summary.json",
    "trace": "runtime/runs/v0-88-real-multi-agent-tea-discussion/logs/trace_v1.json"
  }
}
```

The example above shows one turn for readability. A conforming contract must
declare every turn in order.

## Validation Rules

The validator must check:

- the transcript file exists
- the transcript is valid UTF-8
- the title is present
- the provenance statement is present
- all declared turn headings are present exactly once
- declared turn headings appear in order
- the transcript contains the declared number of turn sections
- the transcript does not contain unresolved template markers

The validator must not:

- call providers
- modify files
- infer missing turns
- accept out-of-order turns
- validate broad runtime behavior outside the transcript contract

## Canonical Validation Command

From repository root:

```bash
python3 adl/tools/validate_multi_agent_transcript.py \
  artifacts/v088/real_multi_agent_discussion/transcript.md \
  --contract artifacts/v088/real_multi_agent_discussion/transcript_contract.json
```

## Reviewer Checklist

- `transcript.md` exists at the documented output path.
- `transcript_contract.json` declares `multi_agent_discussion_transcript.v1`.
- The transcript contains the declared number of ordered turns.
- Each turn heading names the speaker.
- The transcript states it was assembled from runtime-written step outputs.
- Companion manifest, run summary, and trace artifacts exist.
- Any optional `synthesis.md` surface is truthful and derived from declared turns.

## Non-Goals

- general chat transcript schema
- provider-native transcript capture
- long-lived session persistence
- autonomous conversation management
- transcript signing
