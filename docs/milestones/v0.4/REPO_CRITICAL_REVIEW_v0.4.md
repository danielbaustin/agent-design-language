# ADL v0.4 Repository Critical Review

Date: 2026-02-18  
Scope: full repository review from `main` with emphasis on security, ease of use, code quality, and professionalism.

## Executive Summary

The project is in strong shape technically: local quality gates pass (`fmt`, `clippy -D warnings`, `test`), v0.4 runtime behavior is validated, and CI coverage flow is functioning.

The main risks are now maturity/consistency risks, not core runtime correctness risks:
- one high-impact code-quality bug in bounded execution error handling
- version/documentation drift that undermines trust and onboarding quality
- a few security-hardening gaps in scripts/CI hygiene

## Evidence Collected

Validation run on `swarm/`:
- `cargo fmt --all -- --check` -> pass
- `cargo clippy --all-targets -- -D warnings` -> pass
- `cargo test` -> pass (all test suites green)

Representative files reviewed:
- Runtime: `swarm/src/execute.rs`, `swarm/src/bounded_executor.rs`, `swarm/src/provider.rs`, `swarm/src/trace.rs`
- CLI/docs: `README.md`, `swarm/README.md`, `docs/README.md`, `swarm/Cargo.toml`
- Tooling/CI: `swarm/tools/*.sh`, `.github/workflows/ci.yaml`

## Findings (Prioritized)

## P1 (High)

### 1) Bounded executor can silently drop failures if a worker panics
- Category: Code Quality / Reliability
- Evidence: `swarm/src/bounded_executor.rs:56`, `swarm/src/bounded_executor.rs:57`, `swarm/src/bounded_executor.rs:61`
- Problem:
  - Worker thread joins are ignored (`let _ = h.join()`), and the function returns collected channel items even if one worker panics.
  - This can produce incomplete output without surfacing an error.
- Impact:
  - Potential silent data loss/incomplete execution under panic conditions.
- Recommended fix:
  - Propagate worker panic as `Err(anyhow!(...))`.
  - Verify output count equals input job count before returning.
- Acceptance criteria:
  - Panic in any job causes `run_bounded` to return `Err`.
  - Add tests for panic propagation + output-count integrity.

## P2 (Medium)

### 2) Runtime ordering semantics are deterministic but under-specified vs docs
- Category: Ease of use / Correctness expectations
- Evidence: `swarm/src/execute.rs:544`, `swarm/src/execute.rs:545`, `swarm/README.md:39`, `swarm/README.md:50`
- Problem:
  - Concurrent ready steps are sorted lexicographically by `step_id`, not explicitly by declaration order.
  - Some docs still describe declared-order behavior in v0.3 text.
- Impact:
  - User expectations can diverge from observed execution ordering.
- Recommended fix:
  - Choose one canonical policy (declaration order or lexical order) and enforce/document it consistently.
  - Add an explicit test that encodes the chosen policy.
- Acceptance criteria:
  - Docs and tests state and enforce the same ordering rule.

### 3) v0.4 repository docs still contain v0.3 spine and roadmap pointers
- Category: Professionalism / Onboarding
- Evidence: `docs/README.md:1`, `README.md:124`, `swarm/README.md:3`
- Problem:
  - Root docs point to v0.3 artifacts while root README claims v0.4 release.
- Impact:
  - Confusing first impression; looks less production-ready.
- Recommended fix:
  - Update docs spine to v0.4, add v0.4 primary links, and mark legacy docs as historical.
- Acceptance criteria:
  - Top-level docs have a single coherent v0.4 narrative.

### 4) `swarm` package metadata is stale (`0.1.0`, v0.1 description)
- Category: Professionalism / Release hygiene
- Evidence: `swarm/Cargo.toml:3`, `swarm/Cargo.toml:6`
- Problem:
  - Runtime package metadata still advertises v0.1 despite v0.4 release positioning.
- Impact:
  - Registry/readme tooling and external consumers see contradictory version signals.
- Recommended fix:
  - Align package version/description with release strategy (or clearly document semantic difference between ADL doc version and runtime milestone version).
- Acceptance criteria:
  - Metadata and release docs no longer conflict.

