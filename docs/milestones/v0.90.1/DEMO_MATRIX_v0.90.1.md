# Demo Matrix - v0.90.1

## Status

Issue wave closed through WP-20. Runtime v2 foundation, CSM Observatory
implementation rows, quality evidence, review disposition, release evidence,
and release ceremony preflight have landed.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D0 | Compression enablement proof | WP-02/WP-03/WP-04 | Issue-wave, worktree, and execution-policy surfaces are ready before runtime coding starts | template/generator proof, worktree guardrail proof, policy validation note | LANDED |
| D1 | Manifold boot | WP-05/WP-06 | Runtime v2 can create one bounded persistent manifold and start kernel services | manifold manifest, kernel loop report | LANDED |
| D2 | Provisional citizen admission | WP-07 | Provisional citizens can be admitted without claiming true Gödel-agent birth | citizen records, admission trace | LANDED |
| D3 | Snapshot and wake | WP-08 | Manifold state can be snapshotted and rehydrated without duplicate activation | snapshot, wake report | LANDED |
| D4 | Invariant violation | WP-09 | Illegal state transition is rejected and recorded | violation artifact, negative test | LANDED |
| D5 | Operator control | WP-10 | Operator can inspect, pause, resume, and terminate bounded runtime state | operator control report | LANDED |
| D6 | Security boundary | WP-11 | One invalid action is rejected through normal kernel/policy path | security-boundary proof packet, negative test, CLI proof hook | LANDED |
| D7 | Runtime v2 integrated prototype | WP-12 | Reviewer can inspect the foundation prototype end to end | integrated proof packet, artifact graph, reviewer boundary notes, CLI/demo hook | LANDED |
| D8 | Release evidence packet | WP-19 | Reviewer can trace demo, quality, review, and release-readiness evidence without reconstructing the milestone by hand | `RELEASE_EVIDENCE_v0.90.1.md` | LANDED |
| D9A | CSM Observatory static console | #2189 | Reviewer can inspect a read-only fixture-backed console whose fallback packet and render path are semantically checked against the canonical packet fixture | static console HTML/CSS/JS/docs plus semantic render validation | LANDED |
| D9 | CSM Observatory CLI bundle | #2191 | Reviewer can regenerate packet, operator report, console reference, and demo manifest from one read-only ADL CLI command | visibility_packet.json, operator_report.md, console_reference.md, demo_manifest.json | LANDED |
| D10 | Quality gate walkthrough | WP-14 | Reviewer can inspect local quality, coverage, Runtime v2 proof, CSM Observatory proof, and Rust module watch posture in one manifest | `artifacts/v0901/quality_gate/quality_gate_record.json` plus per-check logs | LANDED |

## Non-Proving Boundaries

- These demos do not prove first true Gödel-agent birth.
- These demos do not prove full emotion, morality, kindness, or governance.
- These demos do not prove complete migration or multi-polis interaction.
- These demos do not prove a complete red/blue/purple defense ecology.
- D10 does not replace CI or prove live CSM Observatory mutation; it aggregates
  the quality-gate proof posture and read-only fixture-backed Observatory checks.
