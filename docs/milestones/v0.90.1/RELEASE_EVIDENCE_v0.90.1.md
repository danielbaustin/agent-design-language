# Release Evidence Packet - v0.90.1

## Purpose

This packet is D8 in the v0.90.1 demo matrix. It gives reviewers one place to
trace the milestone from claims to proof surfaces before WP-20 release ceremony.

## Milestone Claims

v0.90.1 claims:

- Runtime v2 foundation prototype
- compression-enablement work for faster, safer milestone execution
- CSM Observatory read-only visibility surfaces
- quality gate hardening and review-tail remediation
- clean handoff into v0.90.2, v0.91, and v0.92

v0.90.1 does not claim:

- first true Gödel-agent birthday
- full moral/emotional civilization
- full identity/capability rebinding
- complete cross-polis migration
- full red/blue/purple security ecology

## Evidence Index

| Evidence | File |
| --- | --- |
| Milestone overview | README.md |
| Work package map | WBS_v0.90.1.md |
| Sprint and release-tail ordering | SPRINT_v0.90.1.md |
| Issue wave map | WP_ISSUE_WAVE_v0.90.1.yaml |
| Feature index | FEATURE_DOCS_v0.90.1.md |
| Demo matrix | DEMO_MATRIX_v0.90.1.md |
| Quality gate | QUALITY_GATE_v0.90.1.md |
| Internal review | INTERNAL_REVIEW_v0.90.1.md |
| Third-party review disposition | THIRD_PARTY_REVIEW_v0.90.1.md |
| Release readiness | RELEASE_READINESS_v0.90.1.md |
| v0.91/v0.92 handoff | V091_V092_HANDOFF_v0.90.1.md |
| Release plan | RELEASE_PLAN_v0.90.1.md |
| Release notes | RELEASE_NOTES_v0.90.1.md |

## Demo Evidence

The demo matrix is the canonical demo index. Its D8 row is this packet.

Key proof families:

- D0: compression enablement
- D1 through D7: Runtime v2 foundation
- D9A and D9: CSM Observatory
- D10: quality gate walkthrough
- D8: release evidence packet

## Review Evidence

Internal review found proof-quality gaps and routed them to WP-16 bundles.
Third-party review found no P0 or P1 issues. Its P2 findings were the README
badge and handoff-state truth, both handled in the release-tail docs. Its P3
finding was D8 itself, which this packet resolves.

## Release Ceremony Input

WP-20 should consume this packet as navigation evidence. It should not repeat
the full review. The ceremony should confirm:

- root main is fast-forwarded and clean
- release notes still match landed scope
- checklist remains green except ceremony-only items
- Cargo.toml and Cargo.lock report 0.90.1
- no release blocker has reappeared

Once those checks pass, WP-20 may perform the normal tag and GitHub release
ceremony.
