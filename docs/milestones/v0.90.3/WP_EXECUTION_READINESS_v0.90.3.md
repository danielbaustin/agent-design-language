# WP Execution Readiness - v0.90.3

## Status

Planning guidance for WP-01 card authoring.

This document prevents the v0.90.3 issue wave from becoming generic. When
WP-01 creates the GitHub issues and local cards, each issue body should inherit
the relevant source docs, required outputs, validation, and non-goals below.

## Global Rules

- Treat v0.90.3 as the citizen-state substrate milestone.
- Do not claim first true Gödel-agent birthday.
- Do not claim full v0.91 moral, emotional, kindness, humor, wellbeing,
  cultivation, or harm-prevention substrate.
- Do not claim v0.92 identity/capability rebinding, migration, or birthday
  record.
- Do not implement full economics, contract markets, bids, payment rails,
  subcontracting, or inter-polis trade.
- Do not make cloud enclaves mandatory.
- Do not allow unrestricted operator inspection of private citizen state.
- Preserve worktree-first execution and focused validation profiles.

## Sprint 1: Planning, Inheritance, And State Authority

### WP-01 Promote v0.90.3 milestone package

Required outputs:

- reviewed milestone docs under `docs/milestones/v0.90.3/`
- GitHub issue wave from `WP_ISSUE_WAVE_v0.90.3.yaml`
- local task cards with this readiness guidance copied into issue bodies
- issue-number mapping recorded in WBS and YAML

Required validation:

- YAML parse check for `WP_ISSUE_WAVE_v0.90.3.yaml`
- path-leak scan across touched tracked docs
- stale-status scan for pre-wave language after issue creation

### WP-02 Citizen-state inheritance and gap audit

Required outputs:

- audit of v0.90.2 CSM run, citizen, snapshot, wake, quarantine, hardening, and
  Observatory artifacts
- unsafe-assumption list for provisional JSON state, operator visibility,
  continuity, keying, and access boundaries
- explicit source-to-v0.90.3 requirement map

Required validation:

- referenced v0.90.2 artifacts exist
- no inherited v0.90.2 proof is promoted to durable citizen identity without a
  stated gap or requirement

### WP-03 Canonical private state format

Required outputs:

- format decision record
- authoritative private-state schema or fixture
- deterministic projection fixture proving JSON is projection, not authority
- compatibility and schema-evolution notes

Required validation:

- deterministic serialization/projection check
- rejection of missing required identity, lineage, schema, or projection fields

### WP-04 Signed envelope and trust root

Required outputs:

- signed envelope schema
- local trust-root fixture
- negative cases for missing, unknown, revoked, mismatched, regressed, and
  broken-predecessor state

Required validation:

- signature, key id, content hash, prior hash, and sequence checks
- negative tests for rejected envelopes

## Sprint 2: Sealed Checkpoints, Lineage, And Continuity

### WP-05 Local-first key management and sealing

Required outputs:

- local key policy
- sealed checkpoint fixture or deterministic sealing fixture
- backend seam for later TPM, Secure Enclave, HSM, or cloud confidential
  computing adapters

Required validation:

- unavailable-key and wrong-key safe-failure cases
- proof that sealed checkpoint is not treated as raw JSON

### WP-06 Append-only lineage ledger

Required outputs:

- ledger schema
- accepted-head calculation
- tamper, truncation, fork, and replay fixtures

Required validation:

- materialized head must match lineage
- ledger/head disagreement enters recovery or quarantine instead of trusting the
  convenient copy

### WP-07 Continuity witnesses and receipts

Required outputs:

- witness schema
- citizen-facing receipt schema
- admission, snapshot, wake, quarantine, and release-from-quarantine examples

Required validation:

- receipts explain continuity without exposing unrelated private state
- witnesses tie to ledger and envelope evidence

### WP-08 Anti-equivocation

Required outputs:

- conflicting-successor fixture
- duplicate active-head or forked-continuity negative test
- disposition into sanctuary or quarantine

Required validation:

- conflicting signed successors for the same sequence cannot both become active
- evidence is preserved for review

### WP-09 Sanctuary and quarantine behavior

Required outputs:

- sanctuary/quarantine state semantics
- ambiguous wake fixture
- quarantine artifact and operator report

Required validation:

