#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/sweep_aws_spot_orphans.sh"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/adl-spot-orphan-test.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *"describe-instances"* ]]; then
  cat <<'JSON'
[
  {"id":"i-0123456789abcdef0","run_id":"adl-wp-5243-old","launch_time":"2020-01-01T00:00:00Z","state":"running"},
  {"id":"i-0123456789abcdef1","run_id":"adl-wp-5243-new","launch_time":"2099-01-01T00:00:00Z","state":"running"},
  {"id":"i-0123456789abcdef2","run_id":"adl-wp-5243-old","launch_time":"2099-01-01T00:00:00Z","state":"running"}
]
JSON
elif [[ "$*" == *"terminate-instances"* ]]; then
  printf '%s\n' "$*" >>"${ADL_FAKE_TERMINATIONS:?}"
else
  echo "unexpected AWS invocation: $*" >&2
  exit 1
fi
EOF
chmod +x "$fake_bin/aws"

ADL_AWS_CLI="$fake_bin/aws" \
  bash "$SCRIPT" --profile env --region us-west-2 --max-age-minutes 30 \
  --artifact-dir "$TMP/dry" >"$TMP/dry.out"
grep -F 'action=dry_run_candidate' "$TMP/dry.out" >/dev/null
test ! -e "$TMP/terminations"
grep -F '"run": false' "$TMP/dry/orphan-sweep.json" >/dev/null

ADL_AWS_CLI="$fake_bin/aws" ADL_FAKE_TERMINATIONS="$TMP/terminations" \
  bash "$SCRIPT" --profile env --region us-west-2 --run --max-age-minutes 30 \
  --run-id adl-wp-5243-old \
  --artifact-dir "$TMP/live" >"$TMP/live.out"
grep -F 'action=termination_requested' "$TMP/live.out" >/dev/null
grep -F 'i-0123456789abcdef0' "$TMP/terminations" >/dev/null
if grep -F 'i-0123456789abcdef2' "$TMP/terminations" >/dev/null; then
  echo "fresh same-run builder must not be terminated" >&2
  exit 1
fi
grep -F '"run": true' "$TMP/live/orphan-sweep.json" >/dev/null

if ADL_AWS_CLI="$fake_bin/aws" bash "$SCRIPT" --profile env --max-age-minutes 29 >/dev/null 2>"$TMP/invalid.err"; then
  echo "expected age guard to reject values below 30 minutes" >&2
  exit 1
fi
grep -F 'max age must be at least 30 minutes' "$TMP/invalid.err" >/dev/null

if ADL_AWS_CLI="$fake_bin/aws" bash "$SCRIPT" --profile env --run --max-age-minutes 30 >/dev/null 2>"$TMP/missing-run-id.err"; then
  echo "expected live sweep without exact run id to fail closed" >&2
  exit 1
fi
grep -F 'requires an exact --run-id' "$TMP/missing-run-id.err" >/dev/null

cat >"$TMP/bad-aws" <<'EOF'
#!/usr/bin/env bash
printf 'null\n'
EOF
chmod +x "$TMP/bad-aws"
if ADL_AWS_CLI="$TMP/bad-aws" bash "$SCRIPT" --profile env --max-age-minutes 30 >/dev/null 2>"$TMP/malformed.err"; then
  echo "expected malformed AWS output to fail closed" >&2
  exit 1
fi
grep -F 'was not a JSON array' "$TMP/malformed.err" >/dev/null

echo "sweep_aws_spot_orphans: ok"
