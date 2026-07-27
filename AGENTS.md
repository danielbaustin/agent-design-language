# ADL Agent Guidelines

## Final C-SDLC v2 authority (Gate 10D2)

The v1 C-SDLC command wrappers, `pr.sh` lifecycle wrappers, prompt-template
wrappers, and `csdlc-import` binary are sunset. For C-SDLC work, the sole
operational authority is the independent Rust v2 binary set under
`csdlc-v2/`, routed through the typed skills in `csdlc-v2/operator/skills/`.
Use the typed v2 binaries and do not invoke the removed v1 wrappers. The final
coexistence inventory explicitly records `v1_sunset`; historical Gate 10A-C
records remain immutable evidence and are not an instruction to retain deleted
binaries. Session ownership and stale-claim recovery remain required
invariants, retained in the shared ledger until a later ADL overlap cleanup.

This file is the repository-local operating contract for coding agents working
in ADL.

It follows the OpenAI `AGENTS.md` pattern of keeping one predictable,
high-signal instruction surface at the repo root, then adapts that pattern to
ADL's real workflow and review discipline.

## Core Principles

These are the four behavioral principles at the center of this file.

1. Think before coding.
   - Understand the goal, the acceptance surface, and the smallest safe change
     before editing.
2. Simplicity first.
   - Prefer the simplest truthful solution over cleverness, abstraction churn,
     or framework theater.
3. Make surgical changes.
   - Change only the files and behavior needed for the issue you are working
     on.
4. Stay goal-driven.
   - Keep work tied to the issue outcome, not to adjacent cleanup or tempting
     side quests.

## Workflow Rules

These rules are mandatory for ADL issue work.

1. Use the typed v2 C-SDLC route for C-SDLC issues and lifecycle stages.
   - `workflow-conductor` and repo-native `pr.sh` are historical v1 routes and
     are not valid C-SDLC v2 lifecycle commands.
   - GitHub operations should use the shared token resolver. When an explicit
     token-file source is needed, use
     `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token`. Never print, copy,
     commit, or expose the token contents. When no explicit override is set,
     repo-native GitHub commands may also discover the operator-approved
     default token file at `$HOME/keys/github.token`; explicit environment
     sources still take precedence.
   - Provider credentials, when available, may also be sourced from
     operator-approved files outside the repo under `$HOME/keys/`. Do not scan,
     print, copy, commit, or expose that directory or file contents. Map the
     approved source into the expected provider environment variable only for
     the command that needs it, such as `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`,
     or `DEEPSEEK_API_KEY`. For provider-specific names such as Gemini/Google,
     follow the active provider setup docs instead of assuming aliases, and
     prefer those docs when preparing reusable local environment files.
   - AWS work for ADL must default to the Agent Logic business AWS account, not
     the operator's personal/default AWS account. Use the approved business
     profile `agent-logic-admin` for AWS billing checks, runtime experiments,
     SSM, CodeBuild, SNS, EC2/Spot, storage, and other ADL-related AWS work.
     Before relying on AWS account state, verify that the profile resolves to
     the approved Agent Logic business account rather than recording a static
     account identifier in repo policy.
     Personal/default profiles such as `default` or legacy user profiles may be
     used only when the operator explicitly authorizes a bounded task against a
     personal account. Do not print, copy, commit, or expose AWS credentials or
     credential-file contents.
2. Edit cards only with editor skills.
   - Use `sip-editor`, `stp-editor`, `spp-editor`, `srp-editor`, `sor-editor`,
     or other issue-card editor skills when card surfaces need normalization.
   - Do not hand-edit cards opportunistically.
   - Do not hand-roll new cards from memory. New issue cards must come from
     the active versioned prompt templates in `docs/templates/prompts/`.
   - The current prompt-template registry is `docs/templates/prompts/current.json`;
     use it rather than hard-coding a template version unless an issue
     explicitly requires a compatibility path.
   - For new or fully re-rendered cards, use the independent v2
     `csdlc-edit` and `csdlc-validate` typed routes over direct Markdown/state
     edits. The v1 prompt-template wrappers are historical and sunset.
   - For supported field-level card updates, send a typed `csdlc-edit apply`
     request, then run `csdlc-validate`. Do not patch rendered Markdown when a
     declared semantic field edit is sufficient.
   - When starting from an existing rendered card, use the v2 markdown.rs AST
     importer through a typed edit request and validate before accepting a
     rewrite.
   - Treat the tracked structure schemas under
     `docs/templates/prompts/<version>/schemas/` as the template-shape
     authority. If a rendered card fails structure validation, fix the values or
     intentionally version/regenerate the template schema; do not patch locked
     template prose by hand.
