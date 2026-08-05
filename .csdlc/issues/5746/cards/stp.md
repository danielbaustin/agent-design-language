# Structured Task Prompt

Template: 1.0.0

Issue: 5746

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Publish the already-completed typed terminal projection batch and stop at truthful closeout of #5746.

## Deliverables

- Tracked terminal projections for the included issue set
- Receipt-equality and doctor validation result
- Independent review result
- Ready PR closing #5746
- Explicit excluded-issue blocker summary

## Acceptance

1. AC-1: Every included projection is closed_out, claim-null, terminal-evidence complete, and receipt-identical
2. AC-2: Every included issue passes csdlc-doctor
3. AC-3: The aggregate diff contains only declared .csdlc/issues paths and passes diff hygiene
4. AC-4: Independent review has no actionable findings
5. AC-5: Unsafe or unsupported cases remain excluded and explicitly reported
6. AC-6: Publication closes #5746 only and does not claim milestone release or worktree pruning

## Dependencies

- Git-common retained terminal receipts
- Merged GitHub issue and PR observations already captured by typed closeout
- Current origin/main baseline

## Inputs

- GitHub issue #5746
- Included issue projections: 4739, 4741, 4758, 4761, 4762, 5107, 5338, 5340, 5341, 5343, 5344, 5345, 5349, 5350, 5361, 5384, 5497, 5498, 5500, 5501, 5502, 5526, 5563, 5589, 5590, 5592, 5594, 5605, 5613, 5615, 5624, 5627, 5648, 5653, 5658, 5666, 5671, 5683, 5686, 5691, 5695, 5697, 5698, 5702, 5710, 5715, 5717, 5719, 5727, 5737
- Git-common csdlc-v2/closeout receipts
- csdlc-v2 typed closeout and doctor binaries

## Non Goals

- No product implementation
- No repair of excluded lifecycle histories
- No manual receipt/card editing
- No worktree pruning
- No #5595 sprint closure
