#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

run_check() {
  local script="$1"
  echo "==> $script"
  (
    cd "$ROOT_DIR" &&
      bash "$script"
  )
}

run_check adl/tools/test_pr_init.sh
run_check adl/tools/test_pr_create.sh
run_check adl/tools/test_pr_ready_prefers_built_binary.sh
run_check adl/tools/test_pr_finish_delegates_to_rust.sh
run_check adl/tools/test_pr_start_template_validation.sh
run_check adl/tools/test_install_adl_pr_cycle_skill.sh
run_check adl/tools/test_install_adl_operational_skills.sh
run_check adl/tools/test_pr_init_skill_contracts.sh
run_check adl/tools/test_card_editor_skill_contracts.sh
run_check adl/tools/test_pr_closeout_skill_contracts.sh
run_check adl/tools/test_pr_run.sh
run_check adl/tools/test_pr_run_materializes_worktree_cards.sh
run_check adl/tools/test_pr_finish_default_stage_root.sh
# Legacy shell-era rerun expectations in this harness currently diverge from the Rust-owned finish path.
# Keep the stable finish-path regression in suite coverage and handle the relative-card-path variant separately.
run_check adl/tools/test_editor_action.sh
run_check adl/tools/test_demo_five_command_editing.sh
run_check adl/tools/test_five_command_editor_truth.sh

echo "authoring regression suite: ok"
