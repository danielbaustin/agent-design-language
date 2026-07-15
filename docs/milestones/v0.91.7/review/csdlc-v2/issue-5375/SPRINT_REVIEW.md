# C-SDLC v2 Full Sprint Review

## Findings

Severity policy: P0 is reserved for a demonstrated active catastrophic failure;
P1 for a high-impact authority, integrity, isolation, or lifecycle failure that
can invalidate canonical truth or block the supported workflow; P2 for a
material but bounded correctness, recoverability, provenance, or governance
gap; and P3 for low-impact hygiene. No P0 or P3 finding is supported by this
packet. Passing current validation does not lower a finding whose triggering
path is absent from that validation.

### P1-01: PVF network, credential, redaction, and evidence policies are declarations, not execution controls

- Source roles: code, security, tests.
- Affected gates/issues: Gate 4 / #5234; retained-behavior and deletion proof in
  Gate 10D2 / #5306.
- Testing-discovery relation: review-only. None of #5364-#5373 establishes this
  execution-isolation defect.
- Scenario and impact: A lane can declare denied network and no credentials,
  then execute with inherited host environment and network access. Undeclared
  secrets can reach the child or its log while `redaction_ok` is reported true,
  invalidating deterministic/offline proof and credential-isolation claims.
- Evidence: `csdlc-v2/src/pvf.rs:269-286` checks caller declarations;
  `csdlc-v2/src/pvf.rs:621-628` spawns without environment filtering or network
  isolation; `csdlc-v2/src/pvf.rs:680-707` redacts only configured literals and
  reports success; `csdlc-v2/tests/gate4.rs:65` proves labels, not confinement.

### P1-02: Readiness and closeout can consume remote truth from the wrong repository or PR identity

- Source roles: code, security, architecture.
- Affected gates/issues: Gate 7 / #5237, with Gate 6 publication identity from
  #5236.
- Testing-discovery relation: review-only mechanism. #5370 and #5371 concern
  PR-state collection and shepherd integration, but do not establish
  cross-repository identity substitution.
- Scenario and impact: The caller selects a repository for readiness or
  terminal observation. Repository, base, head, draft/open state, and full PR
  identity are not retained in the normalized commit guard, so another remote
  object sharing a PR number and commit SHA can advance canonical state to
  `MergeReady` or `ClosedOut`.
- Evidence: `csdlc-v2/src/bin/csdlc-closeout.rs:155-305` and
  `csdlc-v2/src/bin/csdlc-closeout.rs:308-344` query request-selected remotes;
  `csdlc-v2/src/store.rs:305-315` and `csdlc-v2/src/store.rs:419-451` reconcile
  only PR number and revision; `csdlc-v2/src/readiness.rs:94-169` has no field
  for repository, base, head, draft, or open state.

### P1-03: Merge-readiness requirements are caller-selected instead of canonical policy

- Source roles: security.
- Affected gates/issues: Gate 7 / #5237; VPP and branch-policy authority across
  the sprint.
- Testing-discovery relation: review-only. #5370 and #5371 are adjacent but do
  not prove policy weakening through request fields.
- Scenario and impact: A caller can submit no required checks and disable review
  requirements. Canonical state may become `MergeReady` without satisfying the
  repository's real CI or approval policy.
- Evidence: `csdlc-v2/src/bin/csdlc-closeout.rs:54-66` exposes
  `required_checks` and `require_review`; `csdlc-v2/src/bin/csdlc-closeout.rs:215-230`
  forwards them; `csdlc-v2/src/readiness.rs:94-149` accepts an empty minimum;
  `csdlc-v2/src/store.rs:275-377` does not bind them to VPP, publication, or
  branch protection.

### P1-04: Symlinked `.csdlc` ancestors can redirect state writes and cleanup outside the repository

- Source roles: security.
- Affected gates/issues: Gate 2 / #5232 and every later state-mutating gate.
- Testing-discovery relation: review-only.
- Scenario and impact: An untrusted checkout can symlink `.csdlc` control
  directories outside the repository. Locks, staged writes, atomic renames,
  publication intent, compatibility output, and recursive recovery cleanup then
  operate on predictable external paths.
- Evidence: `csdlc-v2/src/store.rs:23-75` uses lexical joins and ordinary opens;
  `csdlc-v2/src/store.rs:95-139` removes and renames staging/backup paths without
  ancestor no-follow checks; `csdlc-v2/src/bin/csdlc-publish.rs:173` and
  `csdlc-v2/src/migration.rs:566` repeat the lexical-parent pattern.

### P1-05: Reviewing a dirty worktree creates an unrecoverable `Reviewed` state

- Source roles: code; corroborated by gap analysis.
- Affected gates/issues: Gate 5 / #5235 and Gate 6 / #5236.
- Testing-discovery relation: independently corroborates testing issue #5368.
  The code review derived the defect before consulting that issue.
