# v0.91.2 Work Breakdown Structure

## Status

Active WBS. The complete v0.91.2 WP issue wave was opened as `#3000` through
`#3023`, with sprint-conductor umbrella issues `#3025` through `#3028`.
Every WP and sprint umbrella has a prepared primary-checkout local STP, SIP,
SPP, SRP, and SOR bundle before execution binding.

## WBS Summary

v0.91.2 is the backlog pressure-release and operating-system-for-the-project
milestone. It should make the project easier to test, review, explain,
productize, publish, and maintain.

## Work Areas

| Area | Work Area | Description | Primary Deliverable | Key Dependencies |
| --- | --- | --- | --- | --- |
| A | Design pass | Promote accepted v0.91.2 plan into issues and cards. | tracked docs, issue wave, and validated cards | v0.91.1 closeout |
| B | UTS + ACC multi-model benchmark | Test JSON proposal and provider-native tool-call behavior across models. | benchmark harness, fixtures, and comparison report | governed tools, ACIP |
| C | Runtime/test-cycle recovery | Reduce redundant or overbroad proof phases without weakening release truth. | test-cycle recovery implementation and report | current CI evidence |
| D | Coverage and quality ergonomics | Make changed-source and workspace coverage gates actionable and less surprising. | coverage diagnostics and focused-test guidance | C |
| E | CodeFriend productization | Promote review packet, product report, skill, and demo surfaces. | CodeFriend product/review package | review skills |
| F | Review skill and demo suite | Add review heuristics, skill demos, and repeatable review proof surfaces. | review demo matrix and skill docs | E |
| G | Google Workspace CMS bridge | Build bounded bridge/demo for draft docs, comments, and promotion packets. | GWS CMS demo and adapter boundary | governed tools |
| H | Code modernization | Define and prove a bounded Moderne/OpenRewrite modernization workflow grounded in deterministic recipes over the Lossless Semantic Tree (LST). | modernization demo packet | C, E |
| I | Speculative decoding | Evaluate bounded runtime acceleration without weakening deterministic commit semantics. | feature contract, prototype plan, and proof posture | B, C |
| J | Repo visibility follow-on | Turn the v0.90 prototype into a practical manifest/linkage follow-on for reviewers and planners. | manifest/linkage follow-on package | E, F |
| K | Publication program | Prepare arXiv/Medium paper packets without direct publication. | publication backlog and first packets | review/evidence docs |
| L | General intelligence paper packet | Advance the Mathematical Theory of General Intelligence source packet. | claim/citation/review-ready paper packet | K |
| M | Rustdoc and documentation cleanup | Address rustdoc gaps and tracked doc hygiene debt. | doc cleanup report and patches | D |
| N | Workflow guardrails | Prevent main writes, unsafe shell report generation, hung watchers, and card drift. | guardrail implementation and process docs | C, D |
| O | Review, quality, and release | Validate the milestone, remediate findings, and hand off later work. | review/remediation/release package with WP-20B truth preserved | all prior work |

## Candidate WP Sequence

