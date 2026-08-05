# Structured Output Record

Template: 1.0.0

Issue: 5746

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Materialized fifty receipt-authoritative v0.91.8 terminal projections through typed C-SDLC v2 reconciliation.

## Artifacts

- .csdlc/issues/4739 through the declared fifty-issue projection set
- .csdlc/evidence/5746

## Execution

- Projected fifty retained terminal receipts into tracked .csdlc issue records
- Excluded unsupported or inconsistent cases for separate recovery in #5748

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...a12e9212d"
    ],
    "purpose": "Prove the aggregate terminal projection commit has no whitespace errors",
    "outcome": "passed",
    "evidence_ref": "aggregate-diff-hygiene.log"
  },
  {
    "command": [
      "/bin/bash",
      "-lc",
      "git diff --name-only origin/main...a12e9212d | awk 'BEGIN{ok=1} !/^\\.csdlc\\/issues\\/(4739|4741|4758|4761|4762|5107|5338|5340|5341|5343|5344|5345|5349|5350|5361|5384|5497|5498|5500|5501|5502|5526|5563|5589|5590|5592|5594|5605|5613|5615|5624|5627|5648|5653|5658|5666|5671|5683|5686|5691|5695|5697|5698|5702|5710|5715|5717|5719|5727|5737)\\// {print; ok=0} END{exit !ok}'"
    ],
    "purpose": "Prove the aggregate commit contains only the declared fifty issue projections",
    "outcome": "passed",
    "evidence_ref": "aggregate-scope.log"
  },
  {
    "command": [
      "/bin/bash",
      "-lc",
      "for i in 4739 4741 4758 4761 4762 5107 5338 5340 5341 5343 5344 5345 5349 5350 5361 5384 5497 5498 5500 5501 5502 5526 5563 5589 5590 5592 5594 5605 5613 5615 5624 5627 5648 5653 5658 5666 5671 5683 5686 5691 5695 5697 5698 5702 5710 5715 5717 5719 5727 5737; do /Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue $i >/dev/null || exit 1; done"
    ],
    "purpose": "Run C-SDLC v2 doctor for every included terminal projection",
    "outcome": "passed",
    "evidence_ref": "terminal-doctor-sweep.log"
  },
  {
    "command": [
      "/bin/bash",
      "-lc",
      "for i in 4739 4741 4758 4761 4762 5107 5338 5340 5341 5343 5344 5345 5349 5350 5361 5384 5497 5498 5500 5501 5502 5526 5563 5589 5590 5592 5594 5605 5613 5615 5624 5627 5648 5653 5658 5666 5671 5683 5686 5691 5695 5697 5698 5702 5710 5715 5717 5719 5727 5737; do jq -e -s '.[0].record == .[1]' \"$(git rev-parse --git-common-dir)/csdlc-v2/closeout/$i.json\" .csdlc/issues/$i/index.json >/dev/null || exit 1; done"
    ],
    "purpose": "Compare every tracked index byte-for-byte with retained terminal authority",
    "outcome": "passed",
    "evidence_ref": "terminal-receipt-equality.log"
  },
  {
    "command": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "<declared-50@c4ef77c46>"
    ],
    "purpose": "Revalidated C-SDLC doctor for all fifty receipt-authoritative projections at c4ef77c46.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5746/current-terminal-doctor-sweep.log"
  },
  {
    "command": [
      "jq",
      "-e",
      "-s",
      "<receipt.record == tracked index for declared-50@c4ef77c46>"
    ],
    "purpose": "Revalidated all fifty tracked indexes against retained terminal receipt records at c4ef77c46.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5746/current-terminal-receipt-equality.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--name-only",
      "origin/main...c4ef77c46"
    ],
    "purpose": "Revalidated that c4ef77c46 contains only the declared fifty terminal projection directories.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5746/current-aggregate-scope.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...c4ef77c46"
    ],
    "purpose": "Revalidated whitespace and patch hygiene for the rebased fifty-projection commit c4ef77c46.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5746/current-aggregate-diff-hygiene.log"
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