- Scenario and impact: Review accepts a digest of `HEAD` plus dirty content,
  publication requires a clean-commit digest, and committing necessarily changes
  the digest. Reassignment is forbidden after `Reviewed`, leaving no typed
  recovery path.
- Evidence: `csdlc-v2/src/git.rs:55-112` hashes dirty state;
  `csdlc-v2/src/review.rs:39-112` accepts and advances it;
  `csdlc-v2/src/publication.rs:117-140` requires the clean commit revision;
  `csdlc-v2/src/store.rs:607-611` permits assignment only from `Implemented`.

### P1-06: Alternate binding and recovery paths bypass exclusive ownership invariants

- Source roles: code, architecture.
- Affected gates/issues: Gate 3 / #5233, with Gate 2 state authority / #5232.
- Testing-discovery relation: review-only. #5364 and #5368 show post-bind
  workflow pressure but do not establish these bypasses.
- Scenario and impact: Three distinct paths weaken the binding owner: stale-claim
  recovery replaces claims without the global binding lock, collision/topology
  checks, or replacement validation; an existing ordinary directory can be
  recorded as a worktree; and generic `csdlc-edit` can forge `Ready -> Bound`
  without any Git side effects. Each can create false or colliding canonical
  ownership.
- Evidence: recovery at `csdlc-v2/src/lifecycle.rs:300-347` versus normal guards
  at `csdlc-v2/src/lifecycle.rs:53-87` and `135-189`; existing-directory handling
  at `csdlc-v2/src/lifecycle.rs:142-165` and `225-260`; generic authorization at
  `csdlc-v2/src/store.rs:831-868` and `1202-1209`, exercised by
  `csdlc-v2/tests/gate7_lifecycle.rs:150-157`.

### P1-07: Review scope is recorded but does not constrain the reviewed revision

- Source roles: code; corroborated by gap analysis.
- Affected gates/issues: Gate 5 / #5235 and Gate 6 publication guard / #5236.
- Testing-discovery relation: review-only.
- Scenario and impact: Narrow declared scope can accompany a whole-tree digest
  containing out-of-scope changes. Publication then claims exact-revision review
  although evidence only repeats, rather than proves, scope coverage.
- Evidence: `csdlc-v2/src/git.rs:55-112` ignores supplied paths and hashes `.`;
  `csdlc-v2/src/review.rs:53-69` stores arbitrary nonempty scope;
  `csdlc-v2/src/review.rs:139-149` checks equality with the assignment only;
  `csdlc-v2/src/publication.rs:119-132` trusts that pair.

### P1-08: Truthful validation failure cannot be superseded by a successful retry

- Source roles: code; corroborated by gap analysis.
- Affected gates/issues: Gate 9 / #5239 and readiness across Gate 7 / #5237.
- Testing-discovery relation: review-only.
- Scenario and impact: Append-only SOR retains a failed attempt, while readiness
  requires every historical result to pass or skip. A normal fail, fix, and pass
  sequence cannot reach `MergeReady` without deleting truthful evidence.
- Evidence: append behavior at `csdlc-v2/src/cards.rs:761-765`; all-history
  aggregation at `csdlc-v2/src/store.rs:1117-1127`; phase/readiness guards at
  `csdlc-v2/src/store.rs:325-339` and `1153-1174`.

### P1-09: Terminal closeout bypasses SOR completion invariants

- Source roles: code; corroborated by gap analysis.
- Affected gates/issues: Gate 7 / #5237 and terminal truth for all 18 issues.
- Testing-discovery relation: review-only.
- Scenario and impact: Closed-without-PR or closed-unmerged paths can directly
  mark SOR complete without nonempty successful validation, producing canonical
  output that the semantic card API itself rejects.
- Evidence: `csdlc-v2/src/cards.rs:892-919` defines the completion guard;
  `csdlc-v2/src/store.rs:454-513` assigns `Complete` directly;
  `csdlc-v2/src/readiness.rs:293-297` permits terminal dispositions that do not
  imply successful validation.

### P1-10: Canonical state and checkout identity share one root and can fork across linked worktrees

- Source roles: architecture.
- Affected gates/issues: Gate 2 / #5232, Gate 3 / #5233, and all post-bind owners
  #5234-#5237.
- Testing-discovery relation: review-only. #5364 and #5368 are adjacent workflow
  symptoms, not proof of the state-root/Git-root conflation.
- Scenario and impact: Using the primary root preserves its untracked ledger but
  inspects `main`; using the issue-worktree root inspects the branch but creates
  another `.csdlc` ledger and lock namespace. Generations, leases, collisions,
  and transaction serialization can diverge.
- Evidence: `csdlc-v2/src/store.rs:24-39` roots state under checkout-local
  `.csdlc`; `csdlc-v2/src/lifecycle.rs:135-166` creates a linked worktree without
  sharing state; `csdlc-v2/src/review.rs:39-64` computes Git truth from the same
  root. `csdlc-v2/tests/gate2.rs:107-137` stays on the primary store, while
  `csdlc-v2/tests/gate7_lifecycle.rs:58-97` starts already on the issue branch.

