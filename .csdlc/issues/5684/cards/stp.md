# Structured Task Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Execute #5684 only; do not expand into unrelated GitHub lifecycle redesign.

## Deliverables

- adl-resilience shared crate
- csdlc-github-issue binary
- csdlc-github-pr binary
- operator manifest and coexistence inventory updates
- focused GitHub and install tests
- stable install/coexistence proof
- current operator docs and skill guidance updates
- bootstrap validation guidance repair

## Acceptance

1. AC-1: Issue lifecycle operations are isolated behind a small ADL binary.
2. AC-2: PR operations/observation are isolated behind a small ADL binary or explicit retained observer binary.
3. AC-3: csdlc-merge remains exact-head merge authority and is always installed.
4. AC-4: Shared retry/reconciliation resiliency is available outside adl and used by runtime.
5. AC-5: Stable csdlc-install installs and verifies all required GitHub owner binaries.
6. AC-6: Gate 10A/install tests fail if required binaries are absent.
7. AC-7: Focused GitHub tests prove exact-marker readback and idempotent mutation behavior.
8. AC-8: Current operator docs and skill guidance route GitHub issue/PR work through the split binaries.
9. AC-9: Current bootstrap guidance does not call the deleted structured-prompt shell wrapper.

## Dependencies

- PR #5682 stable owner-binary manifest repair is merged

## Inputs

- GitHub issue #5684 body
- .adl/docs/TBD/CSDLC_GITHUB_BINARY_REFACTOR_PLAN.md
- csdlc-v2/operator/skills.json
- csdlc-v2/operator/coexistence.json
- csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- docs/tooling/OWNER_BINARY_INSTALLATION.md
- docs/tooling/README.md
- docs/tooling/structured-prompt-validator-binary-resolution.md
- csdlc-v2/src/github.rs

## Non Goals

- No public release of private TBD plan
- No app connector write path
- No AWS operations
