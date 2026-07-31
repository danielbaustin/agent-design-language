#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

required_surfaces=(
  "AGENTS.md"
  "docs/default_workflow.md"
  "adl/tools/codex_pr.sh"
  "adl/tools/codexw.sh"
  "adl/tools/generate_active_command_reference_scan.py"
  "adl/tools/generate_tool_surface_registry.py"
  "docs/tooling/editor/command_adapter.md"
  "docs/tooling/editor/current_skill_wiring_demo.md"
)

forbidden_guidance_surfaces=(
  "AGENTS.md"
  "docs/default_workflow.md"
  "adl/tools/codex_pr.sh"
  "adl/tools/codexw.sh"
  "docs/tooling/editor/command_adapter.md"
  "docs/tooling/editor/current_skill_wiring_demo.md"
)

forbidden_patterns=(
  "adl/tools/pr.sh run <issue>"
  "./adl/tools/pr.sh run"
  "bash adl/tools/pr.sh run"
  "adl/tools/pr.sh run"
  "adl-csdlc issue run"
  "adl pr run"
)

for pattern in "${forbidden_patterns[@]}"; do
  if rg -n --fixed-strings "$pattern" "${forbidden_guidance_surfaces[@]}"; then
    echo "FAIL: live command guidance still teaches deprecated command: $pattern" >&2
    exit 1
  fi
done

required_patterns=(
  "csdlc-install resolve"
  "csdlc-bind"
  "csdlc-review"
)

for pattern in "${required_patterns[@]}"; do
  if ! rg -n --fixed-strings "$pattern" "${required_surfaces[@]}" >/dev/null; then
    echo "FAIL: live command guidance is missing required command: $pattern" >&2
    exit 1
  fi
done

echo "PASS: live CLI owner command guidance is aligned with final typed C-SDLC v2 authority"
