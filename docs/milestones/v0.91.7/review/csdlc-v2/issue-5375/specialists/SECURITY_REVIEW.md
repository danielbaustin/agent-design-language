# C-SDLC v2 Sprint Security Review

## Findings

### P1 (High): PVF network and credential policies do not constrain executed processes

- **File:** `csdlc-v2/src/pvf.rs:269`, `csdlc-v2/src/pvf.rs:611`, `csdlc-v2/src/pvf.rs:680`; `csdlc-v2/src/bin/csdlc-validate.rs:9`
- **Role:** security
- **Scenario:** A repository or request author supplies a PVF lane with `network: denied` and an empty credential declaration, but chooses an executable that reads inherited environment variables and opens an outbound connection. `csdlc-validate` accepts the request JSON directly. Selection checks only the declared enum and credential-name list; `run_lane` then starts the executable without `env_clear`, credential allowlisting, network isolation, or another sandbox.
- **Impact:** A validation lane can execute arbitrary host commands with every credential inherited by the validator process and can exfiltrate those credentials despite a denied network declaration. Output containing undeclared secrets is also written to evidence logs because redaction covers only caller-supplied literal values, while `redaction_ok` is always recorded as `true`.
- **Evidence:** `select` rejects only lanes that self-declare `NetworkPolicy::External` or unavailable credential names (`csdlc-v2/src/pvf.rs:269-286`). `Command::new` inherits the parent environment and host network at `csdlc-v2/src/pvf.rs:622-628`. Log scrubbing iterates only `lane.evidence.redact_values`, then unconditionally emits `redaction_ok: true` (`csdlc-v2/src/pvf.rs:680-707`). Gate 4 tests exercise declaration checks and literal redaction but contain no environment isolation or denied-network enforcement assertion.

### P1 (High): Merge readiness policy is supplied by the observation request instead of canonical issue truth

- **File:** `csdlc-v2/src/bin/csdlc-closeout.rs:54`, `csdlc-v2/src/bin/csdlc-closeout.rs:215`; `csdlc-v2/src/readiness.rs:94`; `csdlc-v2/src/store.rs:275`
- **Role:** security
- **Scenario:** A caller submits `ObserveReadiness` with `required_checks: []` and `require_review: false`. The observer classifies every check as optional and the policy accepts no approval as required. If GitHub reports a clean conflict state and no active changes-requested review, the request becomes ready even when the repository's actual required CI and review policy have not passed.
- **Impact:** Canonical state can advance to `MergeReady` on caller-selected weaker requirements, allowing downstream operators or automation to treat an unreviewed or unvalidated revision as mergeable.
- **Evidence:** `GithubReadinessRequest` makes both controls caller fields (`csdlc-v2/src/bin/csdlc-closeout.rs:54-66`). The GitHub normalizer derives required status solely from `input.required_checks` and forwards both values unchanged (`csdlc-v2/src/bin/csdlc-closeout.rs:215-230`, `csdlc-v2/src/bin/csdlc-closeout.rs:289-304`). `classify_readiness` has no minimum required checks and enforces approval only when the caller sets `require_review` (`csdlc-v2/src/readiness.rs:94-149`). `commit_readiness` checks publication identity and local PVF evidence but does not bind these remote requirements to VPP, branch protection, or another canonical policy (`csdlc-v2/src/store.rs:275-377`).

### P1 (High): Readiness and terminal observations can be substituted from another GitHub repository

- **File:** `csdlc-v2/src/bin/csdlc-closeout.rs:155`, `csdlc-v2/src/bin/csdlc-closeout.rs:308`; `csdlc-v2/src/store.rs:305`, `csdlc-v2/src/store.rs:419`
- **Role:** security
- **Scenario:** A caller names a different repository in an observe-readiness or closeout request. That repository can contain the public canonical commit SHA in a PR with the same number and attacker-controlled checks, reviews, conflict state, or merged state. The observer fetches from the caller's repository, but the normalized readiness and terminal requests omit repository identity before committing.
- **Impact:** Green checks or a merge in an unrelated repository can promote the canonical issue to `MergeReady` or `ClosedOut`, release its claim, and create false publication/closeout truth without the canonical PR satisfying those conditions.
- **Evidence:** Both observer paths split and query `input.repository` without comparing it to the stored issue or publication repository (`csdlc-v2/src/bin/csdlc-closeout.rs:155-165`, `csdlc-v2/src/bin/csdlc-closeout.rs:308-326`). `ReadinessRequest` and `TerminalObservation` carry no repository field. The store reconciles only PR number and normalized head revision (`csdlc-v2/src/store.rs:305-314`, `csdlc-v2/src/store.rs:419-451`). Existing Gate 7 tests cover false state and SHA mismatches but not cross-repository substitution.

