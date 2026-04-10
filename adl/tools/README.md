# adl/tools

Utility scripts for ADL development workflows, PR hygiene, burst execution, and report maintenance.
This directory is tooling-focused: command wrappers, safety checks, and operator helpers.
Keep behavioral and milestone narrative in canonical docs, not here.

## What Is Here

- `pr.sh`: canonical issue init/ready/run/finish workflow helper.
- `demo_v0871_operator_surface.sh`: canonical `v0.87.1` operator-surface proof wrapper for runtime bring-up and artifact inspection.
- `demo_v0871_runtime_state.sh`: canonical `v0.87.1` runtime-state proof wrapper for paused-vs-completed persistence inspection.
- `demo_v0871_suite.sh`: canonical `v0.87.1` WP-13 demo-suite entrypoint for the implemented provider, operator, runtime-state, review-surface, and multi-agent proof surfaces.
- `normalize_adl_cards.sh`: repairs `.adl/cards/<issue>/` compatibility links and materializes missing bootstrap `sor.md` files for existing task bundles.
- `demo_v0871_multi_agent_discussion.sh`: canonical `v0.87.1` bounded multi-agent discussion proof wrapper for a five-turn Claude + ChatGPT runtime demo.
- `demo_v0871_real_multi_agent_discussion.sh`: live-provider companion demo that calls real OpenAI and Anthropic APIs through a local ADL completion adapter using operator-managed keys; a missing-credentials skip in the paired test is a non-proving disposition, not live-provider proof.
- `real_multi_agent_provider_adapter.py`: local adapter that translates ADL's bounded HTTP completion contract to vendor-native OpenAI and Anthropic calls without recording secret material.
- `validate_multi_agent_transcript.py`: validates the bounded multi-agent transcript artifact contract for the D13 discussion demo.
- `worktree_doctor.sh`, `worktree_prune.sh`: deterministic worktree governance and safe cleanup helpers.
- `archive_run_artifacts.sh`: dry-run/apply helper that inventories local run roots, copies unique run artifacts into `.adl/trace-archive/milestones/<milestone>/runs/`, and can move archived active `.adl/runs` entries into `.adl/trace-archive/source-roots/`.
- `adl tooling ...`: Rust-owned tooling surface for prompt/card/review validation helpers, with legacy wrapper scripts preserved at the historical `adl/tools/*` paths.
- `burst_worktree.sh`, `burst_continue.sh`: burst lane/worktree helpers.
- `batched_checks.sh`, `preflight_review.sh`: quality/preflight checks.
- `enforce_coverage_gates.sh`: deterministic coverage threshold enforcement (workspace + per-file).
- `report_large_rust_modules.sh`: non-blocking Rust implementation-module size report for maintainability review.
- `open_artifact.sh`: convenience opener for cards/reports.
- `update_reports_index.sh`, `update_latest_reports.sh`: report index maintenance.

## Canonical Workflow Commands

From repo root:

```bash
# install or resync the canonical operational skill bundles into $CODEX_HOME/skills (default: copy mode)
bash adl/tools/install_adl_operational_skills.sh

# install the same bundles as symlinks so Codex reads the tracked repo copies directly
ADL_OPERATIONAL_SKILLS_INSTALL_MODE=symlink bash adl/tools/install_adl_operational_skills.sh

# install or resync the legacy adl_pr_cycle compatibility skill from the tracked contract
bash adl/tools/install_adl_pr_cycle_skill.sh

# normalize compatibility card links for existing task bundles
bash adl/tools/normalize_adl_cards.sh --root "$(pwd)" --version v0.87.1

# run the bounded Claude + ChatGPT discussion demo through the real runtime
bash adl/tools/demo_v0871_multi_agent_discussion.sh

# run the live-provider Claude + ChatGPT discussion demo using env vars or explicit key-file overrides
bash adl/tools/demo_v0871_real_multi_agent_discussion.sh

# validate the generated bounded multi-agent transcript artifact
python3 adl/tools/validate_multi_agent_transcript.py artifacts/v0871/multi_agent_discussion/transcript.md

# run the current v0.87.1 milestone demo-suite proof package
bash adl/tools/demo_v0871_suite.sh

# bootstrap the local root task bundle for an existing issue
bash ./adl/tools/pr.sh init <issue_num> --slug <slug>

# inspect readiness and workflow drift through the canonical doctor surface
bash ./adl/tools/pr.sh doctor <issue_num> --slug <slug> --mode full --json

# bind execution context at the last responsible moment
bash ./adl/tools/pr.sh run <issue_num> --slug <slug>

# inspect worktree status/fate across managed, stale, orphan, and Codex-ephemeral namespaces
./adl/tools/worktree_doctor.sh

# dry-run safe cleanup of merged clean worktrees + stale registrations
./adl/tools/worktree_prune.sh

# inventory local run artifacts without deleting or moving the source data
./adl/tools/archive_run_artifacts.sh --include-worktrees

# copy unique run artifacts into .adl/trace-archive/milestones/
./adl/tools/archive_run_artifacts.sh --include-worktrees --apply

# copy unique run artifacts, then clear active .adl/runs by preserving source dirs under .adl/trace-archive/source-roots/
./adl/tools/archive_run_artifacts.sh --include-worktrees --apply --prune-active-runs

# run standard checks
./adl/tools/batched_checks.sh

# enforce coverage thresholds from coverage-summary.json
cd ./adl/ && bash tools/enforce_coverage_gates.sh coverage-summary.json

# report large Rust implementation modules without failing the build
./adl/tools/report_large_rust_modules.sh

# generate deterministic execution prompt from an input card
adl tooling card-prompt --issue <issue_num> --out /tmp/prompt.txt

# finish issue and open/update PR
bash ./adl/tools/pr.sh finish <issue_num> --title "<title>" --paths "<paths>"
```

## Compatibility / Maintenance Surfaces

The repo still carries a few compatibility and maintenance entrypoints, but they
are not the preferred public workflow:

- `pr ready` and `pr preflight` remain deprecated aliases over `pr doctor`
- `pr start` remains a narrow compatibility alias over the same Rust-backed
  execution-context binding path as `pr run`
- `pr card`, `pr output`, `pr cards`, `pr open`, and `pr status` remain
  maintenance-oriented helpers rather than the taught workflow surface

## See Also / Canonical Docs

- Root project entrypoint: `../../README.md`
- Runtime/CLI usage: `../README.md`
- Operational skills guide: `skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
  - includes concrete usage patterns for `pr-init`, `pr-ready`, `pr-run`,
    `pr-janitor`, `pr-finish`, `pr-closeout`, and the helper card editors
  - when touching `stp.md`, `sip.md`, or `sor.md`, use the matching editor
    skill explicitly rather than doing ad hoc card edits
- Active milestone docs: `../../docs/milestones/v0.87/`
- Documentation index: `../../docs/README.md`
- ADRs: `../../docs/adr/`
- ADL spec entrypoint: `../../adl-spec/README.md`

## Retired In v0.87 Cleanup

The following low-confidence legacy residue was retired during the bounded `adl/tools`
cleanup because it no longer backed the live PR control plane, active demos, or
current regression-test surfaces:

- `BURST_PLAYBOOK.md`
- `REPORT_SCHEMA.md`
- `default.rules.profiles.example`
- `demo_v0_4.sh`
- `pr_smoke.sh`
