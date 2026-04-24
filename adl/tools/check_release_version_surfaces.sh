#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

manifest_version="$(
  python3 - <<'PY'
import pathlib
import tomllib

manifest = pathlib.Path("adl/Cargo.toml")
data = tomllib.loads(manifest.read_text())
print(data["package"]["version"])
PY
)"

lock_version="$(
  python3 - <<'PY'
import pathlib
import tomllib

lock = pathlib.Path("adl/Cargo.lock")
data = tomllib.loads(lock.read_text())
for package in data.get("package", []):
    if package.get("name") == "adl":
        print(package["version"])
        break
else:
    raise SystemExit("adl package entry missing from Cargo.lock")
PY
)"

if [ "$manifest_version" != "$lock_version" ]; then
  echo "release-version check: adl/Cargo.toml version $manifest_version does not match adl/Cargo.lock package version $lock_version" >&2
  exit 1
fi

resolved_version="$(
  cargo metadata --manifest-path adl/Cargo.toml --no-deps --format-version 1 \
    | python3 -c 'import json, sys
data = json.load(sys.stdin)
for package in data["packages"]:
    if package["name"] == "adl":
        print(package["version"])
        break
else:
    raise SystemExit("adl package missing from cargo metadata")'
)"

if [ "$resolved_version" != "$manifest_version" ]; then
  echo "release-version check: cargo metadata resolved $resolved_version but manifest says $manifest_version" >&2
  exit 1
fi

cli_version="$(cargo run -q --manifest-path adl/Cargo.toml -- --version)"
if [ "$cli_version" != "$manifest_version" ]; then
  echo "release-version check: CLI reports $cli_version but manifest says $manifest_version" >&2
  exit 1
fi

echo "PASS release-version-surfaces manifest=$manifest_version lock=$lock_version cli=$cli_version"
