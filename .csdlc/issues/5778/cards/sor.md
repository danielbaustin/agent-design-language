# Structured Output Record

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Completed the idempotent C-SDLC v2 finish path, retained the current-main formatter repair, made the rehome-authority proof deterministic for #5784, and repaired #5785 so a normally published record can finish through metadata-only publication head drift without weakening live remote gates.

## Artifacts

- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate_finish.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md
- adl-runtime/src/runtime_api.rs
- .csdlc/evidence/5778/post-finalize-remediation.md

## Execution

- Held the canonical per-issue lifecycle authority lock across record validation, GitHub reads, merge, post-merge re-observation, and terminal cache retention.
- Derived minimal terminal authority from exact live GitHub state and logically released stale claims without tracked post-merge closeout commits.
- Reduced exact-head GitHub review state using only decisive review events so later comment-only reviews cannot erase authority.
- Applied current stable rustfmt to the Runtime API endpoint inventory defect tracked by #5783.
- Replaced the scheduling race with an explicitly injected test observer that completes source mutation before revalidation resumes while the concurrent typed writer remains lock-blocked; the operational entrypoint supplies only a no-op observer.
- Replaced the legacy MergeReady-only finish authority check with finish-native canonical identity and a non-expired owned claim bound to the canonical generation, branch, and worktree while retaining exact live remote checks and review gates.
- Required the local checkout to be clean at the exact requested head, strictly validated clean-revision envelopes, and accepted publication metadata advancement only for .csdlc-only descendants whose review evidence exactly matches the publication commit and whose reviewed scope remains unchanged; all drift fails closed.

## Validation

[
  {
    "command": [
      "cargo",
      "+stable",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate_finish"
    ],
    "purpose": "Prove Published finish authority and clean metadata-only publication lineage while rejecting dirty or wrong heads, malformed revisions, changed review scope, substantive drift, and stale claim generations.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--lib",
      "--test",
      "gate_finish",
      "--test",
      "gate7_lifecycle",
      "--test",
      "gate10a",
      "--test",
      "gate10b",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Prove finish serialization, exact-head review reduction, derived terminal behavior, lifecycle compatibility, installation, and GitHub behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  },
  {
    "command": [
      "cargo",
      "+stable",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate9"
    ],
    "purpose": "Re-run the complete Gate 9 soak surface containing the exact CI failure after installing deterministic injected post-materialization ordering.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove warning-free C-SDLC v2 production and test targets after the deterministic observer repair.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  },
  {
    "command": [
      "cargo",
      "+stable",
      "fmt",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Prove the Runtime API formatter repair on the current-main merge tree.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
