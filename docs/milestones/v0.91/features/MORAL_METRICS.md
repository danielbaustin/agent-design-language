# Moral Metrics

## Milestone Boundary

This v0.91 feature defines bounded moral metrics derived from WP-04 moral
traces and WP-05 outcome linkages. The feature exists to support review and
trend detection over explicit evidence, not to turn moral governance into a
public scoreboard.

It does not claim final moral judgment, production moral agency, scalar karma,
scalar happiness, public reputation, or replacement of later trajectory review
and anti-harm work. It is a decomposed diagnostic layer that helps reviewers
notice drift, uncertainty debt, and accountability gaps over time.

WP-06 owns the metric definitions and fixture report. WP-07 and WP-08 consume
the outputs.

## Purpose

Moral metrics answer a narrower question than "is the agent good?".

They must instead show:

- whether moral traces preserve reviewable paths
- whether delegated outcomes keep accountability lineage
- how much unresolved outcome review load is accumulating
- how those signals trend over repeated review windows

The key boundary is interpretive humility: the metrics are signals, not
verdicts.

## Contract Shape

```yaml
moral_metric_fixture_report:
  schema_version: moral_metric_fixture_report.v1
  report_id: stable_report_id
  summary: reviewer_safe_summary
  interpretation_boundary: >
    These metrics are review signals only. They are not a scalar karma score,
    not a scalar happiness score, and not a public reputation system.
  definitions:
    - schema_version: moral_metric_definition.v1
      metric_id: stable_metric_id
      display_name: reviewer_safe_metric_name
      purpose: what_the_metric_tracks
      measurement_kind: ratio | count
      evidence_field_refs:
        - moral_trace.* or outcome_linkage.* field paths only
      trend_detection_summary: how_to_read_change_over_time
      interpretation_boundary: review-signal-only explanation
      limitations:
        - bounded caveat
  fixtures:
    - fixture_id: stable_fixture_id
      summary: review_window_summary
      input_trace_refs:
        - trace:trace_id
      input_outcome_linkage_refs:
        - outcome-linkage:linkage_id
      observations:
        - metric_id: stable_metric_id
          observed_window_ref: review-window:window_id
          numerator: integer
          denominator: optional_integer
          value_summary: reviewer_safe_output_summary
          evidence_refs:
            - moral_trace.* or outcome_linkage.* field paths only
          limitations:
            - bounded caveat
      limitations:
        - bounded caveat
```

## Field Rules

- Every definition must derive from explicit `moral_trace.*` or
  `outcome_linkage.*` evidence fields.
- Definitions and value summaries must avoid scoreboard framing such as moral
  score, karma score, happiness score, or public reputation language.
- The report-level `interpretation_boundary` must explicitly reject scalar
  karma, scalar happiness, and public reputation framing.
- Metrics remain decomposed; there is no overall moral score field.
- Fixture observations must name their input traces and outcome linkages so a
  reviewer can inspect the exact derivation surface.
- Limitations are mandatory for both definitions and fixture outputs.

## Initial v0.91 Metric Set

WP-06 lands three bounded metrics:

1. `trace-review-path-coverage`
2. `delegation-lineage-retention`
3. `unresolved-outcome-attention-count`

Together they prove that ADL can derive reviewable signals from the trace
surface without converting those signals into a scalar moral ranking.

## Fixture Report

The tracked runtime fixture report lives in
`adl/src/runtime_v2/moral_metrics.rs` and combines the required WP-04 and
WP-05 examples into one bounded review window.

The report proves:

- review-path coverage can be computed directly from trace evidence
- delegation lineage retention can be computed directly from trace and linkage
  attribution fields
- unresolved outcome attention can be counted without pretending uncertainty
  is solved

The fixture report is intentionally small. It proves derivation rules and
review framing, not production distributions.

## Non-Claims

This feature does not claim:

- a single scalar karma score
- a single scalar happiness score
- public reputation or surveillance ranking
- final moral judgment
- replacement of trajectory review or anti-harm review
- production moral agency, v0.92 birthday semantics, or v0.93 constitutional
  governance

It claims a narrower result: ADL has an executable, reviewer-safe moral-metric
surface that derives bounded signals from trace evidence while preserving
interpretive humility.
