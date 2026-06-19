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

Future work should add a validation manager agent on top of this selector. That
agent should create a tailored test profile whenever an issue is sent to PR,
handling each issue case separately while consuming this selector as policy
input. The selector itself should stay deterministic and boring.
