# Structured Output Record

Template: 1.0.0

Issue: 5339

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Made each run-target schema branch require the corresponding non-null string or workflow object and added null regressions for both alternatives.

## Artifacts

- adl-v2/crates/adl-language
- adl-v2/crates/adl-language/schema/adl-document.schema.json
- adl-v2/crates/adl-language/CHARACTERIZATION_PARITY.md
- adl-v2/crates/adl-language/src/validate.rs
- adl-v2/crates/adl-language/src/lib.rs
- adl-v2/crates/adl-language/schema/adl-document.schema.json
- adl-v2/crates/adl-language/tests/language.rs
- .csdlc/evidence/5339/implementation-validation
- adl-v2/crates/adl-language/src/lib.rs
- adl-v2/crates/adl-language/schema/adl-document.schema.json
- adl-v2/crates/adl-language/tests/language.rs

## Execution

- Added typed provider, tool, agent, task, workflow, and singular run models with strict unknown-field rejection
- Added duplicate-key-safe YAML and JSON parsing plus version, identity, reference, state, cycle, and run-target validation
- Added deterministic canonical JSON, a checked schema generator, focused tests, and an explicit #5337 corpus parity map
- Kept compiler expansion, runtime execution, provider invocation, control-plane, cloud, storage, and migration outside the crate
- Validate run, inline workflow, saved-state, step, map, and every reference identity with stable invalid_identity diagnostics
- Constrain generated schema versions and exclusive run targets; test invalid version, run shape, nested type, and nested unknown fields against schema and parser
- Count src/examples and tests/schema/parity-map surfaces in the implementation and test-fixture budgets
- Remove trailing blank lines and retain a clean diff
- Add non-null workflow_ref and workflow property constraints inside the generated oneOf branches
- Test explicit null workflow_ref and workflow values against both JSON Schema and parser/semantic validation

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5339/validate-language.sh",
      "focused|quality|parity|budgets"
    ],
    "purpose": "Prove the six-primitives language model, strict parsing and schema alignment, semantic diagnostics, canonical ordering, #5337 corpus mapping, dependency boundary, LoC budgets, and latency budget.",
    "outcome": "passed",
    "evidence_ref": "Focused: 9 tests passed. Quality: strict Clippy passed. Parity: 3 mapped corpus tests passed. Budgets: 637 implementation lines, 254 test lines, exact five-dependency COTS set, no forbidden dependency family, warm all-target validation 0 seconds. git diff --check passed."
  },
  {
    "command": [
      "csdlc-validate",
      "--request",
      ".csdlc/prepared/issues/5339/validate-implementation.json"
    ],
    "purpose": "Prove focused tests, strict Clippy, characterization parity, dependency/LoC/latency budgets, and diff hygiene on the actual merged commit.",
    "outcome": "passed",
    "evidence_ref": "Merge commit 860aa9f18946a2cd9407b610d5c00d44ddc89053; typed PVF disposition local_pass with all four lanes passed. Retained logs: .csdlc/evidence/5339/post-merge/."
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
