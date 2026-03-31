#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

matrix="docs/milestones/v0.86/DEMO_MATRIX_v0.86.md"
readme="docs/milestones/v0.86/README.md"
local_demos="docs/milestones/v0.86/features/LOCAL_AGENT_DEMOS.md"

grep -F '| D1 | Canonical Bounded Cognitive Path |' "$matrix" | grep -F 'READY' >/dev/null
grep -F '| D2 | Fast vs Slow Routing |' "$matrix" | grep -F 'READY' >/dev/null
grep -F '| D3 | Agency / Candidate Selection |' "$matrix" | grep -F 'READY' >/dev/null
grep -F '| D4 | Freedom Gate Enforcement |' "$matrix" | grep -F 'READY' >/dev/null
grep -F '| D5 | Full Review Surface Walkthrough |' "$matrix" | grep -F 'READY' >/dev/null

grep -F './adl/tools/demo_v086_control_path.sh' "$matrix" >/dev/null
grep -F './adl/tools/demo_v086_fast_slow.sh' "$matrix" >/dev/null
grep -F './adl/tools/demo_v086_candidate_selection.sh' "$matrix" >/dev/null
grep -F './adl/tools/demo_v086_freedom_gate.sh' "$matrix" >/dev/null
grep -F './adl/tools/demo_v086_review_surface.sh' "$matrix" >/dev/null

grep -F 'artifacts/v086/control_path/demo-g-v086-control-path/summary.txt' "$matrix" >/dev/null
grep -F 'artifacts/v086/review_surface/demo_manifest.json' "$matrix" >/dev/null
grep -F 'artifacts/v086/review_surface/d1_control_path/summary.txt' "$matrix" >/dev/null

grep -F 'D5' "$readme" >/dev/null
grep -F 'artifacts/v086/review_surface/demo_manifest.json' "$readme" >/dev/null

grep -F './adl/tools/demo_v086_review_surface.sh' "$local_demos" >/dev/null
grep -F 'artifacts/v086/review_surface/d1_control_path/summary.txt' "$local_demos" >/dev/null

echo "V086_DEMO_REVIEW_SURFACE=PASS"
