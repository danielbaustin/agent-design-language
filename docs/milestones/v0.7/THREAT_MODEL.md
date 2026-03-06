

# ADL Threat Model (Draft)

Status: Draft (v0.7 planning)
Last updated: 2026-02-24

This document is a **working draft** for v0.7. The canonical, linkable version will be created/landed by issue **#489** under `docs/security/THREAT_MODEL_v0.7.md`.

## Purpose

ADL executes workflows that may invoke tools, access files, and (in some modes) coordinate remote execution. This threat model:

- defines what ADL defends against in v0.7,
- makes trust boundaries explicit,
- states **testable security invariants**, and
- maps those invariants to concrete implementation work.

This is written for engineers and reviewers; it is intentionally audit-friendly and non-marketing.

## Assets / Security Goals

Primary goals in v0.7:

1. **Integrity of remote execution requests**
   - Remote requests must not be executed unless they satisfy the configured trust requirements.

2. **Integrity of audit artifacts (traces / run summaries)**
   - Security-relevant decisions (allow/deny, verification failures) must be recorded deterministically.

3. **Sandbox confinement for filesystem access**
   - Untrusted workflows and remote requests must not escape configured filesystem boundaries.

4. **Operator-controlled trust decisions**
   - Trust must be explicit; there must be no silent downgrade to weaker modes.

Non-goals are explicitly listed below.

## Attacker Model

We consider the following adversaries (in-scope unless otherwise stated):

- **Malicious remote peer** sending crafted requests to cause execution or data access.
- **Malicious workflow author** attempting to exfiltrate data or escape the sandbox.
- **Compromised tool/provider response** returning malicious output intended to trigger unsafe behavior.
- **Local attacker with filesystem access** who can modify files on disk (partially in-scope: ADL can reduce exposure but cannot fully defend a compromised host).

Acknowledge but largely out-of-scope in v0.7:

- **Supply-chain compromise** (malicious dependencies, compiler toolchain compromise).
- **Kernel/OS compromise** or hypervisor compromise.

## Trust Boundaries

### Boundary A: Local runner boundary
- The local ADL runtime and its process privileges.
- The operator’s filesystem permissions.

### Boundary B: Remote execution boundary
- Any network-facing or cross-process remote execution interface.
- Remote requests are treated as **untrusted inputs**.

### Boundary C: Key material boundary
- Private keys used for signing (where supported).
- Trust store / allowed key identities.

### Boundary D: Sandbox filesystem boundary
- Allowed root(s) for workflow file operations.
- Any path that cannot be proven to resolve inside the allowed root(s) is denied.

## Security Invariants (v0.7)

These invariants should be treated as **testable statements**.

### I-01: Centralized envelope gate
No remote execution request is executed unless it passes a **centralized security envelope validation** step when remote enforcement is enabled.

### I-02: Deny-by-default under uncertainty
If trust cannot be established (missing verification material, malformed metadata, unverifiable signature, unknown algorithm), execution is denied (no silent fallback).

### I-03: Stable error taxonomy
Security failures produce deterministic, stable error codes/kinds that are suitable for:
- tests (assertions),
- tooling (categorization), and
- user debugging (actionable messages).

### I-04: Request integrity (when signing required)
When signing is required by policy, any change to the signed request fields results in verification failure.

### I-05: Trust policy is explicit
Verification behavior is governed by an explicit policy/profile (allowed algorithms, whether `key_id` is required, key source rules). Policy violations are distinct from cryptographic verification failures.

### I-06: Sandbox confinement
Workflow and remote-request filesystem access cannot escape allowed root(s) via:
- `..` traversal,
- symlinks,
- symlink + traversal combinations.

### I-07: Learning cannot bypass trust (guardrail)
Learning overlays/suggestions (v0.7 surfaces) cannot bypass envelope checks, trust policy, or sandbox confinement.

## Non-goals / Out of Scope (v0.7)

- **Confidentiality on a compromised host**: if the local machine is compromised, ADL cannot guarantee secrecy.
- **KMS / key rotation / key distribution**: v0.7 may support local key material and simple trust stores only.
- **TPM / attestation**: no hardware attestation in v0.7.
- **Full OS isolation**: no containerization/seccomp guarantees (best-effort only).
- **Distributed multi-node trust**: cluster distribution and multi-node trust are v0.8+.

## Operational Guidance (v0.7)

Recommended secure defaults (subject to exact config key names in implementation):

- Enable remote envelope enforcement.
- Require signed remote requests.
- Require `key_id` (and configure a minimal trust store/allow-list).
- Keep sandbox roots minimal; do not grant broad filesystem access.

When verification fails:
- prefer inspecting the error code/kind first,
- confirm policy/profile settings,
- confirm the request’s signing material (algorithm, key_id),
- confirm sandbox root resolution for any requested paths.

## Mapping to Implementation Work

This draft maps invariants to the v0.7 issue plan:

- **#370** — Remote execution security envelope enforcement
  - Implements I-01, I-02, I-03 (centralized gate + deterministic failures).

- **#371** — Signing trust policy (verification profile; key_id requirements)
  - Implements I-05 and contributes to I-02/I-03.

- **#386** — Remote request signing (canonical bytes-to-sign; sign/verify)
  - Implements I-04 and contributes to I-02/I-03.

- **#472** — Sandbox hardening (symlink/path escape prevention)
  - Implements I-06.

- **#490** — Learning guardrails (overlays cannot bypass trust)
  - Implements I-07 (tests + explicit guardrails).

- **#489** — Canonical threat model doc
  - Promotes this draft into `docs/security/THREAT_MODEL_v0.7.md` and adds minimal links from `SECURITY.md` and root `README.md`.

## Future Work (v0.8+)

Short list (non-exhaustive):

- Cluster / distributed execution trust model and remote attestation story.
- Durable checkpointing and integrity of persisted state across resumes.
- Stronger sandboxing (containers/VMs) where practical.
- Key rotation and improved trust-store management.
