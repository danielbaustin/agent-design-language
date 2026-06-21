#!/bin/sh
set -eu

json_escape() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

host_label="$(hostname 2>/dev/null || uname -n 2>/dev/null || printf 'unknown')"
os_name="$(grep '^NAME=' /etc/os-release 2>/dev/null | head -1 | cut -d= -f2- | tr -d '"' || printf 'Linux')"
os_version="$(grep '^VERSION_ID=' /etc/os-release 2>/dev/null | head -1 | cut -d= -f2- | tr -d '"' || uname -r 2>/dev/null || printf 'unknown')"
generated_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
ssm_agent_installed=false
if [ -x /opt/aws/ssm/amazon-ssm-agent ] || [ -x /opt/aws/ssm/bin/amazon-ssm-agent ]; then
  ssm_agent_installed=true
fi

printf '{\n'
printf '  "schema_version": "adl.local_polis_status.v1",\n'
printf '  "generated_at_utc": "%s",\n' "$(json_escape "$generated_at")"
printf '  "host_label": "%s",\n' "$(json_escape "$host_label")"
printf '  "os_name": "%s",\n' "$(json_escape "$os_name")"
printf '  "os_version": "%s",\n' "$(json_escape "$os_version")"
printf '  "repo_name": "not_applicable",\n'
printf '  "repo_present": false,\n'
printf '  "git_branch": "unknown",\n'
printf '  "git_commit_short": "unknown",\n'
printf '  "ssm_agent_installed": %s\n' "$ssm_agent_installed"
printf '}\n'
