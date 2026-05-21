#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT=""

usage() {
  cat <<'EOF'
Usage:
  bash adl/tools/demo_v0912_speculative_decoding_showcase.sh [--out <report.json>]

Runs the bounded WP-11 speculative decoding demo, regenerates the report if
requested, and prints a concise operator-facing showcase summary.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out) OUT="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 1 ;;
  esac
done

cmd=(cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin demo_v0912_speculative_decoding_prototype)
if [[ -n "$OUT" ]]; then
  cmd+=("$OUT")
fi

report_path="$("${cmd[@]}")"

python3 - "$report_path" <<'PY'
import json
import sys
from pathlib import Path

report_path = Path(sys.argv[1])
report = json.loads(report_path.read_text())
scenarios = report["scenarios"]

print("WP-11 speculative decoding showcase")
print(f"report: {report_path}")
print(f"worthwhile_for_adl: {str(report['worthwhile_for_adl']).lower()}")
print(f"recommendation: {report['recommendation']}")
print("scenario_summary:")
for scenario in scenarios:
    print(
        "  - "
        f"{scenario['scenario_id']}: status={scenario['status']}, "
        f"worthwhile={str(scenario['worthwhile']).lower()}, "
        f"acceptance_rate={scenario['acceptance_rate']:.2f}, "
        f"speedup_tps_ratio={scenario['speedup_tps_ratio']:.2f}"
    )

best = max(scenarios, key=lambda item: item["speedup_tps_ratio"])
worst = min(scenarios, key=lambda item: item["speedup_tps_ratio"])
print(
    "highlights:"
    f" best={best['scenario_id']}({best['speedup_tps_ratio']:.2f}x),"
    f" weakest={worst['scenario_id']}({worst['speedup_tps_ratio']:.2f}x)"
)
PY
