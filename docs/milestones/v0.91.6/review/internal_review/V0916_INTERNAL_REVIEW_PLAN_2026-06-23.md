# v0.91.6 Internal Review Plan

Date: `2026-06-23`
Owner issue: `#4582` `[v0.91.6][WP-14A][review] Complete internal review and pre-v0.92 burn-down checklist`
Prepared for: v0.91.6 internal review before external review, remediation/final preflight, next-milestone planning, and release ceremony.
Status: `ready_for_execution`

## Purpose

Run a findings-first internal review of v0.91.6 with enough coverage that the external reviewer should find little or nothing new. This review is not a release approval. It is the internal defect-discovery and truth-normalization pass for a large milestone that accumulated substantial runtime, provider, PVF, C-SDLC, sprint-process, template, validation, observability, and review-evidence work.

The review must check code, docs, lifecycle cards, sprint evidence, singleton issue evidence, demos, runtime claims, and release-tail truth. It must also apply the newer process improvements developed during v0.91.6: session-ledger coordination, Sprint Execution Packet expectations, VPP/PVF lane truth, issue/sprint token and time accounting, repo-quality/staleness checks, and sprint-level review/closeout discipline.

## Stop Boundary

This plan prepares the internal review. It does not execute the review, remediate findings, approve external review readiness, close `#4582`, close v0.91.6, or start release ceremony.

The review execution should produce separate tracked artifacts for findings, synthesis, remediation queue, and handoff.

## Required Outputs When Review Executes

Write the review artifacts under:

`docs/milestones/v0.91.6/review/internal_review/`

Expected execution artifacts:

- `V0916_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-23.md`
- `V0916_INTERNAL_REVIEW_SPECIALIST_FINDINGS_2026-06-23.md`
- `V0916_INTERNAL_REVIEW_SYNTHESIS_2026-06-23.md`
- `V0916_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-23.md`
- `V0916_INTERNAL_REVIEW_HANDOFF_2026-06-23.md`
- optional bounded repo/code review packets under `repo_packet_2026-06-23/`

The findings register must be findings-first, severity-ordered, source-backed, and route every accepted finding to an owner issue, follow-on issue candidate, or explicit non-blocking residual risk.

## Entry Gates

Before executing the internal review, confirm:

- `#4582` is the active owner issue. Closed `#3979` is retained
  source/planning evidence only.
- Active sprint and singleton review packets are present or their absence is recorded as a finding.
- Known open/dirty PRs are listed rather than silently ignored.
- The session ledger has no conflicting active claim for `#4582` or the selected review worktree.
- The review session creates an issue-bound goal with an explicit token budget.
- Review execution uses a bound issue worktree, not tracked edits on `main`.
- The review plan, findings, and handoff are tracked repo artifacts, not local-only `.adl` memory.

## Primary Evidence Inputs

### Milestone planning and control docs

- `docs/milestones/v0.91.6/README.md`
- `docs/milestones/v0.91.6/WBS_v0.91.6.md`
- `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md`
- `docs/milestones/v0.91.6/WP_ISSUE_WAVE_v0.91.6.yaml`
- `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md`
- `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md`
- `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md`
- `docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md`
- `docs/milestones/v0.91.6/REVIEW_AND_VALIDATION_CHECKLIST_v0.91.6.md`
- `docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md`
- `docs/milestones/v0.91.6/RELEASE_NOTES_v0.91.6.md`
- `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`
- `docs/milestones/v0.91.6/review/V0916_WP12_QUALITY_GATE_3977.md`

### Review and proof evidence

- `docs/milestones/v0.91.6/review/`
- sprint review packets for WP-02 through WP-10 and all completed mini-sprints
- `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`
- `docs/milestones/v0.91.6/review/V0916_SINGLETON_ISSUE_REVIEW_MATRIX_4432.md` if merged; otherwise record PR/issue state as a finding or caveat
- `docs/milestones/v0.91.6/review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md`
- `docs/milestones/v0.91.6/review/V0916_CSDLC_INTEGRATION_CONTROL_PLANE_SPRINT_REVIEW_4388.md`
- `docs/milestones/v0.91.6/review/V0916_CSDLC_CONTROL_PLANE_RELIABILITY_ROUTE_4396.md`
- provider, runtime, security, observability, scheduler, build-throughput, validation-manager, ACIP, AWS signal bridge, and Observatory review packets

