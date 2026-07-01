#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 4 ]; then
  echo "usage: $0 <probe|tail> <ssh-key-path> <ssh-user> <host> [run-root]" >&2
  exit 2
fi

mode="$1"
ssh_key_path="$2"
ssh_user="$3"
host="$4"
shift 4

ssh_args=(
  -o StrictHostKeyChecking=no
  -o UserKnownHostsFile=/dev/null
  -o ConnectTimeout=10
  -o ServerAliveInterval=5
  -o ServerAliveCountMax=1
  -i "$ssh_key_path"
)

case "$mode" in
  probe)
    exec ssh "${ssh_args[@]}" "${ssh_user}@${host}" true
    ;;
  tail)
    if [ "$#" -lt 1 ]; then
      echo "tail mode requires <run-root>" >&2
      exit 2
    fi
    run_root="$1"
    exec ssh "${ssh_args[@]}" "${ssh_user}@${host}" \
      "while [ ! -d '$run_root' ]; do sleep 1; done; tail -n +1 -F '$run_root/progress.log' '$run_root/command.log' '$run_root/command.err'"
    ;;
  *)
    echo "unsupported mode: $mode" >&2
    exit 2
    ;;
esac
