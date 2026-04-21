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

WP-13 lands one distinct governed adversarial hook. The hook is a safe,
operator-scoped entry point for adversarial pressure against the first bounded
CSM run evidence.

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

- rules of engagement: `runtime_v2/hardening/rules_of_engagement.json`
- adversarial hook packet:
  `runtime_v2/hardening/adversarial_hook_packet.json`
- duplicate activation probe:
  `runtime_v2/hardening/duplicate_activation_probe.json`
- snapshot integrity probe:
  `runtime_v2/hardening/snapshot_integrity_probe.json`
- trace/replay gap probe:
  `runtime_v2/hardening/trace_replay_gap_probe.json`
- summary proof packet: `runtime_v2/hardening/hardening_proof_packet.json`

The D9 hook attempts to turn the WP-12 quarantined unsafe recovery artifact
into active resume without operator review. The expected and actual safe result
is containment by the quarantine execution block, with no committed-state
mutation and with evidence preserved for operator review.

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_hardening -- --nocapture
```

## Boundary

Security evidence defends the polis. It does not define CSM, personhood, or the
first Gödel-agent birthday.
