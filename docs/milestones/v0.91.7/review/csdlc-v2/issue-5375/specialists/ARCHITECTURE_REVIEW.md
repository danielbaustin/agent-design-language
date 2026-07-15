# C-SDLC v2 Architecture Review

## Metadata

- Skill: `repo-architecture-review`
- Target: issue #5375 full 18-issue C-SDLC v2 sprint at `7c3e1e0e86a4ca982231ce91c39073530c5408e6`
- Date: 2026-07-15
- Artifact: `docs/milestones/v0.91.7/review/csdlc-v2/issue-5375/specialists/ARCHITECTURE_REVIEW.md`
- Packet: sibling issue #5375 specialist artifacts plus current source and Gate 1-10 architecture evidence
- Review mode: deep architecture-only review; no implementation, tests, configs, cards, existing docs, GitHub state, or lifecycle state were mutated

## Findings

### P1: Canonical lifecycle state and issue-checkout identity share one root, so binding cannot preserve a single ledger across Git worktrees

- File: `csdlc-v2/src/store.rs:24-39`, `csdlc-v2/src/lifecycle.rs:135-166`, `csdlc-v2/src/review.rs:39-64`
- Role: `architecture`
- Scenario: `csdlc-init` and `csdlc-bind` run from the clean primary checkout, create an issue worktree, and subsequent edit, validation, review, publication, and closeout commands run from that bound worktree as required by repository policy.
- Architecture boundary or layer: state ownership, Git/worktree boundary, transaction and concurrency design.
- Impact: `Store::root()` is both the parent of the untracked `.csdlc/issues` ledger and the checkout used for Git branch, diff, design-path, and topology operations. Linked worktrees do not share untracked working-tree directories. Pointing later owners at the primary root preserves the ledger but reviews/publishes `main`; pointing them at the issue root inspects the correct branch but creates or consumes a separate ledger and separate `.csdlc/locks` namespace. Canonical generations, leases, protected-path collisions, and transaction locks can therefore fork across worktrees.
- Evidence: `Store::issue_dir` resolves state under `root/.csdlc/issues`; bind creates the linked worktree relative to the same root but does not relocate, link, or copy canonical state. Review computes `substantive_revision` against that same store root. The Gate 3 test at `csdlc-v2/tests/gate2.rs:107-137` checks the record only through the original primary-root `Store` after worktree creation, while the end-to-end Gate 7 fixture at `csdlc-v2/tests/gate7_lifecycle.rs:58-97` starts with one checkout already on the issue branch; neither proves a primary-to-linked-worktree handoff. Gate 2 describes one canonical `.csdlc/issues/<number>` record, but no source owner resolves it through `git rev-parse --git-common-dir` or another shared repository-state location.
- Missing proof: A real linked-worktree lifecycle that initializes and binds from the primary checkout, then runs every post-bind owner from the issue worktree against one shared record; assertions that both checkouts observe the same generation/digest; and concurrent primary/worktree writers proving one shared lock and compare-and-swap domain.
- Discovery relation: independently derived. None of testing discoveries #5364-#5373 identifies the state-root/Git-root conflation; #5364 and #5368 demonstrate post-bind editing/review pressure but do not establish this boundary defect.
- Recommended follow-up owner: C-SDLC v2 state-store and Git-adapter owners, with an ADR for canonical repository state placement.

### P1: The generic card editor can forge `Bound` without invoking the sole Git binding owner

