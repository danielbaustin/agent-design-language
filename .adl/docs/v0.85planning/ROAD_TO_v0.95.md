# Road to v0.95

## Purpose

This document buckets the remaining ADL planning work into staged milestones from
`v0.85` through `v0.95`.

The goal is not to stuff every remaining concept into `v0.85`.
The goal is to give every currently planned idea a clear home on the path to a
fully integrated `v0.95` platform.

This is a sequencing and convergence document.
It is intentionally based on the currently visible planning corpus and repo
truth.

This roadmap should be read as the path for defining what ADL `1.0` actually
is.

This roadmap explicitly treats `v0.95` as the final feature-bearing convergence point before `1.0`. After `v0.95`, the default posture shifts from adding new architectural domains to hardening, integration, and launch discipline.
The working intent is:

- every currently known major planning theme is addressed by `v0.95`
- by `v0.95`, the platform shape for `1.0` is nailed down
- after that point, new additions should be treated as exceptional and judged
  against launch discipline rather than added casually

## Scope Rule

This plan is intended to reorganize and stage the remaining work, not to
silently expand or shrink the known concept set midflight.

Working rule:

- do not add speculative new product areas
- do not remove existing planned architectural themes
- do re-bucket overlapping or scattered concepts into milestone-sized slices
- do separate already-implemented work from remaining work

In other words:

This is a roadmap normalization pass, not a large scope rewrite.

Final-expansion rule:

- this roadmap update admits one final explicit gap set that was visible in the
  planning corpus but not yet assigned cleanly to milestone bands
- that gap set includes:
  - moral / constitutional cognition
  - signed trace architecture
  - tooling Rust migration
  - Layer 8 provider-contract maturation
- Zed integration remains recognized but is explicitly deferred unless later
  re-promoted by an explicit scope decision
- no other new architectural domains are added in this pass

The intent is to call time on roadmap sprawl.
After this pass, the planned domain set should be treated as closed for
`v1.0` feature-complete purposes unless an exceptional later decision says
otherwise.

## Framing

The roadmap should be read with three high-level bands:

- `v0.85` through `v0.89`: make the cognition, agency, convergence, and security architecture real, bounded, and demoable
- `v0.90` through `v0.92`: deepen reasoning, affect, and identity into a coherent cognitive substrate
- `v0.93` through `v0.95`: governance, convergence, hardening, and final platform integration
- `v0.95`: convergence and freeze; after this point, new major architectural themes require explicit justification and are not added by default

The intent is to preserve milestone discipline.
`v0.85` remains a strengthening milestone, not a catch-all milestone.

## What Is Already Implemented

The following are already materially present in the repo and therefore are not
the primary remaining-roadmap burden:

- deterministic ADL compilation and schema validation
- ExecutionPlan-based runtime execution
- deterministic sequential and fork/join workflow execution
- bounded concurrency and canonical ready-step ordering
- deterministic retries and step failure controls
- stable run artifacts, trace/debug surfaces, and bounded replay/diff tooling
- signing and verification surfaces
- initial remote execution MVP and early trust-policy guardrails
- HITL pause/resume
- ObsMem integration boundary plus deterministic indexing/retrieval demo
  surfaces
- bounded v0.8 Gödel loop surfaces and canonical Gödel artifacts
- Prompt Spec, card prompt generation, and prompt linting
- initial bounded reviewer tooling, reviewer provenance surfaces, and early
  card-review contracts
- first bounded task-bundle editor surface (early slice)
- tracked public task-record structure plus an initial bounded example bundle
- partial legacy-name to `adl` naming migration

## Explicitly Admitted Remaining Gaps

The following domains are now treated as officially recognized roadmap gaps and
must be assigned cleanly to later milestone bands rather than left implicit:

- moral / constitutional cognition
  - kindness model
  - moral resources subsystem
  - freedom-designed / freedom-gate architecture
- signed trace substrate
- tooling control-plane migration from shell-heavy logic toward Rust
- Layer 8 provider-contract maturation

Recognized but deferred:

- Zed integration

The roadmap should absorb the first four domains explicitly and leave Zed as a
post-`v0.95` path unless it is later promoted by explicit decision.

## Remaining Work by Milestone

### Sizing Rule

From `v0.86` onward, delivery should default to roughly **2-3 issue
clusters** each.

This is deliberate:

- smaller milestones reduce merge risk and cognitive overload
- slightly more milestones are acceptable if they reduce implementation drag
- ceremony should be reduced by process/tooling improvements rather than by
  overstuffing each milestone
