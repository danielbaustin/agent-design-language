# Validation Lane Selector

ADL keeps the full validation surface available, but normal PR work should run
the smallest proving lane for the changed surface.

Use:

```bash
bash adl/tools/select_validation_lanes.sh --changed-files /path/to/changed-files
```

or, for a branch diff:

```bash
bash adl/tools/select_validation_lanes.sh --base origin/main --head HEAD
```

During local authoring, include tracked working-tree changes and untracked new
files with:

```bash
bash adl/tools/select_validation_lanes.sh --include-working-tree
```

The selector reads `adl/config/validation_lane_selector.v0.91.6.json`, classifies
changed paths, and prints a lane plan. It does not make GitHub or CI the source
of C-SDLC truth. It gives local and PR tooling a deterministic answer for which
focused validation commands are sufficient, which release gates remain required,
and which ambiguous Rust surfaces must escalate.

The manifest is now the tracked authority for validation-surface metadata, not
just path matching. Each lane entry carries:

- owner
- lane class
- command and run command
- path selectors
- requirement IDs
- resource class
- determinism posture
- proof role
- risk class
- escalation rule

Top-level `surface_defaults` provide durable defaults for docs, tooling,
runtime, provider, security, CI policy, slow-proof, release-gate, and shared
Rust surfaces. Lane entries may override those defaults, but the selector
validates the references so metadata drift fails closed instead of silently
falling back.

Compatibility note:

- `path_hints` remains accepted as a compatibility alias.
- `path_selectors` is the authoritative field for new manifest work.
- `release_gate_hints` and `rust_path_hints` still exist for backward
  compatibility, but their durable metadata now lives in `special_surfaces`.

To write machine-readable proof:

```bash
bash adl/tools/select_validation_lanes.sh \
  --changed-files /path/to/changed-files \
  --json \
  --report-out /path/to/validation-lane-plan.json
```

To run only the selected lanes:

```bash
bash adl/tools/select_validation_lanes.sh --changed-files /path/to/changed-files --run
```

`--run` refuses plans with `escalated` or `release_gate_required` aggregate
status. That keeps the selector from turning a focused proof into a false green
when the change actually needs broader validation or release-gate handling.

For Rust source paths, the selector delegates filter computation to
`adl/tools/run_pr_fast_test_lane.sh --print-plan` so the existing focused
nextest mapping remains the single implementation of Rust test-filter policy.

The first slice is intentionally small:

- docs paths select diff hygiene unless the path belongs to prompt/template or
  validation-policy surfaces
- ordinary Rust paths select the existing PR-fast focused or family lane
- owner-binary and owner-compatibility paths select owner lanes
- shared Rust paths escalate instead of pretending a small filter is enough
- release/CI-policy paths report `release_gate_required`
- credential and live-provider lanes remain outside ordinary PR validation

The selector remains the deterministic surface classifier. Validation-manager
style profile builders should consume the selector's emitted metadata instead of
re-inventing owner, risk, or escalation inference in a second code path.
