# Model Suitability Selection Proof for #4673

Status: `implemented_with_integrated_scheduler_proof`

Issue: `#4673`

## Scope

This packet records the v0.91.7 WP-05 model-suitability selection proof.

The issue implements a deterministic scheduler-consumable model-suitability
context. It does not run new live provider probes. It consumes retained
v0.91.6 provider/model suitability evidence and proves that scheduler decisions
can attach a bounded model selection for role-specific work while preserving
the advisory-only authority boundary.

## Implemented Surfaces

- `adl/src/scheduler.rs`
  - Adds `model_suitability_context` to scheduler economics bundles.
  - Adds schema-gated bundle version
    `adl.scheduler.economics_input_bundle.model_suitability.v1`.
  - Adds schema-gated decision version
    `adl.scheduler.decision.model_suitability.v1`.
  - Validates retained evidence refs, candidates, role requirements,
    classification thresholds, explicit selection priority, retained evidence
    refs, advisory-only authority, duplicate candidates, duplicate task
    requirements, and task-to-input alignment.
  - Selects the highest-ranked eligible model deterministically by
    classification, explicit selection priority, provider profile, model ref,
    and candidate id.
  - Emits a per-decision `model_suitability_selection` only for tasks with a
    model-suitability requirement.

- `adl/src/chronosense/long_running_proof.rs`
  - Preserves direct scheduler-bundle construction by explicitly setting
    `model_suitability_context: None`.

- `adl/tests/fixtures/scheduler/model_suitability_inputs_v1.json`
  - Retained fixture consuming v0.91.6 suitability evidence.

- `docs/milestones/v0.91.7/review/provider/artifacts/model_suitability_plan_4673.json`
  - CLI-generated scheduler plan proving the integrated path.

## Evidence Consumed

- `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`
- `docs/milestones/v0.91.6/review/provider/openrouter_current_models/openrouter_current_model_suitability_state_2026-06-22.json`
- `docs/milestones/v0.91.6/review/provider/gemini_current_models/gemini_current_model_suitability_state_2026-06-18.json`
- Historical Gemma watcher evidence remains visible as historical-only and is
  not promoted into a current suitable reviewer lane.

## Proof Result

The retained fixture asks the scheduler to select a reviewer-capable model for
`first-pass-review` with minimum classification `useful_with_limits`.

The generated scheduler plan selects:

- role: `reviewer`
- candidate: `openrouter:gpt-5.4`
- provider profile ref: `unprofiled:openrouter:openai/gpt-5.4`
- model ref: `openai/gpt-5.4`
- classification: `useful_with_limits`
- claim boundary: `bounded_role_suitability_not_authority`
- advisory authority only: `true`

The same plan leaves `docs-status-check` on the ordinary scheduler decision
schema with no model suitability selection, proving the feature is task-scoped
instead of globally altering every scheduler decision.

## Validation

Local validation run from the #4673 worktree:

```text
cargo fmt --manifest-path adl/Cargo.toml --all -- --check
cargo test --manifest-path adl/Cargo.toml scheduler::tests::model_suitability --lib -- --nocapture
cargo build --manifest-path adl/Cargo.toml --bin adl
ADL_OBSERVABILITY_LOG=$TMPDIR/adl-4673-model-suitability.log adl/target/debug/adl scheduler plan --input adl/tests/fixtures/scheduler/model_suitability_inputs_v1.json --out docs/milestones/v0.91.7/review/provider/artifacts/model_suitability_plan_4673.json --json
python3 - <<'PY'
import json, pathlib
p = pathlib.Path('docs/milestones/v0.91.7/review/provider/artifacts/model_suitability_plan_4673.json')
data = json.loads(p.read_text())
assert data['schema_version'] == 'adl.scheduler.plan.v1'
assert data['source_schema_version'] == 'adl.scheduler.economics_input_bundle.model_suitability.v1'
review = next(d for d in data['decisions'] if d['task_id'] == 'first-pass-review')
assert review['schema_version'] == 'adl.scheduler.decision.model_suitability.v1'
selection = review['model_suitability_selection']
assert selection['role'] == 'reviewer'
assert selection['selected_candidate_id'] == 'openrouter:gpt-5.4'
assert selection['classification'] == 'useful_with_limits'
assert selection['advisory_authority_only'] is True
docs = next(d for d in data['decisions'] if d['task_id'] == 'docs-status-check')
assert docs['schema_version'] == 'adl.scheduler.decision.v1'
assert 'model_suitability_selection' not in docs
PY
```

Result:

- formatting: `PASS`
- focused scheduler model-suitability tests: `PASS` (`4 passed`)
- scheduler CLI proof: `PASS`
- JSON contract assertion: `PASS`

## Negative Cases

Unit tests prove:

- old scheduler bundle schema rejects `model_suitability_context`;
- candidate entries must preserve `advisory_authority_only`;
- candidate source refs must be included in retained `evidence_refs`;
- overclaiming claim boundaries such as `unbounded_authority` fail closed;
- a task requirement with no eligible candidate fails closed;
- tasks without model-suitability requirements retain the ordinary decision
  schema.

## Non-Claims

- This does not grant any model merge, closeout, release, file-write, or
  workflow authority.
- This does not benchmark general intelligence or broad coding skill.
- This does not claim live-provider replay stability.
- This does not replace provider-profile selection from `#4672`, cheapest
  validated outcome policy from `#4674`, or local-agent delegation readiness
  from `#4675`.
