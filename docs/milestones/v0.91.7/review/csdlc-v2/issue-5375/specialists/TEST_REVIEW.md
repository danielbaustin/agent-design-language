# C-SDLC v2 Test Review

## Metadata

- Skill: `repo-review-tests`
- Target: issue #5375 full C-SDLC v2 sprint, issues #5228, #5232-#5240, #5292-#5295, and #5305-#5308
- Branch: `codex/5375-v0-91-7-csdlc-v2-full-sprint-review`
- Date: 2026-07-14
- Validation mode: targeted current-source execution plus deep test/proof inspection
- Discovery boundary: #5364-#5373 were checked only after the findings below were independently derived; their titles are not finding evidence and were not promoted into this review.

## Findings

### P1: The final stable install can pass proof while omitting the mandatory resolver binary

- File: `csdlc-v2/src/operator.rs:113`, `csdlc-v2/operator/skills.json:1`, `csdlc-v2/operator/coexistence.json:17`, `csdlc-v2/tests/gate10a.rs:31`
- Role: tests
- Scenario: An operator follows `csdlc-v2/AGENTS.md:7` and attempts the required `csdlc-install resolve` route from the stable `.adl/bin/csdlc-v2/` generation directory after v1 sunset.
- Impact: The sole-authority installation can be certified green without containing the binary required to resolve or verify that authority. With v1 removed, the mandated entry route is unavailable even though Gate 10A and Gate 10D2 proof report success.
- Evidence: `SkillManifest::required_binaries` derives the install set only from the nine lifecycle skill routes, none of which owns `csdlc-install`; the final coexistence inventory likewise omits it. The Gate 10A fixture writes arbitrary binary-name bytes, marks them executable, and asserts that an 11-entry receipt is correct without executing any installed binary. Current validation found eleven stable binaries and no executable `.adl/bin/csdlc-v2/csdlc-install`, while an installed lifecycle binary did execute `--help` successfully. The missing resolver is therefore exactly what the test contract currently accepts.
- Missing proof: Build and install the revision-current owner set, require `csdlc-install` in the final inventory/receipt, execute `verify` and `resolve` from the installed directory, and smoke every installed skill route rather than accepting executable dummy files.

### P1: Publication, readiness, and closeout authority has no executable GitHub boundary test

- File: `csdlc-v2/tests/gate6.rs:36`, `csdlc-v2/src/bin/csdlc-publish.rs:49`, `csdlc-v2/src/bin/csdlc-closeout.rs:155`, `docs/architecture/csdlc-v2/gate10d2/PARITY_EVIDENCE.json:19`
- Role: tests
- Scenario: GitHub returns multiple pages, a create request fails after creating the PR, check runs are rerun, reviews change state, push fails, mergeability is unknown, or terminal observation disagrees with the requested disposition.
- Impact: The active lifecycle authority can duplicate or mis-normalize publication, readiness, review, or terminal truth without the suite detecting it. Gate 10D2 nevertheless credits pure in-memory reconciliation/classification tests as complete proof of `github_issue_pr_checks`, `publication`, and `closeout` parity.
- Evidence: All four Gate 6 integration tests construct `PublicationIntent` and `RemotePullRequest` values directly; none launches `csdlc-publish`, a Git remote, or an HTTP fixture. The production binary performs token resolution, PR search/create/update, push, post-failure observation, and normalization through Octocrab. `csdlc-closeout` separately paginates check runs and reviews and observes terminal PR state, but its only binary-local test compares two run timestamps. No `csdlc-v2` test uses a mock HTTP server or a bounded live GitHub fixture.
- Missing proof: Deterministic HTTP/Git fixtures for create-after-timeout reconciliation, duplicate/mismatched PRs, pagination truncation, rerun ordering, review supersession, push failure, unknown mergeability, merged/closed/open terminal observations, token-source precedence, and no-mutation behavior on every remote failure.

### P1: PVF network and credential declarations are tested as labels, not execution controls

