# Sprint Plan - v0.91.2

## Status

Active sprint plan. The complete v0.91.2 WP issue wave is open as `#3000`
through `#3023`, with sprint-conductor umbrella issues `#3025` through
`#3028`. Every WP and sprint umbrella has a prepared primary-checkout local
STP, SIP, SPP, SRP, and SOR bundle before execution binding.

## Sprint 1: Benchmark And Test-Cycle Recovery

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 (#3000) | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.91.1 closeout |
| WP-02 (#3001) | UTS + ACC multi-model benchmark harness | benchmark harness and fixture battery | WP-01; governed-tools baseline; ACIP substrate |
| WP-03 (#3002) | Provider-native tool-call comparison | JSON proposal vs provider-native comparison report | WP-02 |
| WP-04 (#3003) | Runtime/test-cycle recovery | reduced redundant proof phases and validation report | WP-01 |
| WP-05 (#3004) | Coverage gate ergonomics | changed-source diagnostics and focused-test guide | WP-04 |

Goal: stop guessing about tool-call model behavior and stop losing days to
expensive, confusing test cycles.

## Sprint 2: Review Product, Workspace Bridge, And Modernization

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-06 (#3005) | CodeFriend review packet productization | review packet and product-report workflow package | WP-01; review skills and evidence-packet substrate |
| WP-07 (#3006) | Review heuristics skill and demos | review heuristics docs, skill/demo updates, proof examples | WP-06 |
| WP-08 (#3007) | Google Workspace CMS bridge demo | bounded Workspace content-card and promotion demo | WP-01; governed-tools authority and adapter boundary |
| WP-09 (#3008) | Rust-native GWS adapter boundary | adapter feasibility and typed contract boundary | WP-08 |
| WP-10 (#3009) | Moderne / OpenRewrite LST modernization demo | ADL-governed Moderne/OpenRewrite interaction demo | WP-01 |

Goal: turn review, collaborative docs, and Moderne/OpenRewrite LST modernization ideas into bounded
product surfaces without granting silent authority over canonical repo truth.

## Sprint 3: Runtime Ergonomics, Publication, Docs, And Workflow Guardrails

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-11 (#3010) | Speculative decoding prototype | bounded speculative-decoding architecture and proof posture | WP-02, WP-04 |
| WP-12 (#3011) | Repo visibility follow-on | manifest/linkage follow-on package | WP-06, WP-07 |
| WP-13 (#3012) | Publication program package | arXiv/Medium paper-program backlog and process docs | WP-01; review/evidence docs and publication process notes |
| WP-14 (#3013) | General intelligence paper packet | claim, citation, and review packet | WP-13 |
| WP-15 (#3014) | Rustdoc and doc cleanup | rustdoc/doc cleanup patches and report | WP-05 |
| WP-16 (#3015) | Workflow guardrails hardening | main-write, watcher, and safe-report guardrails | WP-04, WP-05 |

Goal: make the project’s public intellectual surface and daily workflow less
fragile, less ambiguous, easier for other humans to review, and more honest
about which runtime accelerations and repo-cognition surfaces are actually
worth carrying forward.

## Sprint 4: Review, Remediation, And Release

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-17 (#3016) | Demo matrix and proof coverage | demo matrix and proof coverage record | WP-02, WP-03, WP-04, WP-05, WP-06, WP-07, WP-08, WP-09, WP-10, WP-11, WP-12, WP-13, WP-14, WP-15, WP-16 |
| WP-18 (#3017) | Coverage / quality gate | validation posture and test/coverage record | WP-17 |
| WP-19 (#3018) | Docs + review pass | review-ready docs package | WP-18 |
| WP-20 (#3019) | Internal review | internal review record | WP-19 |
| WP-21 (#3020) | External / 3rd-party review | external review handoff and record | WP-20 |
| WP-22 (#3021) | Review findings remediation | remediation record and follow-up issues | WP-21 |
| WP-23 (#3022) | Next milestone planning | v0.92/v0.93 handoff update | WP-22 |
| WP-24 (#3023) | Release ceremony | release evidence and end-of-milestone report | WP-23 |

Goal: leave the next identity/governance milestones with cleaner test cycles,
clearer publication and product surfaces, and fewer workflow foot-guns.

## Parallelization Notes

Each sprint umbrella is expected to run sequentially under `sprint-conductor`:
one active child issue at a time, with no `N+1` start before `N` closeout.
Parallelization, if any, should happen only at the umbrella level as an
explicit operator choice after `WP-01` has opened the milestone cleanly.

That means the intended parallelization shape is:

- Sprint 1 may proceed before Sprint 2 only if the milestone owner chooses to
  overlap umbrellas and the dependency truth still holds
- Sprint 2 may proceed beside Sprint 3 only as a separate umbrella, not as
  parallel child issues inside one umbrella
- Publication packet work may sit beside doc cleanup only when they are carried
  as distinct umbrellas or distinct sequential child issues in the same
  umbrella
- Repo visibility may be clustered with review productization in one umbrella,
  but it should still execute as one child issue at a time

No public release should happen before review.

## Sprint Umbrella Issues

| Sprint | Issue | Ordered Children |
| --- | --- | --- |
| Sprint 1 | #3025 | #3000, #3001, #3002, #3003, #3004 |
| Sprint 2 | #3026 | #3005, #3006, #3007, #3008, #3009 |
| Sprint 3 | #3027 | #3010, #3011, #3012, #3013, #3014, #3015 |
| Sprint 4 | #3028 | #3016, #3017, #3018, #3019, #3020, #3021, #3022, #3023 |
