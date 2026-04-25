# WBS - v0.90.5

## Work Package Shape

v0.90.5 should use the standard release shape plus an explicit demo/proof lane.
It is an implementation milestone for Governed Tools v1.0, not a
documentation-only planning lane.

| WP | Issue | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- | --- |
| WP-01 | #2566 | Design pass (milestone docs + planning) | Finalize this planning package and create the issue wave | tracked docs and issue cards | v0.90.4 closeout |
| WP-02 | #2567 | Tool-call threat model and semantics | Define proposal/action boundary, side effects, dangerous categories, and non-goals | threat model and semantics doc | WP-01 |
| WP-03 | #2568 | UTS public compatibility and conformance plan | Define compatibility, examples, invalid examples, extension rules, and conformance | UTS conformance plan | WP-02 |
| WP-04 | #2569 | UTS v1 schema finalization | Finalize portable tool schema and validation rules | UTS schema and tests | WP-03 |
| WP-05 | #2570 | UTS fixture and conformance suite | Build valid, invalid, extension, and dangerous-category fixtures | UTS fixture suite | WP-04 |
| WP-06 | #2571 | ACC v1 authority schema | Define ADL-native authority, identity, policy, risk, and execution schema | ACC schema and authority fixtures | WP-02 |
| WP-07 | #2572 | ACC privacy, visibility, and delegation model | Define who may call, see, delegate, inspect, challenge, and receive redacted views | visibility/delegation matrix | WP-06 |
| WP-08 | #2573 | Tool registry and binding model | Register known tools and bind only approved adapters | registry and binding tests | WP-04, WP-06 |
| WP-09 | #2574 | UTS to ACC compiler | Compile validated UTS/proposals into ACC deterministically | compiler and mapping tests | WP-05-WP-08 |
| WP-10 | #2575 | Normalization and argument validation | Treat all tool arguments as untrusted input | normalization and rejection tests | WP-09 |
| WP-11 | #2576 | Policy injection and authority evaluation | Inject role, standing, environment, capability, and sensitivity policy | policy engine slice and tests | WP-07, WP-09 |
| WP-12 | #2577 | Freedom Gate integration | Evaluate candidate tool actions before execution | decision events and gate tests | WP-11 |
| WP-13 | #2578 | Governed executor | Execute only approved ACC-backed actions | executor and refusal behavior | WP-12 |
| WP-14 | #2579 | Trace, replay, redaction, and evidence contract | Emit accountable evidence without leaking private data | trace/redaction contract and tests | WP-13 |
| WP-15 | #2580 | Dangerous tool negative suite | Prove destructive, process, network, exfiltration, missing actor, unsafe replay, and delegation failures | negative safety suite | WP-08-WP-14 |
| WP-16 | #2581 | Model proposal benchmark harness | Test model tool proposals against schema, authority, privacy, and bypass rubrics | benchmark runner and report | WP-04, WP-15 |
| WP-17 | #2582 | Local model and Gemma-focused evaluation | Run a bounded local/Gemma-focused demo without taking on the full comparison suite | simple model demo scorecard and failure notes | WP-16 |
| WP-18 | #2583 | Governed Tools v1.0 flagship demo | Show proposal, validation, ACC, policy, gate, execution/denial, trace, and redaction | flagship demo proof packet | WP-13-WP-17 |
| WP-19 | #2584 | Demo matrix and feature proof coverage | Verify every governed-tools claim has proof, fixture, non-proving status, or deferral before review convergence | demo matrix update and proof coverage record | WP-18 |
| WP-20 | #2585 | Coverage / quality gate | Establish auditable quality, validation, and coverage posture for the implemented milestone | quality and coverage gate record | WP-19 |
| WP-21 | #2586 | Docs + review pass | Align docs, conformance, feature docs, review packets, public-spec language, and reviewer entry surfaces | review-ready docs package | WP-20 |
| WP-22 | #2587 | Internal review | Perform findings-first internal review against the converged milestone package | internal review record | WP-21 |
| WP-23 | #2588 | External / 3rd-party review | Prepare and complete external review against a legible package | external review handoff and record | WP-22 |
| WP-24 | #2589 | Review findings remediation | Fix accepted review findings or record explicit deferrals | remediation record and follow-up issues | WP-23 |
| WP-25 | #2590 | Next milestone planning | Capture follow-on work before release closeout, including the deferred full model comparison report | next milestone planning handoff | WP-24 |
| WP-26 | #2591 | Release ceremony | Complete release closure and next handoff | release evidence, end-of-milestone report, tag/release, and cleanup | WP-25 |

## Parallel Python Reduction Tranche

v0.90.5 should reserve bounded capacity for Python elimination without turning
the milestone into a Python-only rewrite band. The cross-milestone program is
recorded in [Python Elimination Staged Plan](../../planning/PYTHON_ELIMINATION_STAGED_PLAN.md).

Recommended `v0.90.5` tranche:

- freeze and no-new-tracked-Python rule
- Python inventory and disposition truth surface
- one coherent high-leverage Rust port or deletion wave

Exact issue selection should happen at issue-wave time based on the current
Python inventory, milestone pressure, and what can be finished truthfully.

## Separate Get-Well Runtime Reduction Wave

The test-cycle/runtime reduction work should stay visible in `v0.90.5`, but it
must remain separate from the canonical WP state machine. It should run as a
get-well wave early enough that later WPs benefit from the reduced validation
cost.

Tracked supporting plan:

- `GET_WELL_PLAN_v0.90.5.md`
- `ideas/TEST_RUNTIME_REDUCTION_PLAN_v0.90.5.md`

Opened get-well wave:

| Slice | Issue | Title |
| --- | --- | --- |
| GW-00 | #2592 | Get-well baseline, runtime budget, and wave tracking artifact |
| GW-01 | #2593 | Collapse external counterparty proof-family tests |
| GW-02 | #2594 | Collapse private-state observatory proof-family tests |
| GW-03 | #2595 | Collapse delegation subcontract proof-family tests |
| GW-04 | #2596 | Collapse contract-market and resource-stewardship proof-family tests |
| GW-05 | #2597 | Shrink CLI and demo proof-matrix tail |

Do not let the get-well wave replace the core UTS / ACC / compiler / policy /
executor / demo / review wave. WP-20 owns the final get-well disposition before
release closeout.

## Compression Candidate

The milestone can compress after UTS, ACC, fixtures, and compiler contracts are
stable. It must not compress away threat modeling, public conformance, model
testing, negative security tests, or redaction/visibility review.

Compression must not skip:

- the threat model and proposal/action boundary
- UTS conformance fixtures and invalid examples
- ACC authority, visibility, delegation, and redaction fixtures
- unknown-tool and unregistered-adapter rejection
- dangerous negative safety cases
- model proposal benchmarking
- feature-by-feature proof coverage before quality/review convergence
- coverage/quality gate before docs/review convergence
- findings-first internal review, third-party review, remediation, next
  planning, and release ceremony in that order
