# Work Breakdown Structure - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: tracked planning package

## WBS Summary

This draft WBS treats v0.90 as the long-lived-agent runtime milestone, with
bounded sidecar work for demo expansion, quality ratcheting, milestone
compression, repo visibility, and explicit Rust refactoring.

Issue numbers were assigned by the v0.90 WP-01 issue-wave step. WP-01 is
`#2019`; WP-02 through WP-20 are `#2021` through `#2039`.

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
| --- | --- | --- | --- | --- | --- |
| WP-01 | Milestone planning and issue wave | Finalize the promoted tracked package, reconcile scope, and open issues | tracked v0.90 docs and issue wave | #1986, #1940 | #2019 |
| WP-02 | Long-lived supervisor and heartbeat | Implement or define supervisor state, heartbeat, lease, and scheduling surface | supervisor/heartbeat contract and proof | WP-01 | #2021 |
| WP-03 | Cycle contract and artifact root | Define cycle manifests, observations, decision records, run refs, and memory-write candidates | cycle artifact contract | WP-02 | #2022 |
| WP-04 | State and continuity handles | Define pre-v0.92 continuity files, ledgers, provider-binding history, and migration boundary | continuity contract | WP-02, WP-03 | #2023 |
| WP-05 | Operator control and safety | Define status, stop, guardrail, sanitization, and safety surfaces | operator control contract | WP-02, WP-03 | #2024 |
| WP-06 | Minimal inspection and trace boundary | Decide and implement the smallest status/query/trace slice needed for review | inspection proof surface | WP-02 through WP-05 | #2025 |
| WP-07 | Stock league demo scaffold | Build the bounded stock league demo skeleton and fixtures | demo scaffold and safety docs | WP-02 through WP-05 | #2026 |
| WP-08 | Long-lived demo integration | Integrate recurring cycles, continuity, status, and guardrails into the demo | runnable or reviewer-legible integration demo | WP-06, WP-07 | #2027 |
| WP-09 | Demo extensions and proof expansion | Add or extend selected demos without weakening the stock-league proof path | demo extension packet | WP-06 through WP-08 | #2028 |
| WP-10 | Coverage ratchet to 93 percent | Measure coverage hotspots, add focused tests, and raise the gate only after evidence is green | 93 percent quality gate report | WP-02 through WP-09 | #2029 |
| WP-11 | Milestone compression pilot | Define canonical milestone state and drift checks for issue/docs/release-tail truth | compression pilot and drift-check report | WP-01 through WP-10 | #2030 |
| WP-12 | Repo visibility prototype | Add a bounded manifest and code-doc-demo linkage report for one milestone or feature slice | repo visibility proof packet | WP-01 through WP-10 | #2031 |
| WP-13 | Docs and review pass | Align docs, feature index, demos, issue outputs, compression artifacts, and repo visibility artifacts | review-ready docs package | WP-09 through WP-12 | #2032 |
| WP-14 | Rust refactoring pass | Perform explicit, bounded Rust refactors justified by maintainability, testability, or review findings | refactor PRs and validation record | WP-10 through WP-13 | #2033 |
| WP-15 | Internal review | Conduct internal review and record findings | review artifact | WP-13, WP-14 | #2034 |
| WP-16 | Third-party review | Conduct external/third-party review and record findings | review artifact | WP-13, WP-14 | #2035 |
| WP-17 | Findings remediation | Fix or explicitly defer review findings | remediation PRs or defer log | WP-15, WP-16 | #2036 |
| WP-18 | Final quality and release readiness | Re-run quality, demo, docs, compression, visibility, and refactor readiness checks | release-readiness report | WP-17 | #2037 |
| WP-19 | Next milestone planning | Prepare the following milestone package | tracked planning package | WP-18 | #2038 |
| WP-20 | Release ceremony | Final validation, release notes, tag, and cleanup | release artifact set | WP-19 | #2039 |

## Candidate Scope Split

Default Sprint 1:

- WP-01 through WP-05

Default Sprint 2:

- WP-06 through WP-08

Sidecar / process sprint:

- WP-09 through WP-12

Release tail:

- WP-13 through WP-20

## Acceptance Mapping

- WP-01: tracked v0.90 package exists and issue wave can be created mechanically
- WP-02: supervisor/heartbeat state is explicit and bounded
- WP-03: every long-lived cycle has a durable artifact contract
- WP-04: continuity is explicit without claiming full identity
- WP-05: operator controls and safety reports exist
- WP-06: reviewers can inspect status and relevant trace evidence
- WP-07: stock league demo has safe fixtures and no-financial-advice framing
- WP-08: demo proves bounded multi-cycle continuity
- WP-09: selected demo additions or extensions are named, bounded, and validated
- WP-10: 93 percent coverage ratchet is measured, proven, and documented before threshold change
- WP-11: milestone compression pilot catches drift without autonomous release behavior
- WP-12: repo visibility prototype maps canonical docs to code, tests, demos, and review surfaces
- WP-13: docs and feature mappings are consistent across core and sidecar work
- WP-14: Rust refactors are bounded, validated, and justified by maintainability or testability
- WP-15 through WP-17: review findings are closed or deferred truthfully
- WP-18: final quality and release-readiness checks are complete
- WP-19: next milestone planning is ready before release ceremony
- WP-20: release package is complete
