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
run_check adl/tools/test_pr_finish_delegates_to_rust.sh
run_check adl/tools/test_pr_start_template_validation.sh
run_check adl/tools/test_install_adl_pr_cycle_skill.sh
run_check adl/tools/test_pr_run.sh
run_check adl/tools/test_pr_finish_default_stage_root.sh
run_check adl/tools/test_pr_finish_relative_card_paths.sh
run_check adl/tools/test_editor_action.sh
run_check adl/tools/test_demo_five_command_editing.sh
run_check adl/tools/test_five_command_editor_truth.sh

echo "authoring regression suite: ok"