### P1-11: The certified final installation omits its mandatory resolver/verifier

- Source roles: tests, architecture, dependency.
- Affected gates/issues: Gate 10A / #5292 and final v1 sunset Gate 10D2 / #5306.
- Testing-discovery relation: review-only and independently corroborated by three
  review lanes. #5369 concerns selector-file shape and does not cover the missing
  installed resolver.
- Scenario and impact: The verified stable directory contains lifecycle binaries
  but no `csdlc-install`, even though current policy requires every route to use
  `csdlc-install resolve`. The final authority is not self-resolving and pushes
  operators toward unreceipted build output or direct invocation.
- Evidence: install-set derivation at `csdlc-v2/src/operator.rs:113-125` and
  `289-325`; omission in `csdlc-v2/operator/coexistence.json:17`; accepted dummy
  install fixture at `csdlc-v2/tests/gate10a.rs:31-71`; required route at
  `csdlc-v2/AGENTS.md:6-8`. Current validation observed 11 receipt entries and no
  stable `csdlc-install`.

### P1-13: Gate 10D2's 100% retained-behavior parity claim validates labels, not executable mappings

- Source roles: tests; architecture corroborates the broader provenance gap.
- Affected gates/issues: Gate 9 / #5239 and Gate 10D2 / #5306.
- Testing-discovery relation: review-only.
- Scenario and impact: Misspelled, stale, filtered, duplicated, or semantically
  irrelevant proof references can still support 10,000 basis points and zero
  critical differences, weakening the central deletion precondition.
- Evidence: `csdlc-v2/tests/gate9.rs:270` checks capability names and nonempty
  strings, not test resolution/execution; claims are recorded at
  `docs/architecture/csdlc-v2/gate10d2/PARITY_EVIDENCE.json:5` and mapped at
  `docs/architecture/csdlc-v2/gate10d2/CAPABILITY_MATRIX.json:36`.

### P1-14: Active operator docs and the current card template generate sunset v1 instructions

- Source roles: docs.
- Affected gates/issues: final authority Gate 10D2 / #5306 and every future issue
  initialized from the active templates.
- Testing-discovery relation: independently overlaps #5366's stale-authority
  subject; the docs lane derived the finding from tracked current sources.
- Scenario and impact: Two mechanisms remain: contributor-facing workflow,
  onboarding, playbook, and template documentation directly name deleted v1
  commands; and active template set 1.0.3 deterministically renders those
  commands into new SIPs. Operators and generated lifecycle state begin from an
  unusable and internally contradictory route.
- Evidence: stale commands at `docs/default_workflow.md:1-60`,
  `docs/onboarding.md:18-34`, `docs/codex_playbook.md:31-45`, and
  `docs/templates/prompts/README.md:80-99`; active selection at
  `docs/templates/prompts/current.json:1-12`; generated instructions at
  `docs/templates/prompts/1.0.3/sip.md:21-24`, `119-125`, and `171-175`.

### P1-15: Closed sprint issues do not retain durable, reconciled SRP/SOR and dependency truth

- Source roles: docs; corroborated by issue coverage and gap analysis.
- Affected gates/issues: all 18 issues; acute at #5240, #5295, #5306, #5307,
  and #5308.
- Testing-discovery relation: review-only lifecycle/closeout finding.
- Scenario and impact: Two mechanisms combine to defeat auditability: all 108
  local card files are ignored/untracked and absent from the review worktree,
  while many surviving local projections say not started, not run, worktree
  only, or open despite merged/closed GitHub truth. Umbrella and sunset children
  were closed/superseded without issue-local terminal dispositions.
- Evidence: `.gitignore:4` ignores `.adl`; #5306 SOR lines 17 and 82 remain
  `NOT_STARTED`/`worktree_only` and SRP lines 58-59 remain `not_run`; similar
  states persist for #5307/#5308; #5240 remains in progress. Live ordering in
  `docs/architecture/csdlc-v2/gate10d2/ACCELERATED_OPERATOR_DECISION.md` context
  shows #5240 closed before #5331 merged and #5295 closed before #5307/#5308,
  without reconciled closeout in their cards.

### P1-16: Gate 10D2 approval is not bound to the evaluator code or actual deletion diff

- Source roles: architecture.
- Affected gates/issues: Gate 10D1 / #5305 and Gate 10D2 / #5306.
- Testing-discovery relation: review-only.
- Scenario and impact: Eligibility accepts descendants of an approved revision,
  allowing evaluator logic to change after approval; the typed evaluator never
  proves deletion execution; and no owner compares the actual Git deletion diff
  with the approved manifest. Exact approval can therefore be claimed without
  exact code/diff provenance.
