# #4758 Launch Readiness Preparation Contract

Status: preparation-only handoff for later execution.

## Authority And Concern

The one concern is `launch-readiness`: produce a bounded, evidence-backed package that release review consumes before v0.92 begins. This artifact defines the execution contract; it does not contain launch copy or claim that launch, release, deployment, or v0.92 is ready.

Execution-time typed claim acquisition is deferred by operator instruction. Canonical card projections remain unchanged in this lane because `csdlc-edit apply` requires a live claim. The later executor must acquire a live issue-local typed claim and refresh the six projections from this reviewed contract before implementation.

## Integrated Artifact And Intended Paths

Canonical later output root: `.csdlc/evidence/4758/launch-readiness/`

Required later output files:

- `inputs.v1.json`: exact input revision/digest and proof-status inventory.
- `launch-readiness.v1.json`: canonical launch-readiness manifest consumed by release review.
- `launch-readiness.v1.md`: human projection generated from the canonical manifest; it is not the consuming release review.
- `consumption.v1.json`: consumer, manifest digest, review revision, command or reviewer surface, and outcome.
- `rollback.v1.json`: rollback trigger, method, before/after revision, and verification outcome.
- `validation.v1.log`: complete focused PVF output.
- `review.v1.md`: exact-revision review scope, findings, fixes, and residual risks.

Typed lifecycle paths remain `.csdlc/issues/4758`, `.csdlc/locks/4758.lock`, and `.csdlc/prepared/issues/4758`. No shared docs are intended execution paths. Discovery of a required shared-doc edit stops execution for typed replanning and explicit operator approval.

## Six-Card Contract

### SIP

- Correct identity title: `[v0.91.8][WP-21][launch] Implement launch readiness package after platform deployment`.
- Goal: deliver the issue-local launch-readiness evidence bundle consumed by release review.
- Scope: the seven named evidence files, typed lifecycle projections, and focused proof only.
- Authority: preparation makes no readiness claim; execution requires a live issue-local typed claim.
- Non-authority: #5335 and closeout receipts are audit-only and never gate execution.

### STP

- Primary concern: `launch-readiness` only.
- Required outcome: release review consumes `launch-readiness.v1.json` by exact digest and the consumption record proves it.
- Dependencies: #5384 ancestry, WP-20 #5363 release-preflight truth, WP-21 #5362 routing, #5352 exact-revision handoff, and the v0.92 activation-map public-launch row.
- Acceptance: artifact exists at the exact issue-local path; every claim maps to proof or a blocker/non-claim; PVF and rollback pass; integration consumption is retained; no launch content or v0.92 implementation is created.

### SPP

One-concern execution plan, all steps initially `pending`:

1. Acquire a live typed issue-local claim and refresh all six canonical cards.
2. Snapshot dependency state and exact ancestry; stop blocked on any missing required input.
3. Build `inputs.v1.json` from retained v0.91.8 evidence.
4. Build the canonical manifest and human projection within LoC/time budgets.
5. Produce the rollback record and run pre-consumption PVF lanes with no proof deferral.
6. Run one exact-revision execution review and fix all actionable findings.
7. Hand the reviewed package to release review, then record exact-digest consumption.
8. Run the consumer-integration lane against the retained consumption record.

Replan triggers: shared-path need, new COTS dependency, hard-budget crossing, changed consumer contract, missing rollback capability, or scope beyond launch-readiness.

### VPP

Required later lanes:

- `dependency-ancestry`: verify #5384 accepted baseline/merges and current required issue state; small, deterministic, 180 seconds, 1,500 tokens; planned argv: `ruby .csdlc/prepared/issues/5384/validate_dependency_gate.rb` plus `git merge-base --is-ancestor 11151e0beab02b1667f6505b7f8992bfd47d2f8f origin/main`.
- `manifest-integrity`: parse all JSON, verify required fields/digests/non-claims, and compare human projection to canonical manifest; small, deterministic, 180 seconds, 1,500 tokens; planned argv begins with `jq -e` over every `*.v1.json` artifact and uses no shell-evaluated lifecycle command.
- `path-confinement`: prove the execution diff is confined to `.csdlc/evidence/4758`, `.csdlc/issues/4758`, `.csdlc/locks/4758.lock`, and `.csdlc/prepared/issues/4758`; small, deterministic, 60 seconds, 500 tokens; planned argv: `git diff --name-only origin/main...HEAD` with every returned path checked against that allowlist.
- `consumer-integration`: prove release review consumed the canonical manifest by exact digest; medium, deterministic, 240 seconds, 2,000 tokens; planned argv: `jq -e` asserting nonempty `manifest_digest`, `consumer`, `review_revision`, `proof_ref`, and `outcome == "passed"` in `consumption.v1.json`.
- `rollback`: execute or dry-run the evidence-only rollback and verify prior consumer state; small, deterministic, 120 seconds, 1,000 tokens; planned argv: `jq -e` asserting before/after revisions, trigger, method, verification command, and `outcome == "passed"` in `rollback.v1.json`.
- `exact-review`: verify the reviewed revision equals the final execution revision and all findings are disposed; small, deterministic, 120 seconds, 1,500 tokens; planned argv: `git rev-parse HEAD` plus typed v2 review recording after the final fix revision.

