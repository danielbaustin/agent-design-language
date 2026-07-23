#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: runtime_v3_operational_selector.sh activate --selector <selector-directory> | stop" >&2
  exit 64
}

operation=${1:-}
case "$operation" in
  activate)
    [ "$#" -eq 3 ] && [ "$2" = "--selector" ] || usage
    selector=$3
    ;;
  stop)
    [ "$#" -eq 1 ] || usage
    selector=
    ;;
  *) usage ;;
esac

state_dir=${ADL_RUNTIME_V3_SELECTOR_STATE_DIR:-}
if [ -z "$state_dir" ]; then
  echo "ADL_RUNTIME_V3_SELECTOR_STATE_DIR is required" >&2
  exit 64
fi
if [ -L "$state_dir" ]; then
  echo "selector state directory must not be a symbolic link" >&2
  exit 65
fi
mkdir -p "$state_dir"
state_dir=$(cd "$state_dir" && pwd -P)
if [ ! -O "$state_dir" ]; then
  echo "selector state directory must be owned by the current user" >&2
  exit 65
fi
chmod 700 "$state_dir"

if [ "$operation" = activate ]; then
  if [ ! -d "$selector" ] || [ -L "$selector/launch" ] || [ ! -x "$selector/launch" ]; then
    echo "selector must be a directory containing a regular executable launch file" >&2
    exit 65
  fi
  selector=$(cd "$selector" && pwd -P)
fi

instances_dir="$state_dir/instances"
current_instance_file="$state_dir/current-instance"
current_file="$state_dir/current-selector"
pid_file="$state_dir/runtime.pid"
log_file="$state_dir/runtime.log"
lock_dir="$state_dir/.selector-lock"
if [ -L "$instances_dir" ]; then
  echo "selector instances directory must not be a symbolic link" >&2
  exit 65
fi
mkdir -p "$instances_dir"
chmod 700 "$instances_dir"

for path in "$instances_dir" "$current_instance_file" "$current_file" "$pid_file" "$log_file" "$lock_dir"; do
  if [ -L "$path" ]; then
    echo "selector state path must not be a symbolic link: $path" >&2
    exit 65
  fi
done

if ! mkdir "$lock_dir" 2>/dev/null; then
  echo "another Runtime v3 selector transition is active" >&2
  exit 75
fi
release_lock() {
  rmdir "$lock_dir" 2>/dev/null || true
}
trap release_lock EXIT INT TERM HUP

shutdown_grace_ms=${ADL_RUNTIME_V3_SELECTOR_SHUTDOWN_GRACE_MS:-30000}
case "$shutdown_grace_ms" in
  *[!0-9]*|'') echo "selector shutdown grace must be a positive integer" >&2; exit 64 ;;
esac
if [ "$shutdown_grace_ms" -eq 0 ]; then
  echo "selector shutdown grace must be a positive integer" >&2
  exit 64
fi
shutdown_attempts=$(( (shutdown_grace_ms + 49) / 50 ))

validate_instance_dir() {
  instance_dir=$1
  case "$instance_dir" in
    "$instances_dir"/activation.*) ;;
    *) echo "selector instance state is invalid" >&2; exit 66 ;;
  esac
  if [ -L "$instance_dir" ] || [ ! -d "$instance_dir" ]; then
    echo "selector instance directory is invalid" >&2
    exit 66
  fi
}

stop_current() {
  if [ ! -f "$current_instance_file" ]; then
    if [ -e "$pid_file" ] || [ -e "$current_file" ]; then
      echo "selector state is incomplete" >&2
      exit 66
    fi
    return 0
  fi
  instance_dir=$(cat "$current_instance_file")
  validate_instance_dir "$instance_dir"
  : > "$instance_dir/stop-requested"
  attempts=0
  while [ ! -f "$instance_dir/stopped" ]; do
    attempts=$((attempts + 1))
    if [ "$attempts" -ge "$shutdown_attempts" ]; then
      echo "selected Runtime v3 guardian did not confirm descendant cleanup before deadline" >&2
      exit 70
    fi
    sleep 0.05
  done
  rm -f "$current_instance_file" "$current_file" "$pid_file"
}

run_instance_supervisor() {
  instance_dir=$1
  selector_path=$2
  output_log=$3
  (
    trap - EXIT INT TERM HUP
    "$selector_path/launch" >>"$output_log" 2>&1 &
    guardian_pid=$!
    printf '%s\n' "$guardian_pid" > "$instance_dir/guardian.pid"
    status=0
    while kill -0 "$guardian_pid" 2>/dev/null; do
      if [ -f "$instance_dir/stop-requested" ]; then
        kill -TERM "$guardian_pid" 2>/dev/null || true
        wait "$guardian_pid" || status=$?
        printf '%s\n' "$status" > "$instance_dir/exit-status"
        : > "$instance_dir/stopped"
        exit 0
      fi
      sleep 0.05
    done
    wait "$guardian_pid" || status=$?
    printf '%s\n' "$status" > "$instance_dir/exit-status"
    : > "$instance_dir/stopped"
  ) &
}

stop_current
if [ "$operation" = stop ]; then
  printf 'runtime_v3_selector=stopped\n'
  exit 0
fi

if [ -f "$log_file" ]; then
  mv "$log_file" "$log_file.previous"
fi
: > "$log_file"
instance_dir=$(mktemp -d "$instances_dir/activation.XXXXXX")
chmod 700 "$instance_dir"
run_instance_supervisor "$instance_dir" "$selector" "$log_file"

attempts=0
while [ ! -f "$instance_dir/guardian.pid" ]; do
  attempts=$((attempts + 1))
  if [ "$attempts" -ge 100 ]; then
    echo "selected Runtime v3 guardian did not publish process identity" >&2
    exit 70
  fi
  sleep 0.01
done
pid=$(cat "$instance_dir/guardian.pid")
case "$pid" in *[!0-9]*|'') echo "selector guardian PID state is invalid" >&2; exit 66 ;; esac
sleep 0.1
if [ -f "$instance_dir/stopped" ] || ! kill -0 "$pid" 2>/dev/null; then
  status=$(cat "$instance_dir/exit-status" 2>/dev/null || printf '?')
  echo "selected Runtime v3 process exited before readiness (status=$status)" >&2
  exit 70
fi

instance_tmp="$current_instance_file.tmp.$$"
pid_tmp="$pid_file.tmp.$$"
current_tmp="$current_file.tmp.$$"
printf '%s\n' "$instance_dir" > "$instance_tmp"
printf '%s\n' "$pid" > "$pid_tmp"
printf '%s\n' "$selector" > "$current_tmp"
mv "$instance_tmp" "$current_instance_file"
mv "$pid_tmp" "$pid_file"
mv "$current_tmp" "$current_file"
printf 'runtime_v3_selector=active selector=%s pid=%s instance=%s\n' "$selector" "$pid" "$instance_dir"
