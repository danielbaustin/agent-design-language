# v0.93-v0.95 MVP Feature-Doc Production Plan

## Status

Tracked planning output for issue `#3781`.

This document is a feature-doc production and split plan. It does not implement
runtime behavior, approve milestone execution, or claim MVP readiness. Its job
is to make every known post-`v0.92` MVP surface visible before `v0.95`, while
keeping post-`v0.95` product work outside the MVP claim.

Current verdict: the v0.93-v0.95 surface is too broad for one implementation
issue. The right output is a small set of focused feature-doc issues or
mini-sprints, all consuming the existing feature list and milestone packages.

## Source Evidence

Tracked sources:

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`
- `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.93/`
- `docs/milestones/v0.94/`
- `docs/milestones/v0.95/`

Local authoring sources used as input only:

- local MVP feature-doc production and scope-lock plans
- local CodeFriend and portable adapter v2 planning
- local security and CAV planning
- local upstream-delegation planning
- local guilds planning
- local MVP cleanup and AI character-audit planning
- local Rust/refactoring and tooling-hardening notes

The local authoring sources are not promoted or linked as public output by this
issue. Public work should happen in tracked docs and issue records.

## Scope Rules

- `v0.95` is MVP convergence and feature freeze, not the first implementation
  home for major product or cognitive systems.
- `v0.93` owns governance, security governance, social cognition, delegation,
  upstream delegation, IAM, and guild baseline feature-doc production.
- `v0.94` owns secure execution, trust convergence, signed/queryable trace,
  reasoning/provenance closure, and temporal self-projection feature-doc
  production.
- CodeFriend v1 and portable adapter v2 are required before `v0.95` so the MVP
  can consume external-repo proof.
- Aptitude Atlas productization and baseline-product work are post-`v0.95`;
  `v0.95` consumes capability-testing evidence only.
- The `v0.95` package must keep Aptitude Atlas productization post-`v0.95`
  and describe MVP scope as capability-testing evidence consumption only.
- Rust refactoring before Sprint 4 / MVP convergence must reduce
  change-specific test burden. Merely splitting large files into parts is not
  sufficient.
- Guilds are MVP-scoped and must not become a post-`v0.95` orphan.

## Production Split

| Packet | Target milestone | Required output | Why this must be separate |
| --- | --- | --- | --- |
| Governance, security, and guilds | `v0.93` | Feature-doc refresh or issue split for citizenship, rights/duties, social contract, ToM/social cognition, relationship/reputation/shared memory, delegation/upstream delegation/IAM, enterprise security, and guilds. | Governance and security are implementation-heavy and must consume v0.92 identity evidence rather than being squeezed into MVP packaging. |
| Secure execution, trust, trace, and time projection | `v0.94` | Feature-doc refresh or issue split for secure execution, policy/identity/auth convergence, provider trust/isolation, secrets/data governance, signed/queryable trace, reasoning graph baseline, and mental time travel / temporal self-projection. | These depend on v0.93 governance/security and should not reopen those decisions. |
| CodeFriend v1 and adapter v2 proof | after `v0.92`, before `v0.95` | Feature doc and issue wave for portable adapter v2 plus smallest CodeFriend v1 proof. | CodeFriend needs external-repo readiness before MVP, but broad product UX, accounts, billing, and report UX are post-`v0.95`. |
| Rust refactoring and tooling hardening | last before Sprint 4 / `v0.95` convergence | Semantic refactoring mini-sprint plan that reduces test blast radius for future changes and aligns with control-plane hardening. | This must not become file-splitting theater or destabilize MVP delivery. |
| MVP convergence, demos, cleanup, and post-MVP map | `v0.95` | Convergence packet, demo catalog/walkthrough plan, MVP cleanup plan, character/style cleanup route, capability-evidence consumption boundary, and post-`v0.95` disposition map. | MVP should package proof and freeze scope, not invent new platform domains. |

## v0.93 Feature-Doc Work

Required v0.93 feature-doc surfaces:

- constitutional citizenship, rights, duties, and social contract
- guest, citizen, service actor, human provider, and operator boundary
- bounded Theory of Mind, relationship model, reputation, and shared social
  memory boundary
- standing maintenance, degradation, restoration, suspension, revocation,
  challenge, and appeal
- delegation, upstream delegation, IAM, and authority-chain governance
- enterprise security for the ADL polis
- zero-trust architecture
- policy enforcement and authorization
- secrets, keys, cryptographic trust, signing, encryption, rotation, and
  revocation
- tamper-evident audit, compliance evidence, and incident evidence
- tenant/polis isolation, data governance, retention, and privacy
- security operations, CAV-aligned regression, provenance, and runtime
  hardening
- guilds and collective organization

Guilds minimum MVP baseline:

- guild identity and membership model
- relationship to citizens, guests, operators, services, and polis governance
- authority and delegation boundaries
- resource, workspace, or capability scope
- isolation and privacy expectations
- trace/provenance requirements for guild action
- review/challenge path when guild action affects shared reality

Exit condition: `v0.95` can consume a guild baseline as part of governance
proof, or a tracked operator decision narrows the MVP boundary.

## v0.94 Feature-Doc Work

Required v0.94 feature-doc surfaces:

- secure execution model
- policy engine architecture
- identity/auth convergence
- provider trust and isolation
- sandbox/runtime isolation
- secrets and data governance
- signed/queryable trace closure
- reasoning graph baseline and reasoning/provenance closure
- bounded mental time travel / temporal self-projection

Dependency rule: `v0.94` consumes `v0.93` governance and enterprise-security
decisions. It must not become a second governance milestone.

Payments and settlement remain outside this packet unless a separate tracked
decision promotes the `v0.94.1` economics/payment work into MVP inputs.

## CodeFriend v1 And Adapter v2

CodeFriend v1 happens before `v0.95`.

Portable adapter v2 is required for CodeFriend v1 because CodeFriend must be
able to prove external-repo review without relying on operator memory or
private ADL workspace state.

Smallest acceptable CodeFriend v1 proof:

1. Clone or prepare a target repository outside the ADL repo.
2. Install a portable ADL adapter with repo-local `AGENTS.md` and
   `adl_project.json`.
3. Run an adapter doctor that proves tooling resolution, project profile,
   local-state boundary, validation profile, and redaction policy.
4. Run a bounded CodeFriend review workflow through ADL against a declared
   commit and scope.
5. Produce a review-run manifest naming target repo identity, commit, scope,
   reviewer lanes, allowed artifacts, validation profile, and redaction policy.
6. Export findings, diagrams, tests or test recommendations, residual risks,
   and follow-up issue candidates as review artifacts before any live
   issue/PR mutation.
7. Prove no credentials, unredacted provider logs, absolute host paths, or
   private `.adl` state are present in public artifacts.

Adapter v2 required capabilities:

- `codefriend` project profile
- `csdlc` project profile or mode for external-repo lifecycle work
- conservative installer/bootstrap command
- review-only and C-SDLC issue-work modes
- multi-repo run context
- review-run manifest
- product-safe evidence exporter
- explicit issue-export policy
- fail-closed doctor for setup conflicts, missing tooling, incompatible
  templates, or unclear authority

Non-goals:

- no automatic customer PR mutation by default
- no customer-data ingestion before redaction and credential boundaries exist
- no copying private `.adl` state into target repositories
- no broad product UX/accounts/billing/report UX inside the MVP proof

## Rust Refactoring Mini-Sprint

The Rust refactoring mini-sprint should run after logging/tooling and toolkit
simplification settle, and before Sprint 4 / `v0.95` convergence.

The mini-sprint should focus on semantic boundaries that reduce the amount of
code and validation needed for a given change:

- isolate lifecycle/card validation from publication and GitHub transport
- isolate Git/worktree operations behind typed seams or adapters with focused
  tests
- isolate prompt-template rendering, values editing, schema validation, and
  diagnostics so card changes do not require broad runtime validation
- isolate docs-only/review-only fast paths from runtime/source/test-affecting
  validation lanes
- use module boundaries that map to proof lanes, not just smaller files
- add focused fixtures that prove behavior without forcing full workspace test
  cycles for narrow edits

Acceptance bar:

- every refactoring slice names the behavior it preserves
- every slice states the smaller test surface it enables
- no slice is justified only by line count or file size
- no broad release confidence is weakened by faster local validation

## v0.95 MVP Convergence Work

Required v0.95 convergence surfaces:

- MVP boundary and convergence packet
- polished demo catalog
- coherent MVP walkthrough
- dashboard and compression reporting
- Shepherd/Gemma evidence and evaluator/training path
- capability-testing evidence consumption without Aptitude Atlas
  productization
- CodeFriend v1 and adapter v2 proof packaging, now routed through
  `docs/milestones/v0.95/features/CODEFRIEND_V1_PORTABLE_ADAPTER_V2_PROOF_v0.95.md`
- distributed execution integration closure
- control-plane/tooling hardening and Rust refactoring
- web editor baseline
- Zed decision boundary
- AI character/style cleanup route
- Aptitude Atlas v0.95 package reconciliation so README, WBS, and feature-doc
  language align with evidence-consumption-only MVP scope; productization is
  post-`v0.95`
- logistic split decision gate
- post-`v0.95` disposition map
- feature freeze and `1.0` scope boundary

MVP cleanup should include the AI character/style audit only as a bounded
cleanup route:

- preserve `Godel`/`Goedel`/`Gödel` terminology decisions according to the
  repo's chosen style policy
- preserve mathematical notation where it is meaningful
- normalize obvious style drift only in a dedicated cleanup pass or when
  touching a file for related reasons
- avoid noisy repo-wide punctuation churn during release tail

## Post-v0.95 Boundaries

Explicitly post-`v0.95` unless separately promoted by an operator decision:

- Aptitude Atlas productization and public baseline product
- broad CodeFriend product UX, accounts, billing, repo-connection UI, and
  polished report UX
- broad Google Workspace CMS productization
- Rust-transpiler service-line revival
- external compliance certification claims
- repository logistic split execution
- payment/economics surfaces not explicitly promoted into MVP inputs

`v0.95` may consume evidence from these areas only when the evidence is already
tracked and reviewable. It must not imply the product surface is complete.

Known reconciliation requirement: keep `v0.95` milestone-package language
aligned with the current feature-list boundary: evidence consumption in MVP,
Aptitude Atlas productization after `v0.95`.

## Issue Split Candidates

These are the recommended child issues or mini-sprints after this plan is
reviewed:

| Candidate | Target | Summary |
| --- | --- | --- |
| v0.93 governance/security/guilds feature-doc packet | `v0.93` | Refresh or split governance, security governance, enterprise security, upstream delegation/IAM, and guild feature docs. |
| v0.94 secure execution/trust/trace/time feature-doc packet | `v0.94` | Refresh or split secure execution, trust convergence, signed/queryable trace, reasoning graph baseline, and temporal self-projection docs. |
| CodeFriend v1 and portable adapter v2 proof packet | pre-`v0.95` | Define and execute the smallest external-repo CodeFriend proof after adapter v2 exists; feature-doc home now exists at `docs/milestones/v0.95/features/CODEFRIEND_V1_PORTABLE_ADAPTER_V2_PROOF_v0.95.md`. |
| Semantic Rust refactoring mini-sprint | pre-Sprint 4 / `v0.95` | Refactor by test-surface and proof-lane boundaries rather than file-part splitting. |
| v0.95 MVP convergence packet | `v0.95` | Produce the final MVP boundary, demo catalog/walkthrough, cleanup, post-MVP map, and feature-freeze packet. |
| Aptitude Atlas v0.95 package reconciliation | `v0.95` convergence | Align README, WBS, and feature-doc language so MVP consumes capability-testing evidence only while Aptitude Atlas productization remains post-`v0.95`. |

## Validation Plan

When this plan is updated:

- run `git diff --check`
- verify the planning README links this plan
- scan added public-doc lines for host-local paths, secret markers, and local
  authoring-workspace links
- scan for required surfaces: governance, security, guilds, secure execution,
  trust, trace, time projection, CodeFriend v1, adapter v2, Rust refactoring,
  demo catalog, MVP cleanup, AI character audit, Aptitude Atlas post-`v0.95`,
  and `v0.93` through `v0.95`
- run bounded pre-PR review focused on missing MVP surfaces, milestone
  overclaiming, and post-`v0.95` boundary drift

## Current Verdict

If MVP scope locked today, the known v0.93-v0.95 feature-doc work is accounted
for, but it is not executed. Aptitude Atlas productization remains post-`v0.95`;
the MVP consumes capability-testing evidence only. The next step is to execute
the child feature-doc packets above after the pre-`v0.92` bridge work is
truthfully closed or routed.
