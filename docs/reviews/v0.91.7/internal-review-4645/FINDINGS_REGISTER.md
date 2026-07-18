# v0.91.7 Internal Review Findings Register (#4645)

Status: internal_review_findings_recorded

Issue: #4645

Captured: 2026-07-18

Scope: v0.91.7 milestone internal review across WP-01 through WP-23, retained
sprint packets, live GitHub issue/PR truth, C-SDLC issue cards, CI/coverage
surfaces, docs, architecture, code, security, dependency, and release-evidence
boundaries.

Non-claims:

- This register does not approve v0.91.7 release readiness.
- This register does not approve WP-19 external review.
- This register does not fix any finding.
- This register does not claim #5408 is done; #5408 remains open and underway.
- This register did not use AWS or rerun paid/remote validation lanes.

## Findings

| ID | Severity | Area | Finding | Evidence | Required Disposition Before External Review | Route |
| --- | --- | --- | --- | --- | --- | --- |
| IR-4645-001 | P1 | WP-07 / release gate | WP-07 hardening remains open and cannot be consumed as review-clean. #5408 is open, PR #5419 is open/draft, and the live snapshot captured non-terminal CI/coverage. | `docs/reviews/v0.91.7/internal-review-4645/live-state/dependency_5408_5419.json`; `docs/reviews/v0.91.7/internal-review-4645/live-state/summary.json`; `docs/reviews/v0.91.7/remaining-sprints-5403/WP07_HARDENING_REVIEW_5045.md`; `docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md` | Merge/close #5408 with truthful retained evidence, or record explicit operator-approved blocker disposition before WP-19. | Existing #5408 / PR #5419 |
| IR-4645-002 | P1 | Release sequencing | External review must remain blocked until WP-18 findings and remediation routing are current. The canonical register still records WP-18 as not run and WP-19 as waiting for WP-18 remediation. | `docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md`; `docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml`; `docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md` | Publish this internal review packet, route findings to WP-20/#4647 or existing owners, then refresh the external-review handoff. | #4645, then #4647 before #4646 |
| IR-4645-003 | P1 | Review truth | The canonical sprint review register is stale against live terminal truth and later remediation records. It still marks some closed WP rows as open/no-packet and still lists some remediations as unresolved although their C-SDLC records show terminal evidence. | `docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md`; `.csdlc/issues/5404/index.json`; `.csdlc/issues/5413/index.json`; `docs/reviews/v0.91.7/internal-review-4645/live-state/github_issue_summary.json` | Reconcile the canonical register to current live/C-SDLC truth before sending a third-party packet. | #4644 or #4647 |
| IR-4645-004 | P1 | Provider security | Hosted provider adapters accept `http` endpoint overrides while attaching real hosted provider credentials. This can send bearer/API keys over plaintext for hosted OpenAI/OpenRouter/Z.ai/Gemini-style routes. | `adl/src/provider_adapter.rs` lines around credential resolution and `provider_endpoint_url`; local verification captured in #4645 review notes | Require HTTPS for hosted providers; permit HTTP only for explicit loopback/local providers with focused tests. | New provider/security remediation under #4647 |
| IR-4645-005 | P2 | Lifecycle / closeout | Several closed milestone issues have GitHub closure ahead of local terminal C-SDLC truth or active closeout-reconciliation PRs. The fresh version-labeled open set still includes #5527, and open closeout/audit PRs remain. | `docs/reviews/v0.91.7/internal-review-4645/live-state/github_issue_summary.json`; `docs/reviews/v0.91.7/internal-review-4645/live-state/github_open_pr_summary.json`; `.csdlc/issues/4642/index.json`; `.csdlc/issues/4643/index.json` | Complete/reconcile terminal receipts or document bounded residuals before release ceremony. | Existing #5527 and related closeout PRs |
| IR-4645-006 | P2 | Coverage / CI | Green PR checks are not equivalent to release coverage approval. PR coverage intentionally uses fast/deferred paths, and nightly coverage reporting claims merge-gate parity while filtering only `adl/src`, whereas the merge gate checks both `adl/src` and `adl-runtime/src`. | `.github/workflows/ci.yaml`; `.github/workflows/nightly-coverage-ratchet.yaml`; `adl/tools/enforce_coverage_gates.sh`; `docs/milestones/v0.91.7/review/coverage_policy/ADL_COVERAGE_FAST_PATH_REPAIR_4785.md` | Either run/retain authoritative coverage evidence for release consumption or narrow claims; repair nightly per-file scope mismatch. | Coverage tooling follow-up under #4647 |
| IR-4645-007 | P2 | Runtime observability | CSM `/metrics` reads and parses the entire `operator_events.jsonl` on every request via `read_jsonl_tail(..., usize::MAX)`, which can become memory/latency-heavy for long-lived runtimes. | `adl/src/csm_runtime_api.rs` lines around `metrics_response` and `read_jsonl_tail` | Replace with bounded tail, rolling counter, or metadata-backed event count and add focused regression coverage. | Runtime v3 / CSM follow-up |
| IR-4645-008 | P2 | Runtime API correctness | ACIP websocket fail-closed responses return textual `501 Not Implemented` / `426 Upgrade Required`, but status-code mapping omits both and collapses them to HTTP 500. | `adl/src/csm_runtime_api.rs` lines around `/acip/ws` and `runtime_api_status_code` | Add explicit 426/501 mappings and focused HTTP status regression tests. | Runtime v3 / ACIP follow-up |
| IR-4645-009 | P2 | Dependency / supply chain | Release-review-grade supply-chain coverage for the independent C-SDLC v2 workspace remains incomplete: retained review records no continuous locked/MSRV/advisory/license/SBOM gate, and #5403 synthesis says advisory DB coverage was not claimed. | `docs/milestones/v0.91.7/review/csdlc-v2/issue-5375/specialists/DEPENDENCY_REVIEW.md`; `docs/milestones/v0.91.7/review/csdlc-v2/issue-5375/FINDING_ROUTING.md`; `docs/reviews/v0.91.7/remaining-sprints-5403/FINDINGS_SYNTHESIS.md` | Add retained dependency/advisory/license/SBOM proof or explicitly gate it as external-review residual. | Existing P2-10 path or new dependency/CI follow-up |
| IR-4645-010 | P2 | AWS boundary docs | Manual AWS CodeFriend dispatch accepts an arbitrary `build_command` input and forwards it into CodeBuild without a retained allowlist/trust-boundary explanation. This review did not run AWS. | `.github/workflows/aws-codefriend-build.yaml` | Document who may dispatch it, what commands are allowed, and how artifacts/redaction/provenance are bounded; optionally enforce an allowlist. | AWS/provider boundary follow-up under #4647 |
| IR-4645-011 | P3 | C-SDLC review identity | C-SDLC publication revision identity accepts a review scope but hashes whole-tree state, so unrelated untracked or out-of-scope files can stale/block publication despite a narrower declared review assignment. | `csdlc-v2/src/git.rs`; `csdlc-v2/src/publication.rs` | Either honor scope pathspecs in revision hashing or rename the contract as whole-tree review identity. | C-SDLC v2 review/publication follow-up |
| IR-4645-012 | P3 | Maintainability / architecture | Core ownership remains concentrated in very large modules carrying security, readiness, lifecycle, persistence, and provider responsibilities. This weakens external-review isolation. | `adl/src/long_lived_agent.rs`; `adl/src/csm_runtime_api.rs`; `adl/src/scheduler.rs`; `adl/src/provider_adapter.rs`; `csdlc-v2/src/store.rs`; `docs/reviews/v0.91.7/internal-review-4645/packet/repo_inventory.json` | Plan ownership-first splits after release-tail blockers are resolved; do not hide behavior changes in cosmetic module moves. | v0.91.8 architecture/tooling backlog |

## Discounted Or Superseded Lane Observations

- Some subagent observations reported that #4645 retained packet/live-state
  files did not exist. Those checks ran against root or before this worktree
  retained the packet. The final packet path for this review is
  `docs/reviews/v0.91.7/internal-review-4645/`.
- The packet builder produced a temporary duplicate `docs/docs/...` tree during
  local snapshot iteration. That duplicate was removed; the intended retained
  path is `docs/reviews/v0.91.7/internal-review-4645/`.
- WP-16 quality evidence is not treated as a defect by itself. It truthfully
  records `passed_with_open_downstream_gates`, which this review consumes as a
  release-boundary constraint rather than a failed quality gate.

## Immediate Blocking Set

Before WP-19 external review can be considered:

1. #5408 / PR #5419 must reach terminal truthful state or receive an explicit
   operator-approved blocker disposition.
2. #4645 must publish this internal review packet and route findings.
3. The sprint review register must be reconciled to current WP-14/WP-15/WP-16,
   #5404/#5413, and open closeout-reconciliation truth.
4. WP-20/#4647 must either fix or explicitly disposition P1/P2 findings that
   affect security, coverage, lifecycle truth, or external-review claims.