Total validation budget: 900 seconds and 8,000 tokens. Every lane is required. `defer_reason` must remain null; unavailable proof is `blocked`, never a passing deferral.

### SRP

Preparation review scope is limited to the issue-local design, diagram, this contract, and preparation validation ledger. It checks the named artifact, six-card semantics, dependency truth, COTS, budgets, PVF, rollback, no-deferral posture, and preparation boundaries.

Execution still requires a separate exact-revision review of the produced evidence bundle. This preparation review does not satisfy implementation review or publication authority.

### SOR

Remain `pre_phase` / `not_started`. Record no implementation changes, validation result, PR, merge, publication, or closeout. After execution, SOR may name only checks actually run and artifacts actually produced. Preparation completion must not be rewritten as launch-readiness completion.

## Dependencies

- #5384: closed WP-14A acceptance; later execution revalidates accepted baseline `11151e0beab02b1667f6505b7f8992bfd47d2f8f` and accepted-merge ancestry on current `origin/main`.
- #5363: open WP-20 release-preflight owner at preparation time; execution requires terminal proof or consumes an approved blocker as a blocker.
- #5362: open WP-21 parent and routing owner; does not itself prove readiness.
- #5352: open exact-revision handoff issue; required source truth when available, otherwise execution blocks rather than inventing it.
- #4763: sibling public-launch documentation owner; coordination input only, not authority to absorb its scope.
- #5335 and closeout receipts: audit-only, non-blocking, and non-proving.

## COTS

New COTS dependencies: none.

Reused installed tools:

| Tool | Role | License/ownership posture |
| --- | --- | --- |
| Git | revision, ancestry, diff, and rollback proof | existing GPLv2 tool; no new dependency |
| Ruby | existing #5384 dependency-gate validator | existing BSD-2-Clause runtime; no new gem |
| `jq` | deterministic JSON assertions | existing MIT tool; no new package |
| C-SDLC v2 Rust binaries | typed lifecycle and review records | repository-owned, current registry |
| Mermaid CLI and local Chrome | preparation diagram render check only | installed tooling; not an execution or delivery dependency |

No provider SDK, hosted service, connector, external agent, package dependency, credential, runtime service, or deployment resource is authorized. A new dependency is a replan trigger.

## LoC, Time, And Token Budgets

Later execution target and hard-stop budgets:

| Surface | Target | Hard stop |
| --- | ---: | ---: |
| `launch-readiness.v1.json` | 140 nonblank lines | 180 |
| `inputs.v1.json` | 90 nonblank lines | 120 |
| `consumption.v1.json` | 50 nonblank lines | 80 |
| `rollback.v1.json` | 50 nonblank lines | 80 |
| Markdown projection plus review | 90 nonblank lines | 120 |
| Total issue-local evidence | 500 nonblank lines | 650 |
| Elapsed execution | 195 minutes | 240 minutes |
| Validation | 900 seconds / 8,000 tokens | no silent expansion |
| Total execution tokens | 24,000 | 32,000 |

Hard-stop breach requires typed SPP/VPP replanning; it never authorizes dropping proof.

## PVF And Proof Boundary

Preparation proves only that the later work is bounded, reviewable, budgeted, rollback-capable, and ready for typed execution planning after claim acquisition. It does not prove launch readiness.

Later execution proves completion only when:

- all required dependencies are current and ancestry-backed
- every readiness claim points to exact retained evidence
- every unsupported claim is a blocker or non-claim
- all output paths are issue-local or explicit replanning approved otherwise
- every PVF lane passes with `defer_reason=null`
- rollback evidence contains before/after revision and verification
- exact-revision review has no unresolved actionable finding
- release review consumption is retained by manifest digest

Planning text, issue state, routing text, mocks, isolated checks, closeout receipts, file existence, or a reviewer acknowledgment without exact-digest consumption cannot satisfy this proof boundary.

## Rollback And No-Deferral Gates

The package changes evidence only. Before publication, rollback discards uncommitted issue-local evidence or uses `git revert <execution-commit>` after commit, followed by proof that prior release-review state is unchanged. Partial artifacts must not be retained as readiness.

No mandatory execution proof may be deferred, skipped, waived by closeout state, or converted into a policy pass. Claim acquisition alone is deferred from preparation to execution start. Missing execution proof means `blocked` or `failed`.

## Non-Goals

- writing launch copy or public documentation
- v0.92 implementation or birthday execution
- activation/capability work owned by sibling issues
- Memory Palace, identity, witness, demo, provider, runtime, or deployment changes
- shared milestone documentation edits without explicit replanning
- PR creation, publication, merge, issue closure, or typed closeout