3. Always work in a bound worktree on a specific branch.
   - Never do tracked issue work on `main`.
   - Use the v2 `csdlc-bind` flow to bind execution context.
   - Keep the primary checkout clean on `main` for inspection, bootstrap,
     doctor/readiness, and issue-mode binding only. After binding, tracked
     implementation, janitor, finish, and repair edits happen in the issue
     worktree.
   - Before issue work, check root `git status --short --branch` and
     `git worktree list --porcelain`. If the primary checkout is on a feature
     branch or has tracked changes, stop and route the recovery through
     `workflow-conductor` / repo-native `pr run` or `pr doctor` evidence when
     available. Use only the narrowest manual fallback needed to preserve work
     into an issue worktree and restore the primary checkout to clean `main`.
   - See `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md` for
     the cross-session coordination and broadcast-note contract.
4. Always create an issue-bound session goal before implementation work starts.
   - For tracked issue sessions, call `create_goal` after the issue is ready and
     before bounded implementation begins in the issue worktree.
   - The goal should minimally name the issue number and the concrete session
     objective so token accounting, completion, and blocked-state reporting stay
     tied to the tracked issue.
   - Use `update_goal` only for truthful terminal state changes:
     `complete` when the current session's declared terminal boundary is
     actually satisfied. Handoff-only completion is allowed only when the goal
     explicitly declares a handoff boundary such as setup-only or review-only
     publication. Default tracked implementation goals stay active while the PR
     is red, pending, conflicted, draft, missing required checks, or missing
     current SRP/SOR truth. Use `blocked` only when the repeated blocking
     threshold is met and meaningful progress cannot continue.
5. Always review work with a subagent before opening the PR.
   - Run a bounded review subagent over the changed work product.
   - Fix all actionable findings immediately before publication.
6. Always perform closeout after the issue is closed.
   - Use the normal closeout path so issue truth, cards, artifacts, and GitHub
     state all agree.

## Repository-Specific Working Style

### C-SDLC v2 coexistence (Gate 10A)

- Generation authority is `csdlc-v2/operator/generation-selector.json`. Gate 10A-C records are historical; Gate 10D2 is the current final `v1_sunset` authority.
- Explicit v2 work routes through the ten typed contracts under `csdlc-v2/operator/skills/`; those skills delegate to Rust binaries and never mutate Markdown/state directly.
- Resolve every current lifecycle route through `csdlc-install resolve`, which reads that selector as the sole authority. Install v2 only into the dedicated `.adl/bin/csdlc-v2/` generation directory; the final verifier also fails if forbidden v1 paths reappear.
- Historical rollback and recovery proofs remain immutable evidence. The exact D2 approval authorizes the completed v1 command-surface sunset; retained session ownership remains a shared invariant.

- ADL is deterministic by design. Do not introduce hidden state, undeclared
  side effects, or review-hostile magic.
- Treat model/tool output as governed work, not free authority.
- Keep milestone claims, proof claims, and review claims evidence-bound.
- Prefer repo-relative paths in artifacts and records.
- Do not silently widen issue scope.
- For process liveness or port checks, use the permission-safe helper instead
  of broad host process scans: `adl process status --pid-file <path> --json`,
  `adl process status --pid <pid> --json`, or
  `adl process status --port <port> --json`. See
  `docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md`. Do not use `ps aux`,
  `ps -ef`, broad `pgrep`, or broad `lsof` dumps as normal workflow control.