### 5) CI actions are tag-pinned, not commit-SHA pinned
- Category: Security hardening / Supply chain
- Evidence: `.github/workflows/ci.yaml:16`, `.github/workflows/ci.yaml:19`, `.github/workflows/ci.yaml:24`, `.github/workflows/ci.yaml:77`, `.github/workflows/ci.yaml:98`
- Problem:
  - Third-party actions are pinned by mutable tags (`@v4`, `@stable`, etc.).
- Impact:
  - Elevated supply-chain risk compared with SHA pinning.
- Recommended fix:
  - Pin actions to immutable SHAs and document update cadence.
- Acceptance criteria:
  - All third-party actions in CI pinned by SHA.

### 6) Unnecessary `eval` in demo scripts
- Category: Security hardening / Script safety
- Evidence: `swarm/tools/demo_v0_4.sh:43`, `swarm/tools/demo_v0_4.sh:44`, `swarm/tools/mock_ollama_v0_4.sh:24`
- Problem:
  - `eval` is used for hashing command selection where direct command branching is safer and clearer.
- Impact:
  - Avoidable command-injection surface and maintainability cost.
- Recommended fix:
  - Replace `eval` with explicit branches (`sha256sum` vs `shasum -a 256`).
- Acceptance criteria:
  - No `eval` in demo scripts.

### 7) Root project trust files are missing (`SECURITY.md`, root `CONTRIBUTING.md`, etc.)
- Category: Professionalism / Security process
- Evidence: repository root listing (no `SECURITY.md`)
- Problem:
  - Security disclosure and contribution process are discoverable only in subtrees.
- Impact:
  - Weaker OSS posture and slower external contribution/security reporting path.
- Recommended fix:
  - Add root `SECURITY.md`, root `CONTRIBUTING.md`, and pointer structure to subtree docs.
- Acceptance criteria:
  - Root trust/process files exist and link to canonical policy docs.

## P3 (Low)

### 8) Badge semantics can be clearer
- Category: Ease of use / Professional polish
- Evidence: `README.md:27`, `README.md:29`
- Problem:
  - `swarm-ci` and `swarm-coverage-gate` badges currently map to the same workflow badge endpoint.
- Impact:
  - Users may assume these represent independent signals when they do not.
- Recommended fix:
  - Clarify badge labels or use explicit job-level status links/images.
- Acceptance criteria:
  - Each badge has unambiguous meaning and unique signal.

### 9) CLI help/examples emphasize v0.3 more than v0.4 demos
- Category: Ease of use
- Evidence: `swarm/src/main.rs` usage text and example list
- Problem:
  - New users still land on older examples first.
- Impact:
  - Slower onboarding into current milestone behavior.
- Recommended fix:
  - Promote one v0.4 no-network demo command as first example.
- Acceptance criteria:
  - `--help` and README "first command" align on current recommended path.

## Proposed GH Issue Backlog (Ready to Create)

1. `runtime: make bounded_executor fail fast on worker panic and missing outputs`
- Labels: `area:runtime`, `type:bug`, `priority:high`, `version:v0.4`

2. `docs: reconcile execution ordering semantics (declared vs lexical) across runtime/docs/tests`
- Labels: `area:docs`, `area:runtime`, `type:task`, `priority:medium`

3. `docs: update docs spine and cross-links from v0.3 to v0.4 canonical set`
- Labels: `area:docs`, `type:task`, `priority:medium`

4. `release-hygiene: align swarm Cargo metadata with current release/milestone policy`
- Labels: `area:release`, `type:task`, `priority:medium`

5. `ci-security: pin all GitHub actions to immutable SHAs`
- Labels: `area:ci`, `type:security`, `priority:medium`

6. `tools-security: remove eval from demo hash scripts`
- Labels: `area:tools`, `type:security`, `priority:medium`

7. `repo-professionalism: add root SECURITY.md and CONTRIBUTING.md entrypoints`
- Labels: `area:docs`, `type:process`, `priority:medium`

8. `readme: make badge signals explicit and non-duplicative`
- Labels: `area:docs`, `type:polish`, `priority:low`

9. `cli-ux: promote v0.4 demo command in --help examples`
- Labels: `area:runtime`, `area:docs`, `type:ux`, `priority:low`

## Suggested Execution Order

1. Issue 1 (bounded executor panic handling)
2. Issue 5 (CI action pinning)
3. Issue 6 (remove eval)
4. Issues 3 + 4 + 8 + 9 (documentation/professional polish batch)
5. Issue 7 (root trust/process files)
6. Issue 2 (ordering semantics) after team decision on canonical policy

