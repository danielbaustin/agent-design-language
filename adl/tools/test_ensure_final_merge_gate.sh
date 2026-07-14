#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/ensure_final_merge_gate.sh"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/adl-final-merge-gate-test.XXXXXX")"
trap 'rm -rf "$TMP_ROOT"' EXIT

assert_has() {
  local haystack="$1"
  local needle="$2"
  if [[ "$haystack" != *"$needle"* ]]; then
    printf 'expected output to contain %s\nactual:\n%s\n' "$needle" "$haystack" >&2
    exit 1
  fi
}

assert_not_has() {
  local haystack="$1"
  local needle="$2"
  if [[ "$haystack" == *"$needle"* ]]; then
    printf 'expected output not to contain %s\nactual:\n%s\n' "$needle" "$haystack" >&2
    exit 1
  fi
}

FAKE_BIN="$TMP_ROOT/bin"
mkdir -p "$FAKE_BIN"
cat >"$FAKE_BIN/curl" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
method="GET"
out=""
data=""
url=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    -X)
      method="$2"
      shift 2
      ;;
    -o)
      out="$2"
      shift 2
      ;;
    --data)
      data="$2"
      shift 2
      ;;
    -w)
      shift 2
      ;;
    -K)
      printf '%s\n' "$2" >>"${ADL_FAKE_CURL_CONFIGS:?}"
      shift 2
      ;;
    *)
      if [[ "$1" == http* ]]; then
        url="$1"
      fi
      shift
      ;;
  esac
done
[ -n "$out" ] || exit 7
printf '%s\n' "$method $url $data" >>"${ADL_FAKE_CURL_CALLS:?}"
ruleset_list='[
  {"id":2,"name":"release-branches","target":"branch","enforcement":"active","_links":{"self":{"href":"https://api.github.test/repos/owner/repo/rulesets/2"}}},
  {"id":1,"name":"main-protection","target":"branch","enforcement":"active","_links":{"self":{"href":"https://api.github.test/repos/owner/repo/rulesets/1"}}}
]'
ruleset_detail() {
  local strict="$1"
  local contexts="$2"
  printf '{"id":1,"name":"main-protection","target":"branch","enforcement":"active","conditions":{"ref_name":{"include":["~DEFAULT_BRANCH"],"exclude":[]}},"rules":[{"type":"required_status_checks","parameters":{"strict_required_status_checks_policy":%s,"do_not_enforce_on_create":false,"required_status_checks":%s}},{"type":"pull_request","parameters":{"required_approving_review_count":0}}],"bypass_actors":[]}' "$strict" "$contexts"
}
case "$url" in
  *"/rulesets?targets=branch")
    printf '%s' "$ruleset_list" >"$out"
    printf '200'
    ;;
  *"/rulesets/2")
    printf '{"id":2,"name":"release-branches","target":"branch","enforcement":"active","conditions":{"ref_name":{"include":["refs/heads/release/*"],"exclude":[]}},"rules":[],"bypass_actors":[]}' >"$out"
    printf '200'
    ;;
  *"/rulesets/1")
    case "${ADL_FAKE_GATE_CASE:?}" in
      missing)
        ruleset_detail false '[{"context":"adl-ci","integration_id":15368}]' >"$out"
        ;;
      compliant)
        ruleset_detail true '[{"context":"adl-ci","integration_id":15368},{"context":"adl-coverage","integration_id":15368}]' >"$out"
        ;;
      apply)
        ruleset_detail true '[{"context":"adl-ci","integration_id":15368},{"context":"adl-coverage","integration_id":15368}]' >"$out"
        ;;
      rollback)
        ruleset_detail false '[{"context":"adl-ci","integration_id":15368},{"context":"adl-coverage","integration_id":15368}]' >"$out"
        ;;
      *)
        printf '{"message":"fixture failure"}' >"$out"
        printf '500'
        exit 0
        ;;
    esac
    printf '200'
    ;;
  *)
    printf '{"message":"fixture failure"}' >"$out"
    printf '500'
    ;;
esac
SH
chmod +x "$FAKE_BIN/curl"

export PATH="$FAKE_BIN:$PATH"
export GITHUB_TOKEN="ghp_fixture_secret_that_must_not_print"
export ADL_FAKE_CURL_CONFIGS="$TMP_ROOT/configs.log"
export ADL_FAKE_CURL_CALLS="$TMP_ROOT/calls.log"

export ADL_FAKE_GATE_CASE=missing
if output="$("$SCRIPT" inspect --repo owner/repo --api-base https://api.github.test --json 2>&1)"; then
  printf 'missing gate unexpectedly passed:\n%s\n' "$output" >&2
  exit 1
fi
assert_has "$output" '"strict": false'
assert_has "$output" '"missing_contexts"'
assert_has "$output" '"adl-coverage"'
assert_not_has "$output" "$GITHUB_TOKEN"

export ADL_FAKE_GATE_CASE=compliant
output="$("$SCRIPT" inspect --repo owner/repo --api-base https://api.github.test --json)"
assert_has "$output" '"compliant": true'
assert_has "$output" '"strict": true'
assert_not_has "$output" "$GITHUB_TOKEN"

export ADL_FAKE_GATE_CASE=apply
output="$("$SCRIPT" apply --repo owner/repo --api-base https://api.github.test --json)"
assert_has "$output" '"mode": "apply"'
assert_has "$output" '"compliant": true'
assert_not_has "$output" "$GITHUB_TOKEN"
assert_has "$(cat "$ADL_FAKE_CURL_CALLS")" 'PUT https://api.github.test/repos/owner/repo/rulesets/1'
assert_has "$(cat "$ADL_FAKE_CURL_CALLS")" '"strict_required_status_checks_policy": true'

export ADL_FAKE_GATE_CASE=rollback
output="$("$SCRIPT" rollback --repo owner/repo --api-base https://api.github.test --json)"
assert_has "$output" '"mode": "rollback"'
assert_has "$output" '"rollback_ready": true'
assert_has "$output" '"compliant": true'
assert_not_has "$output" "$GITHUB_TOKEN"
assert_has "$(cat "$ADL_FAKE_CURL_CALLS")" '"strict_required_status_checks_policy": false'
assert_not_has "$(cat "$ADL_FAKE_CURL_CALLS")" "$GITHUB_TOKEN"
assert_not_has "$(cat "$ADL_FAKE_CURL_CONFIGS")" "$GITHUB_TOKEN"

printf 'test_ensure_final_merge_gate PASS\n'