- New tests must be PVF-classifiable at authoring time. When adding a new test
  surface, make lane class, proof role, determinism posture, resource profile,
  and release-gate status explicit in the same issue/PR through the tracked
  manifest, inventory, or tightly-coupled proof packet.
- Keep tests boring. Do not push shard mechanics, CI/release-mode branching, or
  hidden routing policy down into ordinary test logic; that belongs in manifests,
  runners, and policy docs.
- Preserve the canonical card lifecycle:
  `SIP -> STP -> SPP -> VPP -> SRP -> SOR`. `VPP` is validation-planning
  truth; `SRP` is the Structured Review Prompt and review-result surface; `SOR`
  is the truthful execution and integration record.
- Treat prompt cards as durable C-SDLC state, not disposable chat output.
  `SIP`, `STP`, and `SPP` should be issue-specific and design-time ready before
  execution starts. If they are generic, stale, or incomplete, route them through
  the appropriate editor skill before running the issue.
- During v0.91.6 rescue-sprint and release-tail work, also follow
  `docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md` for watcher-owned
  wait states, prep-scout promotion, scheduler non-authority, and binary-first
  workflow command expectations.
- Treat `SPP` as the operative issue-local plan. If real execution diverges
  materially from the tracked plan, update the `SPP` before continuing.
- Treat `SRP` and `SOR` as truth surfaces. `SRP` records review prompts,
  findings, and dispositions; `SOR` records actual execution, validation,
  integration, and closeout truth.
- Prefer the human prompt editor or card editor skills for filling and
  normalizing cards. Do not regenerate complete card prose when a template field
  update is sufficient.
- When the issue context supports values-rendered prompt cards, update the
  values object first, render through the Rust tooling, then run structure and
  schema validation. Use editor skills for lifecycle truth and bounded repairs,
  not as a reason to bypass the renderer/schema path.
- Treat observability as part of workflow truth, not optional garnish. When a
  change touches workflow control-plane paths, runtime/provider execution,
  watchdog behavior, or machine-readable command output, record the relevant
  logging policy and proof in the issue artifacts.
- When ADL lifecycle tooling exposes a real tooling bug, suspicious stuck
  state, or repeatable control-plane anomaly, capture a durable local bug
  packet or follow-on issue instead of silently retrying around it.
- Preserve the current logging channel contract unless the issue explicitly
  changes it:
  - machine-readable payloads belong on stdout
  - human-oriented `adl_event` observability belongs on stderr by default
  - compatibility redirection such as `ADL_OBSERVABILITY_STDERR=0` and
    `ADL_OBSERVABILITY_LOG=<path>` must be documented truthfully when used
- Do not claim OpenTelemetry, runtime/provider correlation, heartbeat coverage,
  or JSON-safe observability beyond what the tracked proofs actually establish.

## Where To Start

For a normal tracked issue:

1. read the source issue prompt and current task bundle
2. route through the typed v2 C-SDLC skill and binary for C-SDLC issues
3. confirm the primary checkout is clean on `main`, inspect active worktrees,
   and preserve any session handoff or collision evidence before binding work
4. confirm all six C-SDLC cards exist and came from the active prompt-template
   registry
5. make sure `SIP`, `STP`, and `SPP` are issue-specific and design-time ready
6. follow the conductor-selected lifecycle step
7. if the issue is ready for execution binding, use `csdlc-bind --root . --request <request.json>`
8. call `create_goal` for the bound tracked issue session before implementation
   starts
9. make the bounded change in the issue worktree, never on `main`
10. before Rust validation in a fresh or cold issue worktree, warm dependency
   artifacts through the shared wrapper when a trusted same-host source target
   is available: `bash adl/tools/rust_validation_warm_cache.sh`; see
   `docs/tooling/HARDLINKED_RUST_DEPENDENCY_CACHE.md`
