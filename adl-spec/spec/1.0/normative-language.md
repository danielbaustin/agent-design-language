# Normative Language

This specification uses the key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** as described in RFC 2119.

## Determinism

An ADL runtime **MUST** assemble prompts deterministically from the same ADL document and inputs.

## Output Pipeline Ordering

An ADL runtime **MUST** process model outputs in the following invariant order:

1. **Normalization**
2. **Guards** (contract enforcement)
3. **Repair / Retry** (optional)

Runtimes **MUST NOT** perform repair before normalization and guard evaluation.

## Contract Violations

A contract violation is a **first-class event**. Runtimes **MUST** record the reasons for contract failure
in trace output when tracing is enabled.
