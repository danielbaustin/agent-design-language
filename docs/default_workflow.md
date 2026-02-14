# Default Workflow (adl_pr_cycle + pr.sh)

This is the default contributor path for ADL development:

`preflight -> start -> codex -> finish -> report`

## 1) Start Issue Branch + Cards

```bash
./swarm/tools/pr.sh start <issue_num> --slug <slug>
```

Canonical cards:
- `.adl/cards/<issue_num>/input_<issue_num>.md`
- `.adl/cards/<issue_num>/output_<issue_num>.md`

## 2) Implement and Validate

Typical local preflight:

```bash
./swarm/tools/batched_checks.sh
```

## 3) Finish PR

```bash
./swarm/tools/pr.sh finish <issue_num> \
  --title "<title>" \
  --paths "<comma-separated paths>" \
  -f .adl/cards/<issue_num>/input_<issue_num>.md \
  --output .adl/cards/<issue_num>/output_<issue_num>.md
```

## Common Pitfalls and Remediations

- Dirty worktree at `start`:
  - Commit/stash first, then re-run `pr.sh start`.
- Wrong paths at `finish`:
  - Ensure `--paths` only includes intended repo paths; do not include `.adl/cards`.
- Missing card files:
  - Re-run `pr.sh start <issue_num> --slug <slug>` to seed canonical card paths.
- Worktree branch base problems:
  - Update from `origin/main`, then re-run `start`.