- ambiguous wake blocks activation
- quarantine preserves evidence and is not treated as recovery success

## Sprint 3: Projection, Standing, Access, Challenge, And Demo

### WP-10 Redacted Observatory projections

Required outputs:

- projection schema
- redaction policy and leakage tests
- Observatory packet/report update for citizen-state continuity status

Required validation:

- raw private state does not appear in operator, reviewer, public, or debug
  projections unless explicitly allowed by policy
- projection remains non-authoritative

### WP-11 Citizen, guest, standing, and communication boundary

Required outputs:

- standing schema and events
- communication examples for citizens, guests, service actors, external actors,
  and prohibited naked actors
- negative tests for silent rights escalation

Required validation:

- guest cannot silently acquire citizen rights
- service actor cannot become hidden social actor through privileges
- communication never grants inspection rights

### WP-12 Access-control semantics

Required outputs:

- authority matrix for inspection, decryption, projection, migration, wake,
  quarantine, challenge, appeal, and release
- access event schema
- denial fixtures

Required validation:

- every sensitive access path emits an auditable event
- denied access does not leak raw private state or mutate citizen continuity

### WP-13 Continuity challenge, appeal, threat model, and economics placement

Required outputs:

- challenge artifact
- appeal or review-resolution artifact
- freeze behavior for challenged wake or projection
- citizen-state threat model
- economics placement record deciding whether only a narrow
  resource-stewardship bridge is needed before v0.90.4

Required validation:

- challenged destructive transition freezes safely
- threat model covers insider/operator abuse, compromised key, malicious guest,
  equivocation, replay, projection leakage, and unsafe release from quarantine
- economics record does not implement markets or payment rails

### WP-14 Integrated citizen-state demo

Required outputs:

- integrated citizen-state proof packet
- generated witness, receipt, projection, access, challenge, and operator-report
  artifacts
- one bounded reviewer command or fixture-backed proof path

Required validation:

- integrated packet ties WP-03 through WP-13 evidence together
- demo remains a bounded local citizen-state proof, not personhood or birthday

### WP-14A Demo matrix and feature proof demos

Required outputs:

- updated demo matrix
- feature proof coverage record
- per-feature proof classification: runnable demo, proof packet,
  fixture-backed artifact, non-proving status, or explicit deferral

Required validation:

- every feature claim has one proof home or a named deferral
- non-proving boundaries are preserved

## Sprint 4: Review, Handoff, And Release

### WP-15 Quality gate, docs, and review convergence

Required outputs:

- aligned README, CHANGELOG, REVIEW, milestone docs, feature docs, demo matrix,
  release notes, and checklist
- quality posture, coverage/tracker truth, and explicit release-gate exceptions
- reviewer entry surface

Required validation:

- version/status scan
- local-path scan
- stale claim scan for future-scope overclaims
- quality and coverage tracker scan before internal review begins

### WP-16 Internal review

Required outputs:

- findings-first internal review packet
- demo/proof register
- issue closeout truth check
- remediation queue

Required validation:

- review records P0/P1/P2/P3 findings or explicit no-finding statements
- skipped checks are recorded as gaps, not proof

### WP-17 External / third-party review

Required outputs:

- external review handoff
- review artifacts
- finding disposition summary

Required validation:

- review artifacts are archived in the review directory
- findings are routed to WP-18 or recorded as zero-finding disposition

### WP-18 Review findings remediation

Required outputs:

- accepted findings fixed or explicitly deferred
- validation evidence per finding
- closure notes linking fixes, deferrals, and follow-up homes

Required validation:

- no accepted P0/P1/P2 finding remains open without explicit release-manager
  disposition

### WP-19 Next-milestone planning and handoff

Required outputs:

- v0.90.4 economics handoff readiness
- v0.91/v0.92 boundary preservation
- release notes/checklist readiness for WP-20
- optional backlog routing for non-blocking review suggestions

Required validation:

- no v0.90.3 docs claim v0.90.4, v0.91, or v0.92 work as shipped
- next-milestone docs are ready to start immediately after ceremony

### WP-20 Release ceremony

Required outputs:

- release ceremony result
- final release notes
- version/status truth confirmed
- release closure note

Required validation:

- ceremony script or release preflight from clean main
- final path/version/stale-claim scans
- root checkout clean after Daniel fast-forwards main
