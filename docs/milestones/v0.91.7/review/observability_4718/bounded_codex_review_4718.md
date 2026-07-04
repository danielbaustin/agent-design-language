The proof harness works for the default location, but its configurable output directory produces misleading retained artifact references. This should be fixed before relying on the generated proof packet.

Review comment:

- [P2] Use the requested output directory in proof references — `adl/tools/test_pr_v0917_integrated_observability_proof.sh`
  When this script is invoked with a custom `OUT_DIR` (as the new test does), the generated event sample still records `artifact_ref` under the default repo `.../generated/proof_summary.json` path instead of the actual summary file written to the requested directory. This makes the retained evidence point at a stale or nonexistent artifact for custom-output runs; the hard-coded generated paths in the summary should be derived from `OUT_DIR` as well.

## Disposition

- Status: fixed before PR publication.
- Fix: `adl/tools/test_pr_v0917_integrated_observability_proof.sh` now derives the provider `artifact_ref`, proof-summary path, and event-sample path from the requested `OUT_DIR`; temporary verifier runs render those retained refs as `<tmp>`.
- Regression proof: `adl/tools/test_pr_v0917_integrated_observability_proof_contract.sh` now asserts `artifact_ref=<tmp>` and `<tmp>` summary paths for custom-output runs.
- Rerun: `bash adl/tools/test_pr_v0917_integrated_observability_proof_contract.sh` passed after the fix.

## Second Review

The second bounded Codex review was run after the validation-lane selector
mapping became part of the #4718 change set.

Review comment:

- [P2] Anchor event-field matching in the proof - `adl/tools/test_pr_v0917_integrated_observability_proof.sh`
  The proof summary used a loose `command=(...)` regex, so a field such as
  `subcommand=doctor` could be counted as a real command value and weaken the
  retained command evidence.

## Second Review Disposition

- Status: fixed before PR publication.
- Fix: `adl/tools/test_pr_v0917_integrated_observability_proof.sh` now parses
  whitespace-delimited event key/value fields before building
  `commands_observed` and `results_observed`.
- Regression proof: `adl/tools/test_pr_v0917_integrated_observability_proof_contract.sh`
  now asserts that `doctor` is not present in `commands_observed` while `pr.sh`
  remains present.
- Rerun: `bash adl/tools/test_pr_v0917_integrated_observability_proof_contract.sh` and
  `bash adl/tools/test_pr_v0917_integrated_observability_proof.sh` passed after the
  fix; regenerated `proof_summary.json` no longer records `doctor` as a
  command.
