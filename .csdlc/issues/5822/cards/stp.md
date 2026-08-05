# Structured Task Prompt

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver measured estimation, reconnection, and simplified lifecycle path.

## Deliverables

- Typed provenance-bearing observation, forecast, accepted-estimate, and outcome schemas
- Deterministic multi-source join with explicit unknown, interruption, and schema-drift handling
- Comparable-issue selection, sparse-data baseline, uncertainty, confidence, and drift reporting
- Typed SPP advisory estimate reference and operator disposition without enforcement
- Terminal forecast-versus-actual record and retained calibration/backtest report
- Measured baseline-versus-candidate operator cycle-time and reconnection proof

## Acceptance

1. Typed observation, forecast, accepted-estimate, and outcome schemas retain per-field provenance and explicit unknown values
2. Lifecycle, GitHub, PVF/validation, approved session, and operator-annotation adapters join deterministically without transcript-content leakage
3. Comparable cohorts exclude target actuals and forecasts report ranges, cohort size, dispersion, confidence, outlier factors, and drift state
4. Static PlanningProfile estimates remain an explicit fallback when data sufficiency or calibration gates fail
5. A typed SPP edit records advisory estimate source and operator disposition without enforcing time, tokens, completion, or phase
6. Terminal closeout records forecast-versus-actual evidence and supports reproducible backtest and calibration reports
7. Equivalent baseline and candidate cohorts demonstrate truthful operator cycle-time and reconnection behavior without gate weakening
8. Focused schema, negative, deterministic, privacy, lifecycle, and exact-revision review proof passes

## Dependencies

- WP-02A issue #5801 complete with stable CI/PVF timing and lifecycle topology
- Retained v0.91.7 session telemetry and deterministic prediction evidence remains readable
- Current C-SDLC v2 card, validation, review, publication, and closeout schemas are authoritative

## Inputs

- .csdlc/prepared/issues/5822/design.md
- csdlc-v2/src/cards.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/finish.rs
- adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_from_codex_session.py
- adl/tools/skills/sprint-conductor/scripts/write_issue_goal_metrics_report.py
- adl/tools/skills/sprint-conductor/scripts/predict_issue_execution_metrics.py
- docs/milestones/v0.91.7/review/V0917_WP04_CODEX_SESSION_TELEMETRY_SAMPLE_4617.prediction.json
- docs/milestones/v0.91.7/review/V0917_WP04_EXECUTION_METRICS_PREDICTION_SAMPLE_4743.json

## Non Goals

- Estimate-based token or time limits, scheduler deadlines, or kill switches
- Restoration of sunset v1 lifecycle commands
- Treating session transcripts as complete ground truth
- Predictive-accuracy claims from one sample or plumbing fixture
- Weakening validation, review, publication, merge, or closeout gates
- Cross-repository analytics platform or raw transcript publication
