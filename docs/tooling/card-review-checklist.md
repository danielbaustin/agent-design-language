# ADL Card Review Checklist Spec

Status: Draft (v0.8 tooling)
Applies to: ADL output-card review
Primary issue: #650

## Purpose

Define a deterministic, machine-readable checklist for reviewing ADL output cards.
This checklist is the canonical rule surface for:

- human reviewers
- Card Reviewer GPT behavior (#649)
- deterministic review output artifacts (#651)
- future CI parsing and enforcement

## Scope

In scope:
- output card structural validity
- acceptance-criteria verification
- determinism and replay assertions
- security/privacy hygiene checks
- artifact and validation evidence checks
- deterministic decision mapping (`PASS`, `MINOR_FIXES`, `MAJOR_ISSUES`)

Out of scope:
- CI enforcement implementation
- runtime feature validation beyond card evidence
- policy changes to card schema

## Review Inputs

Required inputs:
- issue input card (`.adl/cards/<issue>/input_<issue>.md`)
- issue output card (`.adl/cards/<issue>/output_<issue>.md`)
- changed files listed in output card

Optional inputs:
- PR diff and command logs
- generated artifacts referenced by output card

## Deterministic Review Rules

Rule ordering is fixed and must not be changed without versioning this spec.
Reviewers MUST evaluate rules in the following domain order:

1. `structure`
2. `acceptance`
3. `determinism`
4. `security_privacy`
5. `artifacts`
6. `validation`

Within each domain, evaluate rules by ascending `rule_id`.

## Rule Catalog

Each rule has:
- `rule_id`: stable identifier
- `domain`: fixed domain name
- `severity`: `blocker | high | medium | low`
- `check`: deterministic condition
- `evidence`: required evidence source

### Structure Domain

- `CRS-STR-001` (`high`): Output card has required top sections from template.
- `CRS-STR-002` (`high`): `Status` is a valid terminal or in-progress value and matches narrative state.
- `CRS-STR-003` (`medium`): `Execution` metadata is present and non-placeholder.
- `CRS-STR-004` (`high`): `Artifacts produced` contains explicit repo-relative paths.

### Acceptance Domain

- `CRS-ACC-001` (`blocker`): All input-card acceptance criteria are explicitly addressed.
- `CRS-ACC-002` (`high`): Any unmet criterion is listed under deviations/follow-ups.
- `CRS-ACC-003` (`medium`): Scope constraints/non-goals are respected.

### Determinism Domain

- `CRS-DET-001` (`blocker`): Determinism assertions are present for applicable changes.
- `CRS-DET-002` (`high`): Ordering/tie-break behavior is explicit where relevant.
- `CRS-DET-003` (`high`): Replay compatibility impact is stated (`unchanged` or explicit delta).

### Security/Privacy Domain

- `CRS-SEC-001` (`blocker`): No secrets/tokens in output card text.
- `CRS-SEC-002` (`blocker`): No raw prompts/tool arguments leaked in evidence.
- `CRS-SEC-003` (`high`): No absolute host paths in persisted examples/evidence.
- `CRS-SEC-004` (`high`): Security-sensitive deltas include explicit safety statement.

### Artifacts Domain

- `CRS-ART-001` (`high`): Required artifacts from task are listed and present (or justified).
- `CRS-ART-002` (`high`): Artifact schema/version changes are explicit and approved.
- `CRS-ART-003` (`medium`): Missing optional artifacts have rationale.

### Validation Domain

- `CRS-VAL-001` (`blocker`): Required validation commands from input card were executed.
- `CRS-VAL-002` (`high`): Validation results are explicit (`PASS/FAIL`) and consistent.
- `CRS-VAL-003` (`medium`): Failed/skipped checks include deterministic rationale.

## Decision Mapping

Decision mapping is deterministic and rule-based:

- `MAJOR_ISSUES`:
  - any failed `blocker` rule, OR
  - 2+ failed `high` rules
- `MINOR_FIXES`:
  - no failed `blocker` rules, AND
  - 1 failed `high` rule OR any failed `medium/low` rule
- `PASS`:
  - no failed `blocker/high/medium` rules

If two outcomes appear possible, choose the stricter outcome.

## Evidence Requirements

Each finding must include at least one evidence pointer:
- file path
- command
- CI check
- artifact path

Narrative-only claims without evidence should be marked as `needs_evidence` and fail the relevant rule.

## Example Checklist Application (Real ADL Card)

Review target:
- issue `#660` output card (`.adl/cards/660/output_660.md`)

Example outcome summary:
- structure: pass
- acceptance: pass
- determinism: pass (docs-only, replay unchanged)
- security/privacy: pass (no host paths/secrets)
- artifacts: pass
- validation: pass (`cargo fmt`, `cargo clippy`, `cargo test` documented)

Result:
- decision: `PASS`
- failed rules: `[]`

## Versioning

Checklist version: `card_review_checklist.v1`

Future updates must:
- preserve existing `rule_id` semantics, OR
- version bump with explicit migration notes
