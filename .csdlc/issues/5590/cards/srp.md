# Structured Review Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5590/audit.jsonl
.csdlc/issues/5590/index.json
.csdlc/prepared/issues/5590/record-guardian-coverage-portability-finding.json
adl-runtime/tests/guardian_cli.rs
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh

## Prompts

- Does one init model and one Axum/rustls router truthfully cover local and remote access without hard-coded addresses or HTTP?
- Do HTTP and WebSocket Observatory paths share authentication, origin, authority, frame, redaction, and live-state contracts?
- Does discovery report the actual listener and configured public HTTPS base for default, non-default, and ephemeral ports?
- Does the external guardian distinguish intentional stop, invalid config, bounded retry, pressure serialization, and checkpoint restore without sidecars?
- Does Vector own collection/export while Runtime stderr, health, control, and shutdown survive collector absence?
- Is rollback explicit, reviewed, evidence-preserving, and free of Runtime v2 source edits, automatic cutover, AWS, or deployment claims?
- Do S1 through S6 and all lanes cover AC-1 through AC-8 with no deferred or fixture-only parity credit?

## Findings

[
  {
    "id": "guardian-binary-coverage-selection",
    "severity": "p1",
    "summary": "The mapping now selects guardian library tests, the exact binary unit test, and all guardian CLI integration tests; focused llvm-cov reports 125/135 binary lines covered.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1e24b7dd7131cda8fc92ca51e96084f1a723a3e1:7aa622c6bf751f95de6f80fffc44ee827f9cb45d2f2b59e129263048440d1424",
    "route": null
  },
  {
    "id": "guardian-cli-portability",
    "severity": "p2",
    "summary": "The positive black-box proof builds and supervises a native temporary Rust executable using PATH-resolved rustc and the platform executable suffix; no Unix executable path remains.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:1e24b7dd7131cda8fc92ca51e96084f1a723a3e1:7aa622c6bf751f95de6f80fffc44ee827f9cb45d2f2b59e129263048440d1424",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Whole-crate all-target Clippy still reports two pre-existing cav.rs lib-test warnings outside the #5590 protected paths; focused guardian_cli Clippy passes.

## Review Result

Revision: Some("git-blake3:1e24b7dd7131cda8fc92ca51e96084f1a723a3e1:7aa622c6bf751f95de6f80fffc44ee827f9cb45d2f2b59e129263048440d1424")

Reviewer: Some("subagent:019f8692-79df-7fe0-98bd-8d42df9b5f1a")

Result: pass