- Evidence: descendant acceptance at `csdlc-v2/src/eligibility.rs:276-310`;
  non-mutating output at `csdlc-v2/src/eligibility.rs:312-335`; tracked approval
  shape at `docs/architecture/csdlc-v2/gate10d2/ELIGIBILITY_DECISION.json:1-30`;
  no source/test consumes `DELETION_EVIDENCE.json` as an executed-diff verifier.

### P1-17: The locked graph violates the declared Rust 1.85 MSRV

- Source roles: dependency.
- Affected gates/issues: Gate 2 standalone workspace / #5232, Gate 10A install /
  #5292, and final authority / #5306.
- Testing-discovery relation: review-only; none of #5364-#5373 concerns toolchain
  compatibility.
- Scenario and impact: A clean builder honoring `rust-version = "1.85"` cannot
  construct the committed locked graph because the selected `time` family
  requires Rust 1.88. Current green validation used Rust 1.92.
- Evidence: declaration at `csdlc-v2/Cargo.toml:5`; dependency range at
  `csdlc-v2/Cargo.toml:88`; locked `time` 0.3.53, `time-core` 0.1.9, and
  `time-macros` 0.2.31 at `csdlc-v2/Cargo.lock:1847-1875`, whose cached package
  metadata declares Rust 1.88.

### P1-18: Installation receipts prove byte stability, not reviewed build provenance

- Source roles: dependency.
- Affected gates/issues: Gate 10A / #5292 and Gate 10D2 / #5306.
- Testing-discovery relation: review-only.
- Scenario and impact: Arbitrary executable bytes from a caller-selected source
  can receive a valid receipt. Verification cannot distinguish reviewed binaries
  built from the approved revision and lockfile from stale, modified, or
  malicious content.
- Evidence: receipt fields at `csdlc-v2/src/operator.rs:53-65`; install and hash
  at `csdlc-v2/src/operator.rs:289-325`; digest-only verification at
  `csdlc-v2/src/operator.rs:388-427`; arbitrary-text proof fixture at
  `csdlc-v2/tests/gate10a.rs:31-71`; stronger provenance language at
  `docs/architecture/csdlc-v2/gate10a/DESIGN.md:3` and
  `docs/architecture/csdlc-v2/gate10d2/CAPABILITY_MATRIX.json:30`.

### P1-19: Claims have no typed operator route for heartbeat or recovery

- Source roles: direct lifecycle and operator-contract review.
- Affected gates/issues: Gate 3 / #5233 and every long-running bound issue.
- Testing-discovery relation: review-only. None of #5364-#5373 establishes the
  missing operator entrypoint.
- Scenario and impact: A bound issue outlives its initial lease or needs stale
  claim recovery. The library implements both operations, but no installed
  binary or active skill exposes them. The claim expires without an authorized
  way to refresh it, and an operator cannot recover it through the sole typed
  v2 command surface.
- Evidence: heartbeat and recovery exist only as public library functions at
  `csdlc-v2/src/lifecycle.rs:270-347` and are unit-tested directly at
  `csdlc-v2/tests/gate2.rs:241-316`; `csdlc-v2/src/bin/csdlc-bind.rs:1-32`
  accepts only one bind request; the bind skill exposes only `csdlc-bind`; and
  no production binary calls `heartbeat_claim` or `recover_claim`. Recovery has
  a public schema, but no operator owner consumes it.

### P2-01: Exact publication replay fails after a successful canonical commit

- Source roles: code.
- Affected gates/issues: Gate 6 / #5236.
- Testing-discovery relation: review-only.
- Impact: Lost-response retry converges remotely but is rejected locally as
  stale, weakening publication idempotency and requiring request reconstruction.
- Evidence: replay condition at `csdlc-v2/src/publication.rs:245-249`; generation
  change at `csdlc-v2/src/store.rs:228-271`; stale fall-through at
  `csdlc-v2/src/publication.rs:250-256`.

### P2-02: Claims can reserve protected paths indefinitely

- Source roles: security.
- Affected gates/issues: Gate 3 / #5233.
- Testing-discovery relation: review-only.
- Impact: Unbounded initial expiry and heartbeat extension can reach `u64::MAX`;
  collision checks treat every stored claim as active, enabling durable denial
  of overlapping work.
- Evidence: `csdlc-v2/src/store.rs:739-763`,
  `csdlc-v2/src/lifecycle.rs:166-185`, `270-315`.

### P2-03: Legacy import duplicates untrusted instructions and secrets into durable surfaces

- Source roles: security.
- Affected gates/issues: Gate 8 / #5238 and importer sunset #5307.
- Testing-discovery relation: review-only.
- Impact: Complete legacy Markdown is retained in canonical evidence and a
  compatibility copy without a trust label or secret neutralization boundary;
  public library exports remain after standalone importer deletion.