### P1 (High): C-SDLC state writes and cleanup can escape the repository through symlinked control directories

- **File:** `csdlc-v2/src/store.rs:37`, `csdlc-v2/src/store.rs:53`, `csdlc-v2/src/store.rs:95`, `csdlc-v2/src/store.rs:108`; `csdlc-v2/src/bin/csdlc-publish.rs:173`; `csdlc-v2/src/migration.rs:566`
- **Role:** security
- **Scenario:** A malicious checkout provides `.csdlc`, `.csdlc/issues`, `.csdlc/locks`, `.csdlc/publication`, or `.csdlc/compat` as a symlink to a location outside the repository. Store construction performs lexical joins only. Subsequent lock creation, staged generation writes, atomic renames, publication-intent writes, compatibility writes, and backup cleanup follow those parent symlinks.
- **Impact:** Running a typed v2 command in an untrusted checkout can create, overwrite, rename, or recursively remove predictable paths outside the repository. The transaction recovery path increases impact because it calls `remove_dir_all` on attacker-redirected staging and backup locations.
- **Evidence:** `Store::new` retains the supplied root without canonical control-directory validation, and every issue path is `root.join(".csdlc/...")` (`csdlc-v2/src/store.rs:23-51`). Locks are opened with ordinary `OpenOptions` (`csdlc-v2/src/store.rs:53-75`). Recovery and commit remove and rename staging/backup directories without checking ancestor symlinks or using no-follow directory handles (`csdlc-v2/src/store.rs:95-139`). Publication and compatibility writers repeat the same lexical-parent pattern. Gate 10A rejects symlinked installed binary leaves, but the state-store and lifecycle tests do not exercise symlinked control-directory ancestors.

### P2 (Medium): Claims can reserve protected paths indefinitely

- **File:** `csdlc-v2/src/store.rs:739`; `csdlc-v2/src/lifecycle.rs:166`, `csdlc-v2/src/lifecycle.rs:270`, `csdlc-v2/src/lifecycle.rs:300`
- **Role:** security
- **Scenario:** A claimant initializes an issue with `expires_unix_seconds` near `u64::MAX`, or repeatedly heartbeats with an unbounded `extend_seconds`. Bootstrap validates only timestamp ordering, heartbeat uses `saturating_add`, and recovery is forbidden until the recorded expiry. Binding collision checks also treat every stored claim as active without checking its expiry.
- **Impact:** A compromised or abusive claimant can deny all later issues that need overlapping protected paths for an effectively permanent period, defeating stale-claim recovery and requiring manual state intervention.
- **Evidence:** Bootstrap has no maximum TTL or expiry horizon (`csdlc-v2/src/store.rs:746-763`). Heartbeat accepts any extension and saturates at `u64::MAX` (`csdlc-v2/src/lifecycle.rs:270-297`). Recovery requires `now_unix_seconds >= current.expires_unix_seconds` (`csdlc-v2/src/lifecycle.rs:300-315`). Collision scanning rejects overlaps whenever `other.claim` is present and does not validate liveness (`csdlc-v2/src/lifecycle.rs:166-185`).

### P2 (Medium): Legacy Markdown import persists untrusted instructions and secrets verbatim in multiple durable surfaces

- **File:** `csdlc-v2/src/migration.rs:131`, `csdlc-v2/src/migration.rs:157`, `csdlc-v2/src/migration.rs:247`, `csdlc-v2/src/migration.rs:544`; `csdlc-v2/src/model.rs:155`; `csdlc-v2/src/lib.rs:34`
- **Role:** security
- **Scenario:** A legacy card contains credentials, sensitive URLs, hostile agent instructions, or other untrusted Markdown. The public library importer parses structural headings but preserves each complete source string, maps portions into operative card fields, embeds all complete sources in canonical migration evidence, and writes a second exact Markdown compatibility view.
- **Impact:** Accidental secrets are duplicated into repository state and compatibility artifacts, while prompt-injection content crosses from external legacy data into agent-consumed planning and review surfaces without a trust label or neutralization boundary. Later agents can treat imported hostile text as workflow authority.
- **Evidence:** Full UTF-8 card text is inserted into `authored_sources` before semantic mapping (`csdlc-v2/src/migration.rs:131-169`), then cloned into `MigrationEvidence` committed to the issue index (`csdlc-v2/src/migration.rs:247-275`). `render_legacy_archive` copies every source byte into `.csdlc/compat/<issue>.md` (`csdlc-v2/src/migration.rs:544-573`). The final sprint removed the standalone importer binary, but `import_legacy` and compatibility writers remain public library exports (`csdlc-v2/src/lib.rs:34-37`), so the exposure remains reachable by library consumers and tests.

