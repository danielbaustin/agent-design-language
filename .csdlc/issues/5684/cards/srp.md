# Structured Review Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/5684/diff-check.log
.csdlc/evidence/5684/gate10a-install-bootstrap-tests.log
.csdlc/evidence/5684/github-action-split-tests.log
.csdlc/evidence/5684/post-create-race-fmt-check.log
.csdlc/evidence/5684/post-create-race-focused-test.log
.csdlc/evidence/5684/post-create-race-github-action-tests.log
.csdlc/evidence/5684/post-create-race-p3-fmt-check.log
.csdlc/evidence/5684/post-create-race-p3-focused-test.log
.csdlc/evidence/5684/post-create-race-p3-github-action-tests.log
.csdlc/evidence/5684/post-create-race-p3b-fmt-check.log
.csdlc/evidence/5684/post-create-race-p3b-focused-test.log
.csdlc/evidence/5684/post-create-race-p3b-github-action-tests.log
.csdlc/evidence/5684/post-opus-csdlc-v2-clippy.log
.csdlc/evidence/5684/post-opus-csdlc-v2-fmt-check.log
.csdlc/evidence/5684/post-opus-csdlc-v2-full-tests.log
.csdlc/evidence/5684/post-opus-gate10a-tests.log
.csdlc/evidence/5684/post-opus-github-action-tests.log
.csdlc/evidence/5684/post-opus-resilience-tests.log
.csdlc/evidence/5684/resilience-tests.log
.csdlc/evidence/5684/runtime-resilience-check.log
.csdlc/issues/5684/audit.jsonl
.csdlc/issues/5684/cards/sip.md
.csdlc/issues/5684/cards/sip.values.json
.csdlc/issues/5684/cards/sor.md
.csdlc/issues/5684/cards/sor.values.json
.csdlc/issues/5684/cards/spp.md
.csdlc/issues/5684/cards/spp.values.json
.csdlc/issues/5684/cards/srp.md
.csdlc/issues/5684/cards/srp.values.json
.csdlc/issues/5684/cards/stp.md
.csdlc/issues/5684/cards/stp.values.json
.csdlc/issues/5684/cards/vpp.md
.csdlc/issues/5684/cards/vpp.values.json
.csdlc/issues/5684/design.md
.csdlc/issues/5684/diagram.mmd
.csdlc/issues/5684/index.json
adl-resilience/Cargo.lock
adl-resilience/Cargo.toml
adl-resilience/src/lib.rs
adl-runtime/Cargo.lock
adl-runtime/Cargo.toml
adl-runtime/src/guardian.rs
adl-runtime/src/lib.rs
adl-runtime/src/supervision.rs
csdlc-v2/AGENTS.md
csdlc-v2/Cargo.lock
csdlc-v2/Cargo.toml
csdlc-v2/operator/coexistence.json
csdlc-v2/operator/skills.json
csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md
csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md
csdlc-v2/src/bin/csdlc-github-issue.rs
csdlc-v2/src/bin/csdlc-github-pr.rs
csdlc-v2/src/github.rs
csdlc-v2/src/operator.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate_github_actions.rs
docs/default_workflow.md
docs/templates/prompts/1.0.3/schemas/sip.structure.json
docs/templates/prompts/1.0.3/schemas/sor.structure.json
docs/templates/prompts/1.0.3/schemas/spp.structure.json
docs/templates/prompts/1.0.3/schemas/srp.structure.json
docs/templates/prompts/1.0.3/schemas/stp.structure.json
docs/templates/prompts/1.0.3/schemas/vpp.structure.json
docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
docs/tooling/OWNER_BINARY_INSTALLATION.md
docs/tooling/README.md
docs/tooling/structured-prompt-validator-binary-resolution.md

## Prompts

- Review whether split binaries materially reduce the GitHub command surface and whether install/coexistence enforcement covers the new binaries.

## Findings

[
  {
    "id": "OHM-5684-01",
    "severity": "p3",
    "summary": "Post-create marker confirmation retried multiple distinct exact-marker issues, obscuring terminal ambiguity as generic retry failure.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:2afe605fde2134030c4cc4f5dbec5b19bb174bf8:0cc5a8d6e0dbe6cad80acea7de1541ade5e14173cad91c4018a2ef2ba1d7381c",
    "route": null
  },
  {
    "id": "OHM-5684-02",
    "severity": "p3",
    "summary": "Transient zero marker-search results after successful issue creation were not directly covered by regression proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:2afe605fde2134030c4cc4f5dbec5b19bb174bf8:0cc5a8d6e0dbe6cad80acea7de1541ade5e14173cad91c4018a2ef2ba1d7381c",
    "route": null
  },
  {
    "id": "EPICURUS-5684-01",
    "severity": "p3",
    "summary": "The first zero-search regression consumed the empty search before POST, so it did not prove post-create search lag.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:2afe605fde2134030c4cc4f5dbec5b19bb174bf8:0cc5a8d6e0dbe6cad80acea7de1541ade5e14173cad91c4018a2ef2ba1d7381c",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Kierkegaard read-only exact-head review at 2afe605fde2134030c4cc4f5dbec5b19bb174bf8 returned CLEAN. Validation was run by the implementation owner and recorded in SOR evidence logs.

## Review Result

Revision: Some("git-blake3:2afe605fde2134030c4cc4f5dbec5b19bb174bf8:0cc5a8d6e0dbe6cad80acea7de1541ade5e14173cad91c4018a2ef2ba1d7381c")

Reviewer: Some("subagent:019fa184-d21b-7590-a9c3-4827a36638ce")

Result: pass
