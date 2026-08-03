#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT_DIR/adl/tools/run_owner_validation_lane.sh"
INSTALLER="$ROOT_DIR/adl/tools/install_owner_binaries.sh"

plan_output="$(bash "$RUNNER" all --build --print-plan)"
installer_bins="$(
  awk '
    /^  BINS=\($/ { capture = 1; next }
    capture && /^  \)$/ { exit }
    capture { print }
  ' "$INSTALLER" |
    tr -d '\\' |
    xargs -n1 |
    LC_ALL=C sort
)"
manifest_bins="$(
  awk '
    /^\[\[bin\]\]$/ { bin = 1; next }
    bin && /^name = / {
      name = $0
      sub(/^name = "/, "", name)
      sub(/"$/, "", name)
      print name
      bin = 0
    }
  ' "$ROOT_DIR/adl/Cargo.toml" |
    grep -Ev '^(demo-|adl-gws-)' |
    LC_ALL=C sort
)"
if ! diff -u <(printf '%s\n' "$manifest_bins") <(printf '%s\n' "$installer_bins"); then
  echo "owner installer defaults do not match current operational targets" >&2
  exit 1
fi

grep -Fq 'cargo_args=(cargo build --quiet --locked' "$INSTALLER" || {
  echo "owner installer build does not use --locked" >&2
  exit 1
}
grep -Fq 'bash adl/tools/install_owner_binaries.sh' <<<"$plan_output" || {
  echo "owner validation does not delegate to the hardened installer" >&2
  exit 1
}
if grep -Fq 'cargo build' <<<"$plan_output"; then
  echo "owner validation plan reintroduced a duplicate nested Cargo build" >&2
  exit 1
fi
[[ "$(grep -Fc 'bash adl/tools/install_owner_binaries.sh' <<<"$plan_output")" == "1" ]] || {
  echo "owner validation plan must contain exactly one delegated build/install step" >&2
  exit 1
}
for removed in csdlc adl-csdlc adl-pr-closeout adl-session; do
  if grep -Fxq -- "$removed" <<<"$installer_bins"; then
    echo "owner installer still requests removed binary target: $removed" >&2
    exit 1
  fi
done

for expected in \
  "install stable owner binaries" \
  "C-SDLC wrapper migration contract" \
  "C-SDLC control-plane observability contract" \
  "runtime CSM binary availability contract" \
  "runtime CSM binary availability guard" \
  "runtime compatibility boundary" \
  "review compatibility boundary" \
  "PASS run_owner_validation_lane surface=all"; do
  grep -Fq -- "$expected" <<<"$plan_output" || {
    echo "missing expected lane plan entry: $expected" >&2
    echo "$plan_output" >&2
    exit 1
  }
done

set +e
bad_output="$(bash "$RUNNER" unknown 2>&1)"
bad_status=$?
set -e
[[ "$bad_status" -ne 0 ]] || {
  echo "unsupported lane unexpectedly passed" >&2
  exit 1
}
grep -Fq "unsupported argument 'unknown'" <<<"$bad_output" || {
  echo "unsupported lane did not report a useful error" >&2
  echo "$bad_output" >&2
  exit 1
}

echo "PASS test_owner_validation_lane"
