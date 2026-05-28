# PVF Initial Lane Inventory v0.91.4

Status: proposed

## Purpose

Provide the first bounded inventory of ADL validation and proof surfaces using
the PVF lane taxonomy from `#3400`.

This packet is intentionally a first-pass inventory, not a complete executor.
It identifies the primary validation surfaces already exercised by CI, PR
publication, card validation, docs checks, workflow helpers, and heavy release
or live-provider flows so later PVF runner work can start from a truthful lane
map instead of an implicit shell-script pile.

## Classification policy used

- `fast_unit`: cheap, deterministic Rust validation or similarly fast local
  compile/test proof
- `contract_schema_card`: structured prompt, schema, manifest, or contract
  validation
- `docs`: docs-only truth and formatting checks
- `cli_workflow`: deterministic workflow/helper validation in repo-local shell
  scripts
- `integration_worktree`: composed local proof across multiple helpers or
  milestone/demo surfaces
- `provider_live`: networked or credential-backed proof
- `release_gate`: heavy milestone or release evidence lane

## Initial representative inventory

| Surface | Current command or entrypoint | Proposed lane class | Resource profile | Ordinary PR posture | Notes |
| --- | --- | --- | --- | --- | --- |
| Rust format gate | `cargo fmt --all -- --check` | `fast_unit` | low | run when `rust_required=true` | deterministic compile-surface hygiene |
| Rust lint gate | `cargo clippy --all-targets -- -D warnings` | `fast_unit` | medium | run when `rust_required=true` | compile-backed but still ordinary PR proof |
| Rust doc tests | `cargo test --doc` | `fast_unit` | medium | run on Rust-affecting PRs unless replaced by authoritative lane policy | bounded compared with full nextest/coverage |
| PR-fast Rust test lane | `bash adl/tools/run_pr_fast_test_lane.sh` | `fast_unit` | medium | ordinary runtime PR lane | already policy-aware and may focus or fail closed |
| Structured prompt / manifest validation | `bash adl/tools/validate_structured_prompt.sh ...` plus PVF schema/manifest parse checks | `contract_schema_card` | low | ordinary PR-ready proof for card, schema, and manifest changes | canonical contract lane for cards, schemas, and manifest fixtures |
| Prompt-template contract suite | `bash adl/tools/test_prompt_templates_1_0_0.sh` | `contract_schema_card` | low | run when prompt-template surfaces change | schema/template contract proof |
| Structured prompt validation suite | `bash adl/tools/test_structured_prompt_validation.sh` | `contract_schema_card` | low | run when validator surfaces change | contract behavior proof |
| CI path-policy contract | `bash adl/tools/test_ci_path_policy.sh` | `cli_workflow` | low | ordinary tooling PR lane | validates changed-path routing behavior |
| Coverage-impact contract | `bash adl/tools/test_check_coverage_impact.sh` | `cli_workflow` | low | ordinary tooling/runtime governance lane | validates changed-source coverage classification |
| Sprint conductor helper suite | `bash adl/tools/test_sprint_conductor_helpers.sh` | `cli_workflow` | medium | ordinary PR lane for sprint tooling | deterministic fixture-bound shell proofs |
| Workflow conductor skill contracts | `bash adl/tools/test_workflow_conductor_skill_contracts.sh` | `cli_workflow` | medium | ordinary PR lane for conductor changes | validates routing contracts |
| PR run / issue-mode workflow | `bash adl/tools/test_pr_run_issue_mode.sh` | `cli_workflow` | medium | ordinary PR lane for PR lifecycle tooling | representative issue binding proof |
| PR finish relative-path guard | `bash adl/tools/test_pr_finish_relative_card_paths.sh` | `cli_workflow` | medium | ordinary PR lane for finish-path changes | catches publication-path regressions |
| Docs command truth | `bash tools/check_release_notes_commands.sh` | `docs` | low | docs or release-tail PR lane | command-surfaces truth check |
| Legacy-swarm reference guardrail | `bash adl/tools/check_no_new_legacy_swarm_refs.sh` | `docs` | low | ordinary PR lane when docs/publication language changes | publication-boundary and stale-term guardrail |
| PR closing linkage guardrail | `bash adl/tools/check_pr_closing_linkage.sh` | `docs` | low | ordinary PR lane for issue/PR truth, not only docs changes | issue/PR truth surface |
| v0.91.3 proof validation lane | `bash adl/tools/run_v0913_proof_validation_lane.sh` | `integration_worktree` | high | not ordinary unless proof surfaces changed | composed milestone proof lane |
| Demo smoke lane | `bash adl/tools/demo_smoke_v07_story.sh` | `integration_worktree` | medium | conditional ordinary PR lane for demo surfaces | bounded composed demo proof |
| Authoritative coverage lane | `bash adl/tools/run_authoritative_coverage_lane.sh` | `release_gate` | high | no, except policy-authority cases | full or bounded authoritative coverage authority |
| Local authoritative coverage gate | `bash adl/tools/run_local_authoritative_coverage_gate.sh` | `release_gate` | high | local heavy proof only | release-tail or explicit coverage-governance lane |
| UTS benchmark runner | `bash adl/tools/run_uts_benchmark.sh` | `provider_live` | high | never ordinary PR CI | live benchmark/provider dependency |

