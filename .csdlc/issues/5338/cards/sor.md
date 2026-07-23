# Structured Output Record

Template: 1.0.0

Issue: 5338

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Closed third-pass review findings by hashing canonical effective node semantics without unused-declaration churn and comparing only compiler payload lines across subprocess replays.

## Artifacts

- adl-v2/crates/adl-compiler
- adl-v2/crates/adl-compiler/CHARACTERIZATION_PARITY.md
- .csdlc/evidence/5338
- .csdlc/prepared/issues/5338/validate-compiler.sh
- adl-v2/crates/adl-compiler/src/lib.rs
- adl-v2/crates/adl-compiler/tests
- adl-v2/crates/adl-compiler/README.md
- .csdlc/prepared/issues/5338/design.md
- adl-v2/crates/adl-compiler/src/lib.rs
- adl-v2/crates/adl-compiler/tests/deterministic_replay.rs
- adl-v2/crates/adl-compiler/tests/stable_identity.rs
- adl-v2/crates/adl-compiler/README.md
- adl-v2/crates/adl-compiler/src/lib.rs
- adl-v2/crates/adl-compiler/tests/deterministic_replay.rs
- adl-v2/crates/adl-compiler/tests/stable_identity.rs
- adl-v2/crates/adl-compiler/README.md

## Execution

- Added versioned ExecutionPlan, node, edge, run, and workflow data contracts without scheduling or execution authority
- Resolved workflows, tasks, agents, providers, models, tools, sequential order, and saved-state dependencies with deterministic ordered collections
- Added domain-separated length-delimited SHA-256 node identities, canonical source digests, explicit graph and input limits, stable fail-closed diagnostics, and lexical topological ordering
- Added real #5339 characterization fixture coverage, format-permutation and repeated replay proof, identity locality and golden vectors, quality gates, and strict dependency/LoC/time budgets
- Explicitly excluded legacy pattern syntax because the landed adl-language model rejects and cannot represent it
- Added typed input/output ports, prompt payload, and bounded source provenance to every plan node
- Enforced edge limits before allocation growth and aggregate value/depth limits across run and step inputs
- Mechanically inventoried and classified all nineteen landed characterization fixtures with fail-on-new-fixture behavior
- Included the execution-plan contract in stable identity preimages and exposed node_v1_ IDs with refreshed golden proof
- Updated and reapproved the canonical design status after the dependency gate and implementation completed
- Added a canonical digest of resolved step, task, agent, and provider declarations to the stable node identity preimage
- Added locality tests for prompt, model, and tool changes while retaining stability for unrelated run inputs
- Preserved the language-declared Task.inputs vector order in plan ports
- Added equivalent JSON/YAML diagnostic-byte proof and an actual subprocess replay harness for plan and diagnostic bytes
- Replaced raw declaration hashing with resolved refs/model, effective sorted tools, prompt, ports, step inputs, and output
- Proved agent tool reordering, unused provider config, and task-filtered tools do not churn unaffected node identities
- Extracted and compared exactly the PLAN and DIAGNOSTICS payload lines from two child-process runs, excluding libtest harness timing

## Validation

[
  {
    "command": [
      "csdlc-validate",
      "preparation-contract-5338",
      "typed-doctor-5338"
    ],
    "purpose": "Prove six-card integrity, bound protected scope, reviewed design and diagram, executable dependency and budget contract, root safety, and typed doctor health without running product implementation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5338/preparation-validation: local_pass; preparation-contract 163ms; typed doctor 29ms; bounded preparation review PASS with no remaining actionable findings"
  },
  {
    "command": [
      "validate-compiler.sh focused",
      "validate-compiler.sh quality",
      "validate-compiler.sh determinism",
      "validate-compiler.sh budgets"
    ],
    "purpose": "Prove deterministic pure lowering, landed characterization mapping, stable node identity, diagnostics and limits, dependency/COTS restrictions, formatting/clippy quality, LoC ceilings, and FastWork execution-time ceilings.",
    "outcome": "passed",
    "evidence_ref": "local FastWork proof: 12 tests passed; clippy -D warnings passed; implementation 447 LoC; tests/fixtures 289 LoC; full budget lane 1s; all declared ceilings satisfied"
  },
  {
    "command": [
      "validate-compiler.sh focused",
      "validate-compiler.sh quality",
      "validate-compiler.sh determinism",
      "validate-compiler.sh budgets"
    ],
    "purpose": "Prove deterministic pure lowering, landed characterization mapping, stable node identity, diagnostics and limits, dependency/COTS restrictions, formatting/clippy quality, LoC ceilings, and FastWork execution-time ceilings.",
    "outcome": "passed",
    "evidence_ref": "post-review-remediation local FastWork proof: 13 tests passed; clippy -D warnings passed; implementation 508 LoC; tests/fixtures 380 LoC; full budget lane 0s warm; all declared ceilings satisfied"
  },
  {
    "command": [
      "validate-compiler.sh focused",
      "validate-compiler.sh quality",
      "validate-compiler.sh determinism",
      "validate-compiler.sh budgets"
    ],
    "purpose": "Prove deterministic pure lowering, landed characterization mapping, stable node identity, diagnostics and limits, dependency/COTS restrictions, formatting/clippy quality, LoC ceilings, and FastWork execution-time ceilings.",
    "outcome": "passed",
    "evidence_ref": "second post-review-remediation local FastWork proof: 18 tests passed including clean subprocess replay and canonical diagnostic parity; clippy -D warnings passed; implementation 561 LoC; tests/fixtures 486 LoC; full budget lane 0s warm; all declared ceilings satisfied"
  },
  {
    "command": [
      "validate-compiler.sh focused",
      "validate-compiler.sh quality",
      "validate-compiler.sh determinism",
      "validate-compiler.sh budgets"
    ],
    "purpose": "Prove deterministic pure lowering, landed characterization mapping, stable node identity, diagnostics and limits, dependency/COTS restrictions, formatting/clippy quality, LoC ceilings, and FastWork execution-time ceilings.",
    "outcome": "passed",
    "evidence_ref": "third post-review-remediation local FastWork proof: 18 tests passed including effective-semantic identity locality, clean subprocess replay payload comparison, and canonical diagnostic parity; clippy -D warnings passed; implementation 577 LoC; tests/fixtures 519 LoC; full budget lane 0s warm; all declared ceilings satisfied"
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
