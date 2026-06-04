#!/usr/bin/env bash
set -euo pipefail

ROOT="adl/src"
TOP=25
FORMAT="table"

usage() {
  cat <<'EOF'
Usage: adl/tools/report_module_navigability.sh [options]

Report Rust module navigability signals without failing the build.

Options:
  --root <path>         Rust source root to scan (default: adl/src)
  --top <n>             Number of hotspot rows to print (default: 25)
  --format <table|tsv>  Output format (default: table)
  -h, --help            Show this help

The report is a review aid for architecture and quality-gate work. It measures
file size and directory clusters; it does not decide whether a refactor is safe.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      ROOT="$2"
      shift 2
      ;;
    --top)
      TOP="$2"
      shift 2
      ;;
    --format)
      FORMAT="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ ! -d "$ROOT" ]]; then
  echo "Scan root does not exist: $ROOT" >&2
  exit 2
fi

case "$FORMAT" in
  table|tsv) ;;
  *)
    echo "Unsupported format: $FORMAT" >&2
    exit 2
    ;;
esac

python3 - "$ROOT" "$TOP" "$FORMAT" <<'PY'
import sys
from pathlib import Path

root = Path(sys.argv[1])
top = int(sys.argv[2])
fmt = sys.argv[3]

files = []
for path in sorted(root.rglob("*.rs")):
    with path.open("r", encoding="utf-8") as handle:
        loc = sum(1 for _ in handle)
    files.append((path.as_posix(), loc))

total_loc = sum(loc for _, loc in files)
total_files = len(files)
top_files = sorted(files, key=lambda row: (-row[1], row[0]))[:top]

clusters = {}
for path_string, loc in files:
    path = Path(path_string)
    parent = path.parent
    if parent == root:
        cluster = root.as_posix()
    else:
        relative_parts = parent.relative_to(root).parts
        cluster = (root / relative_parts[0]).as_posix()
    current = clusters.setdefault(cluster, {"files": 0, "loc": 0})
    current["files"] += 1
    current["loc"] += loc

top_clusters = sorted(
    ((cluster, data["files"], data["loc"]) for cluster, data in clusters.items()),
    key=lambda row: (-row[2], -row[1], row[0]),
)[:top]

if fmt == "tsv":
    print("schema\tadl.module_navigability_report.v1")
    print(f"root\t{root.as_posix()}")
    print(f"total_rust_files\t{total_files}")
    print(f"total_rust_loc\t{total_loc}")
    print("section\tpath\tloc")
    for path, loc in top_files:
        print(f"top_file\t{path}\t{loc}")
    print("section\tpath\tfiles\tloc")
    for cluster, count, loc in top_clusters:
        print(f"top_cluster\t{cluster}\t{count}\t{loc}")
    sys.exit(0)

print("Rust module navigability report")
print("schema: adl.module_navigability_report.v1")
print(f"root: {root.as_posix()}")
print(f"total_rust_files: {total_files}")
print(f"total_rust_loc: {total_loc}")
print("")

print("Top file hotspots")
path_width = max([len("Path"), *(len(path) for path, _ in top_files)] or [len("Path")])
loc_width = max([len("LoC"), *(len(str(loc)) for _, loc in top_files)] or [len("LoC")])
print(f"{'Path':<{path_width}}  {'LoC':>{loc_width}}")
print(f"{'-' * path_width}  {'-' * loc_width}")
for path, loc in top_files:
    print(f"{path:<{path_width}}  {loc:>{loc_width}}")

print("")
print("Top directory clusters")
cluster_width = max([len("Path"), *(len(cluster) for cluster, _, _ in top_clusters)] or [len("Path")])
files_width = max([len("Files"), *(len(str(count)) for _, count, _ in top_clusters)] or [len("Files")])
cluster_loc_width = max([len("LoC"), *(len(str(loc)) for _, _, loc in top_clusters)] or [len("LoC")])
print(f"{'Path':<{cluster_width}}  {'Files':>{files_width}}  {'LoC':>{cluster_loc_width}}")
print(f"{'-' * cluster_width}  {'-' * files_width}  {'-' * cluster_loc_width}")
for cluster, count, loc in top_clusters:
    print(f"{cluster:<{cluster_width}}  {count:>{files_width}}  {loc:>{cluster_loc_width}}")
PY