### Code and tooling surfaces

Review code, not only docs. Include at least these surfaces when present:

- `adl/src/cli/pr_cmd/`
- `adl/src/cli/session_cmd.rs`
- `adl/src/session_ledger.rs`
- `adl/src/scheduler.rs`
- `adl/src/provider/`
- `adl/src/provider_communication.rs`
- `adl/src/runtime/` and runtime-v2 surfaces
- `adl/src/acip*` and ACIP runtime/message surfaces
- validation manager / PVF / owner-binary validation surfaces
- prompt-template renderer/schema/editor surfaces
- sprint-conductor helper scripts and skills
- issue goal metrics scripts and templates
- GitHub/octocrab transport surfaces

### Lifecycle and issue truth

- Local task bundles under `.adl/v0.91.6/tasks/` where available
- Live GitHub issue state for v0.91.6 release-tail issues
- Live PR state for open review/remediation/control-plane PRs
- Session ledger state for active/stale/released claims
- Worktree list and dirty-state inventory

## Review Lanes

### 1. C-SDLC workflow and lifecycle truth lane

Check:

- `AGENTS.md` compliance
- issue worktree discipline
- session-ledger use and stale claim behavior
- `SIP -> STP -> SPP -> VPP -> SRP -> SOR` truth
- card completeness before execution
- issue goal and token/time accounting
- sprint execution packet quality
- sprint closeout and issue closeout truth
- singleton/mis-labeled sprint work routing

Questions:

- Can an issue start before cards are complete and validated?
- Can an agent halt from a bad token budget or missing issue goal?
- Are stale claims, open PRs, or dirty worktrees visible to the reviewer?
- Are review findings routed rather than hidden in narrative prose?

### 2. Code correctness and tooling lane

Check:

- CLI behavior for `pr create`, `pr run`, `pr finish`, `pr closeout`, `session`, validation manager, and prompt-template tools
- GitHub/octocrab transport behavior and any remaining `gh` fallback paths
- fail-closed behavior for dirty roots, stale cards, broken PR waves, and missing validation lanes
- tests for recently repaired control-plane issues
- brittle hard-coded paths, stale version strings, or shell-only paths that should be Rust-owned

Questions:

- Does the code implement what the docs claim?
- Are failure modes observable and recoverable?
- Are validators fast enough for normal workflow use?
- Are docs-only paths paying unnecessary broad validation tax?

### 3. PVF, validation, and test-tax lane

Check:

- VPP adoption and lane assignment in current issue workflows
- PVF lane manifests and large/small test separation
- validation-manager path-profile selection
- slow-test index and build-log analysis
- S3/archive plans for CI logs and runtime traces
- owner-binary validation lane correctness

Questions:

- Are validation claims tied to the right lane and proof role?
- Can the review distinguish skipped, blocked, failed, and passed lanes?
- Are expensive tests isolated from fast docs/tooling validation where safe?

### 4. Runtime, ACIP, provider, and observability lane

Check:

- runtime fire-up and soak evidence
- Tokio runtime substrate and follow-on resilience work
- ACIP message/runtime slices and provider-boundary behavior
- AWS heartbeat/SNS/SSM/DDNS evidence and non-claims
- provider profiles, role suitability, OpenAI/Anthropic/Gemini/DeepSeek/OpenRouter evidence
- logging channel separation, redaction, heartbeat/progress, and OTel/Observatory consumption claims

Questions:

- Is the runtime assembled or still pre-assembly?
- Are provider/model claims evidence-bound?
- Are runtime logs and signals durable enough for review without leaking secrets or host-only paths?

### 5. Security, CAV, and public-record safety lane

Check:

- security bridge / CAV proof packets
- public prompt records, redaction, and publication boundaries
- AWS/SSM operational security docs
- provider credential handling and key-directory references
- tool-call authorization or ACC/UTS backlog routing where relevant