11. run the smallest meaningful validation for the touched surface
12. run a pre-PR subagent review and fix findings
13. run `csdlc-review` before `csdlc-publish`; publication must fail closed without current review truth
14. use `update_goal` for truthful terminal session state, then perform closeout
   after merge/closure

## Validation Expectations

- Run the smallest proving validation that matches the issue's outcome type.
- Do not skip required proof just because the change is small.
- Do not run broad validation reflexively when focused proof is enough.
- Separate local preflight proof from CI integration proof. Local records must
  say what ran locally, what is deferred to GitHub CI, and why that deferral is
  safe for the touched surface.
- For owner-binary surfaces, prefer the focused lane runner when it matches the
  change: `bash adl/tools/run_owner_validation_lane.sh csdlc|runtime|review|all`.
- Operational ADL owner binaries must be installed into the stable generated
  repo-local directory outside Cargo build output: `.adl/bin/` by default, via
  `bash adl/tools/install_owner_binaries.sh`. Treat `adl/target/` as disposable
  build/cache output only; do not rely on live issue-worktree target directories
  as the operational source of truth, and do not replace stable binaries unless
  their recorded source provenance changes.
- Before Rust-heavy validation in a fresh issue worktree or on an EC2/remote
  builder, use the dependency-cache warmup wrapper only when a trusted warm
  target from the same host, same filesystem, same checkout family, and same
  toolchain is available:
  `ADL_RUST_WARM_CACHE_SOURCE_TARGET=<warm-target> ADL_RUST_WARM_CACHE_DEST_TARGET=<issue-worktree>/adl/target ADL_RUST_WARM_CACHE_MANIFEST_PATH=<issue-worktree>/adl/Cargo.toml bash adl/tools/rust_validation_warm_cache.sh`.
  Treat this as build acceleration only, not validation proof, and never replace
  the required validation lane with cache-warmup evidence.
- Keep review records and output cards truthful about what was and was not run.
- Docs-only and policy-only PVF work should prefer focused docs/path/contract/
  guardrail proof unless tracked runtime behavior changed.
- Prompt-card generation or repair work should include the focused renderer
  checks that apply to the touched surface: values validation, rendered
  structure validation, schema parity validation, and the Python-readable schema
  smoke check when schema artifacts are touched.
- Logging- or observability-affecting work should also record the smallest
  proving checks for:
  - stdout/stderr separation when machine-readable output is involved
  - redaction and path hygiene for emitted log lines or durable log artifacts
  - compatibility-log behavior when `ADL_OBSERVABILITY_LOG` or quiet-stderr
    mode is part of the claimed workflow

## Review And Publication Rules

- No PR should open before the work has had bounded subagent review.
- Verify the intended base branch before publication and verify the actual PR
  base immediately after creation, especially for stacked issue work.
- Findings come before summary.
- Fixes should stay within the issue's scope unless the operator explicitly
  widens it.
- If review uncovers a separate problem, open or route a follow-on issue
  instead of hiding new scope inside the current one.

## Non-Goals For This File

This root `AGENTS.md` is intentionally compact.

It is not:

- the full milestone manual
- a replacement for skill docs
- a substitute for issue cards
- the final word on nested package-specific agent guidance

## Source Baseline Used

Last reviewed: 2026-06-19.

This file was shaped from the OpenAI/source baselines named by `#2986`, plus
ADL-specific workflow policy:

- issue-named OpenAI `agents.md` GitHub baseline:
  `https://github.com/openai/agents.md`
- official OpenAI guide for `AGENTS.md` in Codex:
  `https://developers.openai.com/codex/guides/agents-md`
- practical OpenAI repository example:
  `https://github.com/openai/openai-cookbook/blob/main/AGENTS.md`
- broader open-format companion reference:
  `https://agents.md/`
- ADL's conductor, worktree, review, and closeout discipline
- ADL's shared GitHub token resolver and permission-safe process-status helper

The issue named the GitHub `openai/agents.md` baseline explicitly. That
repository now routes into the broader `agents.md` effort, so this file keeps
both the issue-named GitHub source and the live `agents.md` reference visible
for traceability.
