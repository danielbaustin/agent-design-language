# Intelligence Metric Architecture

## Metadata

- Feature Name: Intelligence Metric Architecture
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-10
- Source Docs: `.adl/docs/TBD/intelligence/`
- Proof Modes: architecture, fixtures, report
- Proof Route:
  - `adl/src/runtime_v2/intelligence_metric_architecture.rs`
  - `adl/src/runtime_v2/tests/intelligence_metric_architecture.rs`
  - `adl/tests/fixtures/runtime_v2/intelligence/intelligence_metric_architecture.json`
  - `docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/`

## Purpose

Define ADL intelligence metrics as evidence-bound architecture rather than
reputation, mystique, or a single scalar. The metric story should connect
runtime traces, capability tests, compression, and review surfaces.

## Scope

In scope:

- Intelligence metric architecture.
- Cognitive Compression Cost boundary where appropriate.
- Fixture report explaining limits and non-claims.
- Connection to v0.92 identity readiness without absorbing birthday work.

Out of scope:

- Universal intelligence claims.
- Punitive productivity scoring.
- Public ranking without review context.

## Acceptance Criteria

- Metrics derive from explicit traces or test artifacts.
- Limitations are visible in reports.
- The architecture can feed v0.92 without becoming an identity substitute.

## Landed Slice

WP-10 now lands the first bounded intelligence metric architecture packet over
the already-landed WP-09 capability harness and WP-08 Theory-of-Mind packet.
The slice keeps intelligence interpretation evidence-bound instead of turning
it into a scalar leaderboard or identity claim.

The landed packet exposes:

- explicit evidence surfaces from capability and ToM artifacts
- three bounded metric dimensions:
  - contracted capability evidence
  - uncertainty preservation
  - Cognitive Compression Cost
- a machine-readable scorecard fixture
- a human-readable fixture report that states what the metric does and does not prove

## Landed Artifacts

- `adl/src/runtime_v2/intelligence_metric_architecture.rs`
- `adl/src/runtime_v2/tests/intelligence_metric_architecture.rs`
- `adl/tests/fixtures/runtime_v2/intelligence/intelligence_metric_architecture.json`
- `docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/scorecard.json`
- `docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/final_report.md`

## Proof Notes

- WP-10 reuses the landed WP-09 capability review bundle instead of inventing a new benchmark lane.
- Cognitive Compression Cost remains exploratory and explicitly rejects productivity punishment, public ranking, and universal-intelligence framing.
- The fixture report stays internal and does not claim birthday, identity completion, or production certification.
