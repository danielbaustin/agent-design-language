# Post-v0.95 ADL Repository Split Plan

## Status

Planned post-`v0.95` repository split strategy.

This document records the intended direction: after `v0.95`, ADL should become
stable enough to use as a dependable substrate, and a split-off fast-moving
repository should carry the next wave of cognitive-system development.

The split is not a fork in the hostile or abandonment sense. It is a governed
two-repository operating model:

- stable ADL remains the usable substrate
- the split-off repo becomes the fast-moving innovation lane
- mature work flows back into ADL regularly
- substrate fixes and compatibility requirements flow forward into the fast
  repo regularly

This document plans the split. It does not itself create the new repository,
move code, or change current `v0.91.x` through `v0.95` milestone scope.

## Purpose

The project needs two speeds after `v0.95`.

ADL needs to become stable enough for real use:

- predictable releases
- stable docs
- clear APIs and schemas
- reliable demos
- reproducible evidence
- boring upgrade paths
- enterprise and investor legibility

At the same time, the cognitive-system frontier needs to keep moving quickly:

- new CSM ideas
- polis and civilization experiments
- social cognition
- mental time travel and temporal selfhood
- economic societies
- long-lived citizens
- speculative runtime structures
- research demos that should not destabilize the stable substrate

Trying to force both speeds through one repo indefinitely will make ADL harder
to use and harder to trust. The split gives ADL a stable center while preserving
the project's capacity for rapid discovery.

## Source Inputs

- `.adl/docs/TBD/ADL_LOGISTIC_SPLIT.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md`
- `docs/explainers/CSM.md`
- `docs/milestones/v0.95/`

## Core Operating Model

The post-`v0.95` operating model is:

```text
fast-moving repo  <---- frequent forward sync ----  stable ADL
      |                                               ^
      |                                               |
      +---- governed mature mergeback ----------------+
```

Stable ADL is the root of substrate truth.

The fast-moving repo is where new cognitive-system work can move quickly
without making ADL unstable for users.

The two repos should not drift apart for long. They should merge fairly often,
but through explicit gates:

- stable substrate changes flow forward quickly into the fast repo
- mature fast-lane features merge back into ADL when they meet substrate quality
  and compatibility bars
- experiments stay in the fast repo until they are stable enough to support
  ADL users

## Naming

This plan intentionally leaves the final repository name open.

Working names:

- `adl-next`
- `adl-labs`
- `csm`
- `csm-labs`
- `agent-design-language-labs`

The name should communicate that the split-off repo is fast-moving but still
governed. Avoid names that imply the stable ADL repo is obsolete.

## Repository Roles

### Stable ADL Repository

The current ADL repo becomes the stable substrate repo.

It owns:

- deterministic execution
- replay and artifact contracts
- trace and signed trace substrate
- workflow and task models
- provider and transport substrate
- policy and governance primitives
- capability and tool authority
- identity, continuity, witness, and receipt primitives
- security, isolation, audit, and compliance evidence
- control-plane lifecycle and review discipline
- stable public docs
- release notes, migration notes, and compatibility guarantees
- demos that prove stable substrate behavior

It optimizes for:

- usability
- stability
- compatibility
- reviewability
- enterprise trust
- operator confidence
- reproducible evidence

### Fast-Moving Repository

The split-off repo becomes the fast-moving version.

It owns:

- frontier cognitive-system experiments
- CSM and polis evolution above the stable substrate
- advanced long-lived citizen behavior
- social cognition experiments
- mental-time-travel experiments beyond the stable trace/memory substrate
- speculative memory, identity, and world-model structures
- economic society demos above stable economic primitives
- new demos and research papers that need high iteration speed
- candidate substrate changes before they are ready for ADL

It optimizes for:

- speed
- experimentation
- conceptual discovery
- demo velocity
- research synthesis
- pressure-testing ADL substrate boundaries

The fast repo should still use the C-SDLC discipline where practical. Fast does
not mean unreviewed chaos.

## Dependency Direction

The intended dependency direction is:

```text
fast-moving repo -> stable ADL
```

Stable ADL should not depend on fast-moving repo internals.

The fast repo may depend on ADL through:

- crates
- CLI commands
- schemas
- trace contracts
- public docs
- examples
- stable interface packages

If the fast repo needs to change ADL, that change should enter ADL through a
normal issue, review, PR, and release process.

## Merge Rhythm

The repos should merge often enough that the fast repo does not become a
permanent fork.

Recommended baseline:

- forward-sync ADL into the fast repo at least weekly during active development
- merge urgent ADL security, compatibility, or schema fixes into the fast repo
  immediately
- review mature fast-lane features for ADL mergeback at least once per
  milestone or once per focused sprint
- keep a standing mergeback queue so promising work is not lost

The cadence can tighten during intense work, but should not loosen enough that
the repos become culturally or technically unrelated.

## Forward Sync From ADL To Fast Repo

Forward sync keeps the fast repo honest.

What flows forward:

- stable ADL runtime changes
- schema changes
- trace/replay changes
- governance/security changes
- CLI/control-plane changes
- docs that define substrate truth
- release notes and migration guidance

Forward sync rules:

- fast repo must track the ADL version it is based on
- conflicts should be resolved in favor of stable ADL substrate truth unless a
  fast-lane experiment explicitly declares a divergence
- divergences must be documented
- the fast repo should not silently patch around ADL compatibility breaks

Forward sync proof:

- sync commit or merge commit
- compatibility note
- minimal smoke check
- updated supported ADL version

## Mergeback From Fast Repo To ADL

Mergeback is how successful experiments become stable substrate.

A fast-lane change can merge back into ADL only when it satisfies the ADL bar:

- clear substrate value
- stable public contract
- tests or proof artifacts
- docs
- migration notes if behavior changes
- no hidden dependency on experimental fast-repo state
- no unresolved governance, security, or trace ambiguity
- review findings fixed or explicitly deferred through follow-on issues

Mergeback classes:

| Class | Description | ADL bar |
| --- | --- | --- |
| Bug/security fix | Correctness, compatibility, or security repair discovered in fast repo | Fast-track into ADL with focused validation |
| Substrate improvement | Runtime, trace, schema, control-plane, provider, or governance improvement | Normal ADL issue/PR/review |
| Stabilized feature | Fast-lane feature mature enough for ADL substrate | Feature doc, tests, docs, migration notes, review |
| Demo/proof artifact | Demo or evidence useful to stable ADL | Must prove stable substrate behavior, not only research novelty |
| Research note | Idea or finding, not implementation-ready | Tracked planning or backlog, not direct merge |

## What Must Stay Stable In ADL

These should remain stable ADL responsibilities:

- deterministic runtime
- trace and signed trace
- replay and artifact schema
- policy and governance primitives
- Freedom Gate substrate
- ACIP and transport substrate
- capability and tool authority
- identity, continuity, witness, and receipt primitives
- security and audit evidence
- control-plane lifecycle
- C-SDLC process once it becomes default
- release evidence and milestone closeout discipline

The fast repo can pressure test these surfaces, but it should not become their
source of truth.

## What Should Start In The Fast Repo

These categories should generally start in the fast repo after `v0.95`:

- new Cognitive Spacetime experiments above stable substrate
- new polis/civilization simulations
- speculative agent selfhood experiments
- new social cognition models beyond the stable v0.93 baseline
- frontier mental-time-travel extensions
- experimental contract-market societies
- high-risk demos
- speculative papers with runnable artifacts
- major architecture ideas that could destabilize ADL if admitted too early

## What Should Merge Back Often

Not everything in the fast repo should stay there. Frequent mergeback is part
of the design.

Likely mergeback candidates:

- fixes to ADL APIs found through fast-lane pressure tests
- improved validators
- better trace schemas
- improved demo harnesses
- clearer docs
- stable examples
- generalized runtime utilities
- matured governance patterns
- reusable test fixtures
- stable proof surfaces

The fast repo should be an incubator, not a graveyard.

## Split Preparation Before v0.95

Before `v0.95`, do not move code. Prepare the split by making ADL cleaner.

Preparation work:

- keep feature list and milestone docs accurate
- make public schema boundaries explicit
- complete signed/queryable trace planning and implementation
- complete C-SDLC default operation
- clarify stable docs versus experimental docs
- improve demo reliability
- reduce long test-cycle friction
- keep release evidence boring and reproducible
- label post-`v0.95` candidates without moving them

This preparation is what makes the split safe.

## Split Execution After v0.95

### Phase 1: Freeze Stable Baseline

Create a stable ADL baseline after `v0.95`.

Outputs:

- stable release tag
- release evidence packet
- supported-feature list
- compatibility notes
- known experimental/deferred list
- post-`v0.95` split issue

### Phase 2: Create Fast Repo

Create the split-off repo from the stable baseline or a curated subset.

Decisions required:

- repository name
- visibility
- license posture
- package/crate strategy
- history strategy
- branch protection
- CI baseline
- issue template and C-SDLC expectations

Outputs:

- new repository
- README
- relationship-to-ADL doc
- supported ADL baseline version
- initial roadmap
- initial demo/proof commands

### Phase 3: Establish Sync Contract

Define how the repos exchange changes.

Outputs:

- forward-sync procedure
- mergeback procedure
- compatibility matrix
- schema/version policy
- release cadence
- conflict resolution policy
- issue labels for upstream/downstream changes

### Phase 4: Move Or Mirror Experimental Material

Move only material that clearly belongs in the fast lane.

Possible initial material:

- experimental CSM docs
- frontier demos
- speculative research notes
- labs-style examples
- candidate features above stable substrate

Do not move stable ADL substrate docs or runtime contracts.

### Phase 5: First Mergeback Trial

Run one small mergeback soon after the split.

Good candidates:

- doc clarification
- fixture improvement
- validator improvement
- demo harness repair

The goal is to prove that mergeback works before large features depend on it.

### Phase 6: Establish Normal Cadence

After the first mergeback succeeds, move to the regular rhythm:

- frequent ADL forward-sync
- regular fast-lane review
- standing mergeback queue
- stable ADL releases
- fast repo research releases or snapshots

## Compatibility Contract

The fast repo must declare:

- ADL baseline version
- supported ADL version range
- schema versions
- trace versions
- runtime assumptions
- governance assumptions
- known divergences

ADL must declare:

- stable public interfaces
- deprecation policy
- migration windows
- compatibility guarantees
- unsupported experimental surfaces

## Documentation Contract

Stable ADL docs should remain authoritative for:

- runtime behavior
- trace/replay
- governance
- security
- schemas
- release policy
- stable demos
- user/operator workflows

Fast repo docs should own:

- experiments
- research demos
- candidate features
- speculative architecture
- fast-lane notes
- integration examples against specific ADL versions

Shared or duplicated docs must state their source of truth.

## Governance Contract

ADL remains authoritative for:

- substrate governance
- trace truth
- policy authority
- Freedom Gate semantics
- identity/continuity primitives
- signed evidence rules
- capability authority
- security boundaries

The fast repo may propose governance changes, but ADL accepts them only through
normal ADL issues, ADRs, review, and release discipline.

## C-SDLC Contract

The split should not abandon the C-SDLC.

Stable ADL should use the mature, default C-SDLC process.

The fast repo should use a lighter but compatible form:

- tracked issues
- explicit plans
- review records
- output records
- mergeback-ready evidence when a change is intended for ADL

Fast-lane experimentation can be lighter, but mergeback candidates must be
raised to ADL evidence quality before entering stable ADL.

## Risk Register

| Risk | Why it matters | Mitigation |
| --- | --- | --- |
| Stable ADL becomes stagnant | Users need stability, but the project still needs innovation. | Keep fast repo active and merge mature work back often. |
| Fast repo becomes a fork | If sync is too rare, the repos become incompatible. | Weekly forward sync during active development and standing mergeback queue. |
| ADL remains too unstable | If everything keeps landing in ADL, it never becomes dependable. | Route frontier experiments to the fast repo after `v0.95`. |
| Governance duplicates | Fast repo could invent conflicting policy semantics. | ADL remains governance source of truth; changes require ADL ADRs. |
| Documentation drift | Two repos can contradict each other. | Define source-of-truth docs and shared-interface docs. |
| API drift | Fast repo may depend on internals. | Use versioned public interfaces and compatibility matrix. |
| Investor confusion | Split could look like fragmentation. | Position as stable substrate plus fast innovation lane. |
| Mergeback friction | Good ideas may never return to ADL. | Run an early mergeback trial and maintain a mergeback queue. |
| Premature extraction | Moving code too early can break v0.95 convergence. | No code movement before stable baseline freeze. |

## Immediate Pre-v0.95 Actions

Before the split:

- finish `v0.95` convergence planning and evidence
- make ADL docs coherent enough to stand alone
- ensure C-SDLC is operational
- make trace and signed-trace story clear
- identify likely fast-lane candidates
- define labels for `fast-lane`, `mergeback-candidate`, and `stable-substrate`
- draft the future repo README before creation
- decide repository name

## First Fast-Repo README Requirements

The first README for the split-off repo should explain:

- this repo is the fast-moving innovation lane
- stable ADL is the substrate
- supported ADL baseline version
- how to run demos
- how to forward-sync ADL changes
- how to propose mergeback to ADL
- which docs are experimental
- which interfaces are stable
- what not to treat as production-ready

## Mergeback Acceptance Checklist

Before a fast-lane change merges into ADL:

- [ ] ADL issue exists.
- [ ] Feature or fix is classified as substrate-worthy.
- [ ] Public contract is documented.
- [ ] Tests or proof artifacts exist.
- [ ] Trace/replay implications are clear.
- [ ] Governance/security implications are clear.
- [ ] Migration notes exist if behavior changes.
- [ ] Docs identify the stable source of truth.
- [ ] Review findings are fixed or explicitly routed.
- [ ] No dependency on fast-repo-only internals remains.

## Non-Goals

- No code movement before `v0.95` convergence.
- No hidden fork.
- No abandonment of ADL.
- No claim that stable ADL is obsolete.
- No permanent divergence between repos.
- No unreviewed fast-lane mergeback.
- No weakening of governance, trace, replay, or C-SDLC discipline.

## Open Decisions

- What is the split-off repository name?
- Should the fast repo preserve full git history or start from curated import?
- Should it be public immediately, private initially, or staged?
- Which package/crate boundaries should be shared versus duplicated?
- What is the exact sync cadence after the first month?
- Which first demo proves the split is working?
- Which first mergeback proves the repos remain connected?

## Recommended Next Step

After this plan is reviewed, open a post-`v0.95` split-preparation issue that
does not move code yet, but prepares:

- repo name
- README
- sync contract
- mergeback contract
- compatibility matrix
- initial candidate inventory

The first actual repository split should happen only after the `v0.95` stable
baseline is tagged and its release evidence is reviewable.