| WP | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 (#3000) | Design pass (milestone docs + planning) | docs | tracked docs, reviewed YAML, and issue cards | v0.91.1 closeout |
| WP-02 (#3001) | UTS + ACC multi-model benchmark harness | tools | benchmark harness and fixture battery | WP-01; governed-tools baseline; ACIP substrate |
| WP-03 (#3002) | Provider-native tool-call comparison | tools | JSON proposal vs provider-native comparison report | WP-02 |
| WP-04 (#3003) | Runtime/test-cycle recovery | quality | reduced redundant proof phases and validation report | WP-01 |
| WP-05 (#3004) | Coverage gate ergonomics | quality | changed-source diagnostics and focused-test guide | WP-04 |
| WP-06 (#3005) | CodeFriend review packet productization | tools | review packet and product-report workflow package | WP-01; review skills and evidence-packet substrate |
| WP-07 (#3006) | Review heuristics skill and demos | demo | review heuristics docs, skill/demo updates, proof examples | WP-06 |
| WP-08 (#3007) | Google Workspace CMS bridge demo | tools | bounded Workspace content-card and promotion demo | WP-01; governed-tools authority and adapter boundary |
| WP-09 (#3008) | Rust-native GWS adapter boundary | tools | adapter feasibility and typed contract boundary | WP-08 |
| WP-10 (#3009) | Moderne / OpenRewrite LST modernization demo | tools | ADL-governed Moderne/OpenRewrite interaction demo | WP-01 |
| WP-11 (#3010) | Speculative decoding prototype | runtime | bounded speculative-decoding architecture and proof posture | WP-02, WP-04 |
| WP-12 (#3011) | Repo visibility follow-on | docs | manifest/linkage follow-on package | WP-06, WP-07 |
| WP-13 (#3012) | Publication program package | docs | arXiv/Medium paper-program backlog and process docs | WP-01; review/evidence docs and publication process notes |
| WP-14 (#3013) | General intelligence paper packet | docs | claim, citation, and review packet | WP-13 |
| WP-15 (#3014) | Rustdoc and doc cleanup | docs | rustdoc/doc cleanup patches and report | WP-05 |
| WP-16 (#3015) | Workflow guardrails hardening | tools | main-write, watcher, and safe-report guardrails | WP-04, WP-05 |
| WP-17 (#3016) | Demo matrix and proof coverage | demo | demo matrix and proof coverage record | WP-02, WP-03, WP-04, WP-05, WP-06, WP-07, WP-08, WP-09, WP-10, WP-11, WP-12, WP-13, WP-14, WP-15, WP-16 |
| WP-18 (#3017) | Coverage / quality gate | quality | validation posture and test/coverage record | WP-17 |
| WP-19 (#3018) | Docs + review pass | docs | docs review-entry package; later corrected by WP-20B truth updates | WP-18 |
| WP-20 (#3019) | Internal review | review | internal review record; first packet superseded for handoff by `WP-20B` | WP-19 |
| WP-21 (#3020) | External / 3rd-party review | review | external review handoff and record after accepted `WP-20B` remediation closure | WP-20 plus completed corrective `WP-20B` gate |
| WP-22 (#3021) | Review findings remediation | review | remediation record and follow-up issues | WP-21 |
| WP-23 (#3022) | Next milestone planning | docs | v0.92/v0.93 handoff update | WP-22 |
| WP-24 (#3023) | Release ceremony | release | release evidence and end-of-milestone report | WP-23 |

## Sprint Umbrellas

| Sprint | Issue | Title | Ordered Children |
| --- | --- | --- | --- |
| Sprint 1 | #3025 | Benchmark And Test-Cycle Recovery | WP-01, WP-02, WP-03, WP-04, WP-05 |
| Sprint 2 | #3026 | Review Product, Workspace Bridge, And Modernization | WP-06, WP-07, WP-08, WP-09, WP-10 |
| Sprint 3 | #3027 | Runtime Ergonomics, Publication, Docs, And Workflow Guardrails | WP-11, WP-12, WP-13, WP-14, WP-15, WP-16 |
| Sprint 4 | #3028 | Review, Remediation, Planning, And Release | WP-17, WP-18, WP-19, WP-20, WP-21, WP-22, WP-23, WP-24 |

Corrective review note: the original issue wave did not include `WP-20B`.
`WP-20B` now controls handoff truth because the first `WP-20` packet was too
thin for external review. Accepted `WP-20B` remediation issues have closed, so
`WP-21` starts from the refreshed post-remediation handoff.

## Sequencing Pressure

Runtime/test-cycle recovery should begin early because every later milestone
benefits from it. UTS+ACC benchmark work should separate model proposal quality
from execution authority. Google Workspace and modernization demos must stay
operator-gated and bounded. Speculative decoding should stay bounded to
deterministic commit semantics and must not smuggle in opaque runtime behavior
under the banner of acceleration. Repo visibility should consume the delivered
v0.90 baseline rather than pretending it never landed. Publication work should
produce packets and review surfaces, not direct publishing.
