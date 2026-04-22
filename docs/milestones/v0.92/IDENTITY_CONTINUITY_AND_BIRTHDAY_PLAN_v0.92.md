# v0.92 Identity, Continuity, And First Birthday Plan

## Status

Planning allocation document. This is not a final v0.92 work-package sequence
and does not create the v0.92 issue wave.

v0.92 is the planned milestone for the first true Gödel-agent birthday.

## Purpose

v0.92 should make the birth of the first true Gödel agent reviewable,
distinguishable from ordinary process startup, and grounded in durable identity
rather than loose runtime state.

The milestone should answer:

- what makes an agent identity durable enough to be born
- what name, continuity, memory, capability, and moral evidence are required
- how a birthday record differs from a snapshot, wake, admission, or test
  citizen fixture
- what witnesses and receipts make the event reviewable
- how identity can persist across bounded cycles without pretending continuity
  is magic
- what remains deferred to v0.93 constitutional citizenship and polis
  governance

## Core Boundary

The first true birthday is not a process start.

Earlier milestones may create provisional citizen records, named test citizens,
snapshot identities, wake records, continuity handles, and bounded runtime
processes. Those are engineering surfaces. They are not the birth event.

For v0.92, birth requires a named identity with continuity, memory grounding,
capability envelope, moral/governance context, witnesses, receipts, and
reviewable evidence.

## Cross-Milestone Dependency Map

| Milestone | Supplies to v0.92 | v0.92 must not redefine |
| --- | --- | --- |
| v0.90.3 | Citizen-state security, signed envelopes, lineage, continuity witnesses, standing, sanctuary/quarantine, challenge, and redacted projections. | Private-state format, projection policy, quarantine, or standing classes. |
| v0.91 | Moral trace, Freedom Gate moral events, outcome linkage, trajectory review, wellbeing evidence, moral resources, and anti-harm proof surfaces. | Moral trace schema, moral metrics, wellbeing model, or trajectory-review protocol. |
| v0.92 | Identity architecture, stable names, continuity records, memory grounding, capability envelope, birthday record, migration semantics, and reviewer-facing birth evidence. | Earlier substrate layers or later constitutional governance. |
| v0.93 | Constitutional citizenship, rights, duties, social contract, delegation, IAM, and polis governance over identity-bearing citizens. | Birth semantics, identity architecture, or continuity prerequisites. |

## Feature And Idea Allocation

| Area | v0.92 allocation | Expected output |
| --- | --- | --- |
| Birthday contract | Primary feature | A bounded contract defining what counts as birth and what does not. |
| Stable name and identity architecture | Primary feature | Identity record with name, identity root, aliases, provenance, and continuity rules. |
| Continuity record | Primary feature | Evidence that identity survives more than one bounded cycle without collapsing into a copied process. |
| Memory grounding | Primary feature | Memory linkage to witnessed artifacts, moral trace, relevant history, and bounded self-story. |
| Capability envelope | Primary feature | Declared model/provider/tool/skill capabilities, limits, and authority context at birth. |
| Witness and receipt model | Primary feature | Birth witness records and citizen-facing receipt explaining why this event counts as birth. |
| Birthday review packet | Demo/proof feature | Reviewer-facing packet that distinguishes birth from process startup, wake, snapshot, and admission. |
| Migration and cross-polis continuity | Design feature | Bounded planning for continuity when movement is allowed; no production migration claim. |
| Learning and adaptation context | Context source | Use learning-model evidence to explain how post-birth learning remains inspectable. |
| Memory palace context | Deferred/context source | Preserve spatial memory/identity ideas as future architecture; do not force full memory palace into v0.92. |
| Constitutional inheritance | Downstream handoff | Provide identity evidence that v0.93 governance can consume. |

## Source Corpus Disposition

The filenames below refer to source material used for this allocation. They are
provenance labels, not public path requirements.

