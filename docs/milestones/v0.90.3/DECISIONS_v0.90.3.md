# Decisions - v0.90.3

| ID | Decision | Status | Rationale |
| --- | --- | --- | --- |
| D-01 | Treat v0.90.3 as the citizen-state substrate milestone | Proposed | v0.90.2 proves the first bounded run; v0.90.3 protects the continuity-bearing state behind that run. |
| D-02 | Treat JSON as projection, not authority | Proposed | Prototype JSON is useful for review, but durable citizen identity needs typed, signed, hash-linked state. |
| D-03 | Use protobuf-backed signed artifacts as the expected canonical direction | Proposed | Protobuf gives typed evolution and compact bytes, but WP-03 remains the formal decision gate. |
| D-04 | Keep sealed quintessence checkpoints local-first | Proposed | v0.90.3 should define enclave-ready envelopes without depending on cloud confidential computing. |
| D-05 | Make lineage append-only and authoritative | Proposed | Continuity must survive file drift, replay, and operator mistakes. |
| D-06 | Emit witnesses and citizen-facing receipts for major transitions | Proposed | Citizens and reviewers need explainable continuity, not only implicit hashes. |
| D-07 | Quarantine and sanctuary are evidence-preserving safety states | Proposed | Ambiguous continuity should pause and preserve evidence rather than continue unsafely. |
| D-08 | Communication does not imply inspection | Proposed | Standing and communication are social/action boundaries; raw private-state access remains separate. |
| D-09 | Keep full economics and contract-market work in v0.90.4 | Proposed | v0.90.3 may decide on a narrow resource-stewardship bridge, but economics should not distort citizen-state safety. |
| D-10 | Preserve v0.91 and v0.92 scope | Proposed | v0.90.3 prepares for moral/emotional civilization and birthday work without claiming either. |
| D-11 | Preserve the v0.90.2 release-tail pattern | Proposed | Demo/proof coverage, docs convergence, internal review, external review, remediation, next-milestone handoff, and release ceremony should remain explicit rather than compressed into one ambiguous closeout issue. |
