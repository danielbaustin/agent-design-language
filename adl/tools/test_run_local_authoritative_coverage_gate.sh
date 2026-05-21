#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

plan="$(bash "$ROOT_DIR/adl/tools/run_local_authoritative_coverage_gate.sh" --print-plan)"

for required in \
  "prereq_install=adl/tools/install_local_authoritative_coverage_prereqs.sh" \
  "runner=adl/tools/run_authoritative_coverage_lane.sh" \
  "gate=adl/tools/enforce_coverage_gates.sh coverage-summary.json" \
  "summary_copy=adl/target/local-authoritative-coverage-summary.json"
do
  if ! grep -F "$required" <<<"$plan" >/dev/null 2>&1; then
    echo "missing local authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

bash -n "$ROOT_DIR/adl/tools/run_local_authoritative_coverage_gate.sh"
bash -n "$ROOT_DIR/adl/tools/install_local_authoritative_coverage_prereqs.sh"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
mkdir -p "$TMP_DIR/bin"
cat >"$TMP_DIR/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
LOG_DIR="${FAKE_CARGO_LOG_DIR:?}"
case "${1:-}" in
  llvm-cov)
    if [[ "${2:-}" == "--version" ]]; then
      exit 0
    fi
    echo "unexpected fake cargo llvm-cov args: $*" >&2
    exit 98
    ;;
  nextest)
    if [[ "${2:-}" == "--version" ]]; then
      exit 1
    fi
    echo "unexpected fake cargo nextest args: $*" >&2
    exit 98
    ;;
  install)
    printf '%s\n' "$*" >>"$LOG_DIR/install.log"
    exit 99
    ;;
  *)
    echo "unexpected fake cargo invocation: $*" >&2
    exit 97
    ;;
esac
EOF
chmod +x "$TMP_DIR/bin/cargo"
cat >"$TMP_DIR/bin/jq" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
chmod +x "$TMP_DIR/bin/jq"

set +e
gate_output="$(
  PATH="$TMP_DIR/bin:$PATH" \
  FAKE_CARGO_LOG_DIR="$TMP_DIR" \
  bash "$ROOT_DIR/adl/tools/run_local_authoritative_coverage_gate.sh" 2>&1
)"
gate_status=$?
set -e

if [[ $gate_status -eq 0 ]]; then
  echo "expected local authoritative coverage gate to fail when cargo-nextest is missing" >&2
  exit 1
fi

if [[ -f "$TMP_DIR/install.log" ]]; then
  echo "local authoritative coverage gate must not auto-install cargo-nextest" >&2
  exit 1
fi

for required in \
  "cargo-nextest is required for local authoritative coverage validation" \
  "Run adl/tools/install_local_authoritative_coverage_prereqs.sh or install cargo-nextest explicitly before rerunning this gate."
do
  if ! grep -F "$required" <<<"$gate_output" >/dev/null 2>&1; then
    echo "missing expected missing-nextest guidance: $required" >&2
    exit 1
  fi
done

echo "PASS test_run_local_authoritative_coverage_gate"
