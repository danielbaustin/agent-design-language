# Sprint Plan - v0.91.2

## Status

Candidate sprint plan for review.

## Sprint 1: Benchmark And Test-Cycle Recovery

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.91.1 closeout |
| WP-02 | UTS + ACC multi-model benchmark harness | benchmark harness and fixture battery | WP-01 |
| WP-03 | Provider-native tool-call comparison | JSON proposal vs provider-native comparison report | WP-02 |
| WP-04 | Runtime/test-cycle recovery | reduced redundant proof phases and validation report | WP-01 |
| WP-05 | Coverage gate ergonomics | changed-source diagnostics and focused-test guide | WP-04 |

Goal: stop guessing about tool-call model behavior and stop losing days to
expensive, confusing test cycles.

## Sprint 2: Review Product, Workspace Bridge, And Modernization

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-06 | CodeBuddy review packet productization | review packet and product-report workflow package | WP-01 |
| WP-07 | Review heuristics skill and demos | review heuristics docs, skill/demo updates, proof examples | WP-06 |
| WP-08 | Google Workspace CMS bridge demo | bounded Workspace content-card and promotion demo | WP-01 |
| WP-09 | Rust-native GWS adapter boundary | adapter feasibility and typed contract boundary | WP-08 |
| WP-10 | Code modernization demo | Moderne/code modernization interaction demo | WP-01 |

Goal: turn review, collaborative docs, and modernization ideas into bounded
product surfaces without granting silent authority over canonical repo truth.

## Sprint 3: Publication, Docs, And Workflow Guardrails

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-11 | Publication program package | arXiv/Medium paper-program backlog and process docs | WP-01 |
| WP-12 | General intelligence paper packet | claim, citation, and review packet | WP-11 |
| WP-13 | Rustdoc and doc cleanup | rustdoc/doc cleanup patches and report | WP-05 |
| WP-14 | Workflow guardrails hardening | main-write, watcher, and safe-report guardrails | WP-04, WP-05 |

Goal: make the project’s public intellectual surface and daily workflow less
fragile, less ambiguous, and easier for other humans to review.

## Sprint 4: Review, Remediation, And Release

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-15 | Demo matrix and proof coverage | demo matrix and proof coverage record | WP-02-WP-14 |
| WP-16 | Coverage / quality gate | validation posture and test/coverage record | WP-15 |
| WP-17 | Docs + review pass | review-ready docs package | WP-16 |
| WP-18 | Internal review | internal review record | WP-17 |
| WP-19 | External / 3rd-party review | external review handoff and record | WP-18 |
| WP-20 | Review findings remediation | remediation record and follow-up issues | WP-19 |
| WP-21 | Next milestone planning | v0.92/v0.93 handoff update | WP-20 |
| WP-22 | Release ceremony | release evidence and end-of-milestone report | WP-21 |

Goal: leave the next identity/governance milestones with cleaner test cycles,
clearer publication and product surfaces, and fewer workflow foot-guns.

## Parallelization Notes

UTS+ACC benchmark work and runtime/test-cycle recovery can proceed in parallel.
CodeBuddy and review-skill work can proceed beside the Google Workspace bridge
if both preserve canonical repo authority. Publication packet work can proceed
beside doc cleanup, but no public release should happen before review.