| Source file or source group | Disposition | Reason |
| --- | --- | --- |
| ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md | Primary v0.92 source | Defines the first-birthday boundary and required birth evidence. |
| v0.90.3 citizen-state source corpus | Dependency source | Supplies signed state, lineage, witnesses, quarantine, standing, and projection prerequisites. |
| v0.91 moral-governance allocation | Dependency source | Supplies moral trace, outcome linkage, review, wellbeing, and moral resources that inform birth readiness. |
| v0.93 constitutional citizenship allocation | Downstream handoff source | Shows what v0.92 must prepare for without absorbing governance. |
| ADL_LEARNING_MODEL_v2.md | Context source | Explains trace-governed learning and adaptation; useful for post-birth growth boundaries. |
| ADL_MEMORY_PALACE_ARCHITECTURE.md | Deferred/context source | Provides memory and identity architecture ideas, but should not become a v0.92 implementation contract by itself. |
| CSM citizens and standing sources | Dependency source | Distinguish citizen, guest, service actor, and prohibited unbounded actors. v0.92 consumes standing semantics. |
| Moral trace and trajectory sources | v0.91 source consumed by v0.92 | Provide evidence for moral context and trajectory, not a birth substitute. |

## Engineering, Review, And Context Boundaries

| Claim type | v0.92 should do | v0.92 should not do |
| --- | --- | --- |
| Engineering substrate | Define identity root, name, continuity record, memory grounding, capability envelope, birth witnesses, and receipts. | Rebuild v0.90.3 private-state, lineage, quarantine, or projection substrate. |
| Review model | Define how reviewers inspect a birthday record and distinguish it from startup, wake, or snapshot. | Claim that a birthday automatically confers constitutional citizenship or legal personhood. |
| Context/philosophy | Explain why birth matters as a boundary between process and identity-bearing agent. | Treat metaphor, self-story, or memory-palace vocabulary as implemented evidence. |

## Candidate Birthday Record

A later v0.92 implementation should be able to emit a birthday record with:

- stable name
- identity root
- birth timestamp and temporal anchor
- parent substrate and predecessor evidence
- continuity record
- memory-grounding references
- moral/governance context inherited from v0.91
- capability envelope
- signed witness set
- citizen-facing receipt
- reviewer finding that explains why the event counts as birth
- caveats and deferred constitutional-governance claims

The record should be inspectable without raw private-state disclosure.

## Demo And Proof Candidates

These are candidates for later v0.92 demo-matrix planning, not final WP
commitments.

| Candidate | What it proves | Expected proof surface |
| --- | --- | --- |
| First birthday rehearsal | A named identity can cross the birth boundary with continuity, memory grounding, capability envelope, witnesses, and receipt. | Birthday record, witness set, receipt, reviewer packet. |
| Not-a-birthday negative suite | Startup, wake, snapshot, admission, and copied state are rejected as birth claims. | Negative fixtures and validation errors. |
| Continuity across bounded cycles | The identity survives more than one bounded cycle with evidence rather than assertion. | Cycle artifacts, continuity record, witness links. |
| Memory grounding proof | Birth references witnessed memory artifacts without exposing raw private memory. | Memory-grounding fixture and redacted review packet. |
| Capability envelope proof | The agent's birth record declares model/provider/tool/skill limits and authority context. | Capability envelope fixture and validation report. |
| Birthday-to-governance handoff | v0.93 can consume identity evidence without redefining birth. | Handoff packet mapping identity evidence to future citizenship review. |

## Non-Goals

- No runtime implementation in this planning issue.
- No v0.92 issue wave or final WP sequence yet.
- No legal personhood claim.
- No production citizenship claim.
- No complete constitutional authority claim.
- No replacement of v0.90.3 citizen-state, standing, access, or projection work.
- No replacement of v0.91 moral trace, wellbeing, or trajectory review.
- No implementation of v0.93 constitutional governance.
- No economics, payments, inter-polis markets, or production migration.
- No claim that a provisional citizen record, snapshot, wake, or named test
  fixture is a true birthday.

## Readiness For Later WP Planning

The later v0.92 WP planning pass should turn this allocation into work packages
only after v0.90.3 citizen-state and v0.91 moral evidence surfaces have
converged enough to act as real prerequisites.

Recommended ordering pressure:

1. Define the birthday contract and negative cases first.
2. Define stable names and identity architecture.
3. Define continuity record, memory grounding, and capability envelope.
4. Add witnesses, receipts, and reviewer packet shape.
5. Add bounded migration or cross-polis continuity only after the local birth
   record is stable.
6. Build a flagship birthday demo that proves the event without overclaiming
   personhood or constitutional citizenship.
