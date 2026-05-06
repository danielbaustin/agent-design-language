# UTS + ACC Multi-Model Benchmark

## Metadata

- Feature Name: UTS + ACC Multi-Model Benchmark
- Milestone Target: `v0.91.2`
- Status: planned
- Planned WP Home: WP-02 and WP-03
- Source Docs: `.adl/docs/TBD/tools/UTS_ACC_MULTI_MODEL_BENCHMARK_PLAN.md`
- Proof Modes: harness, fixtures, report

## Purpose

Test whether local and hosted models can stay inside ADL's governed tool-use
discipline: model output proposes tool use, UTS describes portable tool shape,
and ACC owns authority, visibility, redaction, trace, replay, and mediation.

## Scope

In scope:

- Fixture benchmark harness.
- Safe-read, bounded-write, missing-authority, destructive-action,
  exfiltration, ambiguity, injection, and correction cases.
- ADL JSON proposal mode vs provider-native tool/function-call comparison.
- Model scorecards and non-claims.

Out of scope:

- Real destructive tool execution.
- Production secrets.
- Treating UTS validity as execution authority.

## Acceptance Criteria

- Harness never executes real tools from model output.
- Reports separate proposal behavior from ACC authority.
- Skips, credentials, costs, and model identifiers are recorded.