- File: `csdlc-v2/src/store.rs:831-868`, `csdlc-v2/src/store.rs:1202-1209`, `csdlc-v2/src/lifecycle.rs:135-230`
- Role: `architecture`
- Scenario: A caller submits `SemanticOperation::AdvancePhase { phase: "bound" }` through `csdlc-edit apply` for a `Ready` record without running `csdlc-bind`.
- Architecture boundary or layer: lifecycle/state-machine authority and Git binding boundary.
- Impact: The record becomes `Bound` without the repository binding lock, primary-branch check, branch/worktree collision checks, protected-path collision checks, worktree creation, or compensation behavior owned by `csdlc-bind`. Downstream owners trust phase and claim truth, so execution and review can proceed against a directory that was never bound while another issue legitimately owns the same Git topology or protected paths.
- Evidence: `authorize_card_operation` unconditionally accepts every `AdvancePhase` before card/phase ownership checks; `apply` returns the requested phase and `edit_issue` calls the generic transition table. The specialized bind path performs the missing global and Git checks before its own `Ready -> Bound` transition. Gate 7's lifecycle fixture uses the bypass directly at `csdlc-v2/tests/gate7_lifecycle.rs:150-157`, proving it is accepted behavior rather than an unreachable API. Gate 3's design states that successful binding records `Ready -> Bound` after Git worktree creation, which the generic route contradicts.
- Missing proof: Reject `Ready -> Bound` from the generic editor; prove only `csdlc-bind` can commit that transition; and run negative integration fixtures for absent worktree, overlapping protected paths, branch-at-other-worktree, and concurrent bind attempts.
- Discovery relation: independently derived and not covered by testing discoveries #5364-#5373.
- Recommended follow-up owner: lifecycle state-machine and `csdlc-bind` owners.

### P1: Final routing authority is not part of the installation it certifies

- File: `csdlc-v2/AGENTS.md:6-8`, `csdlc-v2/src/operator.rs:113-125`, `csdlc-v2/operator/coexistence.json:17`
- Role: `architecture`
- Scenario: After Gate 10D2 removes v1, an operator uses the stable `.adl/bin/csdlc-v2/` generation directory and follows the mandatory `csdlc-install resolve` route before invoking one of the nine lifecycle owners.
- Architecture boundary or layer: authority routing, installation/provenance boundary, cutover architecture.
- Impact: The certified stable generation can contain every required lifecycle binary while omitting `csdlc-install`, the only owner allowed to resolve the selector and verify provenance. The final authority is therefore not self-hosting: operators must reach outside the verified generation for its router, or invoke lifecycle binaries directly and bypass the declared sole authority.
- Evidence: The nested agent contract mandates `csdlc-install resolve` and `verify`. `SkillManifest::required_binaries` derives the install set exclusively from the nine lifecycle skill entries and auxiliaries; none names `csdlc-install`. The final coexistence inventory lists ten lifecycle binaries and omits the installer/resolver. Gate 10A's installer test at `csdlc-v2/tests/gate10a.rs:31-52` explicitly accepts the resulting eleven receipt entries (ten binaries plus receipt) without requiring the resolver.
- Missing proof: Include the resolver/verifier in the reviewed generation inventory and receipt, then execute installed `csdlc-install verify` and `resolve` before smoke-running every installed route at the final selector state.
- Discovery relation: independently derived. Testing discovery #5369 reports a selector symlink mismatch in a different live layout; at revision `7c3e1e0e86a4` the selector is a regular tracked file, and #5369 does not cover the missing stable resolver. The sibling test review independently corroborates the omitted-binary result.
- Recommended follow-up owner: Gate 10 install/selector authority owner.

### P1: Gate 10D2 approval does not bind the code that evaluates it or the deletion diff that executes it

