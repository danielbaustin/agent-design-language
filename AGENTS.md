# ADL Agent Guidelines

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

1. Use `workflow-conductor` for every issue and lifecycle stage.
   - Do not bypass the conductor for issue execution, review routing,
     publication, janitor work, or closeout.
   - GitHub operations should use the shared token resolver. When an explicit
     token-file source is needed, use
     `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token`. Never print, copy,
     commit, or expose the token contents.
   - Provider credentials, when available, may also be sourced from
     operator-approved files outside the repo under `$HOME/keys/`. Do not scan,
     print, copy, commit, or expose that directory or file contents. Map the
     approved source into the expected provider environment variable only for
     the command that needs it, such as `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`,
     or `DEEPSEEK_API_KEY`. For provider-specific names such as Gemini/Google,
     follow the active provider setup docs instead of assuming aliases, and
     prefer those docs when preparing reusable local environment files.
2. Edit cards only with editor skills.
   - Use `sip-editor`, `stp-editor`, `spp-editor`, `srp-editor`, `sor-editor`,
     or other issue-card editor skills when card surfaces need normalization.
   - Do not hand-edit cards opportunistically.
   - Do not hand-roll new cards from memory. New issue cards must come from
     the active versioned prompt templates in `docs/templates/prompts/`.
   - The current prompt-template registry is `docs/templates/prompts/current.json`;
     use it rather than hard-coding a template version unless an issue
     explicitly requires a compatibility path.
   - For new or fully re-rendered cards, prefer the deterministic
     prompt-template values renderer over direct Markdown structure edits:
     `adl-csdlc tooling prompt-template validate-values`, `edit-values`,
     `render`, `render-all`, `validate-structure`, and `validate-schemas`.
   - For supported field-level card updates, edit the values object first with
     `adl-csdlc tooling prompt-template edit-values --kind <kind> --values <path> --set <field=value>`,
     then render and validate structure. Do not patch the rendered Markdown
     when a declared values-field edit is sufficient.
   - When starting from an existing rendered card, test representability first
     with `adl-csdlc tooling prompt-template import-values --kind <kind> --input <card.md> --out <values.yaml>`,
     then validate the imported values and re-render before accepting a rewrite.
   - Treat the tracked structure schemas under
     `docs/templates/prompts/<version>/schemas/` as the template-shape
     authority. If a rendered card fails structure validation, fix the values or
     intentionally version/regenerate the template schema; do not patch locked
     template prose by hand.
3. Always work in a bound worktree on a specific branch.
   - Never do tracked issue work on `main`.
   - Use the repo-native issue-mode `pr run` flow to bind execution context.
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
     `complete` when the current session's bounded objective is actually
     achieved, including a truthful handoff into review/wait state after
     publication when that was the session goal, or `blocked` only when the
     repeated blocking threshold is met and meaningful progress cannot continue.
5. Always review work with a subagent before opening the PR.
   - Run a bounded review subagent over the changed work product.
   - Fix all actionable findings immediately before publication.
6. Always perform closeout after the issue is closed.
   - Use the normal closeout path so issue truth, cards, artifacts, and GitHub
     state all agree.

## Repository-Specific Working Style

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
- Preserve the canonical card lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`.
  `SRP` is the Structured Review Prompt and review-result surface; `SOR` is the
  truthful execution and integration record.
- Treat prompt cards as durable C-SDLC state, not disposable chat output.
  `SIP`, `STP`, and `SPP` should be issue-specific and design-time ready before
  execution starts. If they are generic, stale, or incomplete, route them through
  the appropriate editor skill before running the issue.
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
2. route through `workflow-conductor`
3. confirm the primary checkout is clean on `main`, inspect active worktrees,
   and preserve any session handoff or collision evidence before binding work
4. confirm all five C-SDLC cards exist and came from the active prompt-template
   registry
5. make sure `SIP`, `STP`, and `SPP` are issue-specific and design-time ready
6. follow the conductor-selected lifecycle step
7. if the issue is ready for execution binding, use `adl/tools/pr.sh run <issue>`
8. call `create_goal` for the bound tracked issue session before implementation
   starts
9. make the bounded change in the issue worktree, never on `main`
10. run the smallest meaningful validation for the touched surface
11. run a pre-PR subagent review and fix findings
12. verify PR base/stack topology, then publish through the normal PR workflow
13. use `update_goal` for truthful terminal session state, then perform closeout
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