- each milestone should still be large enough to feel substantive and reviewable
- it is acceptable to use sub-milestones such as `v0.86.1`, `v0.86.2`, etc.
  when that produces better slices than forcing a large parent milestone

### Parent-Band Rule

Use the major versions as architectural umbrellas:

- `v0.86` = cognitive control band
- `v0.88` = persistence, instinct, and bounded agency band
- `v0.89` = AEE convergence and security/threat modeling band
- `v0.90` = reasoning graph, signed trace, and trace query band
- `v0.91` = affect and moral cognition band
- `v0.92` = identity, continuity, and capability band
- `v0.93` = governance and delegation band
- `v0.95` = MVP convergence, tooling migration, and optional Zed band

Use sub-milestones as the actual executable planning slices.

### v0.85 - Operational Maturity

Theme:
make the current system more operationally mature without overloading the
milestone.

Issue clusters:

- execution substrate:
  deterministic queue/checkpoint/steering substrate, replay-compatible steering,
  and bounded cluster groundwork
- trust and proof surfaces:
  dependable execution, verifiable inference, provenance, demo matrix, and
  review packaging
- milestone convergence:
  Prompt Spec completion, editor/review finishing work, doc and issue-graph
  reconciliation, and final active-surface legacy-name to `adl` cutover

Why this belongs here:

- these items are close to current runtime/tooling reality
- they strengthen existing ADL claims instead of opening major new architecture
- they are needed before larger cognitive and identity layers become trustworthy

### Demo Table

| Milestone | Demo |
| --- | --- |
| `v0.86` | cognitive agent with arbitration + instinct + freedom gate |
| `v0.88` | agent shows persistence over time with instinct influence |
| `v0.89` | agent shows AEE convergence under explicit security/threat constraints |
| `v0.90` | reasoning graph + signed trace + query |
| `v0.91` | agent exhibits affect + kindness + bounded humor |
| `v0.92` | agent maintains identity across runs |
| `v0.93` | agent follows social contract + delegation |

### v0.86 - Cognitive Control Band

Theme:
make the first bounded cognitive-control surfaces real, inspectable, and
demoable.

Sub-milestones:

- `v0.86.1`:
  cognitive loop baseline
  observable loop artifacts and the first bounded cognitive cycle

- `v0.86.2`:
  arbitration + routing
  arbitration surfaces, fast/slow routing, and bounded decision control

- `v0.86.3`:
  freedom gate baseline
  bounded refusal, deferral, and explicit action selection centered on the
  Freedom Gate

Why this belongs here:

- the loop, stack, arbitration, and Freedom Gate already form one coherent
  first cognition band
- this milestone establishes a real decision architecture before persistence,
  convergence, and reasoning-graph work
- the proof surface is concrete and demoable

### v0.88 - Persistence, Instinct, and Bounded Agency Band

Theme:
make persistence-over-time and instinct-shaped agency real, inspectable, and
demoable.

Sub-milestones:

- `v0.88.1`:
  persistence over time
  temporal self-location, chronosense, and the first continuity-bearing runtime
  surfaces

- `v0.88.2`:
  instinct signals + runtime surface
  instinct declaration, instinct runtime surfaces, and simple prioritization
  hooks influenced by instinct/arbitration

- `v0.88.3`:
  bounded agency demo
  one clear demo showing persistence, prioritization, constraint, and
  accountable choice

Why this belongs here:

- persistence, instinct, and bounded agency are more coherent together than as
  separate tiny milestones
- the Freedom Gate belongs in the earlier cognition-control band, while
  instinct/runtime shaping and continuity belong here
- one strong persistence-plus-agency demo is more valuable than many
  speculative slices

### v0.89 - AEE Convergence Band

Theme:
make AEE minimally real and demonstrable under bounded security and threat
constraints, not fully generalized.

Sub-milestones:

- `v0.89.1`:
  convergence loop
  simple convergence signals and stop conditions

- `v0.89.2`:
  retry/adaptation loop
  visible adaptation behavior within bounded retries

- `v0.89.3`:
  convergence demo
  one demo proving bounded convergence under explicit security/threat
  constraints

### v0.90 - Reasoning Graph, Signed Trace, and Trace Query Band

Theme:
turn the bounded reasoning surfaces into a real inspectable provenance and query
stack.

Sub-milestones:

