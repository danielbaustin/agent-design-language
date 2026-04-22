# Cognitive Compression Cost v0

## Status

First practical instrumentation pass for issue #2380.

## Purpose

Cognitive Compression Cost, or CCC, is a small trace-derived effort signal. It
computes how much work a bounded ADL-style run spent turning a messy task into a
framed, explored, executed, and validated result.

This pass is deliberately humble. It proves that CCC can be computed from
stable fixture counters and explained to an operator. It does not claim pricing,
moral worth, absolute intelligence, productivity ranking, or cross-agent
normalization.

## Source Planning

- `.adl/docs/TBD/CCC_METRIC_v0.md`
- `.adl/docs/TBD/ccc/CCC_FIRST_PASS_PLAN.md`
- `.adl/docs/TBD/MILESTONE_COMPRESSION_PLAN.md`
- `.adl/docs/TBD/capability_testing/`
- `.adl/docs/TBD/economics/REVIEW_SUMMARY_SHAPE_v0.md`
- `.adl/docs/TBD/economics/EVALUATION_MODEL_v0.md`
- `docs/planning/ADL_FEATURE_LIST.md`

The `.adl/docs/TBD` inputs are local planning evidence. The tracked proof
surface for this issue is the fixture set, extractor, validation command, and
generated report.

## Fixture Shape

Fixtures live in `demos/fixtures/ccc_v0/` and use schema version
`adl.ccc.v0.fixture.v1`.

Required top-level fields:

- `schema_version`
- `run_id`
- `task_label`
- `trace_family`
- `source_note`
- `termination_reason`
- `counters`

Required counters:

- `num_reframes`
- `num_low_adequacy_events`
- `num_iterations`
- `num_retries`
- `num_steps`
- `num_tool_calls`
- `num_model_calls`
- `num_validation_failures`
- `num_contradictions`

Supported termination reasons:

- `success`
- `bounded_failure`
- `no_progress`

## Cost Formula

CCC v0 uses the draft weights from the local metric plan:

```text
framing = 3 * num_reframes + 1 * num_low_adequacy_events
exploration = 1 * num_iterations + 2 * num_retries
execution = 0.5 * num_steps + 1 * num_tool_calls + 1 * num_model_calls
residual_error = 2 * num_validation_failures + 2 * num_contradictions + termination_penalty
total = framing + exploration + execution + residual_error
```

Termination penalty:

- `success`: `0`
- `bounded_failure`: `5`
- `no_progress`: `5`

## Commands

Generate the tracked-style report:

```bash
python3 adl/tools/compute_ccc_v0.py
```

Run the validation proof:

```bash
bash adl/tools/test_ccc_v0_instrumentation.sh
```

The test generates the report twice and compares bytes, checks expected totals,
verifies missing-counter rejection, scans generated artifacts for host-local
paths, and checks that the report avoids forbidden overclaims.

## Report Outputs

The default generator writes:

- `demos/v0.90.3/ccc_v0/ccc_v0_report.json`
- `demos/v0.90.3/ccc_v0/ccc_v0_report.md`

The report includes:

- machine-readable component costs
- total CCC per fixture
- dominant cost driver
- operator-readable interpretation
- comparison summary
- caveats and claim boundaries

## Interpretation

CCC v0 explains where effort accumulated:

- framing dominance means the run spent most effort finding a workable frame
- exploration dominance means search, retries, or branching drove the cost
- execution dominance means the selected frame required many steps or
  invocations
- residual-error dominance means validation churn or contradictions drove the
  cost

The number is useful as a local inspection signal. It is not a universal score.

## Future Links

Later work may connect CCC to:

- milestone compression status surfaces
- Aptitude Atlas model/task-family evaluation
- review summaries
- contract-market evaluation
- real ADL trace ingestion

Those integrations should remain separate from this v0 fixture proof.