- Evidence: `csdlc-v2/src/migration.rs:131-169`, `247-275`, and `544-573`;
  public reachability at `csdlc-v2/src/lib.rs:34-37`.

### P2-04: Explicit v1 resolution remains accepted after `v1_sunset`

- Source roles: code.
- Affected gates/issues: Gate 10D2 / #5306, rollback sunset #5308, and importer
  sunset #5307.
- Testing-discovery relation: review-only; #5369's selector deployment defect was
  not reproduced at this revision and is a different mechanism.
- Impact: The final resolver can select a generation whose binaries are forbidden
  and absent.
- Evidence: `csdlc-v2/src/soak.rs:33-54` returns explicit v1;
  `csdlc-v2/src/operator.rs:271-286` does not enforce coexistence sunset;
  `csdlc-v2/operator/coexistence.json` records `v1_sunset: true`.

### P2-05: `csdlc-edit bootstrap` bypasses guarded initialization

- Source roles: code.
- Affected gates/issues: Gate 2 / #5232 and Gate 3 / #5233.
- Testing-discovery relation: review-only.
- Impact: The alternate public entrypoint can consume absolute/traversing design
  paths and create overlapping protected-path ownership without the global lock
  or collision checks.
- Evidence: direct route at `csdlc-v2/src/bin/csdlc-edit.rs:54-57`; weaker store
  path at `csdlc-v2/src/store.rs:739-827`; guarded path at
  `csdlc-v2/src/lifecycle.rs:53-115`.

### P2-06: Gate 10A tests mutate the real checkout during parallel execution

- Source roles: tests.
- Affected gates/issues: Gate 10A / #5292 and final v1-absence proof / #5306.
- Testing-discovery relation: review-only.
- Impact: A test temporarily recreates forbidden `adl/tools/pr.sh` in the actual
  checkout; concurrent verification can observe it, and interruption can leave
  the repository dirty.
- Evidence: `csdlc-v2/tests/gate10a.rs:65` creates and later removes the path
  without an isolated repository or interruption-safe cleanup guard.

### P2-07: Final test-count and size evidence is not reproducible from its recorded method

- Source roles: tests; corroborated by validation and gap analysis.
- Affected gates/issues: Gate 9 / #5239 and Gate 10D2 / #5306.
- Testing-discovery relation: review-only.
- Impact: The suite executes 101 cases while evidence counts 100 annotations;
  the recorded line-count command counts file names, not lines. Budget evidence
  can drift or accidentally match without reproducible measurement.
- Evidence: textual count at `csdlc-v2/src/proof.rs:318`; duplicate inclusion at
  `csdlc-v2/tests/gate9.rs:10`; recorded method at
  `docs/architecture/csdlc-v2/gate10d2/SIZE_EVIDENCE.json:4`.

### P2-08: Current operator guidance incompletely describes final selector authority

- Source roles: docs.
- Affected gates/issues: Gate 10A / #5292 through Gate 10D2 / #5306.
- Testing-discovery relation: review-only, separate from #5369.
- Impact: All nine thin skills invoke bare owner binaries without mandatory
  resolution/provenance verification, while `csdlc-v2/README.md` still describes
  v1 as protected from deletion. PATH state and obsolete coexistence assumptions
  can bypass final authority.
- Evidence: skill language under `csdlc-v2/operator/skills/*/SKILL.md`, including
  `csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md`; required route at
  `AGENTS.md:135-138` and `csdlc-v2/AGENTS.md:6-8`; stale sunset text at
  `csdlc-v2/README.md:44-56`.

### P2-09: Gate 1 release identity is split between v0.91.7 and v0.92

- Source roles: docs.
- Affected gates/issues: Gate 1 / #5228 and sprint/release accounting.
- Testing-discovery relation: review-only.
- Impact: Issue, label, and local cards identify v0.91.7, while PR #5231, branch,
  and merge commit identify v0.92; no retained disposition resolves the split.
- Evidence: `.adl/v0.91.7/tasks/issue-5228__v0-91-7-csdlc-v2-clean-room-architecture-baseline/sor.md:1` context; merged PR #5231 and commit `4f53f049a` as recorded in `ISSUE_COVERAGE.md`.

### P2-10: The independent workspace has no continuous dependency or supply-chain gate

- Source roles: dependency.
- Affected gates/issues: Gate 2 / #5232, Gate 10B / #5293, and final authority /
  #5306.
- Testing-discovery relation: review-only.
- Impact: Lockfile, MSRV, feature, vulnerability, license, and provenance drift can
  reach the final authority without a focused CI gate; the active MSRV mismatch
  demonstrates the control gap.
- Evidence: deferred paired construction at
  `docs/architecture/csdlc-v2/CSDLC_V2_GATE1_VALIDATION.md:6-35`; no-deps proof at
  `docs/architecture/csdlc-v2/gate2/GATE2_VALIDATION.md:8-18`; unlocked commands
  at `csdlc-v2/operator/pre-switch-proof.json:8-11`; no C-SDLC v2 build/test/MSRV
  lane in `.github/workflows/ci.yaml:584-598`.

