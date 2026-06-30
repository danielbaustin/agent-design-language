#!/usr/bin/env bash
# Usage/help functions for adl/tools/pr.sh. Source this file; do not execute directly.

usage() {
  cat <<'EOF'
pr.sh — reduce git/PR thrash while preserving human review

Commands:
  help
  create  --title "<title>" [--slug <slug>] [--body "<markdown>" | --body-file <path>] [--labels <csv>] [--version <v>]
  init    <issue> [--slug <slug>] [--title "<title>"] [--no-fetch-issue] [--version <v>]
  run     <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue] [--version <v>] [--allow-open-pr-wave]
  run     <adl.yaml> [--trace] [--print-plan] [--print-prompts] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--runs-root <dir>] [--quiet] [--open] [--allow-unsigned]
  doctor  <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--mode full|ready|preflight] [--allow-open-pr-wave] [--json]
  finish  <issue> --title "<title>" ... [-f <input_card.md>] [--output-card <output_card.md>] [--no-open] [--merge]
  validation <pr-number-or-url> [-R owner/repo] [--watch] [--json]
  pr-inventory [-R owner/repo] [--json]
  watch   <issue-number-or-url> [--slug <slug>] [--version <v>] [-R owner/repo] [--json]
  closing-linkage [--event-name <event>] [--event-path <path>] [--head-ref <branch>] [-R owner/repo]
  issue   <list|search|view|create|comment|edit|close> ...
  projection-map [--json]
  closeout <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue]

Compatibility / maintenance commands:
  card    <issue> [input|output] ... [--version <v0.2>] [-f <input_card.md>]
  output  <issue> [input|output] ... [--version <v0.2>] [-f <output_card.md>]
  cards   <issue> [--version <v0.2>] [--no-fetch-issue]
  ready   <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--json]
  preflight <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--allow-open-pr-wave] [--json]
  open
  status

Flags:
  (create)  --version <v0.85|v0.87.1>         Override detected version (otherwise inferred from labels/title).
  (init)    --version <v0.85|v0.87.1>         Override detected version (otherwise inferred from issue labels or title version:vX.Y[.Z...])
  (init)    --no-fetch-issue                  Do not fetch issue title/labels; requires --slug.
  (run issue-mode) --slug <slug> --title "<title>" --prefix <pfx> --no-fetch-issue --version <v> --allow-open-pr-wave
  (doctor/preflight) --allow-open-pr-wave     Skip the same-queue open milestone PR scan and report an explicit override warning.
  (run adl-mode) --runs-root <dir>            Override canonical run artifact root (default: <repo>/.adl/runs or ADL_RUNS_ROOT).
  (card)    -f, --file <input_card.md>         Output path for the generated input card (default: <cards_root>/<issue>/input_<issue>.md)
  (output)  -f, --file <output_card.md>        Output path for the generated output card (default: <cards_root>/<issue>/output_<issue>.md)
  (cards)   --version <v0.2>                   Override detected version (otherwise inferred from issue labels version:vX.Y)
  (cards)   --no-fetch-issue                   Do not fetch issue title/labels (uses issue-<n> title)
  (card/output) --version <v0.2>               Override detected version (otherwise inferred from issue labels version:vX.Y)
  (finish) --output-card <output_card.md>          REQUIRED: output card path (must exist)
  (finish) --merge                              Opt-in: ready + squash-merge + local closeout; remote branch deletion is not implied.
  (finish) --idempotent                         Safe no-op only when existing merged PR matches current finish inputs.
  (card/run) --slug <slug>                     Use an explicit slug instead of fetching the issue title.
  (run)     --title "<title>"                  Optional; accepted for UX symmetry and used to derive slug when --slug is omitted.
  (run)     --version <v0.85|v0.87.1>          Override detected version when the caller already knows the intended milestone band.
  (run)     --allow-open-pr-wave               Override the same-queue open milestone PR guard.

