#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
SELECTOR="$ROOT_DIR/adl/tools/runtime_v3_operational_selector.sh"
TRANSITION="$ROOT_DIR/.csdlc/prepared/issues/5590/run_operational_selector_transition.sh"
TMP=$(mktemp -d "${TMPDIR:-/tmp}/adl-runtime-v3-selector.XXXXXX")
cleanup() {
  ADL_RUNTIME_V3_SELECTOR_STATE_DIR="$TMP/state" "$SELECTOR" stop >/dev/null 2>&1 || true
  if [ -n "${unrelated_pid:-}" ]; then
    kill -TERM "$unrelated_pid" 2>/dev/null || true
    wait "$unrelated_pid" 2>/dev/null || true
  fi
  rm -rf "$TMP"
}
trap cleanup EXIT INT TERM

mkdir -p "$TMP/candidate" "$TMP/prior" "$TMP/state"
cat > "$TMP/runtime-child.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
sleep 300 &
descendant=$!
printf '%s\n' "$descendant" > "$ADL_SELECTOR_DESCENDANT_FILE"
shutdown() {
  kill -TERM "$descendant" 2>/dev/null || true
  wait "$descendant" 2>/dev/null || true
  : > "$ADL_SELECTOR_CLEANUP_FILE"
  exit 0
}
trap shutdown TERM INT
printf '%s\n' "$ADL_SELECTOR_ID" > "$ADL_SELECTOR_READY_FILE"
while :; do sleep 1; done
SH
chmod +x "$TMP/runtime-child.sh"

for name in candidate prior; do
  cat > "$TMP/$name/launch" <<SH
#!/usr/bin/env bash
export ADL_SELECTOR_ID=$name
export ADL_SELECTOR_READY_FILE=$TMP/$name.ready
export ADL_SELECTOR_DESCENDANT_FILE=$TMP/$name.descendant
export ADL_SELECTOR_CLEANUP_FILE=$TMP/$name.cleaned
exec $TMP/runtime-child.sh
SH
  chmod +x "$TMP/$name/launch"
done

cat > "$TMP/health-probe" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in https://*) ;; *) exit 65 ;; esac
expected=${1##*/}
for _ in $(seq 1 100); do
  if [ -f "$ADL_SELECTOR_PROOF_ROOT/$expected.ready" ] &&
     [ "$(cat "$ADL_SELECTOR_PROOF_ROOT/$expected.ready")" = "$expected" ]; then
    exit 0
  fi
  sleep 0.02
done
exit 1
SH
chmod +x "$TMP/health-probe"

export ADL_RUNTIME_V3_SELECTOR_STATE_DIR="$TMP/state"
export ADL_SELECTOR_PROOF_ROOT="$TMP"
"$TRANSITION" "$SELECTOR" "$TMP/health-probe" "$TMP/candidate" "$TMP/prior" \
  "https://runtime.test/candidate" "https://runtime.test/prior"
test "$(cat "$TMP/state/current-selector")" = "$(cd "$TMP/prior" && pwd -P)"
test "$(cat "$TMP/prior.ready")" = prior
test -f "$TMP/candidate.cleaned"
candidate_descendant=$(cat "$TMP/candidate.descendant")
if kill -0 "$candidate_descendant" 2>/dev/null; then
  echo "candidate descendant survived confirmed selector shutdown" >&2
  exit 1
fi

"$SELECTOR" stop
test -f "$TMP/prior.cleaned"
prior_descendant=$(cat "$TMP/prior.descendant")
if kill -0 "$prior_descendant" 2>/dev/null; then
  echo "prior descendant survived confirmed selector shutdown" >&2
  exit 1
fi

sleep 300 &
unrelated_pid=$!
printf '%s\n' "$unrelated_pid" > "$TMP/state/runtime.pid"
printf '%s\n' "$TMP/prior" > "$TMP/state/current-selector"
if "$SELECTOR" activate --selector "$TMP/prior" 2>/dev/null; then
  echo "incomplete selector state unexpectedly activated" >&2
  exit 1
fi
kill -0 "$unrelated_pid"
rm -f "$TMP/state/runtime.pid" "$TMP/state/current-selector"

mkdir "$TMP/state/.selector-lock"
if "$SELECTOR" activate --selector "$TMP/prior" 2>/dev/null; then
  echo "concurrent selector activation unexpectedly acquired the lock" >&2
  exit 1
fi
rmdir "$TMP/state/.selector-lock"

if "$SELECTOR" activate --selector "$TMP/missing" 2>/dev/null; then
  echo "missing selector unexpectedly activated" >&2
  exit 1
fi
if ADL_RUNTIME_V3_SELECTOR_STATE_DIR= "$SELECTOR" activate --selector "$TMP/prior" 2>/dev/null; then
  echo "selector unexpectedly accepted missing state root" >&2
  exit 1
fi
if "$TRANSITION" "$SELECTOR" "$TMP/health-probe" "$TMP/candidate" "$TMP/prior" \
  "http://runtime.test/candidate" "https://runtime.test/prior" 2>/dev/null; then
  echo "transition unexpectedly accepted plaintext health" >&2
  exit 1
fi

printf 'runtime_v3_operational_selector_tests=pass\n'
