# V0.91.6 Stdout/Stderr Observability Boundary Proof for #4520

Issue: `#4520`  
Title: `[v0.91.6][tools][observability] Preserve machine-readable stdout while moving compatibility observability fully to stderr/log sinks`

## Scope

This proof packet is intentionally bounded to the workflow/tooling command
surfaces named by `#4520`:

- `adl/tools/pr.sh`
- `adl/tools/pr_delegate.sh`
- the Rust-owned `adl pr ...` / `adl issue ...` delegated JSON paths those
  wrappers expose

It does not claim a repo-wide observability redesign. It only classifies the
JSON-facing workflow surfaces that matter to the current control-plane contract.

## Command Inventory

| Surface | Current classification | Evidence |
| --- | --- | --- |
| `adl/tools/pr.sh ready ... --json` | already safe | `adl/tools/test_pr_json_observability.sh` proves JSON stdout stays parse-safe while `adl_event` remains on stderr |
| `adl/tools/pr.sh preflight ... --json` | already safe | `adl/tools/test_pr_json_observability.sh` |
| `adl/tools/pr.sh doctor ... --json` | already safe | `adl/tools/test_pr_json_observability.sh` |
| `adl/tools/pr.sh validation ... --json` | fixed/proven in `#4520` | expanded `adl/tools/test_pr_json_observability.sh` now enforces the same stdout/stderr split for the validation delegate path |
| `adl/tools/pr.sh issue search ... --json` | fixed/proven in `#4520` | expanded `adl/tools/test_pr_json_observability.sh` |
| `adl/tools/pr.sh issue view ... --json` | fixed/proven in `#4520` | expanded `adl/tools/test_pr_json_observability.sh` |
| `adl/tools/pr.sh projection-map --json` | fixed/proven in `#4520` | expanded `adl/tools/test_pr_json_observability.sh` |

## Out-of-Scope / Deferred

- Human-oriented non-JSON surfaces such as `adl/tools/pr.sh status`, issue-mode
  `run`, and help/usage text are not treated as machine-readable stdout
  contracts in this issue.
- Broader direct CLI families like `adl agent ... --json`, `adl session ...
  --json`, and `adl process status --json` are outside the repo-input scope of
  `#4520` and remain separate proof surfaces.

## Validation Run

Command:

```bash
bash adl/tools/test_pr_json_observability.sh
```

What it proves:

- the delegated JSON payload remains on stdout and parses successfully
- `adl_event` compatibility observability does not appear on stdout
- `adl_event` remains available on stderr for the same invocation
- the wrapper preserves exact delegated argv for:
  - `ready`
  - `preflight`
  - `doctor`
  - `validation`
  - `issue search`
  - `issue view`
  - `projection-map`

## Live Command Spot Checks

The retained fixture proof above was also spot-checked against live repo
commands with stdout/stderr split into separate files and stdout parsed as JSON:

- `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token bash adl/tools/pr.sh validation 4525 --json`
- `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token bash adl/tools/pr.sh issue search --query "validation manager" --state open --json`
- `bash adl/tools/pr.sh projection-map --json`

Those live checks confirmed the same contract on the current repo surfaces:

- stdout remained parse-safe JSON
- stderr retained `adl_event` observability lines
- no `adl_event` compatibility output leaked onto stdout

## Result

`#4520` closes the `#4434` audit gap at the workflow-wrapper level by turning
the stdout/stderr boundary from a partially retained claim into an enforced
proof surface for the currently in-scope JSON-facing `pr.sh` commands.
