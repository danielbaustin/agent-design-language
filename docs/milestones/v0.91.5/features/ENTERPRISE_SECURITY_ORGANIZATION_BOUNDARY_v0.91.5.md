# Enterprise Security Organization Boundary v0.91.5

## Status

Architecture/planning packet for issue `#3538`.

This document is not an implementation closeout record. It does not move code,
create crates, introduce feature gates, or claim enterprise-security completion.
It records the recommended organization boundary before later v0.93/v0.94
security implementation work.

## Purpose

ADL already has many security, trust, policy, signing, adversarial, isolation,
ACIP, and governed-execution surfaces. The question is no longer whether
enterprise security belongs in ADL. It does. The question is how to organize it
so the core runtime stays understandable, buildable, and reviewable while the
enterprise-security band remains first-class.

The recommendation is a staged separation:

1. Keep normal local ADL development on the mainline runtime path.
2. Group enterprise-security surfaces into explicit capability-band modules,
   docs, fixtures, tests, and proof packets.
3. Defer crate or workspace extraction until module boundaries, public schemas,
   and proof ownership are stable.
4. Never treat separation as de-scoping; enterprise security remains part of the
   roadmap and proof model.

## Current Surface Inventory

### Core runtime and policy-adjacent Rust surfaces

These surfaces are already part of the main ADL runtime or runtime-adjacent
substrate and should remain available to ordinary ADL development:

| Surface | Current role | Organization note |
| --- | --- | --- |
| `adl/src/signing.rs` | Signing and verification primitives. | Keep as core trust primitive until enterprise key lifecycle requires a deeper module split. |
| `adl/src/trace_schema_v1.rs` | Trace schema baseline. | Keep core; signed/queryable enterprise trace work should extend through explicit trace/security modules. |
| `adl/src/instrumentation/trace_formatting.rs` | Trace formatting support. | Keep core instrumentation. |
| `adl/src/instrumentation/trace_normalization.rs` | Trace normalization support. | Keep core instrumentation. |
| `adl/src/delegation_policy.rs` | Delegation policy rules. | Core governance primitive; enterprise policy modules may depend on it. |
| `adl/src/policy_authority.rs` | Policy/authority substrate. | Core governance primitive; enterprise policy modules may depend on it. |
| `adl/src/runtime_v2/access_control.rs` | Runtime v2 access-control substrate. | Core runtime authority primitive. |
| `adl/src/runtime_v2/security.rs` | Runtime v2 security boundary. | Core runtime security primitive; later enterprise hardening should not hide here without explicit module boundary. |
| `adl/src/remote_exec/security.rs` | Remote execution security surface. | Candidate for `enterprise_security::remote_execution` grouping after v0.93 design. |
| `adl/src/remote_exec/signing_support.rs` | Remote execution signing support. | Candidate for trust/key lifecycle grouping after v0.93 design. |

### Governed tools, ACC, ACIP, and communication surfaces

| Surface | Current role | Organization note |
| --- | --- | --- |
| `adl/src/acc.rs` and `adl/src/acc/` | ADL Capability Contract policy, visibility, failure, and governance validation. | Keep as governed-tool capability substrate; enterprise authorization may build on it but should not absorb it. |
| `adl/src/uts_acc_compiler.rs` and `adl/src/uts_acc_compiler/` | UTS+ACC compilation/evidence path. | Keep governed-tools substrate; enterprise security should consume its proof posture. |
| `adl/src/agent_comms.rs` | ACIP message substrate. | Keep core communication substrate; transport security and cross-polis hardening belong to later security modules. |
| `docs/explainers/ACIP.md` | ACIP conceptual boundary. | Keep public explainer; add enterprise-security links as transport/security work matures. |
| `docs/milestones/v0.91.1/features/ACIP_HARDENING.md` | Earlier ACIP hardening feature surface. | Treat as historical baseline consumed by v0.93/v0.94. |
| `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md` | Planned binary/schema/transport readiness. | v0.93/v0.94 security depends on this, but should not make it a hidden prerequisite for local runtime work. |

### Adversarial, red/blue, audit, and proof surfaces

