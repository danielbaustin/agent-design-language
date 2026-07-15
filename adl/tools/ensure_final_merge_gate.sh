#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  ensure_final_merge_gate.sh inspect --repo owner/repo [--branch main] [--api-base URL] [--json]
  ensure_final_merge_gate.sh apply --repo owner/repo [--branch main] [--api-base URL] [--json]
  ensure_final_merge_gate.sh rollback --repo owner/repo [--branch main] [--api-base URL] [--json]

Ensures the final base-plus-head merge gate by requiring GitHub required status
checks to be strict/up-to-date for stable ADL checks: adl-ci and adl-coverage.
The tool prefers repository rulesets and falls back to the legacy branch
protection required-status-checks subresource.

Credentials are resolved from GITHUB_TOKEN, GH_TOKEN, ADL_GITHUB_TOKEN_FILE, or
$HOME/keys/github.token. Token values are never printed.
USAGE
}

die() {
  printf 'ERROR final_merge_gate %s\n' "$*" >&2
  exit 1
}

json_escape() {
  python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))'
}

github_token() {
  if [ -n "${GITHUB_TOKEN:-}" ]; then
    printf '%s' "$GITHUB_TOKEN"
    return 0
  fi
  if [ -n "${GH_TOKEN:-}" ]; then
    printf '%s' "$GH_TOKEN"
    return 0
  fi
  if [ -n "${ADL_GITHUB_TOKEN_FILE:-}" ] && [ -r "$ADL_GITHUB_TOKEN_FILE" ]; then
    sed -n '1p' "$ADL_GITHUB_TOKEN_FILE"
    return 0
  fi
  if [ -r "$HOME/keys/github.token" ]; then
    sed -n '1p' "$HOME/keys/github.token"
    return 0
  fi
  return 1
}

mode="${1:-}"
if [ -z "$mode" ] || [ "$mode" = "--help" ] || [ "$mode" = "-h" ]; then
  usage
  exit 0
fi
shift

case "$mode" in
  inspect|apply|rollback) ;;
  *) die "unknown mode '$mode'" ;;
esac

repo=""
branch="main"
api_base="${ADL_GITHUB_API_BASE:-https://api.github.com}"
json=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo)
      repo="${2:-}"
      shift 2
      ;;
    --branch)
      branch="${2:-}"
      shift 2
      ;;
    --api-base)
      api_base="${2:-}"
      shift 2
      ;;
    --json)
      json=1
      shift
      ;;
    *)
      die "unknown argument '$1'"
      ;;
  esac
done

