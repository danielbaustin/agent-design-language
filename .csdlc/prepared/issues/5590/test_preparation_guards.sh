#!/usr/bin/env bash
set -euo pipefail

root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
mkdir -p "$root/bin"

cat >"$root/bin/cargo" <<'SCRIPT'
#!/usr/bin/env bash
set -euo pipefail
if [[ " $* " == *" --list "* ]]; then
  if [ "${FAKE_MATCH_COUNT:-0}" -gt 0 ]; then
    i=1
    while [ "$i" -le "$FAKE_MATCH_COUNT" ]; do
      printf 'guarded_case_%s: test\n' "$i"
      i=$((i + 1))
    done
  fi
  exit 0
fi
: >"$FAKE_RUN_MARKER"
SCRIPT
chmod +x "$root/bin/cargo"

export PATH="$root/bin:$PATH"
export FAKE_RUN_MARKER="$root/tests-ran"
export FAKE_MATCH_COUNT=0
if bash .csdlc/prepared/issues/5590/run_filtered_test_lane.sh fake/Cargo.toml absent; then
  echo "zero-match lane unexpectedly passed" >&2
  exit 1
fi
test ! -e "$FAKE_RUN_MARKER"

export FAKE_MATCH_COUNT=2
bash .csdlc/prepared/issues/5590/run_filtered_test_lane.sh fake/Cargo.toml present
test -e "$FAKE_RUN_MARKER"

cat >"$root/bin/selector" <<'SCRIPT'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"$FAKE_SELECTOR_LOG"
SCRIPT
cat >"$root/bin/health-probe" <<'SCRIPT'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"$FAKE_HEALTH_LOG"
SCRIPT
chmod +x "$root/bin/selector" "$root/bin/health-probe"
export FAKE_SELECTOR_LOG="$root/selector.log"
export FAKE_HEALTH_LOG="$root/health.log"

if bash .csdlc/prepared/issues/5590/run_operational_selector_transition.sh selector health-probe candidate prior http://candidate/health https://prior/health; then
  echo "plaintext health URL unexpectedly passed" >&2
  exit 1
fi
test ! -e "$FAKE_SELECTOR_LOG"

bash .csdlc/prepared/issues/5590/run_operational_selector_transition.sh selector health-probe candidate prior https://candidate/health https://prior/health
test "$(wc -l <"$FAKE_SELECTOR_LOG" | tr -d ' ')" = 2
test "$(wc -l <"$FAKE_HEALTH_LOG" | tr -d ' ')" = 2
grep -Fx 'activate --selector candidate' "$FAKE_SELECTOR_LOG" >/dev/null
grep -Fx 'activate --selector prior' "$FAKE_SELECTOR_LOG" >/dev/null
grep -F 'https://candidate/health' "$FAKE_HEALTH_LOG" >/dev/null
grep -F 'https://prior/health' "$FAKE_HEALTH_LOG" >/dev/null

echo "filtered_zero_match=denied filtered_positive_count=2 selector_transition=2 health_checks=2"
