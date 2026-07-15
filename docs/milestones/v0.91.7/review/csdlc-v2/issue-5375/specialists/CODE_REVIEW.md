# C-SDLC v2 Code Review

## Findings

### P1: A review assigned against a dirty worktree creates an unrecoverable Reviewed state

- File: `csdlc-v2/src/git.rs`
- Role: code
- Scenario: An issue reaches `Implemented` while its substantive changes are still uncommitted, then `csdlc-review assign` and `record` run before the changes are committed.
- Impact: The review is accepted over a digest containing `HEAD` plus the dirty diff, but publication requires the reviewed digest to equal the clean-commit digest. Committing the reviewed files necessarily changes both `HEAD` and the digest. The issue is already in `Reviewed`, while review assignment is allowed only from `Implemented`, so no typed operation can assign a replacement review. The canonical lifecycle is dead-ended and requires out-of-band state repair.
- Evidence:
  - `csdlc-v2/src/git.rs:55-112` hashes `HEAD`, the entire dirty diff, and untracked files into `substantive_revision`.
  - `csdlc-v2/src/review.rs:39-75` accepts that revision without requiring a clean worktree, and `csdlc-v2/src/review.rs:78-112` advances the accepted result.
  - `csdlc-v2/src/publication.rs:117-140` rejects publication unless the current substantive revision equals `clean_commit_revision(HEAD)`.
  - `csdlc-v2/src/store.rs:607-611` permits a review assignment only in `Implemented`; there is no reviewed-to-implemented or re-review transition.
  - This finding was independently derived from the code. Testing discovery #5368 corroborates the same dirty-review dead end.

### P1: Claim recovery can install a colliding or structurally corrupt replacement claim

- File: `csdlc-v2/src/lifecycle.rs`
- Role: code
- Scenario: An operator recovers an expired claim with protected paths that overlap another issue, a branch/worktree already owned by another binding, or internally inconsistent claim timestamps.
- Impact: Recovery can create simultaneous ownership of the same paths or Git topology. It can also commit a replacement that the next doctor/load verification rejects, converting a recoverable stale claim into corrupt canonical state.
- Evidence:
  - `csdlc-v2/src/lifecycle.rs:300-347` replaces the claim without taking the repository binding lock, checking other issue claims, checking Git worktree topology, or validating protected paths.
  - `csdlc-v2/src/model.rs:166-184` validates only claim identity, branch/worktree presence, and expiry; it does not validate owner, purpose, protected paths, acquisition ordering, or heartbeat ordering.
  - `csdlc-v2/src/store.rs:144-162` verifies the old record and cards but does not verify the replacement record before committing it.
  - The omitted checks are performed during normal initialization and binding at `csdlc-v2/src/lifecycle.rs:53-87` and `csdlc-v2/src/lifecycle.rs:135-189`, so recovery is a weaker alternate entrypoint to the same ownership state.
  - The same replacement path lets `heartbeat_claim` persist `expires == heartbeat` when `extend_seconds` is zero (`csdlc-v2/src/lifecycle.rs:270-297`), although record verification later requires expiry to be strictly greater.

### P1: Binding accepts an existing ordinary directory as a verified Git worktree

- File: `csdlc-v2/src/lifecycle.rs`
- Role: code
- Scenario: The requested worktree directory already exists but is absent from `git worktree list`.
- Impact: The issue transitions to `Bound` and records ownership of a directory that Git does not recognize as a worktree. Subsequent commands can run against an unrelated checkout or arbitrary directory while canonical lifecycle truth claims a valid branch/worktree binding.
- Evidence:
  - `csdlc-v2/src/lifecycle.rs:142-165` rejects mismatched listed paths and branches but never requires an existing requested path to have a matching topology entry.
  - `csdlc-v2/src/lifecycle.rs:225-231` calls `git worktree add` only when the path does not exist.
  - `csdlc-v2/src/lifecycle.rs:232-260` then commits the bound transition regardless of whether the pre-existing directory was listed by Git.
  - `docs/architecture/csdlc-v2/gate3/DESIGN.md:7-10` defines exact branch/worktree topology as part of binding authority.

### P1: Review scope is recorded but never constrains the revision being reviewed

