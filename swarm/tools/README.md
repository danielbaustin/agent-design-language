# swarm/tools

Utility scripts for ADL workflow automation and PR hygiene.

## Scripts

- `pr.sh`: Canonical start/finish helper for branching, checks, commit/push, and PR creation.
  `finish` exits non-zero on stage/commit/push/PR mutation failures.
- `codex_pr.sh`: Wrapper that composes `pr.sh start`, Codex execution, and `pr.sh finish`.
- `codexw.sh`: Codex runner wrapper used by `codex_pr.sh`.
- `card_paths.sh`: Canonical card path helpers.
- `BURST_PLAYBOOK.md`: Sequential burst operator guide using `adl_pr_cycle`.
- `burst_continue.sh`: Generates deterministic resume/continue commands for halted bursts.
- `burst_worktree.sh`: Creates/drops deterministic issue-scoped git worktrees for burst isolation.
- `open_artifact.sh`: Opens canonical cards or burst report directories from one command.
- `batched_checks.sh`: Runs tooling sanity + swarm checks through one stable command shape (quiet summaries by default; use `--verbose` or `ADL_CHECKS_VERBOSE=1` for full logs).
- `demo_one_command.sh`: Recommended one-command entrypoint for demo workflows.
- `demo_v0_4.sh`: Runs the v0.4 demo pass (fork/join swarm, bounded parallelism, deterministic replay) with a no-network mock provider.
- `mock_ollama_v0_4.sh`: Deterministic local mock for `ollama run` used by v0.4 demos.
- `preflight_review.sh`: One-command preflight that runs batched checks + schema/demo tests, with optional PR hygiene checks.
- `branch_hygiene.sh`: Safe branch pruning helper (dry-run by default; apply merged/stale/remote cleanup via explicit flags).
- `update_latest_reports.sh`: Refreshes `LATEST.md` pointers for automation and pr-cycle report directories.
- `update_reports_index.sh`: Refreshes `LATEST.md` pointers and updates `.adl/reports/INDEX.md`.
- `REPORT_SCHEMA.md`: Standard report schema reference (`adl-report/v1`).
## Demo UX Quickstart

From repo root:

```bash
cd swarm
cargo run -- demo demo-b-one-command --run --quiet --open --out ../.adl/reports/demo
```

This is the recommended low-noise default path for demo UX.
## Default Workflow Guide

- `docs/default_workflow.md`: Canonical contributor flow for `adl_pr_cycle` + `pr.sh`, including card paths and common remediations.
## Branch Hygiene

Recommended safe flow:

```bash
# Dry-run report first (no deletes)
swarm/tools/branch_hygiene.sh

# Delete local merged branches only
swarm/tools/branch_hygiene.sh --apply

# Optionally include stale local branches and merged remote codex/* branches
swarm/tools/branch_hygiene.sh --apply --include-stale --remote-merged
```
## Codex.app Permissions: `default.rules` and `writable_roots`

This section documents how to reduce Codex permission prompts during ADL workflows.

### 1) Command approval rules (`default.rules`)

Codex command approvals are prefix-match based. On macOS, the likely rules file is:

- `~/.codex/rules/default.rules`

Quick discovery command if that path does not exist:

```bash
find ~/.codex -maxdepth 3 -type f -name "*.rules"
```

Edit the file manually and keep an in-repo template as source-of-truth:

- `swarm/tools/default.rules.profiles.example`

### 2) Why "yes + don't ask again" can still prompt

Approvals are shape-sensitive. A rule for:

- `["./swarm/tools/pr.sh", "start"]`

does not always match:

- `["/bin/zsh", "-lc", "./swarm/tools/pr.sh start 233 --slug ..."]`
- a different argv prefix, wrapper, or segmented shell pipeline

Keep command shapes stable by running direct forms:

- `./swarm/tools/pr.sh ...`
- `./swarm/tools/burst_worktree.sh ...`
- `gh ...`
- `git ...`

Avoid wrapping these in `/bin/zsh -lc ...` in automation prompts unless absolutely required.

### 3) Recommended rule profiles

Use one of these profiles from `swarm/tools/default.rules.profiles.example`:

- `safe-default`: narrower allowlist for routine ADL work
- `unattended-full-speed`: broader allowlist for low-touch burst execution

Typical stable entries:

- `prefix_rule(pattern=["./swarm/tools/pr.sh"], decision="allow")`
- `prefix_rule(pattern=["./swarm/tools/burst_worktree.sh"], decision="allow")`
- `prefix_rule(pattern=["gh", "pr"], decision="allow")`
- `prefix_rule(pattern=["gh", "issue"], decision="allow")`
- `prefix_rule(pattern=["git", "-C", "/Users/daniel/git/agent-design-language"], decision="allow")`

### 4) `writable_roots` is different from `default.rules`

- `default.rules` controls command approval prompts (can I run this command shape?).
- `writable_roots` controls filesystem write scope (can this command write to this path?).

In Codex.app, `writable_roots` may be injected per run/session and not persisted in `default.rules`.
That means command approval can be "allow", but writes can still fail with sandbox denial if the path is outside the active writable roots.

Practical mitigations:

- Run from repo-root worktrees under a known writable root.
- Keep report/output paths inside the repo whenever possible.
- Ensure automation runs include required writable paths (for example, repo root plus any automation state directories).
- If an automation memory path is outside writable roots, either add that root to the run config or move memory into a repo path.

### 5) 3-lane burst checklist (near-zero prompts)

