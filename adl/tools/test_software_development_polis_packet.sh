#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
packet_dir="$repo_root/docs/milestones/v0.91.4/review/software_development_polis"

python3 "$repo_root/adl/tools/validate_software_development_polis_packet.py" "$packet_dir"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cp -R "$packet_dir" "$tmpdir/packet"

python3 - <<'PY' "$tmpdir/packet/fixtures/actor_standing_allowed.json"
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
for actor in data["actors"]:
    if actor["role"] == "implementation_owner":
        actor["authorities"].append("merge_approval")
path.write_text(json.dumps(data, indent=2) + "\n")
PY

if python3 "$repo_root/adl/tools/validate_software_development_polis_packet.py" "$tmpdir/packet" >/dev/null 2>&1; then
  echo "expected unauthorized merge authority mutation to fail closed" >&2
  exit 1
fi

rm -rf "$tmpdir/packet"
cp -R "$packet_dir" "$tmpdir/packet"

python3 - <<'PY' "$tmpdir/packet/fixtures/shard_ownership_allowed.json"
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["shards"][1]["writable_paths"] = [
    "docs/milestones/v0.91.4/features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md"
]
path.write_text(json.dumps(data, indent=2) + "\n")
PY

if python3 "$repo_root/adl/tools/validate_software_development_polis_packet.py" "$tmpdir/packet" >/dev/null 2>&1; then
  echo "expected overlapping shard write mutation to fail closed" >&2
  exit 1
fi

echo "PASS: Software Development Polis packet contract checks passed"