- File: `csdlc-v2/src/git.rs`
- Role: code
- Scenario: A review assignment declares a narrow scope such as one documentation directory while the commit or worktree also changes source outside that scope.
- Impact: Publication can claim exact-revision review even though the reviewer evidence declares coverage of only a subset of the changed paths. The review gate proves that the evidence repeats the assigned scope, not that the scope covers the substantive revision.
- Evidence:
  - `csdlc-v2/src/git.rs:55-112` checks only that `scope` is nonempty; all Git commands are hard-coded to `.` and never use the supplied paths.
  - `csdlc-v2/src/review.rs:53-69` stores any nonempty scope alongside the whole-tree digest.
  - `csdlc-v2/src/review.rs:139-149` checks only that evidence repeats the assignment's scope and revision.
  - `csdlc-v2/src/publication.rs:119-132` relies on that digest/evidence pair as the publication review guard.

### P1: A truthful failed validation permanently prevents a repaired issue from becoming merge-ready

- File: `csdlc-v2/src/store.rs`
- Role: code
- Scenario: Validation fails, the defect is fixed, and the same or replacement lane passes on retry; both attempts are truthfully appended to SOR.
- Impact: The historical failed attempt remains an unconditional readiness blocker. There is no attempt identity, supersession marker, or typed operation to replace an obsolete result, so normal failure-and-repair cannot reach `MergeReady` without deleting evidence.
- Evidence:
  - `csdlc-v2/src/cards.rs:761-765` implements `RecordValidation` as append-only.
  - `csdlc-v2/src/store.rs:1117-1127` defines validation success as every historical result being passed or skipped.
  - `csdlc-v2/src/store.rs:1153-1174` applies that aggregate to the `MergeReady` phase guard, and `csdlc-v2/src/store.rs:325-339` repeats the all-history requirement in readiness commit.
  - Gate 9 names validation failure/retry as a qualification scenario, but the current state model has no successful retry disposition for an already-recorded failure.

### P1: Readiness and terminal closeout are not bound to the canonical remote PR identity

- File: `csdlc-v2/src/bin/csdlc-closeout.rs`
- Role: code
- Scenario: A caller submits synthetic readiness JSON, or asks live observation to inspect a different repository/PR that happens to share the recorded PR number and commit SHA. A draft or already-closed PR can also be observed because those states are not represented in readiness input.
- Impact: Caller-controlled data or the wrong remote object can advance `Published` to `MergeReady` and then terminal state. The state machine can therefore record remote approval, clean checks, merge, or closure without proving that the authoritative published PR in the recorded repository/base/head is the object observed.
- Evidence:
  - `csdlc-v2/src/bin/csdlc-closeout.rs:31-34` and `csdlc-v2/src/bin/csdlc-closeout.rs:97-104` expose `record-readiness`, which directly commits caller-supplied check, review, conflict, PR-number, and SHA fields.
  - `csdlc-v2/src/bin/csdlc-closeout.rs:155-305` uses request-supplied repository and PR identity for live observation without comparing repository, base ref, head ref, draft state, or open state to publication evidence.
  - `csdlc-v2/src/store.rs:305-315` checks only PR number and the clean SHA-derived revision before committing readiness; repository/base/head/state are absent from the guard.
  - `csdlc-v2/src/bin/csdlc-closeout.rs:308-344` repeats the request-supplied remote selection for closeout, while `csdlc-v2/src/store.rs:429-513` again lacks repository/base/head binding.
  - `csdlc-v2/src/readiness.rs:94-169` classifies only the supplied checks, review, conflict, SHA, and PR number; it cannot reject a draft, closed, or wrong-repository PR.
  - Testing discoveries #5370 and #5371 concern readiness/closeout collection, but they do not establish this identity-confusion defect; this finding remains independently derived.

### P1: Terminal closeout bypasses the SOR completion validation invariant

- File: `csdlc-v2/src/store.rs`
- Role: code
- Scenario: A reviewed issue closes without a PR, or a published issue closes unmerged, while SOR contains no validation result or a failed validation result.
- Impact: The terminal commit sets SOR status to `Complete` even though the semantic card API explicitly forbids completion without successful terminal validation. Canonical rendered state can therefore violate its own lifecycle invariant and still be accepted by the closeout path.
- Evidence:
  - `csdlc-v2/src/cards.rs:892-919` requires nonempty, structurally valid, passed-or-skipped validation before SOR may become `Complete`.
  - `csdlc-v2/src/store.rs:454-513` assigns `sor_values.status = CardStatus::Complete` directly and never calls the status guard.
  - `csdlc-v2/src/readiness.rs:293-297` allows `ClosedNoPr` from `Reviewed` and `ClosedUnmerged` from `Published`, neither of which guarantees that successful SOR validation was recorded.

