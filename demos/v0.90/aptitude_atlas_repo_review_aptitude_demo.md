# Aptitude Atlas Repo-Review Aptitude Demo

Version: `v0.90`
Status: `bounded`
Issue: `2162`

## Scope

This demo defines the first bounded Aptitude Atlas prototype for one task family:
repo-review aptitude.

The intent is to compare at least two AI subjects on the same bounded review
surface and produce evidence-backed scorecards with caveats and false-positive
controls.

## Inputs

- .adl planning docs:
  - `.adl/docs/TBD/capability_testing/README.md`
  - `.adl/docs/TBD/capability_testing/MVP_BUILD_PLAN.md`
  - `.adl/docs/TBD/capability_testing/FIRST_TEST_FAMILIES.md`
  - `.adl/docs/TBD/capability_testing/REPORT_AND_SCORECARD_SPEC.md`
- Fixture package:
  - `demos/fixtures/aptitude_atlas_repo_review/`

## Deliverables

- A bounded fixture definition with:
  - one real defect
  - one tempting false-positive trap
  - one docs/release-truth wrinkle
  - one residual-risk surface
- Subject, test, and run manifest templates
- Scorecard template and final report template
- Validation protocol for manual subject execution and scoring

## Demo Command (manual protocol)

Because this is the first bounded prototype, execution is manual and evidence-first:

```bash
python3 demos/fixtures/aptitude_atlas_repo_review/validation_protocol.py
```

No live providers are required for phase-one preparation. The protocol is also
valid without provider APIs.

## What This Demo Builds (artifact contract)

- `demos/fixtures/aptitude_atlas_repo_review/fixture_definition.md`
- `demos/fixtures/aptitude_atlas_repo_review/subject_manifest_template.json`
- `demos/fixtures/aptitude_atlas_repo_review/test_manifest_template.json`
- `demos/fixtures/aptitude_atlas_repo_review/run_manifest_template.json`
- `demos/fixtures/aptitude_atlas_repo_review/scorecard_template.json`
- `demos/fixtures/aptitude_atlas_repo_review/final_report_template.md`
- `demos/fixtures/aptitude_atlas_repo_review/target_repo_validator.py`
- `demos/fixtures/aptitude_atlas_repo_review/target_repo_readme.md`
- `demos/fixtures/aptitude_atlas_repo_review/target_repo_deployment.md`

## Protocol Summary

1. Choose at least two subjects for the same family (e.g. frontier model,
   smaller model, local model, or skill-backed review team).
2. Run each subject through the same fixture content from
   `fixture_definition.md`.
3. Score each subject with `scorecard_template.json` dimensions:
   true-positive detection, false-positive restraint, severity calibration,
   evidence quality, remediation usefulness, uncertainty handling, and repair
   burden.
4. Populate `run_manifest_template.json` and `final_report_template.md`.
5. Classify proof quality as proving, non-proving, skipped, or failed.

## Validation

```bash
python3 -m json.tool demos/fixtures/aptitude_atlas_repo_review/subject_manifest_template.json
python3 -m json.tool demos/fixtures/aptitude_atlas_repo_review/test_manifest_template.json
python3 -m json.tool demos/fixtures/aptitude_atlas_repo_review/run_manifest_template.json
python3 -m json.tool demos/fixtures/aptitude_atlas_repo_review/scorecard_template.json
```

Optional checks for source alignment:

```bash
rg -n "REAL_FINDING|FALSE_POSITIVE_TRAP|DOCS_WRINKLE|RESIDUAL_RISK" demos/fixtures/aptitude_atlas_repo_review/fixture_definition.md
rg -n "subject_id|test_family|overall_band|confidence" demos/fixtures/aptitude_atlas_repo_review/scorecard_template.json
```

## Truth Boundaries

- This demo does **not** claim universal intelligence ranking.
- This demo is **not** a public leaderboard.
- This demo does **not** build the full Aptitude Atlas platform.
- This demo does **not** auto-route work; it is a bounded evidence packet.

## Proof Classification

Current expected proof state: `non_proving` (manual protocol, deterministic
template surface, no production provider orchestration).

