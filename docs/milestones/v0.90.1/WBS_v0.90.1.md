# WBS - v0.90.1

## Work Package Shape

The issue wave uses the standard 20-WP milestone shape, but it front-loads a
short compression-enablement sprint before Runtime v2 coding begins. v0.90.1 is
still a follow-on foundation slice, not a full major milestone.

| WP | Issue | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- | --- |
| WP-01 | #2141 | Issue wave and task cards | Open the v0.90.1 issue wave from this tracked planning package | GitHub issues, task cards, and issue-number mapping | v0.90 closeout |
| WP-02 | #2142 | Issue-wave template and generator alignment | Make the WP wave seed mechanically reusable for this and later milestones | aligned template/generator proof | WP-01 |
| WP-03 | #2143 | Worktree-first workflow hardening | Prevent tracked root-checkout drift before compressed execution starts | workflow guardrails and tests | WP-02 |
| WP-04 | #2144 | Compression-era execution policy | Align skills, subagent rules, validation profiles, and SOR evidence expectations | operator policy and validation guidance | WP-02, WP-03 |
| WP-05 | #2145 | Manifold contract | Define and implement the persistent manifold root | manifest, tests, docs | WP-04 |
| WP-06 | #2146 | Kernel service loop | Implement bounded kernel loop and service registry | loop artifact, service state, tests | WP-05 |
| WP-07 | #2147 | Provisional citizen records | Add provisional citizen record and lifecycle states | citizen records, validation | WP-05 |
| WP-08 | #2148 | Snapshot and rehydration | Persist and restore manifold/citizen state | snapshot, wake report, tests | WP-06, WP-07 |
| WP-09 | #2149 | Invariant violation artifacts | Emit reviewable invariant failure evidence | violation packet, negative tests | WP-06 |
| WP-10 | #2150 | Operator controls | Add inspect, pause, resume, terminate controls | operator report, CLI/demo hook | WP-06, WP-08 |
| WP-11 | #2151 | Security-boundary proof | Prove one rejected invalid action through kernel policy | security proof packet | WP-09, WP-10 |
| WP-12 | #2152 | Runtime v2 demo | Integrate the foundation prototype into one demo | bounded demo and proof packet | WP-05-WP-11 |
| WP-13 | #2153 | Runtime v2 docs pass | Align feature docs, demo matrix, and README | coherent docs package | WP-12 |
| WP-14 | #2154 | Quality and coverage gate | Run focused tests and quality posture | quality report | WP-12 |
| WP-15 | #2155 | Internal review | Review scope, claims, and proof surfaces | review report and findings | WP-13, WP-14 |
| WP-16 | #2156 | Review remediation | Fix accepted internal review findings | patches and closure notes | WP-15 |
| WP-17 | #2157 | Release readiness | Finalize release notes, checklist, and handoff | readiness packet | WP-16 |
| WP-18 | #2158 | v0.91/v0.92 handoff | Preserve moral/emotional and birthday boundaries | handoff doc | WP-17 |
| WP-19 | #2159 | Release-evidence packet | Assemble the proof trail across demos, quality, review, and readiness | release evidence packet | WP-17, WP-18 |
| WP-20 | #2160 | Release ceremony | Complete tag/release ceremony | release closure | WP-18, WP-19 |

## Compression Candidate

This milestone is a strong compression candidate because WP-02 through WP-04
remove process friction before Runtime v2 coding starts, and the runtime outputs
can be proved by a compact artifact set.

Compression must not remove review discipline. It should reduce waiting and
redundant local validation only when focused proof is adequate.
