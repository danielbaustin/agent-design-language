#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT_DIR"

bash adl/tools/demo_v087_trace_truth.sh
bash adl/tools/demo_v087_provider_portability.sh
bash adl/tools/demo_v087_shared_obsmem.sh
bash adl/tools/demo_v087_operational_skills.sh
bash adl/tools/demo_v087_control_plane.sh
bash adl/tools/demo_v087_reviewer_package.sh

echo "v0.87 demo suite complete: artifacts/v087/"