Questions:

- Are secrets excluded from docs, logs, and artifacts?
- Are public records safe and intentionally scoped?
- Are security claims routed correctly between ADL, runtime, and future enterprise-security work?

### 6. Docs, demos, ADR, and release-truth lane

Check:

- README/CHANGELOG/version/staleness risks manually, even if automated checks are future work
- ADR candidate and release-tail truth
- demo matrix truth, including Unity/Observatory status
- v0.91.7/v0.92 handoff truth
- feature docs and proof coverage consistency
- open issue/PR state against milestone docs

Questions:

- Are milestone docs current after late control-plane work?
- Are demos launchable/proven to the level claimed?
- Are ADRs reviewed, routed, accepted, or deferred truthfully?

### 7. Synthesis and remediation routing lane

Merge specialist findings into one findings register. Deduplicate without hiding severity. Every finding must include:

- severity
- title
- source evidence
- expected vs observed truth
- recommended owner route
- whether it blocks external review, final preflight, release ceremony, or only future cleanup

## Burn-Down Checklist Requirements

The internal review must include or reference a pre-v0.92 burn-down checklist that classifies each bridge item as:

- complete in v0.91.6
- owned by a still-open v0.91.6 issue
- routed to v0.91.7
- v0.92 blocker
- non-activation companion work
- deferred beyond v0.92 with rationale

Required bridge categories:

- C-SDLC integration/control-plane reliability
- VPP/PVF lane-template and validation-manager work
- sprint execution packets and sprint closeout automation
- issue goals, token/time accounting, and budget gates
- session ledger and watcher surfaces
- provider/model reliability and role suitability
- runtime fire-up, soak, observability, and AWS signals
- ACIP and agent communication substrate
- Security/CAV/public-record safety
- Observatory/Unity demos and demo proof matrix
- ADR/release-tail truth
- repo-quality/staleness checks
- v0.91.7/v0.92 planning handoff

## Validation Expectations For The Review Issue

The review execution should run only focused validation needed to prove the review artifacts are coherent. It should not use broad validation as a substitute for review judgment.

Minimum expected checks:

- referenced tracked files exist or missing files are called out as findings
- review artifacts contain no unjustified absolute host paths
- links to issues/PRs use live state where possible
- findings register is findings-first and severity-ordered
- remediation queue maps each accepted finding to an owner route
- `git diff --check` before PR publication
- card truth is updated in `SRP`/`SOR` during finish/closeout

## Known Process Additions Since Earlier Internal Reviews

This v0.91.6 internal review should explicitly account for improvements that did not exist, or were weaker, in v0.91.4/v0.91.5:

- session-ledger coordination for active/stale/released issue claims
- VPP as validation-planning surface
- PVF lane assignment and validation cost/test-tax modeling
- Sprint Execution Packet as sprint-level conductor artifact
- sprint-level review in addition to issue-level review
- singleton issue retained-review matrix
- issue goal metrics and token/time accounting
- planned pre-start card-completeness and token-budget gates
- repo-native GitHub/octocrab convergence, with explicit `gh` fallback caveats while tooling is repaired
- manual README/CHANGELOG/repo-quality/staleness checks until automated checks land

## Handoff Prompt For Review Execution

```text
Complete #4582 for v0.91.6 internal review. Use AGENTS.md, workflow-conductor, the session ledger, sprint-review/repo-review skills, and bounded subagents. Use ADL repo-native tools only; do not use gh. Keep all lifecycle cards and SOR/SRP truth explicit.

Do not remediate findings inside the review issue except tiny typo/path fixes. Do not close v0.91.6. Do not start external review. Produce tracked artifacts under docs/milestones/v0.91.6/review/internal_review/:
- findings register
- specialist findings
- synthesis
- remediation queue
- handoff
- pre-v0.92 burn-down checklist

Review code, docs, demos, ADRs, lifecycle cards, sprint evidence, singleton evidence, runtime/provider/security/observability/PVF/ACIP/build-validation surfaces, and live issue/PR truth. Findings first, severity ordered, source-backed, and routed to owner issues or explicit follow-ons.
```
