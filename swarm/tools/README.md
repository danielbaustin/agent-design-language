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
- `batched_checks.sh`: Runs tooling sanity + swarm checks through one stable command shape (quiet summaries by default; use `--verbose` or `ADL_CHECKS_VERBOSE=1` for full logs).
- `demo_one_command.sh`: Recommended one-command entrypoint for demo workflows.
- `branch_hygiene.sh`: Safe branch pruning helper (dry-run by default; apply merged/stale/remote cleanup via explicit flags).
- `update_latest_reports.sh`: Refreshes `LATEST.md` pointers for automation and pr-cycle report directories.
- `update_reports_index.sh`: Refreshes `LATEST.md` pointers and updates `.adl/reports/INDEX.md`.
- `REPORT_SCHEMA.md`: Standard report schema reference (`adl-report/v1`).

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

## Recommended Allowlist Rules

For unattended burst runs, a minimal allowlist typically includes:

- `./swarm/tools/pr.sh start`
- `./swarm/tools/pr.sh finish`
- `./swarm/tools/pr.sh new`
- `gh pr ready -R danielbaustin/agent-design-language <pr-url>`
- `gh pr merge -R danielbaustin/agent-design-language --squash --delete-branch <pr-url>`

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
