# End Of Milestone Report - v0.90.5

## Status

Final end-of-milestone report for `v0.90.5`.

The implementation wave, feature-proof coverage, quality gate, docs/review
convergence, internal review, third-party review, accepted-finding disposition,
next-milestone handoff, and release ceremony are complete.

## What v0.90.5 Delivered

`v0.90.5` delivered ADL's Governed Tools v1.0 substrate plus the first landed
Agent Communication and Invocation Protocol tranche:

- tool-call threat model and governed-capability non-goals
- Universal Tool Schema (`UTS`) public-compatible schema and conformance
- ADL Capability Contracts (`ACC`) for authority, privacy, visibility,
  delegation, trace, and replay
- deterministic tool-registry binding and `UTS` to `ACC` compilation
- normalization, policy, and Freedom Gate mediation before execution
- governed executor behavior and refusal boundaries
- trace, replay, and redaction evidence
- dangerous negative tests that fail closed
- bounded model-proposal benchmark and local / Gemma evaluation
- Governed Tools v1.0 flagship demo
- explicit feature-proof coverage through WP-19
- first landed ACIP / Comms tranche with protocol architecture, canonical
  envelope and identity shape, invocation/Freedom Gate linkage, conformance
  fixtures, review/coding specialization, and proof coverage

## Review Result

- Internal review: complete (`WP-22` / `#2587`)
- External review: complete (`WP-23` / `#2588`)
- Accepted findings remediation: complete as explicit no-remediation result
  (`WP-24` / `#2589`)

The third-party review reported zero findings.

## Quality And Coverage Outcome

`v0.90.5` has a canonical quality and coverage record in:

- `docs/milestones/v0.90.5/QUALITY_GATE_v0.90.5.md`

That record preserves the important truth:

- milestone proof and PR-gate evidence are green and reviewable
- a main-branch coverage exception remained explicit rather than hidden
- the get-well runtime-reduction wave was recorded as support work rather than
  quietly treated as solved

## Architecture Outcome

Architecture outcome for `v0.90.5`: `architecture-update`

The milestone did not remain architecture-neutral. It now has an accepted ADR
for governed-tools execution authority:

- `docs/adr/0015-governed-tools-execution-authority-architecture.md`

That record captures the authority stack:

- model proposal
- UTS description
- registry binding
- `UTS` to `ACC` compilation
- policy / Freedom Gate mediation
- governed execution
- trace and redacted evidence

## Scope Discipline Held

`v0.90.5` still does not claim:

- `UTS` by itself is execution authority
- arbitrary shell or network execution from model output
- payment rails, legal contracting, billing, or inter-polis economics
- full `v0.91` moral/cognitive-being implementation
- full `v0.91.1` adjacent-systems implementation

## Handoff Result

The handoff is clean:

- there is no vague leftover governed-tools debt bucket
- `v0.91` is now the full core moral/cognitive-being and secure intra-polis
  communication milestone
- `v0.91.1` is the adjacent-systems lane
- the reviewed candidate `v0.91` issue-wave YAML now exists
- structured planning, `SRP`, and A2A planning surfaces are promoted into the
  tracked `v0.91` package

## Ceremony Result

- `WP-26` / `#2591` release ceremony: complete

At milestone close, the project leaves `v0.90.5` with a real governed-tools
substrate, a zero-finding third-party review, and a prepared `v0.91` package
rather than a loose planning backlog.
