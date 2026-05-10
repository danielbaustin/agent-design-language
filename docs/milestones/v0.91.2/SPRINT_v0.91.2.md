# Sprint Plan - v0.91.2

## Status

Candidate sprint plan for review.

## Sprint 1: Benchmark And Test-Cycle Recovery

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.91.1 closeout |
| WP-02 | UTS + ACC multi-model benchmark harness | benchmark harness and fixture battery | WP-01; governed-tools baseline; ACIP substrate |
| WP-03 | Provider-native tool-call comparison | JSON proposal vs provider-native comparison report | WP-02 |
| WP-04 | Runtime/test-cycle recovery | reduced redundant proof phases and validation report | WP-01 |
| WP-05 | Coverage gate ergonomics | changed-source diagnostics and focused-test guide | WP-04 |

Goal: stop guessing about tool-call model behavior and stop losing days to
expensive, confusing test cycles.

## Sprint 2: Review Product, Workspace Bridge, And Modernization

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-06 | CodeBuddy review packet productization | review packet and product-report workflow package | WP-01; review skills and evidence-packet substrate |
| WP-07 | Review heuristics skill and demos | review heuristics docs, skill/demo updates, proof examples | WP-06 |
| WP-08 | Google Workspace CMS bridge demo | bounded Workspace content-card and promotion demo | WP-01; governed-tools authority and adapter boundary |
| WP-09 | Rust-native GWS adapter boundary | adapter feasibility and typed contract boundary | WP-08 |
| WP-10 | Moderne / OpenRewrite LST modernization demo | ADL-governed Moderne/OpenRewrite interaction demo | WP-01 |

Goal: turn review, collaborative docs, and Moderne/OpenRewrite LST modernization ideas into bounded
product surfaces without granting silent authority over canonical repo truth.

## Sprint 3: Runtime Ergonomics, Publication, Docs, And Workflow Guardrails

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-11 | Speculative decoding prototype | bounded speculative-decoding architecture and proof posture | WP-02, WP-04 |
| WP-12 | Repo visibility follow-on | manifest/linkage follow-on package | WP-06, WP-07 |
| WP-13 | Publication program package | arXiv/Medium paper-program backlog and process docs | WP-01; review/evidence docs and publication process notes |
| WP-14 | General intelligence paper packet | claim, citation, and review packet | WP-13 |
| WP-15 | Rustdoc and doc cleanup | rustdoc/doc cleanup patches and report | WP-05 |
| WP-16 | Workflow guardrails hardening | main-write, watcher, and safe-report guardrails | WP-04, WP-05 |

Goal: make the project’s public intellectual surface and daily workflow less
fragile, less ambiguous, easier for other humans to review, and more honest
about which runtime accelerations and repo-cognition surfaces are actually
worth carrying forward.

## Sprint 4: Review, Remediation, And Release

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-17 | Demo matrix and proof coverage | demo matrix and proof coverage record | WP-02, WP-03, WP-04, WP-05, WP-06, WP-07, WP-08, WP-09, WP-10, WP-11, WP-12, WP-13, WP-14, WP-15, WP-16 |
| WP-18 | Coverage / quality gate | validation posture and test/coverage record | WP-17 |
| WP-19 | Docs + review pass | review-ready docs package | WP-18 |
| WP-20 | Internal review | internal review record | WP-19 |
| WP-21 | External / 3rd-party review | external review handoff and record | WP-20 |
| WP-22 | Review findings remediation | remediation record and follow-up issues | WP-21 |
| WP-23 | Next milestone planning | v0.92/v0.93 handoff update | WP-22 |
| WP-24 | Release ceremony | release evidence and end-of-milestone report | WP-23 |

Goal: leave the next identity/governance milestones with cleaner test cycles,
clearer publication and product surfaces, and fewer workflow foot-guns.

## Parallelization Notes

UTS+ACC benchmark work and runtime/test-cycle recovery can proceed in parallel.
CodeBuddy and review-skill work can proceed beside the Google Workspace bridge
if both preserve canonical repo authority. Publication packet work can proceed
beside doc cleanup. Speculative decoding can proceed beside publication and doc
cleanup once the benchmark and recovery surfaces define the bounded runtime
constraints. Repo visibility can proceed beside review productization because
it is meant to help reviewer navigation rather than bypass it. No public
release should happen before review.