### P2-11: GitHub owners do not implement the required shared token-file resolver

- Source roles: direct code and operator-contract review.
- Affected gates/issues: Gate 6 / #5236, Gate 7 / #5237, and final operator
  authority after Gate 10D2 / #5306.
- Testing-discovery relation: independently derived. #5370 requests a
  shared-token PR-state collector but does not establish this publish/closeout
  resolver mismatch.
- Impact: Under the documented operator setup, `csdlc-publish` searches an
  unrelated default path and `csdlc-closeout` requires a request-local path.
  Both ignore `ADL_GITHUB_TOKEN_FILE`. Publication and closeout can therefore
  fail or select a different local token source after v1 sunset unless every
  caller reconstructs the approved path in request JSON.
- Evidence: current authority requires the shared resolver and names
  `ADL_GITHUB_TOKEN_FILE=<approved-token-file>` in `AGENTS.md:35-42`;
  `csdlc-v2/src/bin/csdlc-publish.rs:134-170` reads token-value environment
  variables then defaults to `<implementation-fallback-token-file>`;
  `csdlc-v2/src/bin/csdlc-closeout.rs:372-405` reads the same token-value
  variables but has no default token file. Neither owner reads
  `ADL_GITHUB_TOKEN_FILE` or delegates to a shared resolver.

### P2-12: Publication, readiness, and closeout lack executable GitHub-boundary proof

- Source roles: tests.
- Affected gates/issues: Gate 6 / #5236, Gate 7 / #5237, and parity claims in
  Gate 10D2 / #5306.
- Testing-discovery relation: review-only proof gap. #5370 and #5371 describe
  adjacent missing operational collection/shepherd behavior, but were not used
  as evidence for this finding.
- Impact: Pagination, create-after-timeout, push failure, check reruns, review
  supersession, unknown mergeability, and terminal-state disagreements can
  regress without detection while parity evidence remains green. This is a
  validation/control gap, not a separately demonstrated production failure.
- Evidence: `csdlc-v2/tests/gate6.rs:36` builds in-memory reconciliation values;
  production HTTP/Git behavior begins at `csdlc-v2/src/bin/csdlc-publish.rs:49`
  and `csdlc-v2/src/bin/csdlc-closeout.rs:155`; no suite test uses a mock HTTP
  server or bounded live fixture; `docs/architecture/csdlc-v2/gate10d2/PARITY_EVIDENCE.json:19`
  credits the narrower proof.

## Scope And Issue Coverage

- Scope type: `sprint`.
- Controlling umbrella: #5240, with Gate 10D decomposition under #5295.
- Reviewed revision: `7c3e1e0e86a4ca982231ce91c39073530c5408e6`.
- Review issue: #5375.
- Sprint issues: #5228, #5232-#5240, #5292-#5295, and #5305-#5308.
- Gate coverage: Gate 1 architecture; Gates 2-9 implementation and proof; Gate
  10A coexistence/install; Gate 10B pre-switch proof; Gate 10C reversible
  switch; Gate 10D1 eligibility; Gate 10D2 deletion; Gate 10D3 rollback sunset;
  and Gate 10D4 importer sunset.
- GitHub/PR coverage: all 18 issues were observed closed and all 15 identified
  implementation or preparation PRs were observed merged. #5240 had no direct
  implementation PR; #5295's PR #5320 was setup-only; #5307 and #5308 had setup
  through #5320 and were superseded by #5331 without issue-local execution PRs.
- Complete PR list, reviewed source inventory, lifecycle/closeout artifacts,
  changed-surface boundary, and skipped/unavailable surfaces:
  `SCOPE_EVIDENCE_INDEX.md`.
- Publication intent: public repository review for maintainers and contributors;
  customer-facing reuse requires a fresh redaction/evidence audit.

## Lane Coverage

| Lane | Status | Artifact or reason |
| --- | --- | --- |
| `gap_analysis` | run | `GAP_ANALYSIS.md` |
| `code` | run | `specialists/CODE_REVIEW.md` |
| `docs` | run | `specialists/DOCS_LIFECYCLE_REVIEW.md` |
| `tests` | run | `specialists/TEST_REVIEW.md` |
| `evidence_and_closeout` | run | `ISSUE_COVERAGE.md`, `LOCAL_CARD_OBSERVATIONS.json`, and `GITHUB_OBSERVATIONS.json` |
| `synthesis` | run | `SPRINT_REVIEW.md` |
| `review_quality` | run | `QUALITY_EVALUATION.md`; initial blockers corrected and reevaluated before publication |
| `security` | run | `specialists/SECURITY_REVIEW.md` |
| `architecture` | run | `specialists/ARCHITECTURE_REVIEW.md` |
| `dependency` | run | `specialists/DEPENDENCY_REVIEW.md` |
| `release_evidence` | skipped | Not a milestone release-proof bundle; sprint delivery evidence is indexed here. |
| `redaction_and_evidence` | run | `REDACTION_EVIDENCE_AUDIT.md`; initial blockers corrected and reevaluated before publication |
| Supplemental direct contract | run | `specialists/DIRECT_CONTRACT_REVIEW.md` |
| Supplemental validation | run | `VALIDATION.md` and `VALIDATION_EVIDENCE.json` |

