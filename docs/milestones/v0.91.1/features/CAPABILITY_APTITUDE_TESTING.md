# Capability And Aptitude Testing

## Metadata

- Feature Name: Capability and Aptitude Testing Foundation
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-09
- Source Docs: `.adl/docs/TBD/capability_testing/`
- Proof Modes: harness, fixtures, report
- Proof Route:
  - `adl/src/capability_aptitude_testing.rs`
  - `adl/src/bin/demo_v0911_capability_aptitude_testing.rs`
  - `docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/`

## Purpose

Create the first executable capability and aptitude testing foundation for
models, skills, and agents. This is the ADL path toward Aptitude Atlas without
turning evaluation into benchmark theater.

## Scope

In scope:

- First executable test harness slice.
- Report and scorecard shape.
- Initial test-family fixtures.
- Clear limits on what each result proves.

Out of scope:

- Public leaderboard launch.
- Universal intelligence ranking.
- Production certification.

## Acceptance Criteria

- Fixture-mode output is deterministic.
- Reports distinguish evidence from reputation.
- Later product work can consume the harness without redefining evaluation.

## Landed Slice

WP-09 now lands the first deterministic fixture-mode capability/aptitude
harness bundle for ADL. The slice covers the first three families called for in
the source planning set:

- contract following
- review aptitude
- planning aptitude

The landed harness emits:

- `subject_manifest.json`
- `test_manifest.json`
- `run_manifest.json`
- `scorecard.json`
- `final_report.md`
- `evaluator_notes.md`
- `redaction_report.md`
- `raw_outputs/*.json`

The report packet is explicitly internal-only and carries non-claims against
universal intelligence ranking, public leaderboard publication, and production
certification.
