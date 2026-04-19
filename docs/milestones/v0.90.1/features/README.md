# v0.90.1 Feature Contracts

These feature contracts define the implementation-facing Runtime v2 foundation
prototype. They were promoted into tracked milestone docs during the v0.90
WP-19 release-tail planning gate.

The contracts are intentionally narrower than the full Runtime v2 idea corpus:

- prove a bounded manifold
- prove a kernel service loop
- prove provisional citizens
- prove snapshot and wake
- prove invariant and security evidence
- make the CSM visible through a bounded Observatory packet
- preserve v0.91 and v0.92 scope

Compression-enabling process policy lives beside the runtime feature contracts
because WP-02 through WP-04 must be reviewable before Runtime v2 coding starts:

- `COMPRESSION_ERA_EXECUTION_POLICY.md`

The CSM Observatory packet contract is the first visibility bridge from Runtime
v2 artifacts to reviewer-facing console, report, CLI, and future operator
command surfaces:

- `CSM_OBSERVATORY_VISIBILITY_PACKET.md`
