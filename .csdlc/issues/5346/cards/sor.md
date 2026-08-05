# Structured Output Record

Template: 1.0.0

Issue: 5346

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Executed WP-13 #5346 by deleting the exact reviewed legacy ADL CLI tooling surfaces while preserving #5347 external-band scope and recording truthful serialized merge state. The substantive deletion commit e9035ceff removes 58 exact manifest rows under adl/src/cli/tooling_cmd and adl/src/cli/tests/pr_cmd_inline; the exact commit accounting is 1,880 additions, 46,502 deletions, net -44,622. The #5347 external deletion manifest is a durable #5346 evidence input and is proven disjoint by exact path, prefix, symlink-target, and Cargo membership checks. #5347 remains GitHub-open and must merge before #5346 merge; that is a serialized merge gate, not a publication blocker.

## Artifacts

- docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json
- docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json
- .csdlc/evidence/5346/5347-external-band-deletion-manifest.json
- .csdlc/prepared/issues/5346/check-dependencies.rb
- .csdlc/prepared/issues/5346/run-validation-lane.rb
- .csdlc/prepared/issues/5346/write-deletion-evidence.rb
- adl/src/cli/tooling_cmd
- adl/src/cli/tests/pr_cmd_inline

## Execution

- Deleted the #5346-owned sunset ADL CLI tooling module tree adl/src/cli/tooling_cmd.
- Deleted the #5346-owned sunset inline PR command test tree adl/src/cli/tests/pr_cmd_inline.
- Added durable #5346 deletion eligibility and post-deletion validation evidence with execution revision and exact git diff accounting.
- Added the #5347 external-band manifest as a durable #5346 evidence input and made check-dependencies.rb prove concrete #5346/#5347 disjointness instead of emitting a constant true.
- Recorded live #5347 state truthfully as open/no PR with #5347-first serialized merge required before #5346 merge.
- Kept reviewed_revision null pending the fresh GPT-5.5 rereview.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5346/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "purpose": "Validate the exact reviewed #5346 head after serialized #5347 integration, including manifest identity, dependency ancestry, reduction accounting, and offline compilation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5346/post-merge-exact.log"
  },
  {
    "command": [
      "/usr/bin/ruby",
      ".csdlc/prepared/issues/5346/check-dependencies.rb"
    ],
    "purpose": "Validate terminal upstreams, live #5354/#5352 closure, live #5347 open state, durable #5347 manifest copy, and exact path/prefix/symlink/Cargo disjointness.",
    "outcome": "passed",
    "evidence_ref": "dependency-disjointness.log"
  },
  {
    "command": [
      "/opt/homebrew/bin/git",
      "diff",
      "--check",
      "HEAD"
    ],
    "purpose": "Confirm the corrected #5346 diff has no whitespace errors.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