- File: `csdlc-v2/src/pvf.rs:269`, `csdlc-v2/src/pvf.rs:611`, `csdlc-v2/tests/gate4.rs:65`
- Role: tests
- Scenario: A lane declares `network: denied` or declares no credentials while the parent process has network access and secrets in its environment.
- Impact: A supposedly deterministic/offline proof lane can access the network and inherit undeclared credentials while still returning `local_pass`. This invalidates the claimed network/credential posture and can leak operator secrets into executed commands or evidence.
- Evidence: Selection rejects only an `external` declaration when `allow_network` is false and compares credential names against `available_credentials`. Execution then calls `Command::new(...).spawn()` without clearing/filtering the environment or applying network isolation. Gate 4's negative test checks manifest selection only; no test launches a denied-network probe or verifies that undeclared environment credentials are absent from the child.
- Missing proof: An execution-level denied-network fixture, loopback-versus-external cases, explicit credential injection, inherited-secret removal, and evidence assertions proving that policy is enforced rather than merely declared.

### P1: The 100% retained-behavior parity gate validates labels, not executable proof

- File: `csdlc-v2/tests/gate9.rs:270`, `docs/architecture/csdlc-v2/gate10d2/PARITY_EVIDENCE.json:5`, `docs/architecture/csdlc-v2/gate10d2/CAPABILITY_MATRIX.json:36`
- Role: tests
- Scenario: A proof reference is misspelled, points to a removed/renamed test, names a test that does not exercise the claimed capability, or the referenced test is filtered out/fails in the final installed configuration.
- Impact: Gate 10D2 can continue reporting 10,000 basis points and zero critical differences after its supporting proof disappears or ceases to prove the mapped behavior. That makes the deletion decision's central parity precondition non-executable.
- Evidence: `every_retained_behavior_has_current_parity_proof` checks only that every capability name is present, each `proof_refs` array is non-empty, and the JSON self-reports `10000`/`0`. It does not resolve test names, validate file/function existence, execute the mapped tests, bind results to a revision, or assess whether assertions cover the capability. Several broad capabilities are represented by one narrow unit test, including publication and GitHub issue/PR checks.
- Missing proof: A machine-readable capability-to-test manifest consumed by a runner that resolves every reference, rejects duplicates/stale names, executes the exact tests at the recorded revision, captures outcomes, and fails if the mapping does not cover declared negative behavior.

### P2: Gate 10A mutates the real checkout during a parallel test run

- File: `csdlc-v2/tests/gate10a.rs:65`
- Role: tests
- Scenario: `installer_records_provenance_without_replacing_other_files` runs concurrently with another test or command that verifies final v1 absence, or the test panics/interruption occurs after creating `adl/tools/pr.sh` and before removing it.
- Impact: The suite is non-hermetic and order-dependent, another verifier can observe a transient forbidden v1 command, and an interrupted run can leave the checkout dirty with a resurrected lifecycle wrapper.
- Evidence: The test points `repo` at `CARGO_MANIFEST_DIR/..`, creates `adl/tools/pr.sh` in that checkout, verifies its detection, and removes it. Other Gate 10A tests concurrently verify the same real repo under Rust's default parallel test runner. The fixture does not use an isolated repository and has no cleanup guard for panic or process termination.
- Missing proof: Copy only the bounded inventory fixture into a temporary repository and test forbidden-path discovery there; add a post-suite cleanliness assertion and avoid shared mutable checkout state.

### P2: Final test and size evidence uses a textual annotation count and a non-reproducible LoC command

- File: `csdlc-v2/src/proof.rs:318`, `csdlc-v2/tests/gate9.rs:10`, `docs/architecture/csdlc-v2/gate10d2/SIZE_EVIDENCE.json:4`
- Role: tests
- Scenario: A test module is included into more than one integration-test crate, a `#[test]` is cfg-disabled, generated by a macro, ignored, or present only as text, or the recorded size command is replayed.
- Impact: Budget evidence can undercount or overcount executable tests and cannot reproduce the claimed production-line measurement, weakening the final reviewability and test-ceiling claims.
- Evidence: `measure` counts literal `#[test]` substrings instead of registered runnable cases. `gate9.rs` includes `gate7_lifecycle.rs` as a module, so the same lifecycle test executes once in `gate7_lifecycle` and again in `gate9`; the current full run executed 101 test cases while `SIZE_EVIDENCE.json` reports 100 annotated functions. The same evidence says its production-line basis is `find csdlc-v2/src -name '*.rs' | wc -l`, which counts file names, not source lines, yet records 10,544 production lines.
- Missing proof: Record `cargo test -- --list`/runner JSON counts with ignored and duplicated identities separated, identify distinct behavioral tests, and replace the size basis with an exact checked command whose output reproduces the recorded value.

