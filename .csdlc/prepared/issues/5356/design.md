# WP-18 Internal Milestone Review Preparation Design

## Purpose

Prepare issue #5356 to run the findings-first v0.91.8 internal milestone
review after WP-17 #5360 has aligned the complete documentation and release
packet. This preparation does not perform the review, change product code,
publish a packet, approve release, or open a PR.

## Authority Boundary

- WP-17 #5360 owns documentation alignment and the exact reviewable milestone
  packet. #5356 consumes only its merged, typed `closed_out`, claim-free,
  retained-receipt-backed, ancestral result.
- #5356 owns the future internal-review packet and finding register. It does
  not remediate findings, grant merge/release authority, execute deployment,
  or send the WP-19 third-party handoff.
- Preparation protects only four issue-local `.csdlc` paths. Review output and
  milestone review paths require a typed claim amendment after #5360 is
  terminal and before review execution.
- Runtime v2, AWS, raw `gh`, provider credentials, paid services, root-main
  writes, product implementation, review execution, publication, and PR
  creation are outside this preparation.

## Exact Revision And Corpus Contract

The future review starts by freezing one target identity containing repository,
base, head branch, exact 40-hex commit, tracked object-record manifest digest,
WP-17 receipt digest, and review-corpus digest. A changed head, base, corpus,
or digest invalidates every specialist result and requires a fresh assignment.

The corpus manifest covers:

1. canonical v0.91.8 planning, feature, proof, quality, release, review, and
   v0.92 handoff documents;
2. landed WP-02 through WP-17 source, tests, deployment/operations records,
   lifecycle projections, CI proof, closeout receipts, and issue/PR truth;
3. ADL v2, Runtime v3, C-SDLC v2, platform acceptance, deployment,
   Observatory, rollback/cutover, deletion, demo, quality, and docs surfaces;
4. public-claim, redaction, secrets, host-path, provenance, dependency,
   architecture, and evidence-integrity boundaries;
5. skipped, unavailable, non-applicable, blocked, deferred, and unproven
   surfaces as explicit dispositions rather than silent omissions.

Every corpus entry must be tracked at the exact revision or be a typed
repo-relative retained-receipt reference validated from the shared Git common
directory. Local scratch output, credentials, private prompts, host-absolute
paths, and untracked evidence are not review authority.

## Specialist Lane Matrix

Six mandatory read-only lanes run against the same frozen identity:

| Lane | Required focus | Required output |
| --- | --- | --- |
| code | correctness, unsafe behavior, duplication, dependency use, module growth | findings with exact file/line evidence |
| security | trust boundaries, authority, credentials, network, replay, redaction | exploit/failure-mode findings and non-claims |
| tests | coverage of acceptance, determinism, negative proof, CI/PVF truth | proof gaps and fragile/false-positive tests |
| docs | canonical truth, links, feature status, deployment and release claims | stale/contradictory documentation findings |
| architecture | owner boundaries, COTS reuse, Runtime/ADL/C-SDLC integration | boundary drift, coupling, and fitness findings |
| evidence | issue/PR/receipt/revision identity, closeout, provenance, corpus integrity | lifecycle and release-evidence findings |

No lane may edit files, mutate GitHub, run AWS, execute deployment, or infer a
pass from another lane. Synthesis begins only after all six return an exact
identity, disposition, reviewed paths, commands, findings, and residual risks.

## Finding And Disposition Contract

Findings use stable IDs `IR-5356-NNN` and severity `P0`, `P1`, `P2`, or `P3`.
Each finding records summary, exact evidence, impact, violated invariant,
failure mode, recommended bounded fix, actionable/in-scope flags, route,
disposition, fix revision when applicable, and residual risk.

Allowed dispositions are `open`, `accepted`, `fixed`, `rejected_with_reason`,
`duplicate`, `routed`, and `not_applicable`. P0/P1 findings block downstream
WP-19. P2/P3 findings remain blocking unless the internal review packet records
an evidence-backed operator disposition; silence, prose-only assurances, or a
future issue number do not resolve a finding. Remediation belongs to WP-20
#5363 after WP-19, unless the operator explicitly routes an immediate safety
stop without widening #5356.

## COTS And Budgets

Use existing Git, SHA-256, JSON/YAML parsers, typed C-SDLC v2, repository review
skills, and existing validation/coverage/provenance tooling. Add no dependency
and do not create a second review engine, issue tracker, evidence store, signer,
or lifecycle authority.

- Preparation design, diagram, manifests, requests, and validators: at most
  1,400 nonblank authored lines; each file below 500 lines.
- Future review orchestration and tightly coupled schema/fixtures: at most
  2,500 nonblank lines and fewer than 250 focused assertions.
- Dependency/corpus/preparation gates: 120 seconds each.
- Each specialist lane: 1,200 seconds.
- Synthesis and review-quality lanes: 600 seconds each.
- Complete exact-revision review: 3,600 seconds.
- Any variance requires exact-revision review before publication and does not
  authorize deferred proof.

## PVF, Rollback, And Publication Boundary

Preparation runs only current-registry card integrity, issue-local hygiene,
corpus/matrix schema validation, and the expected-failing #5360 terminal gate.
Future specialist lanes may run in parallel after one frozen identity; corpus
freeze and synthesis are serialized. Cancellation must leave the previous
accepted packet untouched and mark the candidate incomplete. Publication is
allowed only after all mandatory lanes, synthesis, review-quality evaluation,
typed review truth, exact revision recheck, redaction/provenance validation,
and no unresolved blocker. Rollback restores the last accepted tracked packet
and never rewrites a terminal receipt.

## Stop Conditions

Stop without review execution, publication, or claim if #5360 is not merged,
typed `closed_out`, claim-free, receipt-backed, and ancestral; the corpus is
incomplete or mutable; specialist identities differ; any required lane would
be skipped or deferred; evidence is untracked, host-bound, private, or
secret-bearing; protected paths collide; the review would touch Runtime v2 or
AWS; or findings cannot be routed without inventing authority.