### P2: An exact retry after successful publication evidence commit is rejected as stale

- File: `csdlc-v2/src/publication.rs`
- Role: code
- Scenario: PR creation/reconciliation and local publication commit succeed, but the caller loses the response and retries the same publication request.
- Impact: The remote action is safely converged, yet the canonical retry fails rather than returning the already-recorded result. Recovery requires constructing a new request from updated state instead of safely replaying the original operation, weakening the claimed idempotent publication boundary.
- Evidence:
  - `csdlc-v2/src/publication.rs:245-249` returns the existing result only when the current record digest still equals the request's pre-publication expected digest.
  - `csdlc-v2/src/store.rs:228-271` increments generation and recomputes the record digest during the first publication commit, so that equality cannot hold afterward.
  - The retry therefore falls through to `commit_publication` at `csdlc-v2/src/publication.rs:250-256`, where the original digest is rejected as stale.
  - Gate 6 tests cover pure remote reconciliation decisions, but not replay after canonical state commit.

### P2: PVF network, credential, and evidence policies are declarations rather than runtime enforcement

- File: `csdlc-v2/src/pvf.rs`
- Role: code
- Scenario: A lane declares denied network access and a bounded credential set, but its executable uses the host network or reads other inherited environment credentials; a lane also declares relative-path evidence enforcement.
- Impact: The report can classify the lane as policy-compliant even though the child has unrestricted inherited host authority. Credential posture, network denial, path policy, and redaction status are not proof of actual execution constraints.
- Evidence:
  - `csdlc-v2/src/pvf.rs:269-286` compares declarations only to caller-supplied `allow_network` and `available_credentials` values.
  - `csdlc-v2/src/pvf.rs:621-628` spawns the executable with inherited environment and no network isolation or credential allowlist.
  - `csdlc-v2/src/pvf.rs:110-114` defines `require_relative_paths`, but execution never consults that field; `csdlc-v2/src/pvf.rs:690-707` checks only its own generated log filename.
  - `csdlc-v2/src/pvf.rs:680-707` redacts only literal configured values and unconditionally emits `redaction_ok: true`, including when no evidence log was written.

### P2: Final `v1_sunset` authority still resolves an explicit request to v1

- File: `csdlc-v2/src/soak.rs`
- Role: code
- Scenario: After Gate 10D2, an operator runs generation resolution with `--requested v1`.
- Impact: The sole current resolver reports that the sunset generation is valid even though the coexistence authority says v1 is removed and forbidden v1 binaries no longer exist. Operators can be routed into an invalid generation after final cutover.
- Evidence:
  - `csdlc-v2/src/soak.rs:33-54` returns explicit `Generation::V1` without a sunset check.
  - `csdlc-v2/src/operator.rs:271-286` loads only `generation-selector.json` and delegates to that function; it does not read `operator/coexistence.json` or enforce `v1_sunset`.
  - `csdlc-v2/operator/coexistence.json` records `"v1_sunset": true`, and root `AGENTS.md` names Gate 10D2 as final v1-sunset authority.
  - Testing discovery #5369 reported a selector-file deployment defect. The reviewed selector is a regular tracked file, so that separate discovery was not reproduced and is not the basis of this finding.

### P2: `csdlc-edit bootstrap` bypasses initialization path and collision guards

- File: `csdlc-v2/src/bin/csdlc-edit.rs`
- Role: code
- Scenario: An operator invokes the installed `csdlc-edit bootstrap` entrypoint with an absolute/traversing design path or protected paths overlapping another issue.
- Impact: The alternate public bootstrap can read design/diagram content outside the repository and create a canonical issue whose ownership conflicts with an existing issue. It bypasses the stronger `csdlc-init` contract while producing the same Initialized state.
- Evidence:
  - `csdlc-v2/src/bin/csdlc-edit.rs:54-57` routes bootstrap directly to `store::bootstrap_issue`.
  - `csdlc-v2/src/store.rs:739-827` validates claim field presence but does not require repository-relative design/diagram/protected paths, take the binding lock, or scan other issue claims for overlap; `root.join(absolute_path)` preserves the absolute path.
  - `csdlc-v2/src/lifecycle.rs:53-115` is the guarded initialization route and performs relative-path checks, the global binding lock, overlap detection, and placeholder creation before calling the same store function.

