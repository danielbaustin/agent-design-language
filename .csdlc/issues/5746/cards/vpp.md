# Validation Planning Prompt

Template: 1.0.0

Issue: 5746

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5746/design.md

Diagram: .csdlc/prepared/issues/5746/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-doctor-sweep",
    "proof_role": "Run C-SDLC v2 doctor for every included terminal projection",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "/bin/bash",
      "-lc",
      "for i in 4739 4741 4758 4761 4762 5107 5338 5340 5341 5343 5344 5345 5349 5350 5361 5384 5497 5498 5500 5501 5502 5526 5563 5589 5590 5592 5594 5605 5613 5615 5624 5627 5648 5653 5658 5666 5671 5683 5686 5691 5695 5697 5698 5702 5710 5715 5717 5719 5727 5737; do .adl/bin/csdlc-v2/csdlc-doctor --repo . --issue $i >/dev/null || exit 1; done"
    ],
    "parallel_group": "terminal-audit",
    "defer_reason": null
  },
  {
    "lane": "terminal-receipt-equality",
    "proof_role": "Compare every included tracked index byte-for-byte with retained terminal authority",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "/bin/bash",
      "-lc",
      "for i in 4739 4741 4758 4761 4762 5107 5338 5340 5341 5343 5344 5345 5349 5350 5361 5384 5497 5498 5500 5501 5502 5526 5563 5589 5590 5592 5594 5605 5613 5615 5624 5627 5648 5653 5658 5666 5671 5683 5686 5691 5695 5697 5698 5702 5710 5715 5717 5719 5727 5737; do jq -e -s '.[0].record == .[1]' \"$(git rev-parse --git-common-dir)/csdlc-v2/closeout/$i.json\" .csdlc/issues/$i/index.json >/dev/null || exit 1; done"
    ],
    "parallel_group": "terminal-audit",
    "defer_reason": null
  },
  {
    "lane": "aggregate-scope-and-review-preflight",
    "proof_role": "Prove the aggregate commit contains only the declared fifty issue projections and freeze the bounded review scope",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "/bin/bash",
      "-lc",
      "git diff --name-only origin/main...c4ef77c46 | awk 'BEGIN{ok=1} !/^\\.csdlc\\/issues\\/(4739|4741|4758|4761|4762|5107|5338|5340|5341|5343|5344|5345|5349|5350|5361|5384|5497|5498|5500|5501|5502|5526|5563|5589|5590|5592|5594|5605|5613|5615|5624|5627|5648|5653|5658|5666|5671|5683|5686|5691|5695|5697|5698|5702|5710|5715|5717|5719|5727|5737)\\// {print; ok=0} END{exit !ok}'"
    ],
    "parallel_group": "terminal-audit",
    "defer_reason": null
  },
  {
    "lane": "aggregate-diff-hygiene",
    "proof_role": "Prove the aggregate terminal projection commit has no whitespace errors",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...c4ef77c46"
    ],
    "parallel_group": "terminal-audit",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `/bin/bash -lc for i in 4739 4741 4758 4761 4762 5107 5338 5340 5341 5343 5344 5345 5349 5350 5361 5384 5497 5498 5500 5501 5502 5526 5563 5589 5590 5592 5594 5605 5613 5615 5624 5627 5648 5653 5658 5666 5671 5683 5686 5691 5695 5697 5698 5702 5710 5715 5717 5719 5727 5737; do .adl/bin/csdlc-v2/csdlc-doctor --repo . --issue $i >/dev/null || exit 1; done`
- `/bin/bash -lc for i in 4739 4741 4758 4761 4762 5107 5338 5340 5341 5343 5344 5345 5349 5350 5361 5384 5497 5498 5500 5501 5502 5526 5563 5589 5590 5592 5594 5605 5613 5615 5624 5627 5648 5653 5658 5666 5671 5683 5686 5691 5695 5697 5698 5702 5710 5715 5717 5719 5727 5737; do jq -e -s '.[0].record == .[1]' "$(git rev-parse --git-common-dir)/csdlc-v2/closeout/$i.json" .csdlc/issues/$i/index.json >/dev/null || exit 1; done`
- `/bin/bash -lc git diff --name-only origin/main...c4ef77c46 | awk 'BEGIN{ok=1} !/^\.csdlc\/issues\/(4739|4741|4758|4761|4762|5107|5338|5340|5341|5343|5344|5345|5349|5350|5361|5384|5497|5498|5500|5501|5502|5526|5563|5589|5590|5592|5594|5605|5613|5615|5624|5627|5648|5653|5658|5666|5671|5683|5686|5691|5695|5697|5698|5702|5710|5715|5717|5719|5727|5737)\// {print; ok=0} END{exit !ok}'`
- `git diff --check origin/main...c4ef77c46`

## Failure Semantics

Fail closed on receipt drift, invalid terminal evidence, missing PVF/review truth, dirty foreign ownership, non-portable retained artifacts, unrelated paths, or actionable review findings.

## Handoff

Retain typed evidence before convergence.