## Missing Proof Map

| Risk surface | Existing proof | Missing negative or realism proof |
| --- | --- | --- |
| Stable final authority | Gate 10A receipt/provenance fixture | Installed `csdlc-install verify/resolve`; installed-route smoke; real executable format |
| GitHub publication | Pure reconciliation structs | HTTP pagination/outage/create/update plus Git push and atomic no-mutation failures |
| Readiness/closeout | Fabricated observations and one timestamp helper | Octocrab response normalization, review/check reruns, terminal remote-state cross-product |
| PVF policy | Manifest-label rejection | Child-process network denial and credential/environment isolation |
| Parity completeness | Non-empty proof-reference strings | Reference resolution, exact test execution, revision/result binding, semantic mapping review |
| Store concurrency/recovery | Sequential stale-request and injected-backup recovery tests | Simultaneous thread/process writers, lock contention, crash at each rename boundary, concurrent heartbeat/recovery CAS |
| Path realism | Temporary UTF-8 paths and basic traversal/symlink cases | Spaces/non-UTF-8 paths, cross-device rename behavior, permissions/read-only filesystems, worktree common-dir variants |
| Counts/budgets | Literal annotation count and historical wall times | Registered test inventory, distinct-test count, reproducible current size/time commands |

## Reviewed Surfaces

- Every integration test file: `csdlc-v2/tests/gate2.rs`, `gate4.rs`, `gate5.rs`, `gate6.rs`, `gate7.rs`, `gate7_lifecycle.rs`, `gate8.rs`, `gate9.rs`, `gate10a.rs`, and `gate10b.rs`.
- Every source file with unit tests: `csdlc-v2/src/cutover.rs`, `csdlc-v2/src/eligibility.rs`, `csdlc-v2/src/bin/csdlc-publish.rs`, and `csdlc-v2/src/bin/csdlc-closeout.rs`.
- Test-adjacent owners and boundaries: cards/store/lifecycle/doctor/git, PVF/scheduler/shepherd, review/publication/readiness/closeout, migration/shadow/soak, operator/install/proof/cutover/eligibility, all binary entrypoints, schemas, and Cargo manifests.
- Gate proof claims and machine evidence from Gate 1 through Gate 10D2, including Gate 9 samples/soak/budget records, Gate 10A coexistence/install contracts, Gate 10B pre-switch proof, Gate 10C cutover evidence, Gate 10D1 eligibility, and Gate 10D2 parity/capability/deletion/size/final-recheck records.
- Sprint acceptance surfaces for #5228, #5232-#5240, #5292-#5295, and #5305-#5308.

## Validation Performed

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked` passed: 101 executed test cases, 0 failed, including the duplicated path-included lifecycle test; wall time was approximately 55.6 seconds to build plus 59.6 seconds of reported test execution.
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings` passed.
- Stable-install inspection found eleven receipt entries under `.adl/bin/csdlc-v2/`, no `csdlc-install` executable, and a working `csdlc-init --help` entrypoint.
- `git status --short --branch` was clean after validation; the Gate 10A checkout mutation did not remain after this successful run.
- No live mutating GitHub publication/closeout command was run; the missing controlled boundary proof is Finding 2.

## Residual Risk

- The current suite is green, but it does not establish that the installed final authority is self-resolving, that GitHub-backed lifecycle transitions behave correctly under real response sequences, or that PVF policy constrains child processes.
- Store locking and transaction recovery were inspected, but no adversarial concurrent writer or process-kill campaign was executed. Filesystem behavior outside the local macOS/tempdir path remains unproved.
- Historical Gate 10A-C evidence was treated as immutable and was not regenerated. Gate 10D2 proof was assessed as current final-authority evidence without editing any lifecycle artifact.
- No implementation, tests, config, cards, existing docs, or GitHub issues were changed or created by this review.