Notes:
- `pr create` creates the GitHub issue and bootstraps the local root STP/SIP/SOR bundle for a new issue.
- `pr init <issue> ...` bootstraps the same local root bundle for an issue that already exists.
- `pr run <issue> ...` is the preferred public execution-context binder for issue work.
- `pr doctor <issue> ...` is the preferred public readiness and drift diagnostic surface.
- `pr watch <issue> ...` is the typed tracked-issue lifecycle watcher for issue/PR wait states.
- `pr pr-inventory ...` is the typed release-tail PR inventory surface; use it instead of raw `gh pr list`.
- `pr closeout <issue> ...` finalizes a closed issue locally and safely prunes its execution worktree when possible.
- `pr closing-linkage ...` is the Rust-owned CI/linkage guard and prefers live PR metadata over stale event payloads when token context exists.
- `pr start <issue> ...` remains only as a legacy alias over the same Rust binding path and is no longer part of the taught public flow.
- `pr ready` and `pr preflight` remain only as deprecated compatibility aliases over `pr doctor`.
- `card`, `output`, `cards`, `open`, and `status` are maintenance-oriented compatibility surfaces rather than the preferred workflow entrypoints.
- PRs are created as DRAFT by default to preserve human review.
- Uses "Closes #N" by default so GitHub auto-closes issues when merged.
- run is a bounded v0.85 wrapper over the Rust adl runtime; browser/editor direct invocation remains follow-on work.
- Runs Rust checks in adl/ by default (fmt, clippy -D warnings, test).
- finish stages only the tracked repo-root paths selected by `--paths`; do not pass local `.adl` SIP/STP/SPP/SRP/SOR task-bundle files there. Use `--output-card` for the SOR truth surface; canonical `.adl` issue bundles remain local-only and must not be tracked or force-staged.
- `--allow-gitignore` only permits staged `.gitignore` / `adl/.gitignore` changes during finish publication; it does not widen generic ignored-path staging.
- C-SDLC prompt templates are stored in docs/templates/prompts/1.0.0/ (legacy SIP/SOR fallback: adl/templates/cards/ and .adl/templates/).
- Cards are stored locally under cards_root and are not committed to git.
  cards_root resolves as: ADL_CARDS_ROOT (if set) else <primary-checkout>/.adl/cards.

Examples:
  adl/tools/pr.sh help
  adl/tools/pr.sh create --title "[v0.86][tools] Example issue" --labels "track:roadmap,type:task,area:tools"
  adl/tools/pr.sh init 17 --slug b6-default-system --no-fetch-issue --version v0.85
  adl/tools/pr.sh run 17 --slug b6-default-system --version v0.85
  adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml --trace --allow-unsigned
  adl/tools/pr.sh doctor 17 --slug b6-default-system --version v0.85 --json
  adl/tools/pr.sh preflight 17 --slug b6-default-system --version v0.85
  adl/tools/pr.sh card  17 --help
  adl/tools/pr.sh card  17 --version v0.2
  adl/tools/pr.sh card  17 input
  adl/tools/pr.sh card  17 output
  adl/tools/pr.sh output 17 --version v0.2
  adl/tools/pr.sh output 17 input
  adl/tools/pr.sh output 17 output
  adl/tools/pr.sh cards 17 --version v0.2
  adl/tools/pr.sh finish 17 --title "adl: apply run.defaults.system fallback" -f .adl/cards/17/input_17.md --output-card .adl/cards/17/output_17.md
EOF
}

usage_create() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh create --title "<title>" [--slug <slug>] [--body "<markdown>" | --body-file <path>] [--labels <csv>] [--version <v>]

Notes:
- Creates the GitHub issue and bootstraps the local root STP/SIP/SPP/SRP/SOR bundle.
- Requires a shared GitHub token source for live GitHub operations. Supported
  sources are GITHUB_TOKEN, GH_TOKEN, ADL_GITHUB_TOKEN_FILE, or
  ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE. The keychain source uses macOS
  `security find-generic-password`. If no token is present, stop and fix the ADL
  command environment; do not fall back to direct `gh` commands or connector
  issue APIs.
- Runs the doctor-ready structural check immediately after bootstrap and fails if the new issue is not ready for the next step.
- Does not create the branch or worktree execution context.
- After create, do qualitative SIP/STP/SPP/SRP design-time review and then run `adl/tools/pr.sh run <issue> ...`.
EOF
}