Per-finding owner, correction, proof, and disposition are recorded in
`FINDING_ROUTING.md`. The source and evidence boundary for every lane is indexed
in `SCOPE_EVIDENCE_INDEX.md`.

## Testing-Discovery Boundary

Issues #5364-#5373 predate and are independent testing discoveries. They were
comparison inputs, not source evidence, and are not counted as findings merely
because they exist.

| Testing issue(s) | Synthesis disposition |
| --- | --- |
| #5368 | Independently corroborated: P1-05 reaches the same dirty-review/publication dead end from source tracing. |
| #5366 | Independently overlapping subject: P1-14 confirms active stale v1 instructions from current docs/templates; the review finding remains independently evidenced. |
| #5364 | Adjacent planning/replan pressure only; it does not establish P1-10 or another synthesized mechanism. |
| #5369 | Different selector-file deployment defect; not reproduced at the reviewed revision and does not subsume P1-11 or P2-04. |
| #5370-#5371 | Adjacent PR-state collection/shepherd gaps; they do not establish cross-repository identity substitution, caller-weakened policy, or the full missing GitHub fixture in P1-02, P1-03, and P2-12. |
| #5365, #5367, #5372-#5373 | No synthesized finding was imported from these discoveries; their deleted-v1 VPP, planned-milestone/helper authority, v1-origin PR-tail, or shepherd-schema subjects remain separate testing-discovery scope. |

All findings not explicitly labeled as corroborated or overlapping above are
review-only findings from the #5375 specialist packet.

## Lifecycle And Closeout Truth

Implementation delivery and lifecycle acceptance are distinct. The sprint
produced a compact standalone Rust workspace, all 18 issues are closed, and the
15 identified PRs are merged. That delivery truth does not establish canonical
closeout truth: all 108 observed issue cards were ignored/untracked and
frequently stale or internally contradictory. Their raw contents are not
published; `LOCAL_CARD_OBSERVATIONS.json` retains logical identity, hashes,
tracking status, and normalized terminal fields for independent inspection. #5240,
#5295, #5307, and #5308 especially lack retained superseded/no-PR terminal
dispositions, while #5306's local SRP/SOR remains pre-execution despite merged
deletion work. The packet therefore supports "implementation delivered and
cutover recorded," but not "all lifecycle acceptance and closeout criteria are
durably proven."

## Validation Adequacy

