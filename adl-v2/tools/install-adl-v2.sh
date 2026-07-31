#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
install_root="${ADL_V2_INSTALL_ROOT:-${HOME}/.local/lib/adl-v2}"
if [[ "${1:-}" == "--test-root" ]]; then
  install_root="${2:?missing test root}"
fi

target_dir="${CARGO_TARGET_DIR:-/Volumes/FastWork/adl-v2/target}"
export CARGO_TARGET_DIR="$target_dir"
cargo build --locked --manifest-path "$root/Cargo.toml" --bin adl-v2 --release

mkdir -p "$install_root/bin" "$install_root/receipts"
tmp="$(mktemp "$install_root/.adl-v2.XXXXXX")"
cp "$target_dir/release/adl-v2" "$tmp"
chmod 755 "$tmp"
mv -f "$tmp" "$install_root/bin/adl-v2"

digest="$(shasum -a 256 "$install_root/bin/adl-v2" | awk '{print $1}')"
receipt="$install_root/receipts/adl-v2.json"
receipt_tmp="$(mktemp "$install_root/receipts/.adl-v2-receipt.XXXXXX")"
printf '{"schema":"adl.install.receipt.v1","binary":"adl-v2","sha256":"%s"}\n' "$digest" > "$receipt_tmp"
chmod 644 "$receipt_tmp"
mv -f "$receipt_tmp" "$receipt"
test "$(shasum -a 256 "$install_root/bin/adl-v2" | awk '{print $1}')" = "$digest"
printf '%s\n' "$install_root/bin/adl-v2"
