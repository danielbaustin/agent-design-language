# Structured Task Prompt

Template: 1.0.0

Issue: 5632

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Update the canonical skill source and prove installer parity without changing runtime or v2 binaries.

## Deliverables

- v2-routed canonical skill
- architecture design
- block diagram
- installer parity proof

## Acceptance

1. No executable v1 lifecycle command remains in the skill
2. Typed v2 operation sequence is complete
3. Review is required before publication
4. Installed copy is byte-identical to canonical source

## Dependencies

- csdlc-v2 operator skill contracts
- install_adl_pr_cycle_skill.sh

## Inputs

- docs/tooling/adl_pr_cycle_skill.md
- csdlc-v2/operator/skills/
- adl/tools/install_adl_pr_cycle_skill.sh

## Non Goals

- editing other lifecycle skills
- changing C-SDLC binaries
- deleting historical v1 evidence
