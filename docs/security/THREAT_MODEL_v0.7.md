# ADL Threat Model (v0.7)

Status: Accepted

## Scope
This document captures the v0.7 security envelope threat model for the ADL runtime and CLI.
It focuses on implemented controls and explicit non-goals.

## Assets and Security Goals
1. Remote execution request integrity and provenance.
2. Deterministic, auditable run artifacts (`.adl/runs/<run_id>/...`).
3. Sandbox confinement for runtime file access paths.
4. Operator-controlled trust decisions with no silent privilege escalation.

## Attacker Model
1. Malicious remote peer returning malformed or adversarial responses.
2. Malicious workflow author attempting to weaken trust/sandbox constraints.
3. Compromised provider/tool output attempting policy bypass via payload content.
4. Local attacker with filesystem write access in user space.
5. Supply-chain risk acknowledged, mostly out of scope for v0.7 runtime controls.

## Trust Boundaries
1. Local runner boundary:
   - Parses, resolves, validates, and schedules deterministically.
2. Remote execution boundary:
   - `/v1/execute` request accepted only through envelope + policy gates.
3. Key material boundary:
   - Signing/verification material is explicit; no implicit key discovery.
4. Sandbox filesystem boundary:
   - Path resolution confined to configured roots; traversal and symlink escape denied.

## Security Invariants
1. No remote request executes when required envelope validation fails.
2. Trust-policy failures and signature-integrity failures remain distinct and deterministic.
3. No path escapes sandbox roots via `..`, absolute-path injection, or symlink indirection.
4. Deterministic error taxonomy is stable for policy/audit tooling.
5. Learning overlays cannot bypass envelope, trust policy, or sandbox constraints.

## Non-Goals (v0.7)
1. Confidentiality against a fully compromised local host.
2. KMS, key rotation orchestration, HSM/TPM attestation.
3. Distributed trust federation / multi-node attestation.
4. Full OS isolation (sandbox path checks are best-effort hardening, not kernel sandboxing).

## How v0.7 Implements This
1. `#370` Remote security envelope:
   - centralized envelope enforcement in `swarm/src/remote_exec.rs`.
2. `#371` Trust policy:
   - deterministic verification-profile gating (`require_key_id`, allowed algs/sources).
3. `#386` Request signing:
   - canonical bytes + deterministic signature verification path for remote execute requests.
4. `#472` Sandbox hardening:
   - centralized resolver logic in `swarm/src/sandbox.rs`; deny traversal/symlink escapes.
5. `#490` Learning guardrails:
   - overlays are explicitly blocked from weakening trust/sandbox controls.

## Operational Guidance
1. Use signed requests for remote execution where possible (`run.remote.require_signed_requests=true`).
2. For stricter provenance, require key IDs (`run.remote.require_key_id=true` with signed requests).
3. Constrain verification scope explicitly only when needed:
   - `run.remote.verify_allowed_algs`
   - `run.remote.verify_allowed_key_sources` (`embedded`, `explicit_key`)
4. On verification failures, inspect:
   - run-level stderr output
   - `.adl/runs/<run_id>/run.json` and `steps.json`
   - deterministic error codes emitted by envelope/policy checks

## Known Limits
1. Path validation is deterministic and hardened, but not a complete TOCTOU elimination mechanism.
2. Key lifecycle management remains manual in v0.7.

## Future Work (v0.8+)
1. Stronger key lifecycle management and rotation workflows.
2. Additional sandbox hardening layers where portable.
3. Distributed trust and attestation model.
4. Expanded threat model updates aligned with post-v0.7 learning/export surfaces.
