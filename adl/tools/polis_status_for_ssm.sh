#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
derived_repo_root="$(cd "$script_dir/../.." && pwd)"
fallback_home="${HOME:-/var/root}"
repo_root="${ADL_REPO_ROOT:-$derived_repo_root}"
if [ ! -e "$repo_root/.git" ] && [ -d "$fallback_home/git/agent-design-language" ]; then
  repo_root="$fallback_home/git/agent-design-language"
fi
repo_name="$(basename "$repo_root")"
branch="unknown"
commit="unknown"
repo_present=false
git_cmd=(git -c safe.directory="$repo_root" -C "$repo_root")
if command -v git >/dev/null 2>&1; then
  if "${git_cmd[@]}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    repo_present=true
    branch="$("${git_cmd[@]}" rev-parse --abbrev-ref HEAD 2>/dev/null || printf 'unknown')"
    commit="$("${git_cmd[@]}" rev-parse --short HEAD 2>/dev/null || printf 'unknown')"
  fi
fi
hostname_value="$(scutil --get ComputerName 2>/dev/null || hostname)"
os_name="$(sw_vers -productName 2>/dev/null || uname -s)"
os_version="$(sw_vers -productVersion 2>/dev/null || uname -r)"
ssm_agent_installed=false
if [ -x /opt/aws/ssm/bin/amazon-ssm-agent ] || [ -x /usr/local/bin/amazon-ssm-agent ]; then
  ssm_agent_installed=true
fi

export HOST_LABEL="$hostname_value"
export OS_NAME="$os_name"
export OS_VERSION="$os_version"
export REPO_NAME="$repo_name"
export REPO_PRESENT="$repo_present"
export GIT_BRANCH="$branch"
export GIT_COMMIT_SHORT="$commit"
export SSM_AGENT_INSTALLED="$ssm_agent_installed"

python3 - <<'PY2'
import json
import os

payload = {
    "schema_version": "adl.local_polis_status.v1",
    "generated_at_utc": os.popen("date -u +%Y-%m-%dT%H:%M:%SZ").read().strip(),
    "host_label": os.environ["HOST_LABEL"],
    "os_name": os.environ["OS_NAME"],
    "os_version": os.environ["OS_VERSION"],
    "repo_name": os.environ["REPO_NAME"],
    "repo_present": os.environ["REPO_PRESENT"] == "true",
    "git_branch": os.environ["GIT_BRANCH"],
    "git_commit_short": os.environ["GIT_COMMIT_SHORT"],
    "ssm_agent_installed": os.environ["SSM_AGENT_INSTALLED"] == "true",
}
print(json.dumps(payload, indent=2))
PY2
