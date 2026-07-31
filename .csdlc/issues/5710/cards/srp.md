# Structured Review Prompt

Template: 1.0.0

Issue: 5710

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

csdlc-v2/src/bin/csdlc-closeout.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/readiness.rs
csdlc-v2/tests/gate7_lifecycle.rs

## Prompts

- Can terminal reconciliation ever accept a different repository, PR, branch, or unrelated revision?
- Can prune preparation remove tracked lifecycle drift, source files, unknown paths, or unretained evidence?
- Are cleanup and reconciliation idempotent and audit-preserving?
- Does the classifier report legal next actions without mutating lifecycle state?
- Do focused tests cover the #5691 drift and representative dirty-worktree classes?

## Findings

[
  {
    "id": "R-1",
    "severity": "p1",
    "summary": "Validate terminal disposition before metadata reconciliation can mutate canonical readiness.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085",
    "route": null
  },
  {
    "id": "R-2",
    "severity": "p1",
    "summary": "Restrict stale lock cleanup to the target issue.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085",
    "route": null
  },
  {
    "id": "R-3",
    "severity": "p1",
    "summary": "Retain unrecognized prepared JSON instead of deleting it as generated state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085",
    "route": null
  },
  {
    "id": "R-4",
    "severity": "p2",
    "summary": "Add destructive linked-worktree prune coverage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085",
    "route": null
  },
  {
    "id": "R-5",
    "severity": "p2",
    "summary": "Route merged records to remote terminal closeout rather than receipt-only reconciliation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085",
    "route": null
  },
  {
    "id": "R-6",
    "severity": "p2",
    "summary": "Fail closed on staged safe-path changes before prune mutation begins.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Post-merge live recovery remains a separate closeout-stage proof and is not claimed by pre-PR tests.

## Review Result

Revision: Some("git-blake3:e74f9d1b8e30845bb9e62c623812d4f29b7f03b3:22a1a5c717d4e25a7452ec1976585e2b5ea08c2f269c702ea1cc631987b6b085")

Reviewer: Some("subagent:019faf31-2faf-7740-b22c-9bfd9714e9cf")

Result: pass
