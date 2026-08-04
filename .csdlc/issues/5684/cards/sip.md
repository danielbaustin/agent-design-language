# Structured Intent Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Refactor the C-SDLC GitHub command surface into smaller owner binaries and enforce the stable installed binary set.

## Required Outcome

Issue and PR GitHub responsibilities are isolated behind smaller binaries; stable install/coexistence builds and verifies all required binaries.

## Scope

- C-SDLC v2 GitHub command binaries
- owner-binary install/coexistence manifests
- shared retry/backoff resilience primitive
- focused tests for split routing and install proof
- current operator docs and skill guidance for split GitHub surfaces
- current bootstrap validation guidance for deleted structured-prompt wrapper

## Authority

- No app connector write fallback
- No AWS
- Private plan remains under ignored .adl/docs/TBD

## Assumptions

- none

## Operator Constraints

- Use FastWork for build output
- Use repo binaries and make them present
- Keep csdlc-merge installed
- Do not publish the private refactor plan