## Initial lane groups for the first PVF runner fixture

### `fast_unit`

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --doc`
- `bash adl/tools/run_pr_fast_test_lane.sh`

### `contract_schema_card`

- `bash adl/tools/validate_structured_prompt.sh`
- `bash adl/tools/test_prompt_templates_1_0_0.sh`
- `bash adl/tools/test_structured_prompt_validation.sh`

### `docs`

- `bash tools/check_release_notes_commands.sh`
- `bash adl/tools/check_no_new_legacy_swarm_refs.sh`
- `bash adl/tools/check_pr_closing_linkage.sh`

### `cli_workflow`

- `bash adl/tools/test_ci_path_policy.sh`
- `bash adl/tools/test_check_coverage_impact.sh`
- `bash adl/tools/test_sprint_conductor_helpers.sh`
- `bash adl/tools/test_workflow_conductor_skill_contracts.sh`
- `bash adl/tools/test_pr_run_issue_mode.sh`
- `bash adl/tools/test_pr_finish_relative_card_paths.sh`

### `integration_worktree`

- `bash adl/tools/run_v0913_proof_validation_lane.sh`
- `bash adl/tools/demo_smoke_v07_story.sh`

### `release_gate`

- `bash adl/tools/run_authoritative_coverage_lane.sh`
- `bash adl/tools/run_local_authoritative_coverage_gate.sh`

### `provider_live`

- `bash adl/tools/run_uts_benchmark.sh`

## Gaps and migration notes

### Gap 1: large unclassified contract-script tail

The repo contains many additional `adl/tools/test_*.sh` scripts, especially
skill-contract checks, demo packets, and milestone proof surfaces. They are not
yet fully classified in this first pass.

Migration note:
- `#3402` should let the runner accept partial manifests without requiring the
  full repo inventory on day one.
- Follow-on inventory work should classify remaining contract suites by
  ownership domain rather than by filename prefix alone.

### Gap 2: release-gate versus integration-worktree boundary still needs policy tightening

Some milestone/demo proof packets are heavier than ordinary `cli_workflow`, but
lighter than full release evidence. This first pass places only the clearest
surfaces into `integration_worktree` and `release_gate`.

Migration note:
- `#3403` should encode the escalation rule explicitly in CI/release logic.

### Gap 3: live/provider surfaces must remain outside ordinary PR CI

The `run_uts_benchmark.sh` lane is clearly provider/live. Additional benchmark,
demo, or hosted execution surfaces may belong here too, but they were not
expanded in this initial pass.

Migration note:
- Future PVF policy should require explicit operator or release authority before
  any `provider_live` lane is selected.

### Gap 4: path-policy outputs are not yet emitted as PVF manifest entries

Current CI already computes stable policy signals such as `coverage_lane`,
`coverage_authority`, and `reason`, but it does not yet emit a PVF manifest.

Migration note:
- `#3402` should consume this inventory into a first manifest-driven runner.
- `#3403` should wire CI classification to PVF lane selection rather than
  duplicating lane logic in multiple shell scripts.

## Manifest-shape notes

- The initial manifest keeps docs-oriented guardrails as separate entries rather
  than collapsing them into one synthetic lane. That preserves their different
  trigger semantics for later runner work.
- The `contract_schema_card` lane must trigger on PVF schema and manifest files
  themselves, not only on `.adl` card or prompt-template surfaces.

## Non-claims

- This packet does not claim the full repo test inventory is complete.
- This packet does not weaken existing validation requirements.
- This packet does not authorize provider/live tests in ordinary PR CI.
- This packet does not replace authoritative release evidence.