1. Start from direct commands only: `./swarm/tools/...`, `gh ...`, `git ...`.
2. Install one profile from `swarm/tools/default.rules.profiles.example` into `~/.codex/rules/default.rules`.
3. Confirm writable roots include all active lane worktrees and report/memory destinations.
4. Run issue flow via `./swarm/tools/pr.sh start ...` and `./swarm/tools/pr.sh finish ...` (no shell wrappers).
5. Keep automation/log/report writes inside approved roots.

## Worktree Smoke Test

`pr.sh new` should succeed from any repo worktree checkout:

```bash
# 1) Primary checkout
cd /Users/daniel/git/agent-design-language
./swarm/tools/pr.sh new --title "[smoke] pr.sh new from primary" --slug smoke-primary --labels "track:roadmap,version:v0.3,area:tools,type:task,epic:v0.3-tooling-git" --body "smoke" --no-start

# 2) Lane worktree
cd /Users/daniel/git/adl-lane-b
./swarm/tools/pr.sh new --title "[smoke] pr.sh new from lane worktree" --slug smoke-lane --labels "track:roadmap,version:v0.3,area:tools,type:task,epic:v0.3-tooling-git" --body "smoke" --no-start

# 3) Burst worktree
cd /Users/daniel/git/agent-design-language/.worktrees/burst/206-lane-b-demo-ux
./swarm/tools/pr.sh new --title "[smoke] pr.sh new from burst worktree" --slug smoke-burst --labels "track:roadmap,version:v0.3,area:tools,type:task,epic:v0.3-tooling-git" --body "smoke" --no-start
```

## One Obvious Demo Command

```bash
./swarm/tools/demo_one_command.sh
```

## Codex.app Skills
This section documents Codex.app skills used with ADL; skills live in Codex.app but are specified here for versioning.
Default workflow: `adl_pr_cycle` is the authoritative path for new ADL development work.

### `adl_pr_cycle`

Deterministic state-machine workflow for Codex.app:

- `preflight -> start -> codex -> validate_finish -> report`

Required inputs:

- `issue_num`
- `slug`
- `title`
- `paths`

Optional inputs:

- `mode` (`apply|suggest`, default `apply`)
- `open_pr` (default `true`)
- `merge` (default `false`)
- `delete_branch` (default `false`)

Invariants:

- Canonical cards are used:
  - `.adl/cards/<issue_num>/input_<issue_num>.md`
  - `.adl/cards/<issue_num>/output_<issue_num>.md`
- Branch is `codex/<issue_num>-<slug>`.
- Allowed edit paths are restricted to:
  - `<paths>`
  - `.adl/cards`
  - `.adl/logs`
  - `.adl/reports`
- Codex logs are always written to:
  - `.adl/logs/<issue_num>/codex.log`
- Finish/validation gate is always:
  - `./swarm/tools/pr.sh finish <issue_num> --title "<title>" --paths "<paths>" -f .adl/cards/<issue_num>/input_<issue_num>.md --output .adl/cards/<issue_num>/output_<issue_num>.md`
- `.adl/**` must never be staged/committed.
- A report is always written (including on failure) to:
  - `.adl/reports/pr-cycle/<issue_num>/<timestamp>/report.md`

## Skill Prompt (Copy/Paste)

```text
You are running skill: adl_pr_cycle.

Inputs:
- issue_num (required)
- slug (required)
- title (required)
- paths (required, comma-separated)
- mode (optional: apply|suggest, default apply)
- open_pr (optional, default true)
- merge (optional, default false)
- delete_branch (optional, default false)

Hard guardrails:
1) Deterministic state machine only:
   preflight -> start -> codex -> validate_finish -> report
2) Do not edit outside:
   - <paths>
   - .adl/cards
   - .adl/logs
   - .adl/reports
3) Never stage or commit .adl/** files.
4) Retry transient command failures at most 2 times.
5) Always produce a report file even on failure.

Procedure:
1) Preflight
   - Validate required inputs are non-empty.
   - Compute branch: codex/<issue_num>-<slug>.
   - Ensure .adl/logs/<issue_num>/ and .adl/reports/pr-cycle/<issue_num>/<timestamp>/ exist (create the <timestamp> directory before writing the report).
2) Start
   - Run: ./swarm/tools/pr.sh start <issue_num> --slug <slug>
   - Ensure canonical cards exist:
     .adl/cards/<issue_num>/input_<issue_num>.md
     .adl/cards/<issue_num>/output_<issue_num>.md
3) Codex
   - Read the input card.
   - Run Codex in requested mode against the card.
   - Enforce edit fence to <paths> plus .adl/cards,.adl/logs,.adl/reports.
   - Tee Codex output to .adl/logs/<issue_num>/codex.log.
4) Validate/Finish (canonical gate)
   - Run:
     ./swarm/tools/pr.sh finish <issue_num> --title "<title>" --paths "<paths>" -f .adl/cards/<issue_num>/input_<issue_num>.md --output .adl/cards/<issue_num>/output_<issue_num>.md
   - If open_pr=false, include --no-open.
   - If merge=true, include --merge (only when open_pr=true or an existing PR already exists).
5) Report (always)
   - Write:
     .adl/reports/pr-cycle/<issue_num>/<timestamp>/report.md
   - Include:
     - Input values
     - Derived branch
     - Commands attempted (in order)
     - Modified files excluding .adl/**
     - Check/finish results
     - PR URL (if available)
     - Exactly one next action command

Failure policy:
- Fail fast on non-transient errors.
- On failure, still write the report and include one next-action command.
```