- `v0.90.1`:
  reasoning graph baseline
  first-class reasoning-graph artifacts and hypothesis/report structure

- `v0.90.2`:
  signed trace substrate
  signed-trace-ready provenance, Freedom Gate event capture, and durable trace
  structure

- `v0.90.3`:
  trace query
  bounded query and inspection surfaces over reasoning and trace artifacts

Why this belongs here:

- reasoning graph, signed trace, and query belong together as one inspectable
  evidence stack
- this gives reviewers a clean provenance story before later identity and
  governance integration
- the milestone is clearer and less overloaded than the old mixed Gödel v2 band

### v0.91 - Affect and Moral Cognition Band

Theme:
make the emerging cognitive substrate emotionally legible and normatively serious.

Sub-milestones:

- `v0.91.1`:
  affect engine
  minimal affect engine, update rules, and affect traces

- `v0.91.2`:
  moral cognition surfaces
  kindness model, moral resources subsystem, and inspectable moral-evaluation
  surfaces

- `v0.91.3`:
  affect + moral vertical slice
  runnable vertical slice showing affect-informed reasoning and bounded moral
  evaluation, with humor/absurdity support where it clarifies frame shifts

Why this belongs here:

- affect and moral cognition are a coherent band and should be readable as such
- reasoning graphs have already been moved earlier into Gödel work
- this keeps the milestone to three meaningful slices instead of five thin ones

Relevant planning inputs:

- `.adl/docs/v0.91planning/AFFECT_MODEL_v0.90.md`
- `.adl/docs/v0.91planning/KINDNESS_MODEL.md`
- `.adl/docs/v0.91planning/MORAL_RESOURCES_SUBSYSTEM.md`
- `.adl/docs/v0.91planning/HUMOR_AND_ABSURDITY.md`

### v0.92 - Identity and Capability Substrate Band

Theme:
introduce first-class identity as replayable runtime substrate.

Sub-milestones:

- `v0.92.1`:
  identity tuple substrate
  first-class agent/security identity fields, identity-aware replay inputs,
  normalized model refs, and identity store abstraction
- `v0.92.2`:
  capability substrate
  provider capability contracts, runtime probing, and effective capability
  envelopes
- `v0.92.3`:
  continuity bridge
  temporal continuity hooks and the bridge from cognitive continuity to
  identity-bearing agents

Why this belongs here:

- identity is adjacent to trust, replay, and cognition, but it is a large enough theme to deserve its own milestone
- current identity architecture docs are forward-looking and should not be forced into `v0.85`
- this milestone builds the substrate before full IAM and narrative identity rollout
- provider capability probing is part of the same
  replay/trust/accountability substrate and should land before richer
  policy/governance layers, but full Layer 8 provider-contract maturation is
  deferred to the final hardening band

### v0.93 - Governance and Delegation Band

Theme:
turn identity substrate into accountable governance and delegation surfaces.

Sub-milestones:

- `v0.93.1`:
  IAM and policy
  agent principals, authn/authz, audit hooks, and least-privilege policy surfaces

- `v0.93.2`:
  constitutional delegation
  constitution artifacts, delegation contracts, rights/duties, and governed
  autonomy surfaces

- `v0.93.3`:
  social contract and rights/duties
  enforceable social contract surfaces, rights/duties articulation, and
  governed delegation boundaries

Why this belongs here:

- IAM, constitutional delegation, and explicit rights/duties form one
  accountable-governance layer
- signed trace has already been pulled earlier into the reasoning/provenance
  stack
- this keeps v0.93 focused on governed autonomy rather than mixing provenance
  and governance again

### v0.95 - MVP Convergence, Tooling Migration, and Optional Zed Band

Theme:
finish the path by making the major architectural layers work together as one
coherent launch-shape platform.

Sub-milestones:

- `v0.95.1`:
  MVP walkthrough and demos
  end-to-end walkthrough, polished demos, and proof-surface closure for the
  converged platform story

- `v0.95.2`:
  platform convergence
  contract cleanup across docs/spec/runtime/tooling, integrated convergence
  planning, and launch-shape platform alignment

- `v0.95.3`:
  tooling migration, optional Zed, and `1.0` freeze
  Rust migration of the highest-risk tooling surfaces, optional Zed carry-in if
  still desired, and explicit scope freeze for `1.0`

Required cross-cutting catch-up issue:

- demo catalog completion:
  every user-visible implemented feature should have at least one polished,
  sensibly named feature demo or runbook-oriented demo path by `v0.95.3`, with
  the catalog reviewed as part of the final convergence gate

