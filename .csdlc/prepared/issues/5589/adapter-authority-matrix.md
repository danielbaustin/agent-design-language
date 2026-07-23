# #5589 Adapter And Authority Matrix

Status: design-time allocation. Proposed product paths are non-authoritative
until #5591 review and typed claim amendment both pass.

| Capability | Production adapter requirement | Authority and state rule | Required live proof | Zero-credit evidence | Proposed implementation surface |
| --- | --- | --- | --- | --- | --- |
| Freedom Gate and AEE | Kernel component evaluates signed proposed actuation | Decision binds identity, delegation, policy, operation digest, and qualified time before dispatch | allow, deny, appeal disposition, revocation, quarantine, no post-denial invocation | passive metadata, direct unit helper, fixture executor | new collision-free Parity-C governance adapter module after #5591 freeze |
| Delegation and resources | Capability-chain verifier plus bounded reservation/cancellation adapter | Delegation only attenuates; reservations are identity-bound and always released | multi-hop attenuation, widening/expiry/replay rejection, cancellation and exhaustion cleanup | hand-built token fixture without live dispatch | new collision-free delegation/resource module after #5591 freeze |
| Agents and Shepherd | Production resident-agent and supervisory operation adapters | Shepherd coordinates but cannot grant authority or bypass Freedom Gate | two admitted agents, bounded supervision, cancellation and fail-closed child failure | static resident-agent projection | new collision-free agent service module after #5591 freeze |
| Provider | Configured provider port with timeout, auth, quota, cancellation, and malformed-output classification | Provider output is data; only governed kernel actuation has authority | live configured provider request/response plus negative classifications | canned response, mock, degraded fallback | new collision-free provider adapter module aligned with #5349 |
| Scheduler | Bounded queue and deterministic dispatch adapter | Cannot dispatch without current gate receipt and reservation; cancellation/revocation wins races | deterministic ordering, saturation, cancellation, retry/idempotency, graceful drain | fixed bootstrap order or test-only queue | new collision-free scheduler adapter module after #5591 freeze |
| Governed tools | Typed tool port with per-tool capability and bounded result | Tool invocation is an actuation and must carry gate, identity, and cancellation context | one real local governed tool plus unauthorized and malformed negatives | shell fixture or synthetic echo adapter | new collision-free governed-tool adapter aligned with #5349 |
| Identity and memory | Production identity resolver and identity-scoped memory/private-state adapter | Stable citizen identity is authoritative; provider/session/display IDs are not | cross-identity isolation, revoked identity, restart persistence, redaction | in-memory test map only | new collision-free identity/private-state adapter module |
| Chronosense/time | Qualified time source consumed by governance and continuity | Stale, regressing, or unqualified time cannot authorize actuation | monotonic qualification and rollback/staleness rejection | fixed timestamp fixture as acceptance | extend only a disjoint post-freeze time adapter surface |
| Checkpoint | Authenticated production checkpoint store | Sole execution-recovery authority; binds identity, policy, idempotency, and terminal state | pre/post actuation checkpoint, restart, corruption and replay rejection | serialization helper without live recovery | new collision-free Parity-C continuity adapter or reviewed #5591 extension point |
| Lifelog | Redacted append-only production lifelog store | Observational only; cannot restore execution or grant authority | linked lifecycle entries, failure isolation, private-data redaction | JSON fixture or metadata-only projection | new collision-free lifelog adapter module |

## Current Collision Truth

The committed #5591 typed claim reserves the parent paths `adl-runtime-kernel`
and `adl-runtime`. Any present #5589 product claim beneath those roots would
overlap under the typed path-prefix rule. Therefore the active #5589 claim is
preparation-only. The future implementation surface must be selected only after
#5591 records clean review truth and narrows or releases those parent claims.

## Parity Accounting

Each row receives credit only when the configured `adl-runtime-kernel` process
invokes a production or COTS-backed adapter during retained exact-revision
positive and negative proof. A fixture may test a contract but receives zero
parity credit. A degraded adapter may demonstrate fail-closed startup but also
receives zero parity credit. Metadata, diagrams, and this matrix are planning
evidence only.
