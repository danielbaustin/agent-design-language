# PVF Validation Lane Taxonomy v0.91.4

Status: proposed

## Purpose

Define the canonical validation-lane taxonomy for the Parallel Validation
Fabric (`PVF`) and provide one stable vocabulary that issue plans, runner
manifests, CI integration, and release-gate decisions can reuse without
redefining lane meaning per issue.

This packet is intentionally narrow. It does not implement the runner or CI
hooks. It defines the lane classes and the manifest fields those later surfaces
must consume.

## Canonical lane classes

| Lane id | Primary purpose | Typical command shape | Resource profile | Determinism target | Release-gate class |
| --- | --- | --- | --- | --- | --- |
| `fast_unit` | Fast local correctness proof for code-only changes | focused test or script invocation | low | strict deterministic | `required_on_pr` |
| `contract_schema_card` | Structured prompt, schema, or card validation | validator scripts, JSON/YAML parse | low | strict deterministic | `required_on_pr` |
| `docs` | Docs truth, formatting, and publication-boundary checks | link, copy, grep, diff, render-safe checks | low | strict deterministic | `required_on_pr` for docs-only changes, otherwise `optional` |
| `cli_workflow` | Lifecycle CLI and shell workflow proof | bounded issue-mode shell tests | medium | strict deterministic | `required_on_pr` when workflow tools change |
| `integration_worktree` | Multi-step integration proof in a bound worktree | composed repo script or targeted test lane | medium | deterministic with declared fixtures | `release_candidate` |
| `provider_live` | Live external-provider proof | explicit live provider command with credentials and billing risk | high | non-deterministic, evidence-bound | `manual_release_gate` |
| `release_gate` | Final milestone or pre-release proof | curated aggregate lane invocation | high | evidence-bound | `manual_release_gate` |

## Lane semantics

### `fast_unit`

- Smallest proving lane for local code changes.
- Must be cheap enough to run repeatedly during issue execution.
- Should not absorb unrelated integration coverage.

### `contract_schema_card`

- Validates machine-readable contracts and structured prompt surfaces.
- Includes schema parse, manifest parse, card validators, and contract-shape
  checks.
- Preferred lane for prompt-template, JSON schema, and manifest work.

### `docs`

- Validates docs-only work without escalating into broad code lanes.
- Includes truth scans, formatting-safe diff checks, and bounded render checks.
- Must not silently pull in provider or integration proof.

### `cli_workflow`

- Validates issue lifecycle helpers, shell workflow scripts, or conductor
  utilities.
- Expected to remain deterministic and repo-local.
- May use fixture directories or synthetic issue-state artifacts.

### `integration_worktree`

- Validates that multiple repo surfaces cooperate correctly in a bound worktree.
- Heavier than `cli_workflow`, but still expected to avoid live external
  systems.
- Appropriate for composed issue lifecycle flows, sprint state transitions, and
  end-to-end local proofs.

### `provider_live`

- Validates behavior that depends on external providers, accounts, network
  state, or billing-backed APIs.
- Requires explicit operator intent, cost awareness, and truthful evidence
  capture.
- Must never be implied by a docs-only or local-only issue.

### `release_gate`

- Represents a milestone or launch decision surface rather than a single
  primitive test class.
- May aggregate results from other lane classes.
- Requires evidence-bound human interpretation before release claims.

## Manifest requirements

A manifest must declare:

- `lane_classes`: the subset of canonical lane classes actually instantiated in
  that manifest's `lanes` map

Each declared lane manifest entry is keyed by a unique lane id inside the
manifest's `lanes` map and must define:

- `lane_class`: one of the canonical lane classes above
- `owner_surface`: the primary surface the lane proves
- `command`: exact invocation or script entrypoint
- `resource_profile`: `low`, `medium`, or `high`
- `determinism`: `strict`, `fixture_bound`, or `live`
- `cache_strategy`: `none`, `local_reuse`, or `artifact_reuse`
- `release_gate_class`: `required_on_pr`, `release_candidate`, `manual_release_gate`, or `optional`
- `default_trigger`: `always`, `changed_paths`, `manual`, or `release_only`
- `changed_path_hints`: zero or more path prefixes or glob-style hints
- `evidence_outputs`: zero or more files or artifact classes produced by the lane

Optional but encouraged:

- `timeout_minutes`
- `requires_credentials`
- `requires_worktree`
- `notes`

The manifest key, not an inner `lane_id` field, is the authoritative lane
identifier. This keeps uniqueness structural instead of relying on reviewer
discipline.

## Default policy guidance

The taxonomy implies these default policies:

1. Docs-only issues should prefer `docs` and `contract_schema_card` lanes.
2. Prompt-card, schema, and manifest issues should default to
   `contract_schema_card`.
3. Workflow helper issues may use `cli_workflow` and only escalate to
   `integration_worktree` when local composition proof is necessary.
4. `provider_live` and `release_gate` lanes must be explicit and never inferred
   from file paths alone.
5. `resource_profile=high` lanes require stronger operator awareness than
   `low` or `medium` lanes.

## Example mapping

| Change type | Preferred lane class |
| --- | --- |
| JSON manifest schema edit | `contract_schema_card` |
| Card validator script change | `contract_schema_card` or `cli_workflow` |
| PR helper shell script change | `cli_workflow` |
| Milestone doc-only packet update | `docs` |
| Bound worktree lifecycle composition proof | `integration_worktree` |
| Live provider benchmark or hosted deploy proof | `provider_live` |
| Final milestone evidence bundle | `release_gate` |

## Non-goals

- Defining scheduler implementation details
- Defining CI matrix YAML
- Choosing provider credentials or live account routing
- Replacing issue-local validation judgment