What counts as a demo obligation:

- user-visible feature surfaces do count
- combined multi-feature stories do count
- migration-only work does not count
- pure verification work does not count
- internal refactors do not count unless they create a new user-visible
  capability
- tests are not demos and cannot substitute for demo coverage

Relevant planning inputs for this band:

- `.adl/docs/v0.95planning/MVP_WALKTHROUGH_AND_DEMOS.md`
- `.adl/docs/v0.95planning/PLATFORM_CONVERGENCE_PLAN.md`
- `.adl/docs/v0.95planning/TOOLING_RUST_MIGRATION_PLAN.md`
- `.adl/docs/v0.95planning/ZED_INTEGRATION_WITH_ADL.md`

Clarifications:

- the provider abstraction already exists in the repo, so Layer 8 is treated as
  a maturation / contract-hardening problem rather than a greenfield domain
- Zed integration remains recognized but is not part of the `v0.95`
  must-have closure set
- after the additions in this roadmap pass, no additional major architectural
  domains should be admitted silently before `1.0`

Demo policy:

- demos are for users and reviewers to understand behavior, not for replacing
  tests
- tests verify correctness; demos illustrate capability
- not every internal refactor or migration needs a demo
- features do need demos
- demos should be grouped around interesting primary capabilities rather than
  one tiny demo per helper function
- demo names should describe the main thing a reviewer is meant to learn, not
  just expose an internal codename

Demo work as a first-class roadmap issue:

- demo backlog catch-up should be tracked as its own issue stream, not hidden
  inside generic QA or release polish
- each milestone that lands a user-visible feature should either ship its demo
  in the same slice or explicitly create a named carry-forward demo issue
- `v0.95.3` is the mandatory closeout point for any remaining feature-demo debt

Definition of success:

- every major currently planned idea has a working home
- all major layers are represented by real runtime, tooling, or artifact-bearing proof surfaces
- the platform is coherent enough that the roadmap stops looking like separate concept piles
- the known planning corpus has been consumed into the roadmap rather than left
  as scattered orphan concepts
- ADL `1.0` scope is concrete enough that additions after `v0.95` are judged
  against launch discipline, not architectural drift
- user-visible implemented features have polished demo coverage, not just test
  coverage

## Reasoning for the Split

This split is intentionally conservative.

The core reasoning is:

- `v0.85` should remain a strengthening milestone rather than a catch-all milestone
- `v0.86` should establish cognition control cleanly rather than act as a future authoring catch-all
- cognitive control should land before bounded agency, and bounded agency before stronger AEE work
- the Freedom Gate belongs with first cognition-control surfaces, not as a late add-on
- AEE plus bounded security should become concrete before the reasoning/provenance stack deepens
- reasoning-graph artifacts, signed trace, and query belong together rather than being spread across later bands
- affect and moral cognition should form one readable milestone band rather than many thin slices
- identity substrate should come before governance and delegation
- final convergence should focus on tooling migration, integration, demos, and freeze discipline
- `v0.95` should be the point where the `1.0` shape is effectively frozen and the project becomes highly conservative about adding new architectural domains

## Dependency Chain

The intended dependency flow is:

1. `v0.85` operational maturity
2. `v0.86.1` to `v0.86.3` cognitive control
3. `v0.88.1` to `v0.88.3` persistence, instinct, and bounded agency
4. `v0.89.1` to `v0.89.3` AEE convergence and security/threat modeling
5. `v0.90.1` to `v0.90.3` reasoning graph, signed trace, and trace query
6. `v0.91.1` to `v0.91.3` affect and moral cognition
7. `v0.92.1` to `v0.92.3` identity, continuity, and capability substrate
8. `v0.93.1` to `v0.93.3` governance and delegation
9. `v0.95.1` to `v0.95.3` MVP convergence, tooling migration, demo closure, and `1.0` definition freeze

## Guardrail

If future planning updates are made from this document, preserve this rule:

- re-bucket work if needed
- clarify acceptance boundaries if needed
- do not quietly add whole new architectural domains without marking them as new scope
- do not quietly remove currently planned domains without explicitly documenting the cut
- after `v0.95`, do not add new major architectural domains before launch
  unless there is a strong, explicitly documented justification reviewed
  against launch discipline

That guardrail keeps the roadmap stable enough to share externally while still
allowing milestone-sized adjustments.