- File: `csdlc-v2/src/eligibility.rs:276-310`, `csdlc-v2/src/eligibility.rs:312-335`, `docs/architecture/csdlc-v2/gate10d2/ELIGIBILITY_DECISION.json:1-30`
- Role: `architecture`
- Scenario: An approval is authored at one revision, a descendant changes eligibility logic or deletes paths outside the approved manifest, and the descendant is presented as the completed D2 wave.
- Architecture boundary or layer: cutover/deletion authorization, evidence provenance, transaction boundary.
- Impact: A descendant is accepted whenever the approved revision is merely an ancestor, so the authority being approved may change after approval. The typed evaluator is intentionally non-mutating and always emits `deletion_executed: false`; no owner compares the actual Git deletion diff with the approved manifest. D2 can therefore claim exact approval and completed deletion without machine evidence that the approved evaluator authorized the executing revision or that only approved paths were removed.
- Evidence: The evaluator's revision check explicitly accepts `merge-base --is-ancestor` instead of exact equality, contradicting D2's exact-revision statements at `docs/architecture/csdlc-v2/gate10d2/DESIGN.md:9-21`. The tracked approval names `595a7f38347e4337b84addbd28f247cdeefa282f`, while the deletion landed in descendant `13d5a6f36` and the final recheck names later descendant `e0f25d5a9c89a62cc4e3437156dca04bb85b7a19`. The Rust decision schema is `csdlc.deletion_eligibility.v1` and hard-codes `deletion_executed: false`; the tracked D2 file instead uses an unowned `csdlc.deletion_eligibility_decision.v2` shape and asserts `deletion_executed: true`. No source or test consumes `DELETION_EVIDENCE.json` or verifies the executed diff against the manifest.
- Missing proof: Exact evaluator/candidate revision binding; a typed post-deletion verifier that derives the actual name-status/line diff, rejects every unapproved path and retained-path deletion, emits the tracked completion evidence, and reruns from the reviewed merge candidate; and a test showing that any descendant code change after approval fails closed.
- Discovery relation: independently derived and not covered by testing discoveries #5364-#5373. The sibling test review's parity-reference finding corroborates the broader weakness of D2 proof, but not this approval-to-executed-diff gap.
- Recommended follow-up owner: Gate 10D eligibility/deletion authority owner, followed by an ADR for approval and execution provenance.

### P1: Readiness and closeout observe a caller-selected repository that is absent from canonical remote evidence

- File: `csdlc-v2/src/bin/csdlc-closeout.rs:54-80`, `csdlc-v2/src/bin/csdlc-closeout.rs:155-165`, `csdlc-v2/src/store.rs:275-315`
- Role: `architecture`
- Scenario: A request supplies another GitHub `owner/repo` containing the same PR number and head SHA as the canonical publication, then asks `observe-readiness` or `closeout` to record that remote's checks, reviews, or terminal state.
- Architecture boundary or layer: Git/remote identity and lifecycle integration boundary.
- Impact: Green checks, approval, conflict state, or merged/closed state from a different repository can advance the canonical issue to `MergeReady` or `ClosedOut`. PR number and commit SHA are not globally unique repository identities, so matching only those fields does not preserve the publication boundary.
- Evidence: Both GitHub request structs accept `repository` from the caller. The adapter queries that repository, but `ReadinessRequest`, `ReadinessEvidence`, and `TerminalObservation` carry no repository identity. Store reconciliation compares only canonical PR number and clean-commit revision. Publication evidence already owns `repository`, base, head, PR, and revision, but Gate 7 discards the first three before commit despite its design claiming exact PR identity at `docs/architecture/csdlc-v2/gate7/GATE7_READINESS_CLOSEOUT_DESIGN.md:45-58`.
- Missing proof: Derive repository and PR identity from canonical publication evidence rather than request input; retain owner/repo, base, head, PR number, and SHA in normalized observations; and test cross-repository same-number/same-SHA substitutions for readiness and every terminal disposition.
- Discovery relation: independently derived and corroborates the sibling security review. Testing discoveries #5370 and #5371 concern absent first-class PR-state collection and shepherd integration; they do not identify cross-repository identity substitution.
- Recommended follow-up owner: Gate 7 GitHub adapter and canonical remote-evidence owners.

## Assumptions

- The reviewed revision is exactly `7c3e1e0e86a4ca982231ce91c39073530c5408e6`; historical Gate 10A-C evidence was treated as immutable evidence, not current command guidance.
- Root and nested `AGENTS.md` contracts are current architecture authority for worktree and selector routing.
- #5364-#5373 were read only after source findings were established and were used solely to classify overlap or distinction.

## Architecture Map

