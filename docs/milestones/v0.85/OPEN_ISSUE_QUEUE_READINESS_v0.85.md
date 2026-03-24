# Open Issue Queue Readiness v0.85

## Purpose

This note records the mechanics cleanup completed for the remaining open
v0.85/current-queue issues so they are ready for review, editing, and
execution without redoing queue hygiene by hand.

This is a tracked summary of queue readiness. The canonical cards and issue
prompts live under the local `.adl/` tree.

## Scope

The cleanup covered these open issues:

- `#881`
- `#882`
- `#886`
- `#901`
- `#902`
- `#903`
- `#982`
- `#1009`
- `#1012`
- `#1013`
- `#1014`
- `#1015`
- `#1016`
- `#1017`
- `#1018`
- `#1019`
- `#1020`
- `#1021`
- `#1022`
- `#1023`
- `#1024`

## Mechanics Brought Up To Standard

- Canonical issue-body files exist under `.adl/issues/v0.85/bodies/` for the
  new Rust and editing queues (`#1012-#1024`).
- Canonical input/output cards exist under `.adl/cards/<issue>/` for the full
  open queue listed above.
- The active queue cards were normalized to repository-relative references.
- Absolute host-path leakage was removed from the active queue cards.
- Bootstrap validation now passes for both the SIP and SOR cards of every
  issue in scope.
- Repo-local execution clones exist under `.worktrees/adl-wp-<issue>` for every issue in scope.
- The Rust queue issues `#1012-#1015` were corrected from stale
  `version:v0.3` labels to `version:v0.85`.

## Validation Snapshot

The following local validations passed for every issue in scope:

- `bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input .adl/cards/<issue>/input_<issue>.md`
- `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/cards/<issue>/output_<issue>.md`

Additional queue-mechanics checks that passed:

- issue-body existence under `.adl/issues/v0.85/bodies/`
- repo-local execution-clone existence under `.worktrees/adl-wp-<issue>`
- no `/Users/`, `/private/`, or `/tmp/` path leakage in the active queue cards
- corrected `version:v0.85` labels on `#1012-#1015`

## Remaining Truth

- The `.adl/` tree remains local/ignored; these queue mechanics are local
  authoring state rather than tracked repository content.
- This cleanup does not implement the queued work itself. It modernizes the
  issue surfaces so those issues can be executed cleanly.
- Any issue-specific scope changes should still be reviewed in the cards before
  implementation begins.

## Result

The remaining open v0.85/current-queue issues are mechanically modernized and
ready to pull the trigger on, without needing another round of card/path/
worktree/metadata cleanup first.