| Surface | Current role | Organization note |
| --- | --- | --- |
| `docs/explainers/RED_BLUE_SECURITY.md` | Red/blue/purple security explainer. | Keep as public conceptual entry point. |
| `docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md` | Security/threat modeling baseline. | Historical proof baseline. |
| `docs/milestones/v0.89/features/ADL_SECURITY_POSTURE_MODEL.md` | Security posture baseline. | Historical proof baseline. |
| `docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md` | Trust-under-adversary model. | Historical proof baseline. |
| `docs/milestones/v0.89.1/features/ADL_ADVERSARIAL_RUNTIME_MODEL.md` | Adversarial runtime model. | Candidate ancestor for `enterprise_security::adversarial_runtime`. |
| `docs/milestones/v0.89.1/features/ADVERSARIAL_EXECUTION_RUNNER.md` | Adversarial execution runner feature. | Candidate ancestor for red/blue runtime proof lane. |
| `docs/milestones/v0.89.1/features/ADVERSARIAL_REPLAY_MANIFEST.md` | Replay manifest surface. | Keep as proof/replay precedent for later adversarial regression. |
| `docs/milestones/v0.93/RED_BLUE_ADVERSARIAL_SECURITY_ISSUE_WAVE_v0.93.md` | v0.93 red/blue issue-wave planning. | Should consume this organization boundary before issue execution. |
| `adl/tests/demo_tests.rs` adversarial/security demo assertions | Demo proof for adversarial/audit/security packets. | Keep demo proof visible; later enterprise proof lanes can factor manifests without hiding tests. |

### Citizen-state, private-state, isolation, and audit surfaces

| Surface | Current role | Organization note |
| --- | --- | --- |
| `docs/milestones/v0.90.3/SIGNED_PRIVATE_STATE_ENVELOPE_v0.90.3.md` | Signed private-state envelope baseline. | Candidate trust/key/audit dependency. |
| `docs/milestones/v0.90.3/features/CITIZEN_STATE_SECURITY_AND_FORMAT.md` | Citizen-state security/format baseline. | Candidate isolation/privacy dependency. |
| `docs/milestones/v0.90.2/features/SECURITY_BOUNDARY_EVIDENCE.md` | Runtime security-boundary evidence. | Historical proof baseline for security-boundary evidence. |
| `docs/milestones/v0.90.1/features/INVARIANT_AND_SECURITY_BOUNDARY.md` | Invariant/security boundary baseline. | Historical proof baseline. |

### Forward-planning enterprise-security surfaces

| Surface | Current role | Organization note |
| --- | --- | --- |
| `docs/milestones/v0.93/features/ENTERPRISE_SECURITY_v0.93.md` | v0.93 enterprise-security tranche contract. | Should treat this document as the organization boundary source. |
| `docs/milestones/v0.93/features/SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md` | Zero-trust architecture work package. | First enterprise-security module/proof-band candidate. |
| `docs/milestones/v0.93/features/SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md` | Policy enforcement and authorization work package. | Candidate `enterprise_security::policy` band. |
| `docs/milestones/v0.93/features/SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md` | Secrets, keys, and cryptographic trust work package. | Candidate `enterprise_security::trust` / `keys` band. |
| `docs/milestones/v0.93/features/SECURITY_WP_S4_AUDIT_COMPLIANCE_INCIDENT_EVIDENCE_v0.93.md` | Audit/compliance/incident evidence work package. | Candidate `enterprise_security::audit` band. |
| `docs/milestones/v0.93/features/SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md` | Isolation/data governance/privacy work package. | Candidate `enterprise_security::isolation` band. |
| `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md` | Security ops/adversarial/provenance work package. | Candidate `enterprise_security::adversarial_ops` band. |
| `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md` | v0.94 secure-execution convergence contract. | Should consume v0.93 module/proof outputs and converge them with signed trace/reasoning graph work. |
| `docs/milestones/v0.94/features/SIGNED_TRACE_AND_TRACE_QUERY_v0.94.md` | Signed trace/query planning. | Trust and audit integration dependency. |

## Boundary Model

### Core ADL runtime band

Core runtime code should remain the default development and review surface for:

- execution-plan semantics
- provider substrate
- ordinary local execution
- C-SDLC issue workflow
- prompt-card lifecycle
- trace emission and basic artifact inspection
- base signing/verification primitives
- base access-control and policy-authority primitives
- governed-tool and ACC baseline semantics

Core runtime code may expose extension points for security, but it should not
silently inherit enterprise-only assumptions such as production tenant isolation,
SOC-style audit posture, legal compliance posture, key-rotation operations, or
cross-polis transport security.

### Enterprise-security capability band

Enterprise-security work should be organized as a first-class capability band
with explicit packages of code, docs, fixtures, tests, and proof artifacts.

Recommended internal module vocabulary:

```text
enterprise_security
  zero_trust
  policy
  trust
  audit
  isolation
  adversarial_ops
  secure_transport
  proof
```

These names are planning names, not an immediate code-movement command. They
should guide v0.93 issue boundaries and later module names if implementation
work confirms the split.

### Proof and policy coupling rule

Enterprise security must not become a sidecar. Any separated module or crate
must preserve proof coupling:

- policy fixtures stay near policy code
- audit and incident records stay near audit schemas
- adversarial replay manifests stay near adversarial runner/proof code
- ACIP transport security keeps schema, authority, trace, and replay evidence
  together
- docs name exactly which proof packet supports each claim

## Options Considered

