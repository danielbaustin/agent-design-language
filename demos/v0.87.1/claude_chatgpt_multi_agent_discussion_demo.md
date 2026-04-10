# Claude + ChatGPT Multi-Agent Discussion Demo

## Summary

This is a bounded `v0.87.1` demo that runs a five-turn discussion workflow through
the real ADL runtime. It keeps the shape simple:

- one sequential ADL workflow
- two named agents
- five explicit turns
- runtime-visible conversation turn metadata
- saved-state handoff between turns
- a reviewer-facing transcript assembled from runtime outputs

The tone is intentionally light, but the proof surface is technical and auditable.

## Scope Boundary

This demo proves a bounded turn-based multi-agent workflow with explicit runtime
metadata for turn id, speaker, sequence, thread, and response linkage. It does
**not** claim:

- a full conversation-platform runtime abstraction
- free-form autonomous agent chat
- direct vendor-native Claude or ChatGPT transport integration

For now, both personas are served through a deterministic local HTTP compatibility
provider shim so the demo stays runnable and reviewable without external account
setup.

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v0871_multi_agent_discussion.sh
```

## What Runs

- local provider shim:
  - `adl/tools/mock_multi_agent_discussion_provider.py`
- runtime workflow:
  - `adl/examples/v0-87-1-multi-agent-tea-discussion.adl.yaml`
- wrapper:
  - `adl/tools/demo_v0871_multi_agent_discussion.sh`

## Primary Proof Surfaces

- `artifacts/v0871/multi_agent_discussion/transcript.md`
- `artifacts/v0871/multi_agent_discussion/runtime/runs/v0-87-1-multi-agent-tea-discussion/run_summary.json`

## Secondary Proof Surfaces

- `artifacts/v0871/multi_agent_discussion/runtime/runs/v0-87-1-multi-agent-tea-discussion/logs/trace_v1.json`
- `artifacts/v0871/multi_agent_discussion/demo_manifest.json`
- `artifacts/v0871/multi_agent_discussion/run_log.txt`
- `artifacts/v0871/multi_agent_discussion/provider_server.log`

## Success Signal

The demo is successful when:

- the transcript contains five turns
- ChatGPT and Claude are explicit and distinguishable
- the runtime emits the normal run-summary and trace surfaces
- the final transcript/toast still states the bounded proof honestly

## Focused Validation

From repository root:

```bash
bash adl/tools/test_demo_v0871_multi_agent_discussion.sh
```