- Top-level boundary: independent `csdlc-v2` Rust crate; no ADL Runtime or incumbent C-SDLC crate dependency was found in the source boundary.
- Entrypoints: typed binaries for init, bind, edit/validate, review, publish, shepherd, closeout, doctor, install/proof/cutover/eligibility, migration shadow, and soak.
- State boundary: `Store` owns per-issue JSON/card generations, issue locks, backup/staging rename transactions, claims, audit events, and lifecycle transitions under `.csdlc`.
- Git boundary: `git.rs` and lifecycle/publication/closeout adapters own branch, worktree, revision, push, and prune interactions.
- Remote boundary: Octocrab adapters in publish and closeout normalize GitHub PR/check/review/terminal observations before store commits.
- Cutover boundary: selector resolution, stable installation, coexistence verification, pre-switch proof, reversible cutover, non-mutating deletion eligibility, and D2 tracked evidence.
- Gate evidence: Gate 1 baseline/architecture; Gate 2 state/cards/transactions; Gate 3 binding/claims; Gate 4 PVF/scheduler/shepherd; Gate 5 review; Gate 6 publication; Gate 7 readiness/closeout; Gate 8 import/shadow; Gate 9 soak/decision; Gate 10A-D4 install, proof, cutover, eligibility, deletion, and cleanup designs/evidence.

## Reviewed Surfaces

- `csdlc-v2/src/`, all binary entrypoints, `csdlc-v2/operator/`, schemas, manifests, and integration tests.
- `docs/architecture/csdlc-v2/` Gate 1 through Gate 10D4 designs, diagrams, machine evidence, capability/parity records, and retained-behavior contracts.
- Issue #5375 sibling code, security, test, and documentation specialist artifacts for overlap classification only.
- Git history for the sprint implementation and Gate 10D2 approval/deletion revisions.
- Live issue text for testing discoveries #5364-#5373, used only after independent findings were formed.

## Candidate Diagram Tasks

- `diagram-author`: map the required single canonical state/lock location across primary checkout, linked issue worktree, Git common directory, and terminal receipt/prune flow.
- `diagram-author`: map Gate 10D approval inputs to evaluator revision, approved manifest, actual deletion diff, post-deletion verification, and reviewed merge candidate.

## Candidate ADRs

- Canonical C-SDLC state placement and lock namespace across linked Git worktrees.
- Exclusive lifecycle-transition ownership: which transitions are generic record operations versus side-effecting owner-only operations.
- Exact revision and diff provenance for destructive cutover approvals.

## Candidate Fitness Functions

- Fail if any side-effecting phase transition, especially `Ready -> Bound`, is reachable through generic `csdlc-edit` operations.
- Run a linked-worktree lifecycle and assert one shared state generation and lock namespace from both checkouts.
- Fail final install verification unless the installed generation contains and executes its own resolver/verifier.
- Deserialize every tracked Gate decision/evidence file through an owned schema and reject hand-authored schema variants.
- Reject Gate 10D approval at every non-identical revision and verify the actual deletion diff exactly against the approved manifest.
- Reject readiness/closeout observations whose full repository/base/head/PR/SHA identity differs from publication evidence.

## Validation Performed

- Source and evidence inspection with `rg`, `find`, `nl`, `sed`, `jq`, `git log`, `git show`, and `git diff` at the requested revision.
- Live `gh issue view` inspection of #5364-#5373 solely to classify corroboration and avoid re-reporting testing discoveries as independent findings.
- No mutating lifecycle command, GitHub publication/closeout operation, Rust test, or implementation validation was run; this lane is architecture findings-only.
- `git diff --check` is the required artifact-format validation and is recorded after writing.

## Residual Risk

- No live GitHub mutation or adversarial multi-process filesystem campaign was run. HTTP behavior, OS-level locking semantics, crash points beyond inspected transaction code, and non-macOS filesystems remain unproved.
- The review inspected the complete sprint architecture surface but did not re-run every historical Gate proof. Green historical claims were not treated as proof of the missing cross-boundary scenarios above.
- Code, security, test, documentation, dependency, and synthesis owners retain their own findings and severity decisions; this artifact does not supersede them.
