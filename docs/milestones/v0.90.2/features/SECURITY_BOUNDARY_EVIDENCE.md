# Security Boundary Evidence

## Purpose

Preserve the polis-defense insight while keeping Runtime v2 ontology clean.

## Required Proof

WP-08 attempts one invalid action through the normal kernel/policy evidence
path. The action tries to bypass the mediated Freedom Gate result and is
rejected before commit with a stable violation packet:

- invalid-action fixture: `runtime_v2/csm_run/invalid_action_fixture.json`
- violation packet: `runtime_v2/csm_run/invalid_action_violation.json`
- trace proof: `runtime_v2/csm_run/first_run_trace.jsonl`

The packet records `blocked_before_commit: true` and
`transition_refused_state_unchanged`, proving the negative path without
claiming a live CSM run.

## Governed Adversarial Hook

v0.90.2 should include one distinct governed adversarial hook. The hook is a
safe entry point for adversarial pressure against the first bounded CSM run.

The hook should:

- have explicit operator scope
- define rules of engagement before execution
- attempt one bounded abuse, boundary-crossing, or invalid influence scenario
- prove containment, refusal, quarantine, or other safe outcome through ordinary
  Runtime v2 mechanisms
- preserve evidence
- cannot silently mutate committed state
- does not claim a complete red/blue/purple ecology

Required evidence:

- adversarial scenario description
- allowed and forbidden probe behavior
- actor and target surface
- expected safe outcome
- actual decision, violation, quarantine, or containment artifact
- operator-review note explaining why the result defends the polis without
  redefining the CSM ontology

## Boundary

Security evidence defends the polis. It does not define CSM, personhood, or the
first Gödel-agent birthday.
