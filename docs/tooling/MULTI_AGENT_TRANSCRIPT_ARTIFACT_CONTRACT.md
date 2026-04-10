# Multi-Agent Transcript Artifact Contract

Status: v0.87.1 bounded demo contract

This document defines the canonical transcript artifact shape for bounded
multi-agent discussion demos.

The contract is intentionally narrow. It standardizes the reviewer-facing
transcript file emitted by demos such as the Claude + ChatGPT tea discussion. It
does not define a general conversation runtime, provider protocol, chat memory
model, or long-lived agent collaboration substrate.

## Canonical Artifact

The primary transcript artifact is:

```text
transcript.md
```

For the `v0.87.1` multi-agent discussion demo, the canonical path is:

```text
artifacts/v0871/multi_agent_discussion/transcript.md
```

The transcript is accompanied by a machine-readable contract artifact:

```text
artifacts/v0871/multi_agent_discussion/transcript_contract.json
```

Generated transcript artifacts should be treated as demo output. They are proof
surfaces for review, not source-of-truth inputs to the ADL runtime.

## Required Layout

A conforming bounded multi-agent transcript MUST contain:

- a top-level title
- a short provenance statement explaining that the transcript was assembled from runtime-written step outputs
- exactly one ordered section per turn
- a stable separator between turns
- a human-readable turn heading for each turn

For the `v0.87.1` tea discussion demo, the required turn headings are:

```text
# Turn 1 - ChatGPT
# Turn 2 - Claude
# Turn 3 - ChatGPT
# Turn 4 - Claude
# Turn 5 - ChatGPT
```

The required turn order is the source of reviewer clarity. If a later demo needs
a different speaker order, it should document and validate that order explicitly
instead of reusing this demo-specific sequence silently.

## Required Companion Artifacts

A transcript proof surface is complete only when it is paired with:

- `demo_manifest.json`
- runtime `run_summary.json`
- runtime `logs/trace_v1.json`
- the runtime-written per-turn step output files

The transcript itself is a readable assembly of step outputs. The manifest and
runtime artifacts provide the machine-checkable evidence that the transcript came
from a bounded ADL run.

## Machine-Readable Contract

`transcript_contract.json` MUST use this object shape:

```json
{
  "schema_version": "multi_agent_discussion_transcript.v1",
  "transcript_path": "transcript.md",
  "turn_count": 5,
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
    "run_summary": "runtime/runs/v0-87-1-multi-agent-tea-discussion/run_summary.json",
    "trace": "runtime/runs/v0-87-1-multi-agent-tea-discussion/logs/trace_v1.json"
  }
}
```

The example above shows one turn for readability. A conforming contract for the
tea discussion demo must declare all five turns in order.

## Validation Rules

The transcript validator MUST check:

- the transcript file exists
- the transcript is valid UTF-8 text
- the title is present
- the runtime-output provenance statement is present
- all required turn headings are present exactly once
- required turn headings appear in order
- the transcript contains the expected number of turn sections
- the transcript does not contain unresolved template markers

The validator MUST NOT:

- call providers
- modify files
- infer missing turns
- accept out-of-order turns
- validate broad runtime behavior outside the transcript contract

## Canonical Validation Command

From repository root:

```bash
python3 adl/tools/validate_multi_agent_transcript.py artifacts/v0871/multi_agent_discussion/transcript.md
```

To validate the transcript and its machine-readable contract together:

```bash
python3 adl/tools/validate_multi_agent_transcript.py \
  artifacts/v0871/multi_agent_discussion/transcript.md \
  --contract artifacts/v0871/multi_agent_discussion/transcript_contract.json
```

For the complete demo proof:

```bash
bash adl/tools/test_demo_v0871_multi_agent_discussion.sh
```

## Reviewer Checklist

- `transcript.md` exists at the documented demo output path.
- `transcript_contract.json` declares `multi_agent_discussion_transcript.v1`.
- The transcript has five ordered turns for the bounded tea discussion demo.
- Each turn heading names the speaker.
- The transcript states it was assembled from runtime-written step outputs.
- Companion manifest, run summary, and trace artifacts exist.
- Generated transcript artifacts are not committed unless a review package
  explicitly calls for checked-in evidence.

## Non-Goals

- general chat transcript schema
- provider-native transcript capture
- model-to-model memory semantics
- autonomous conversation management
- transcript hashing or signing
- long-lived multi-agent session persistence