## Metadata

- **Skill:** `repo-review-security`
- **Target:** branch `codex/5375-v0-91-7-csdlc-v2-full-sprint-review` at `7c3e1e0e8`; sprint issues #5228, #5232-#5240, #5292-#5295, and #5305-#5308
- **Date:** 2026-07-14
- **Artifact:** `docs/milestones/v0.91.7/review/csdlc-v2/issue-5375/specialists/SECURITY_REVIEW.md`
- **Review depth:** deep, targeted security inspection

## Trust Boundaries Reviewed

- Typed request JSON into lifecycle, PVF, publication, readiness, closeout, cutover, eligibility, and installer owners.
- Repository-controlled Markdown, manifests, git metadata, `.csdlc` state, worktree topology, and filesystem paths.
- Host command execution, inherited process environment, network authority, evidence logs, and cancellation.
- GitHub token sources, git remote identity, Octocrab PR/check/review observations, and canonical publication/readiness/terminal state.
- Claims, heartbeats, stale recovery, protected-path collision, terminal release, pruning, installer replacement, cutover rollback, and deletion eligibility.

## Assets And Attacker Capabilities Considered

- **Assets:** GitHub/provider credentials, local filesystem integrity, canonical issue/card/audit truth, reviewed revision identity, PR readiness/terminal truth, protected worktrees, and deletion/rollback evidence.
- **Attackers:** a contributor controlling repository files or request JSON; a caller able to choose GitHub observation inputs; a validation command executed from a manifest; a claimant abusing lease parameters; and a concurrent local actor able to manipulate symlinked paths.
- **Assumption:** Claim IDs and local record digests provide coordination and corruption detection, not authentication against an actor who can rewrite repository state.

## Reviewed Surfaces

- `csdlc-v2/src/{store,lifecycle,model,cards,doctor,git,pvf,review,publication,readiness,migration,operator,proof,cutover,eligibility,soak}.rs`
- `csdlc-v2/src/bin/csdlc-{init,bind,edit,validate,review,publish,closeout,install,proof,cutover,eligibility}.rs`
- `csdlc-v2/tests/gate2.rs`, `gate4.rs`, `gate5.rs`, `gate6.rs`, `gate7.rs`, `gate7_lifecycle.rs`, `gate8.rs`, `gate9.rs`, `gate10a.rs`, and `gate10b.rs`
- Gate 2-10 architecture/design, validation, capability, cutover, and deletion evidence under `docs/architecture/csdlc-v2/`
- Final Gate 10D2 deletion diff and retained capability matrix, including the removed v1 GitHub/publication and lifecycle owners.

## Validation Performed

- `cargo test --manifest-path csdlc-v2/Cargo.toml` passed: 101 tests, 0 failures, plus doc tests. The green suite establishes current expected behavior but does not cover the abuse scenarios above.
- Targeted source/test searches confirmed no assertions for child-process environment clearing, enforced denied-network isolation, canonical readiness-policy binding, cross-repository observation rejection, symlinked `.csdlc` ancestors, bounded claim TTLs, or imported-content secret/prompt handling.
- `git diff --check` passed before and after artifact creation.
- No destructive exploit, live GitHub mutation, credential read, network exfiltration, or filesystem escape was executed.

## Residual Risk

- Live GitHub behavior was inspected statically and through offline tests; no authenticated publication/readiness/closeout operation was run.
- The review did not attempt OS-level sandbox bypass, process-group escape, or race exploitation. Leaf checks in installer/eligibility code still have ordinary check-then-read race windows, but no additional finding is raised without a stronger same-host attacker model.
- Removed v1 implementation history was inspected through the Gate 10D2 deletion diff and retained evidence, not exhaustively re-reviewed line by line.
- Dependency vulnerability and license analysis belong to the dependency specialist lane and are not claimed here.
