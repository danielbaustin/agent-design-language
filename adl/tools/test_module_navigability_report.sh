#!/usr/bin/env bash
set -euo pipefail

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

REPORT="$TMPDIR/report.tsv"

bash adl/tools/report_module_navigability.sh --top 5 --format tsv >"$REPORT"

grep -F $'schema\tadl.module_navigability_report.v1' "$REPORT" >/dev/null
grep -F $'root\tadl/src' "$REPORT" >/dev/null
grep -F $'total_rust_files\t' "$REPORT" >/dev/null
grep -F $'total_rust_loc\t' "$REPORT" >/dev/null
grep -F $'top_file\t' "$REPORT" >/dev/null
grep -F $'top_cluster\t' "$REPORT" >/dev/null

python3 - "$REPORT" <<'PY'
import sys
from pathlib import Path

lines = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
totals = dict(line.split("\t", 1) for line in lines if line.startswith("total_"))

files = int(totals["total_rust_files"])
loc = int(totals["total_rust_loc"])
if files <= 0:
    raise SystemExit("expected at least one Rust file")
if loc <= files:
    raise SystemExit("expected total Rust LoC to exceed file count")

top_files = [line for line in lines if line.startswith("top_file\t")]
top_clusters = [line for line in lines if line.startswith("top_cluster\t")]
if not top_files:
    raise SystemExit("expected top file rows")
if not top_clusters:
    raise SystemExit("expected top cluster rows")
PY