usage_init() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh init <issue> [--slug <slug>] [--title "<title>"] [--no-fetch-issue] [--version <v0.85|v0.87.1>]

Notes:
- Initializes the canonical local task-bundle authoring surface.
- Does not create or reconcile the GitHub issue.
- Emits and validates the root STP/SIP/SOR bundle before returning success.
- Fails if the full root task bundle cannot be created cleanly.
EOF
}

usage_start() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh start <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue] [--version <v>] [--allow-open-pr-wave]

Notes:
- Deprecated compatibility shim. Prefer `adl/tools/pr.sh run <issue> ...`.
- Creates or reuses issue worktree at .worktrees/adl-wp-<issue> by default.
- Leaves the primary checkout on its current branch.
- Must be invoked from the primary checkout; started issue worktrees should continue the bound issue work, not bind nested worktrees.
- `--version` overrides inferred issue version when the caller already knows the intended milestone band.
- Refuses to start a later issue when an open PR already exists in the same milestone queue unless `--allow-open-pr-wave` is passed.
EOF
}

usage_run() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh run <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue] [--version <v>] [--allow-open-pr-wave]
  adl/tools/pr.sh run <adl.yaml> [--trace] [--print-plan] [--print-prompts] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--runs-root <dir>] [--quiet] [--open] [--allow-unsigned]

Notes:
- Issue mode:
  - preferred public binder for execution-time branch and worktree creation
  - run from the primary checkout; started issue worktrees should continue the bound issue work, not bind nested worktrees
  - delegates to the Rust PR control-plane binder
- ADL file mode:
  - bounded v0.85 control-plane wrapper over `cargo run --bin adl -- ...`
  - primary proof surface is the canonical run artifact set under `.adl/runs/<run_id>/`
EOF
}

usage_card() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh card <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [--version <v>] [-f|--file <card.md>]

Notes:
- Default behavior (`card <issue>`) creates the input card if missing, then prints its path.
- Positional `input|output` opens/prints that card path and creates it if missing.
EOF
}

usage_output() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh output <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [--version <v>] [-f|--file <card.md>]

Notes:
- Default behavior (`output <issue>`) creates the output card if missing, then prints its path.
- Positional `input|output` opens/prints that card path and creates it if missing.
EOF
}

usage_cards() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh cards <issue> [--version <v>] [--no-fetch-issue]
EOF
}

usage_ready() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh ready <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--json]

Notes:
- Deprecated compatibility alias over `doctor --mode ready`.
- Reports structural execution-readiness.
- Pre-run issues may pass without a bound worktree when the root bundle is authored and execution has not started yet.
- Started issues still validate the issue worktree and started-worktree cards strictly.
- Prints READY=PASS on success and exits non-zero on the first missing or invalid bootstrap surface.
- `--json` emits the stable `adl.pr.doctor.v1` machine-readable result.
EOF
}

usage_preflight() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh preflight <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--allow-open-pr-wave] [--json]

Notes:
- Deprecated compatibility alias over `doctor --mode preflight`.
- Reports whether unresolved open PRs already exist for the same milestone/version band.
- Prints PREFLIGHT=PASS, PREFLIGHT=WARN, or PREFLIGHT=BLOCK.
- `--json` emits the stable `adl.pr.doctor.v1` machine-readable result.
EOF
}

usage_doctor() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh doctor <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--mode full|ready|preflight] [--allow-open-pr-wave] [--json]

Notes:
- Canonical readiness and drift diagnostic surface.
- `--mode full` reports milestone-wave preflight plus lifecycle-aware readiness.
- `--mode ready` reports structural execution-readiness for both pre-run and run-bound issues.
- Pre-run issues may be reported as ready without a worktree when execution has not yet been bound.
- Run-bound issues still validate the bound worktree and cards strictly.
- `--mode preflight` runs only the milestone-wave conflict/open-PR check.
- `--json` emits the stable `adl.pr.doctor.v1` machine-readable result for automation.
EOF
}

