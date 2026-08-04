#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

TARGET_DIR="${ADL_5344_TARGET_DIR:-/Volumes/FastWork/adl-wp-5344/finalize-target}"
mkdir -p "$TARGET_DIR"

required_commit="2f3cd919b"
git cat-file -e "${required_commit}^{commit}"

tracked_paths=(
  "adl-runtime/src/guardian.rs"
  "adl-runtime/Cargo.toml"
  "adl-runtime/Cargo.lock"
  "adl-runtime-kernel/src/bin/adl-runtime-kernel.rs"
  "adl-runtime-kernel/src/continuity.rs"
  "adl-runtime-kernel/src/governed_operations.rs"
  "adl-runtime-kernel/tests/guardian_soak.rs"
  ".csdlc/evidence/5344/wp12-guardian-observatory-tls-2026-07-23.md"
)

git diff --quiet HEAD -- "${tracked_paths[@]}"

evidence=".csdlc/evidence/5344/wp12-guardian-observatory-tls-2026-07-23.md"
test -f "$evidence"

grep -F "Native Windows build of Guardian, kernel, and governed-operations: \`PASS\`." "$evidence" >/dev/null
grep -F "Native Windows Guardian process-0 descendant cleanup: \`PASS\`." "$evidence" >/dev/null
grep -F "Native Windows guardian lease-loss checkpoint shutdown: \`PASS\`." "$evidence" >/dev/null
grep -F "Native Windows strict CA-backed HTTPS/WSS with SAN verification and wrong-host rejection: \`PASS\`." "$evidence" >/dev/null
grep -F "No WSL, Docker, AWS, plaintext API, disabled certificate verification, or insecure curl flags were used." "$evidence" >/dev/null
grep -F "Leaf SANs: \`DNS:localhost\`, \`IP Address:127.0.0.1\`, \`IP Address:::1\`." "$evidence" >/dev/null

cargo test --locked \
  --manifest-path adl-runtime/Cargo.toml \
  --target-dir "$TARGET_DIR" \
  guardian -- --nocapture

cargo test --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --target-dir "$TARGET_DIR" \
  --test guardian_soak \
  guardian_lease_loss_checkpoints_and_stops_the_real_kernel \
  -- --exact --nocapture

cargo test --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --target-dir "$TARGET_DIR" \
  --test guardian_soak \
  signed_https_wss_shutdown_checkpoints_and_forgery_cannot_stop_the_process \
  -- --exact --nocapture

printf "wp12 guardian/windows/tls finalize proof passed at %s\n" "$(git rev-parse HEAD)"
