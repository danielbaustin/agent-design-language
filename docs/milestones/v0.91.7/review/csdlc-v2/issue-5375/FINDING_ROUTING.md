# Finding Routing

This register routes every synthesized finding without creating or modifying a
remediation issue. Owners are component roles, not assignees. `Must fix` means
the defect blocks acceptance of the affected sprint claim; `Follow-up` means
operator triage may schedule the bounded correction after accepting this review
packet. Neither disposition claims remediation has occurred.

| Finding | Owner | Required correction | Proving validation | Disposition |
| --- | --- | --- | --- | --- |
| P1-01 | PVF execution owner | Enforce child environment, credential, network, redaction, and evidence policy at execution time. | Isolated child-process tests proving denied network and credentials plus redacted retained output. | Must fix |
| P1-02 | GitHub evidence owner | Bind readiness and closeout to canonical repository, PR, base, head, state, and revision identity. | Deterministic remote fixture rejects repository, PR, branch, state, and revision substitution. | Must fix |
| P1-03 | Readiness policy owner | Derive required checks and review policy from canonical VPP or repository policy, not caller options. | Fixture proves empty or weakened request policy cannot advance readiness. | Must fix |
| P1-04 | State-store owner | Resolve and confine every control path beneath a trusted repository root with no-follow semantics. | Symlink-ancestor and cleanup escape tests on every mutating owner. | Must fix |
| P1-05 | Review/publication owner | Require a clean reviewable commit or provide a typed re-review transition after commit. | Dirty-review, commit, re-review, and exact-publication regression sequence. | Must fix |
| P1-06 | Binding/lifecycle owner | Make bind and recovery side effects owner-exclusive and validate topology and replacement claims. | Concurrent bind, stale recovery, ordinary-directory, and generic-phase-bypass tests. | Must fix |
| P1-07 | Review owner | Derive reviewed scope from the revision diff and bind declared paths to the digest. | Out-of-scope changed path causes review or publication rejection. | Must fix |
| P1-08 | SOR/readiness owner | Model validation attempts and explicit supersession instead of requiring every historical attempt to pass. | Fail, repair, pass sequence advances only with retained supersession truth. | Must fix |
| P1-09 | Closeout/SOR owner | Enforce semantic SOR completion on every terminal closeout path. | Terminal remote state cannot close out incomplete or contradictory SOR truth. | Must fix |
| P1-10 | State architecture owner | Use one canonical state and lock namespace shared by all linked worktrees. | Two-worktree concurrency and canonical-path identity tests. | Must fix |
| P1-11 | Install owner | Install or safely bootstrap the mandatory resolver and verify final generation authority. | Clean-host install resolves and verifies all nine typed routes with no v1 path. | Must fix |
| P1-13 | Gate 10D parity owner | Replace label-only parity claims with executable capability-to-proof mappings. | Every retained capability resolves to an executable, passing proof reference. | Must fix |
| P1-14 | Docs/template owner | Remove active v1 commands from current operator docs and generated cards. | Current-template render and active-doc scan contain only resolvable v2 commands. | Must fix |
| P1-15 | Lifecycle closeout owner | Retain and reconcile tracked SRP/SOR and dependency disposition for all 18 issues. | Issue-by-issue closeout verifier passes against merged PR and terminal issue truth. | Must fix |
| P1-16 | Gate 10D authority owner | Bind approval to the exact evaluator revision, inputs, and deletion diff. | Tampered evaluator, input, or deletion set invalidates approval. | Must fix |
| P1-17 | Dependency owner | Reconcile declared MSRV and locked dependency graph. | Locked clean build and tests pass on the declared minimum Rust version. | Must fix |
| P1-18 | Supply-chain/install owner | Bind receipts to reviewed source, lockfile, toolchain, build command, and artifact provenance. | Reproducible install verifies provenance and rejects altered build inputs. | Must fix |
| P1-19 | Session ownership owner | Expose typed heartbeat and guarded stale-claim recovery routes. | Operator-level lease renewal and recovery tests cover ownership, expiry, and collision rejection. | Must fix |
| P2-01 | Publication owner | Make canonical publication replay idempotent after the successful commit. | Repeated publish request returns the same normalized result without mutation or failure. | Follow-up |
| P2-02 | Session ownership owner | Bound leases and make expiry and renewal policy explicit. | Time-controlled lease expiry, renewal, and takeover tests. | Follow-up |
| P2-03 | Migration owner | Prevent legacy import from persisting untrusted instructions or secret values. | Adversarial import fixture proves filtering, redaction, and provenance. | Follow-up |
| P2-04 | Resolver owner | Reject explicit v1 selection under final `v1_sunset` authority. | Selector matrix rejects every v1 override and accepts current v2 routes. | Follow-up |
| P2-05 | Edit/init owner | Route bootstrap through guarded initialization invariants. | Direct bootstrap cannot bypass collision, structure, or authority checks. | Follow-up |
| P2-06 | Gate 10A test owner | Isolate coexistence tests from the real checkout. | Parallel tests run in disposable repositories and leave the source tree unchanged. | Follow-up |
| P2-07 | Evidence owner | Record reproducible source-size and executed-test-count methods. | Clean rerun reproduces retained counts from documented commands. | Follow-up |
| P2-08 | Operator documentation owner | State final selector, resolver, install, and typed-skill authority consistently. | Active guidance audit has one complete, non-conflicting operational route. | Follow-up |
| P2-09 | Release owner | Resolve #5228's v0.91.7 versus v0.92 identity in canonical planning truth. | Issue, milestone, cards, and release inventory report one version disposition. | Follow-up |
| P2-10 | Dependency/CI owner | Add continuous locked, MSRV, advisory, license, and supply-chain checks. | Required CI gate runs on clean pull requests and fails on controlled violations. | Follow-up |
| P2-11 | GitHub adapter owner | Use the shared token resolver and documented precedence in every GitHub owner. | Environment/file precedence tests cover publish, readiness, and closeout without exposing values. | Follow-up |
| P2-12 | GitHub test owner | Exercise production HTTP and Git adapters across failure, retry, pagination, and reconciliation boundaries. | Deterministic fixture covers create-after-timeout, push failure, reruns, supersession, unknown mergeability, and terminal disagreement. | Follow-up |
