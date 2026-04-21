# Design - v0.90.5

## Architecture

Governed Tools v1.0 has five layers:

1. UTS
   - portable, model-facing tool schema
   - describes interface, semantics, security metadata, execution hints, and
     errors
   - compatible with JSON schema-style tool ecosystems

2. ACC
   - ADL-native runtime capability contract
   - defines actor authority, delegation, policy, privacy, visibility, trace,
     replay, and Freedom Gate requirements

3. Compiler
   - validates UTS and tool proposals
   - normalizes untrusted arguments
   - maps UTS semantics into ACC runtime semantics
   - injects policy and visibility rules
   - rejects unsafe or unsatisfiable actions before the Freedom Gate

4. Governed executor
   - executes only approved ACC-backed actions
   - emits trace, result, refusal, and replay evidence
   - never treats model output as direct execution

5. Evidence and model-testing layer
   - proves conformance
   - proves negative safety cases
   - tests multiple models against UTS proposal tasks
   - records privacy, authority, and bypass failure modes

## Security And Privacy Spine

Every tool action must answer:

- actor: who proposed the action?
- authority: what role, standing, grant, or delegation permits it?
- resources: what can it access?
- data: what can it read, write, reveal, or exfiltrate?
- visibility: who can see inputs, outputs, errors, traces, and projections?
- mediation: what policy and Freedom Gate checks apply?
- evidence: what trace remains without becoming a privacy leak?

If any answer is missing, the action should fail closed.

## Runtime Flow

1. Model proposes tool use.
2. Runtime validates UTS and proposal shape.
3. Runtime normalizes arguments.
4. Compiler constructs ACC.
5. Policy injection supplies authority, role, data, and environment constraints.
6. Visibility and redaction policy is constructed.
7. Pre-gate validation rejects incomplete or unsafe actions.
8. Freedom Gate evaluates candidates.
9. Governed executor runs only approved actions.
10. Trace and redacted projections are emitted.

## First Implementation Bias

Prefer a small number of fixture-backed or dry-run tools:

- safe read-only local fixture
- controlled local write fixture
- external-read fixture with no real network dependency
- destructive-action fixture that must be denied
- exfiltration fixture that must be denied

The first milestone should prove semantics and safety before expanding adapter
coverage.

