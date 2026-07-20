#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_PATH="${1:-$ROOT_DIR/docs/milestones/v0.91.7/review/wp20_supply_chain/CSDLC_V2_SUPPLY_CHAIN_PROOF_5546.json}"
mkdir -p "$(dirname "$OUT_PATH")"

python3 - "$ROOT_DIR" "$OUT_PATH" <<'PY'
import hashlib
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1])
out = Path(sys.argv[2])
csdlc = root / "csdlc-v2"
manifest = csdlc / "Cargo.toml"
lockfile = csdlc / "Cargo.lock"

def command_status(argv):
    try:
        result = subprocess.run(argv, cwd=root, stdout=subprocess.DEVNULL,
                                stderr=subprocess.PIPE, text=True, timeout=180)
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"status": "unavailable", "reason": str(exc)}
    return {"status": "passed" if result.returncode == 0 else "failed",
            "exit_code": result.returncode}

results = {
    "schema": "adl.csdlc_v2.release_supply_chain_proof.v1",
    "issue": 5546,
    "scope": "csdlc-v2",
    "network_access": "not_requested",
    "locked_dependency_metadata": command_status([
        "cargo", "metadata", "--locked", "--format-version", "1",
        "--manifest-path", str(manifest),
    ]),
    "lockfile": {
        "status": "passed" if lockfile.is_file() else "failed",
        "path": "csdlc-v2/Cargo.lock",
        "sha256": hashlib.sha256(lockfile.read_bytes()).hexdigest() if lockfile.is_file() else None,
    },
    "msrv": {
        "declared": re.search(r'^rust-version\s*=\s*"([^"]+)"', manifest.read_text(), re.MULTILINE).group(1),
        "proof": {"status": "unavailable", "reason": "Rust 1.85 is not installed unless rustup reports it"},
    },
    "advisories": {"status": "unavailable", "reason": "cargo-audit/cargo-deny database checks are not run without the approved tool and database"},
    "licenses": {"status": "unavailable", "reason": "No license policy engine is installed; package declarations and repository LICENSE remain review inputs"},
    "sbom": {"status": "unavailable", "reason": "No SBOM generator is installed; cargo metadata is retained only as a dependency inventory, not an SBOM"},
}

rustup = shutil.which("rustup")
if rustup:
    toolchains = subprocess.run([rustup, "toolchain", "list"], cwd=root, capture_output=True, text=True).stdout
    if re.search(r'^1\.85(?:\.\d+)?\s', toolchains, re.MULTILINE):
        results["msrv"]["proof"] = command_status([
            rustup, "run", "1.85", "cargo", "check", "--locked", "--manifest-path", str(manifest)
        ])

for name in ("cargo-audit", "cargo-deny", "cargo-about", "cargo-cyclonedx"):
    if shutil.which(name):
        results.setdefault("available_tools", []).append(name)

non_passed = [key for key, value in results.items()
              if isinstance(value, dict) and value.get("status") in {"unavailable", "failed"}]
if results["msrv"]["proof"].get("status") == "unavailable":
    non_passed.append("msrv")
elif results["msrv"]["proof"].get("status") == "failed":
    non_passed.append("msrv")
results["overall"] = "partial_with_explicit_dispositions" if non_passed else "passed"
results["release_gate"] = "not_ready_for_supply_chain_certification" if non_passed else "ready_for_review"
results["explicit_dispositions"] = non_passed
out.write_text(json.dumps(results, indent=2, sort_keys=True) + "\n")
print(out)
PY