Current execution passed:

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`: 101 executions,
  zero failures, zero ignored, and zero doctests.
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`.
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`.
- Dependency inspection with locked/offline `cargo tree`, and current-toolchain
  `cargo check --locked --offline` on Rust 1.92.

Portable run commands, toolchain, exit status, counts, logs, and hashes are
retained in `VALIDATION_EVIDENCE.json` and `evidence/`. These runs adequately
prove current expected-path behavior and static quality
for the tested revision. They do not exercise the high-risk negative boundaries
identified above: real GitHub pagination/outage/retry and identity, linked
worktree state sharing, concurrent/crash recovery, symlinked control ancestors,
PVF process confinement, exact installed resolver routing, Rust 1.85
construction, capability-reference execution, or retained lifecycle closeout.
The green suite and the findings are therefore consistent rather than
conflicting.

## Dedupe And Disagreement Notes

- PVF enforcement reports from code, security, and tests were merged into
  P1-01 because they trace the same declaration-versus-execution mechanism.
- Cross-repository readiness/closeout reports from code, security, and
  architecture were merged into P1-02. Caller-selected readiness policy remains
  P1-03 because it is a distinct weakening mechanism even on the correct PR.
- Missing stable `csdlc-install` reports from tests, architecture, and dependency
  were merged into P1-11. Receipt build provenance remains P1-18 because adding
  the resolver would not authenticate how any installed bytes were built.
- Binding/recovery findings were grouped in P1-06 but retain three separate
  entrypoints: recovery replacement, pre-existing ordinary directory, and
  generic phase advance. P1-10 remains separate because it concerns state and
  lock namespace placement even when binding itself is valid.
- Active stale docs and deterministic template generation were grouped in P1-14
  while preserving the direct-instruction and generated-state mechanisms.
  P2-08 remains separate: P1-14 is executable stale authority that emits or
  instructs invalid v1 commands, while P2-08 is incomplete explanatory guidance
  about the final resolver and selector authority even after those commands are
  removed.
- Stale/untracked cards and unreconciled umbrella/child closure were grouped in
  P1-15 because together they produce the sprint-level durable-closeout failure;
  both evidence mechanisms are retained.
- P1-06 and P1-19 are not duplicates. P1-06 identifies existing mutation paths
  that bypass ownership; P1-19 identifies the absence of a supported typed
  operator path for a legitimate heartbeat or recovery. Removing bypasses does
  not supply that route, and adding the route does not by itself repair them.
- P1-02 and P1-03 are demonstrated remote-identity and policy-authority defects.
  P2-12 is the distinct proof-system gap that allowed those defects, and other
  GitHub boundary regressions, to remain untested. Adding fixtures does not
  repair either production defect; repairing them does not provide durable
  regression proof.
- Specialist severities were aligned without downward dilution. There were no
  direct contradictory findings. The main qualification is that current tests,
  Clippy, and formatting pass; specialists agree those checks do not cover the
  reported negative paths.

## Residual Risk

- No authenticated mutating GitHub publication/readiness/closeout sequence was
  run, so additional normalization, retry, and remote-state defects may remain.
- No destructive filesystem escape, credential exposure, network exfiltration,
  multi-process race, or kill-at-every-write campaign was executed.
- Rust 1.85 was not installed or run; the MSRV finding rests on exact locked
  crate metadata, while current compilation proof is Rust 1.92 only.
- Historical Gate 10A-C evidence was inspected as immutable evidence and was not
  regenerated. The deletion/cutover wave was not replayed.
- External vulnerability and license databases were not queried. No claim of a
  vulnerability-free or legally approved dependency graph is made.
- Ignored local card snapshots and live GitHub metadata can change independently
  of this tracked review, increasing future reconstruction risk.

## Follow-up Routing

No issue is created or modified by this synthesis. Recommended routing, subject
to operator triage:

1. Gate 4 PVF owner: enforce process environment/network policy and truthful
   redaction/evidence, then add execution-level confinement tests.
2. Gate 7 GitHub/canonical-evidence owner: derive full remote identity and policy
   from canonical publication/VPP truth; add deterministic HTTP/Git fixtures.
3. Gate 2/3 state and lifecycle owners: establish one shared state/lock namespace
   across worktrees and make side-effecting transitions owner-exclusive; harden
   recovery, directory topology, symlink, lease, and bootstrap paths; expose
   typed heartbeat and recovery owners.
4. Gate 5/6 owners: require reviewable clean revisions, bind scope to changed
   paths, and make post-commit publication replay idempotent.
5. SOR/readiness owners: model validation attempts and supersession, then enforce
   semantic completion invariants on every terminal path.
6. Gate 10 install/supply-chain owners: include or separately bootstrap the
   resolver with verified provenance, reject v1 after sunset, bind receipts to
   reviewed build inputs, and isolate Gate 10A tests.
7. Gate 10D authority owner: bind approval to exact evaluator revision and actual
   deletion diff; make parity mappings executable and reproducible.
8. Documentation/template and lifecycle-closeout owners: replace active v1
   routes, reconcile current skill/README authority, and define durable tracked
   terminal evidence for all 18 issues without rewriting historical Gate 10A-C
   evidence.
9. Dependency/CI owner: reconcile the declared MSRV with the lockfile and add a
   focused locked workspace, MSRV, dependency, and supply-chain gate.
10. GitHub adapter owner: use one shared token resolver for publish, readiness,
    and closeout and test environment/file precedence without exposing secrets.
11. Release owner: record the #5228 v0.91.7/v0.92 disposition and replace
    non-reproducible size/test-count evidence.

## Non-Claims

- This review is not merge approval, release approval, remediation completion,
  or evidence that all sprint acceptance criteria passed.
- It does not reopen, create, close, or route any GitHub issue or PR.
- It does not modify implementation, tests, cards, configs, templates, existing
  artifacts, installed binaries, or lifecycle state.
- It does not claim that the observed security paths were exploited, that live
  GitHub state is immutable, or that every untested boundary contains a defect.
- It does not supersede specialist artifacts; those remain the detailed source
  for role-specific evidence and residual caveats.

## Metadata

- Skill: `repo-review-synthesis`
- Target: branch `codex/5375-v0-91-7-csdlc-v2-full-sprint-review` at
  `7c3e1e0e86a4ca982231ce91c39073530c5408e6`
- Date: 2026-07-15
- Specialist artifacts: code, security, tests, docs/lifecycle, architecture, and
  dependency all present under `specialists/`; `GAP_ANALYSIS.md`,
  `ISSUE_COVERAGE.md`, and `VALIDATION.md` also incorporated.
- Severity policy: preserve the highest justified specialist severity while
  recalibrating only through explicit impact and exploitability analysis.
- Stop boundary: synthesis artifact only; no remediation or approval claim.
