# Local Watcher Agent Proof for #4397

## Scope

This note records the bounded local-agent experiment required by `#4397`.

It proves that the repo-native watcher JSON can be consumed by a local model in
an advisory-only mode while ADL remains the authoritative classifier.

It does not grant the local model authority to merge PRs, close issues,
override ADL classification, or mutate repository state.

## Deterministic state packet

The deterministic watcher packet used for this experiment is:

- `docs/milestones/v0.91.6/review/sprint_execution_packets/ISSUE_4397_LOCAL_WATCHER_STATE_PACKET.json`

That packet came from:

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
  cargo run --manifest-path adl/Cargo.toml --bin adl -- pr watch 4397 --json
```

Observed authoritative result:

- `classification`: `ready_for_run`
- `next_skill`: `pr-run`
- `continuation`: `continue`
- `authoritative_classifier`: `adl`
- `advisory_agent_mode`: `local_agent_advisory_only`

## Local agent experiment

Tested local model:

- `ollama:gemma4:31b`

Execution surface:

- Rust provider adapter via `adl-provider-adapter`
- local Ollama HTTP endpoint at `http://127.0.0.1:11434`

Prompt contract used:

- consume only the supplied watcher packet
- return one classification from:
  `ready_for_run`, `pr_open`, `checks_running`, `checks_failed`,
  `checks_green`, `merged_pending_closeout`, `closeout_needed`, `blocked`,
  `closed`, `unknown`
- return `unknown` when evidence is insufficient
- state explicitly that ADL remains authoritative

Observed provider result:

```md
# Classification
ready_for_run

# Evidence
The packet specifies `classification` as "ready_for_run", `issue_state` as "OPEN", and the `reason` as "issue_ready_without_linked_pr".

# Boundary
ADL remains authoritative and I have no merge, closeout, or repo-mutation authority.
```

Observed runtime notes:

- one local attempt completed successfully
- bounded provider-adapter heartbeats were emitted during execution
- total local-model duration was about 26 seconds

## Conclusion

The local-agent experiment passed for the bounded `ready_for_run` watcher case.

`gemma4:31b` correctly echoed the authoritative ADL classification, cited only
supplied evidence, and preserved the advisory boundary.

That is sufficient to plan near-term watcher assistance around a local model,
provided the local agent consumes the typed `adl.pr.watch.v1` packet and never
becomes the source of truth.

## Recommended near-term usage

- Keep `adl pr watch --json` as the single authoritative state packet.
- Plan the first production watcher-agent integration around a local agent that
  reads that packet after ADL emits it, rather than polling GitHub or
  classifying lifecycle state on its own.
- Allow a local watcher agent to summarize or restate the packet only after ADL
  emits it.
- Route all lifecycle action off ADL fields such as `classification`,
  `next_skill`, and `continuation`, not off the model's prose.
- Expand local-model coverage later to `checks_running`, `checks_failed`,
  `checks_green`, `merged_pending_closeout`, and `closeout_needed` packets
  before depending on it for broader sprint execution assistance.