usage_finish() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh finish <issue> --title "<title>" [--paths "<p1,p2,...>"] [-f|--file <input_card.md>] [--output-card <output_card.md>] [--no-checks] [--no-open] [--merge]

Notes:
- By default, finish stages repo-root changes (`.`), which keeps docs and code changes together unless you narrow with `--paths`.
- `--paths` is for tracked repo publication inputs only. Do not include local `.adl` SIP/STP/SPP/SRP/SOR task-bundle files; pass the SOR through `--output-card`.
EOF
}

usage_validation() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh validation <pr-number-or-url> [-R owner/repo] [--watch] [--json]

Behavior:
- delegates to the Rust-owned PR validation watcher
- reads PR check status through the typed GitHub transport
- infers `owner/repo` from a GitHub PR URL when `-R/--repo` is omitted; otherwise falls back to the current checkout repo
- emits tail-friendly pr.validation.wait events when --watch is set
- prints a JSON status report when --json is set
- returns non-zero when validation is pending, failed, cancelled, or timed out
EOF
}

usage_watch() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh watch <issue-number-or-url> [--slug <slug>] [--version <v>] [-R owner/repo] [--json]

Behavior:
- delegates to the Rust-owned tracked-issue lifecycle watcher
- classifies one tracked issue into ready-for-run, PR-open, checks-running, checks-failed, checks-green, closeout-needed, or blocked-adjacent states
- uses typed GitHub transport plus local doctor readiness; does not fall back to raw `gh`
- keeps the JSON report compact enough to feed a future local watcher agent while ADL remains the authoritative classifier
- emits explicit authority metadata so local watcher agents stay advisory-only
- emits a JSON watcher report when --json is set
EOF
}

usage_issue() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh issue list [-R owner/repo] [--state open|closed|all] [--limit <n>] [--json]
  adl/tools/pr.sh issue search --query "<text>" [-R owner/repo] [--state open|closed|all] [--limit <n>] [--json]
  adl/tools/pr.sh issue view <issue-number-or-url> [-R owner/repo] [--json]
  adl/tools/pr.sh issue create --title "<title>" [--body "<markdown>" | --body-file <path>] [--label <label>]... [--labels <csv>] [-R owner/repo] [--json]
  adl/tools/pr.sh issue comment <issue-number-or-url> [--body "<markdown>" | --body-file <path>] [-R owner/repo] [--json]
  adl/tools/pr.sh issue edit <issue-number-or-url> [--title "<title>"] [--body "<markdown>" | --body-file <path>] [--label <label>]... [--labels <csv>] [-R owner/repo] [--json]
  adl/tools/pr.sh issue close <issue-number-or-url> [--reason completed|not_planned] [-R owner/repo] [--json]

Behavior:
- delegates to the Rust-owned issue inspection and mutation surface
- uses the typed GitHub transport rather than raw `gh issue` commands for covered paths
- requires a shared GitHub token source for live GitHub operations. Supported
  sources are GITHUB_TOKEN, GH_TOKEN, ADL_GITHUB_TOKEN_FILE, or
  ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE. The keychain source uses macOS
  `security find-generic-password`; if no token is present, stop and fix the ADL
  command environment without echoing it; do not fall back to direct `gh`
  commands or connector issue APIs
- defaults `-R/--repo` from the current checkout when omitted
- infers `owner/repo` from a GitHub issue URL on `issue view` or `issue close` when possible
- keeps machine-readable JSON on stdout when `--json` is set
EOF
}

usage_repair_issue_body() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh repair-issue-body <issue> [--slug <slug>] [--title "<title>"] [--body "<markdown>" | --body-file <path>] [--labels <csv>] [--version <v>] [--force]

Behavior:
- validates an authored issue body against the C-SDLC source-prompt contract
- updates the GitHub issue body through the Rust/Octocrab-backed PR command
- rewrites the canonical local source prompt and regenerates the root task bundle
- refuses to overwrite an authored local source prompt unless --force is supplied
EOF
}

usage_closeout() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh closeout <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue]

Behavior:
- verifies the issue is already CLOSED/COMPLETED
- reconciles the canonical task bundle and closed-output truth
- prunes the matching issue worktree when it is safe to do so
EOF
}