## Metadata

- Skill: `repo-review-code`
- Target: branch `codex/5375-v0-91-7-csdlc-v2-full-sprint-review` at `7c3e1e0e86a4`
- Issue: #5375
- Reviewed issue set: #5228, #5232-#5240, #5292-#5295, and #5305-#5308 (18 issues)
- Date: 2026-07-14
- Artifact: `docs/milestones/v0.91.7/review/csdlc-v2/issue-5375/specialists/CODE_REVIEW.md`
- Review depth: deep, findings-only

## Reviewed Surfaces

- Core modules: every Rust module in `csdlc-v2/src`: `cards.rs`, `cutover.rs`, `doctor.rs`, `eligibility.rs`, `error.rs`, `git.rs`, `lib.rs`, `lifecycle.rs`, `migration.rs`, `model.rs`, `operator.rs`, `proof.rs`, `publication.rs`, `pvf.rs`, `readiness.rs`, `review.rs`, `schema.rs`, `soak.rs`, and `store.rs`.
- Binary entrypoints: every file in `csdlc-v2/src/bin`: `csdlc-bind`, `csdlc-closeout`, `csdlc-cutover`, `csdlc-doctor`, `csdlc-edit`, `csdlc-eligibility`, `csdlc-init`, `csdlc-install`, `csdlc-proof`, `csdlc-publish`, `csdlc-review`, `csdlc-schedule`, `csdlc-shadow`, `csdlc-shepherd`, `csdlc-soak`, and `csdlc-validate`.
- Behavioral boundaries: lifecycle transitions and card projections; claim acquisition, heartbeat, recovery, collision, and locking; Git branch/worktree/revision handling; review assignment and evidence; publication reconciliation and retry; readiness observation; merge/no-merge/no-PR closeout; transaction recovery; schema and serialization; migration/shadow paths; PVF selection/execution/evidence; proof, cutover, eligibility, installation, and final generation resolution.
- Supporting contract evidence: root and nested `AGENTS.md`; Gate 3, Gate 6, Gate 7, Gate 10D3, and Gate 10D4 design records; focused integration/unit tests corresponding to the reviewed behaviors.
- Testing-discovery separation: #5364-#5373 were consulted only after the independent source review stabilized. Only #5368 directly corroborated a finding above. #5369 was not reproduced at this revision; #5370/#5371 do not subsume the remote-identity finding. No testing discovery was copied into this artifact as a review finding.

## Validation Performed

- `CARGO_TARGET_DIR=<external-target-dir> cargo test --manifest-path csdlc-v2/Cargo.toml`: passed all 101 unit and integration tests across the library, binary tests, and Gate 2/4/5/6/7/8/9/10A/10B suites.
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`: passed.
- `CARGO_TARGET_DIR=<external-target-dir> cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`: passed.
- Static path tracing covered every source module and binary entrypoint. Focused test inspection checked which recovery, retry, identity-binding, and negative-transition scenarios are absent despite the green suite.
- Validation was inspect-only with build output isolated outside the checkout; no implementation, tests, configs, lifecycle cards, or existing docs were modified.

## Residual Risk

- This lane prioritized correctness and state-machine behavior. Cryptographic trust, credential exposure, remote API abuse, and process isolation should receive independent security severity review; the PVF finding above is limited to the observable mismatch between declared and enforced behavior.
- Live GitHub side effects were not exercised. Publication, review, and closeout conclusions are based on complete source-path tracing plus existing focused tests, avoiding mutation of remote issue/PR state.
- Crash recovery was reviewed statically, but no kill-at-every-write fault-injection campaign was run. Additional interruption defects may remain in transaction replacement, installer backup recovery, cutover selector replacement, and multi-process lock ordering.
- Serialization/schema review covered the canonical record/card paths and public request/response models, but exhaustive backward/forward compatibility fuzzing and malformed-input corpus testing were outside this specialist lane.
- Passing tests and Clippy establish current expected-path conformance, not correctness of the uncovered alternate entrypoints and retry paths. The highest residual concentration is where multiple public binaries can reach the same lifecycle phase through different validation strength.
