#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT_DIR"
python3 adl/tools/generate_tool_surface_registry.py
python3 adl/tools/generate_tool_surface_registry.py --check
git ls-files --error-unmatch docs/milestones/v0.91.5/TOOL_SURFACE_REGISTRY_3734.md >/dev/null
git diff --exit-code -- docs/milestones/v0.91.5/TOOL_SURFACE_REGISTRY_3734.md >/dev/null

echo "tool surface registry generation/check: ok"