case "$repo" in
  */*) ;;
  *) die "--repo must be owner/repo" ;;
esac
[ -n "$branch" ] || die "--branch cannot be empty"
[ -n "$api_base" ] || die "--api-base cannot be empty"

token="$(github_token || true)"
[ -n "$token" ] || die "missing GitHub token source"

repo_api="${api_base%/}/repos/${repo}"
endpoint="${repo_api}/branches/${branch}/protection/required_status_checks"
rulesets_endpoint="${repo_api}/rulesets?targets=branch"
required_contexts_json='["adl-ci","adl-coverage"]'
if [ "$mode" = "rollback" ]; then
  desired_strict=false
else
  desired_strict=true
fi
payload="$(printf '{"strict":%s,"contexts":%s}' "$desired_strict" "$required_contexts_json")"

tmp_body="$(mktemp "${TMPDIR:-/tmp}/adl-final-merge-gate.XXXXXX")"
tmp_ruleset="$(mktemp "${TMPDIR:-/tmp}/adl-final-merge-gate-ruleset.XXXXXX")"
tmp_apply="$(mktemp "${TMPDIR:-/tmp}/adl-final-merge-gate-apply.XXXXXX")"
tmp_curl_config="$(mktemp "${TMPDIR:-/tmp}/adl-final-merge-gate-curl.XXXXXX")"
chmod 600 "$tmp_curl_config"
{
  printf 'header = "Accept: application/vnd.github+json"\n'
  printf 'header = "Authorization: Bearer %s"\n' "$token"
  printf 'header = "X-GitHub-Api-Version: 2022-11-28"\n'
  printf 'header = "Content-Type: application/json"\n'
} >"$tmp_curl_config"
trap 'rm -f "$tmp_body" "$tmp_ruleset" "$tmp_apply" "$tmp_curl_config"' EXIT

curl_json() {
  local method="$1"
  local url="$2"
  local data="${3:-}"
  local code
  if [ -n "$data" ]; then
    code="$(curl -sS -X "$method" "$url" \
      -K "$tmp_curl_config" \
      --data "$data" \
      -o "$tmp_body" \
      -w '%{http_code}')"
  else
    code="$(curl -sS -X "$method" "$url" \
      -K "$tmp_curl_config" \
      -o "$tmp_body" \
      -w '%{http_code}')"
  fi
  printf '%s' "$code"
}

write_report() {
  python3 - "$repo" "$branch" "$mode" "$1" "$json" "$2" <<'PY'
import json
import sys

repo, branch, mode, body_path, json_mode, gate = sys.argv[1:7]
payload = json.loads(open(body_path, encoding="utf-8").read() or "{}")
if gate == "repository_ruleset":
    rule = next((item for item in payload.get("rules", []) if item.get("type") == "required_status_checks"), {})
    params = rule.get("parameters") or {}
    checks = params.get("required_status_checks") or []
    contexts = sorted(set(item.get("context") for item in checks if item.get("context")))
    strict = bool(params.get("strict_required_status_checks_policy"))
    ruleset = {
        "id": payload.get("id"),
        "name": payload.get("name"),
        "enforcement": payload.get("enforcement"),
    }
else:
    contexts = payload.get("contexts")
    if contexts is None and isinstance(payload.get("checks"), list):
        contexts = [item.get("context") for item in payload["checks"] if item.get("context")]
    contexts = sorted(set(contexts or []))
    strict = bool(payload.get("strict"))
    ruleset = None
required = ["adl-ci", "adl-coverage"]
missing = [item for item in required if item not in contexts]
compliant = (not strict and not missing) if mode == "rollback" else (strict and not missing)
report = {
    "schema": "adl.final_merge_gate.v1",
    "repo": repo,
    "branch": branch,
    "mode": mode,
    "gate": gate,
    "strict": strict,
    "required_contexts": required,
    "observed_contexts": contexts,
    "missing_contexts": missing,
    "compliant": compliant,
}
if mode == "rollback":
    report["rollback_ready"] = not strict
if ruleset:
    report["ruleset"] = ruleset
if json_mode == "1":
    print(json.dumps(report, indent=2, sort_keys=True))
else:
    print(
        "final_merge_gate "
        f"repo={repo} branch={branch} mode={mode} gate={gate} strict={str(strict).lower()} "
        f"required=adl-ci,adl-coverage missing={','.join(missing) or 'none'} "
        f"compliant={str(compliant).lower()}"
    )
if not compliant:
    sys.exit(2)
PY
}

list_ruleset_urls() {
  python3 - "$tmp_body" <<'PY'
import json
import sys

items = json.loads(open(sys.argv[1], encoding="utf-8").read() or "[]")
if not isinstance(items, list):
    sys.exit(1)
for item in items:
    if not isinstance(item, dict):
        continue
    if item.get("target") != "branch" or item.get("enforcement") != "active":
        continue
    href = ((item.get("_links") or {}).get("self") or {}).get("href")
    if href:
        print(href)
PY
}

ruleset_matches_branch() {
  python3 - "$tmp_body" "$branch" <<'PY'
import json
import sys

payload = json.loads(open(sys.argv[1], encoding="utf-8").read() or "{}")
branch = sys.argv[2]
if payload.get("target") != "branch" or payload.get("enforcement") != "active":
    sys.exit(1)
ref_name = ((payload.get("conditions") or {}).get("ref_name") or {})
includes = ref_name.get("include") or []
excludes = set(ref_name.get("exclude") or [])
candidates = {branch, f"refs/heads/{branch}"}
if branch == "main":
    candidates.add("~DEFAULT_BRANCH")
if candidates & excludes:
    sys.exit(1)
if not includes or candidates & set(includes):
    sys.exit(0)
sys.exit(1)
PY
}

ruleset_apply_payload() {
  python3 - "$tmp_ruleset" "$tmp_apply" "$mode" <<'PY'
import json
import sys

source_path, out_path, mode = sys.argv[1:4]
payload = json.loads(open(source_path, encoding="utf-8").read() or "{}")
rules = payload.get("rules") or []
for rule in rules:
    if rule.get("type") == "required_status_checks":
        params = rule.setdefault("parameters", {})
        params["strict_required_status_checks_policy"] = (mode != "rollback")
        checks = params.setdefault("required_status_checks", [])
        seen = {item.get("context") for item in checks if isinstance(item, dict)}
        for context in ["adl-ci", "adl-coverage"]:
            if context not in seen:
                checks.append({"context": context})
        break
else:
    rules.append({
        "type": "required_status_checks",
        "parameters": {
            "strict_required_status_checks_policy": (mode != "rollback"),
            "do_not_enforce_on_create": False,
            "required_status_checks": [
                {"context": "adl-ci"},
                {"context": "adl-coverage"},
            ],
        },
    })

out = {
    "name": payload["name"],
    "target": payload.get("target", "branch"),
    "enforcement": payload.get("enforcement", "active"),
    "conditions": payload.get("conditions") or {"ref_name": {"include": ["~DEFAULT_BRANCH"], "exclude": []}},
    "rules": rules,
    "bypass_actors": payload.get("bypass_actors") or [],
}
open(out_path, "w", encoding="utf-8").write(json.dumps(out))
PY
}

code="$(curl_json GET "$rulesets_endpoint")"
case "$code" in
  200)
    ruleset_url=""
    while IFS= read -r candidate_url; do
      [ -n "$candidate_url" ] || continue
      code="$(curl_json GET "$candidate_url")"
      [ "$code" = "200" ] || continue
      if ruleset_matches_branch; then
        cp "$tmp_body" "$tmp_ruleset"
        ruleset_url="$candidate_url"
        break
      fi
    done < <(list_ruleset_urls || true)
    if [ -n "$ruleset_url" ]; then
      if [ "$mode" = "apply" ] || [ "$mode" = "rollback" ]; then
        ruleset_apply_payload
        code="$(curl_json PUT "$ruleset_url" "$(cat "$tmp_apply")")"
        [ "$code" = "200" ] || die "apply_failed status=$code endpoint=ruleset"
        cp "$tmp_body" "$tmp_ruleset"
      fi
      write_report "$tmp_ruleset" "repository_ruleset"
      exit $?
    fi
    ;;
  401|403) die "${mode}_failed status=$code endpoint=rulesets" ;;
esac

if [ "$mode" = "apply" ] || [ "$mode" = "rollback" ]; then
  code="$(curl_json PATCH "$endpoint" "$payload")"
else
  code="$(curl_json GET "$endpoint")"
fi
[ "$code" = "200" ] || die "${mode}_failed status=$code endpoint=required_status_checks"
write_report "$tmp_body" "required_status_checks_strict"
