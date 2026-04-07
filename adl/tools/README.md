# adl/tools

Utility scripts for ADL development workflows, PR hygiene, burst execution, and report maintenance.
This directory is tooling-focused: command wrappers, safety checks, and operator helpers.
Keep behavioral and milestone narrative in canonical docs, not here.

## What Is Here

- `pr.sh`: canonical issue init/ready/run/finish workflow helper.
- `worktree_doctor.sh`, `worktree_prune.sh`: deterministic worktree governance and safe cleanup helpers.
- `card_prompt.sh`: deterministic input-card parser that generates execution prompts.
- `burst_worktree.sh`, `burst_continue.sh`: burst lane/worktree helpers.
- `batched_checks.sh`, `preflight_review.sh`: quality/preflight checks.
- `enforce_coverage_gates.sh`: deterministic coverage threshold enforcement (workspace + per-file).
- `report_large_rust_modules.sh`: non-blocking Rust implementation-module size report for maintainability review.
- `open_artifact.sh`: convenience opener for cards/reports.
- `update_reports_index.sh`, `update_latest_reports.sh`: report index maintenance.

## Common Commands

From repo root:

```bash
# install or resync the canonical operational skill bundles into $CODEX_HOME/skills (default: copy mode)
bash adl/tools/install_adl_operational_skills.sh

# install the same bundles as symlinks so Codex reads the tracked repo copies directly
ADL_OPERATIONAL_SKILLS_INSTALL_MODE=symlink bash adl/tools/install_adl_operational_skills.sh

# install or resync the legacy adl_pr_cycle compatibility skill from the tracked contract
bash adl/tools/install_adl_pr_cycle_skill.sh

# bootstrap the local root task bundle for an existing issue
bash ./adl/tools/pr.sh init <issue_num> --slug <slug>

# inspect structural readiness separately from current preflight gates
bash ./adl/tools/pr.sh ready <issue_num> --slug <slug>

# bind execution context at the last responsible moment
bash ./adl/tools/pr.sh run <issue_num> --slug <slug>

# inspect worktree status/fate across managed, stale, orphan, and Codex-ephemeral namespaces
./adl/tools/worktree_doctor.sh

# dry-run safe cleanup of merged clean worktrees + stale registrations
./adl/tools/worktree_prune.sh

# run standard checks
./adl/tools/batched_checks.sh

# enforce coverage thresholds from coverage-summary.json
cd ./adl/ && bash tools/enforce_coverage_gates.sh coverage-summary.json

# report large Rust implementation modules without failing the build
./adl/tools/report_large_rust_modules.sh

# generate deterministic execution prompt from an input card
./adl/tools/card_prompt.sh --issue <issue_num> --out /tmp/prompt.txt

# finish issue and open/update PR
bash ./adl/tools/pr.sh finish <issue_num> --title "<title>" --paths "<paths>"
```

## See Also / Canonical Docs

- Root project entrypoint: `../../README.md`
- Runtime/CLI usage: `../README.md`
- Active milestone docs: `../../docs/milestones/v0.87/`
- Documentation index: `../../docs/README.md`
- ADRs: `../../docs/adr/`
- ADL spec entrypoint: `../../adl-spec/README.md`