| Option | Description | Pros | Cons | Recommendation |
| --- | --- | --- | --- | --- |
| Keep everything in core modules | Continue adding security code and docs wherever current runtime work lands. | Lowest short-term friction. | Hidden enterprise assumptions accumulate; harder review; core runtime becomes intimidating. | Reject as long-term architecture. |
| Immediate separate crate | Move enterprise security into a new crate now. | Strong boundary. | Premature; current surfaces are interdependent and proof ownership is not stable. | Defer until v0.93 module boundaries are proven. |
| Feature gates only | Keep code colocated but gate enterprise behavior. | Simple build control. | Feature gates alone do not create proof, docs, or review boundaries. | Use only as a later implementation tactic. |
| Docs packages only | Keep implementation scattered but make docs clearer. | Fast and helpful. | Does not prevent future code sprawl. | Accept as v0.91.5 planning step only. |
| Staged module/proof separation | Inventory now, define boundary, route v0.93 work packages, then extract modules/crates only when stable. | Preserves core runtime simplicity while keeping enterprise security first-class. | Requires disciplined follow-through. | Recommended. |

## Recommendation

Adopt staged module/proof separation.

For `v0.91.5`, publish this organization boundary and update planning docs.
For `v0.93`, implement enterprise-security work packages as explicit security
bands with local proof surfaces. For `v0.94`, converge those bands into secure
execution and signed/queryable trace. Only after those boundaries are stable
should ADL consider a separate enterprise-security crate or workspace package.

Recommended path:

1. `v0.91.5`: organization boundary and inventory.
2. `v0.92`: keep identity, model identity, ACIP transport, and provider session
   events compatible with this boundary.
3. `v0.93`: implement security WPs as explicit module/proof bands.
4. `v0.94`: converge secure execution, signed trace/query, and reasoning graph
   provenance.
5. `v0.95`: ensure MVP baseline includes reviewable enterprise-security posture
   without claiming subsystem completion if post-MVP work remains.
6. Post-MVP: decide whether to extract a separate crate or package after the
   module/proof seams have real usage evidence.

## Migration Sequence

### Phase 0: Planning boundary

This issue completes Phase 0 by publishing this document and linking it from the
current planning docs.

### Phase 1: v0.93 work-package routing

Before implementing v0.93 security work, update the six security WP issues to
identify their target band:

- `zero_trust`
- `policy`
- `trust`
- `audit`
- `isolation`
- `adversarial_ops`

Each issue should state whether it changes core runtime primitives, adds an
enterprise-security module, adds proof fixtures, or only updates docs.

### Phase 2: Module grouping without crate extraction

As implementation begins, prefer explicit Rust modules and docs paths before a
new crate:

- `adl/src/enterprise_security/` for new enterprise-only code after approval
- `docs/milestones/v0.93/security/` or the existing feature-WP paths for proof
  packets
- `adl/tests/enterprise_security_*` or scoped module tests for security bands

Do not move existing core primitives merely to make the directory tree pretty.
Move only when ownership and proof surfaces are clear.

### Phase 3: Feature gates and dependency review

If enterprise-security work adds heavyweight dependencies or production-only
runtime behavior, add feature gates and dependency review. Feature gates are a
build boundary, not a proof boundary; they must be paired with docs, fixtures,
and tests.

### Phase 4: Crate/package extraction decision

Consider a separate crate only after:

- the enterprise-security module tree has stable public interfaces
- proof fixtures can run without broad core-runtime coupling
- core ADL builds remain simple
- v0.93/v0.94 review packets agree that extraction would reduce complexity

## Review Gates

Before any code movement, require:

- source inventory updated with affected paths
- proof-owner map for fixtures/tests/docs
- dependency-impact note
- build-impact note
- no hidden enterprise prerequisites for local ADL workflow
- reviewer-facing non-claims for compliance/certification
- follow-on issue that names exactly which files move or which module is added

## Planning Doc Updates

The following docs should point to this boundary:

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.93/features/ENTERPRISE_SECURITY_v0.93.md`
- `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md`

## Follow-On Issue Candidates

Do not implement these inside `#3538`; route them as separate issues if needed.

1. `[v0.93][security] Bind security WPs to enterprise-security module/proof bands`
2. `[v0.93][security] Create enterprise-security proof-owner map before code movement`
3. `[v0.93][security] Add zero-trust and policy module skeletons with fixtures`
4. `[v0.94][security] Converge signed trace/query with enterprise audit evidence`
5. `[post-MVP][security] Decide enterprise-security crate extraction after v0.93/v0.94 evidence`

## Non-Claims

- This document does not claim enterprise-security implementation is complete.
- This document does not claim external compliance certification.
- This document does not move code.
- This document does not add feature gates or new dependencies.
- This document does not make enterprise security optional.

## Validation Expectations

Focused validation for this planning issue should confirm:

- this document exists in the tracked milestone docs
- named planning docs link to it
- source/path inventory searches are recorded in the output card
- no broad Rust validation is claimed
