#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_nessus_remote_validation.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "expected file to exist: $path" >&2
    exit 1
  fi
}

origin_src="$TMP/origin-src"
origin_bare="$TMP/origin.git"
mkdir -p "$origin_src"
git -C "$origin_src" init -q
git -C "$origin_src" branch -M main
cat >"$origin_src/README.md" <<'EOF'
# remote validation fixture
EOF
git -C "$origin_src" add README.md
git -C "$origin_src" -c user.name=Codex -c user.email=codex@example.com commit -q -m "fixture"
git clone -q --bare "$origin_src" "$origin_bare"

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "rustc 1.96.0 (fixture)"
  exit 0
fi
echo "unexpected rustc invocation: $*" >&2
exit 1
EOF
cat >"$fake_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "cargo 1.96.0 (fixture)"
  exit 0
fi
echo "unexpected cargo invocation: $*" >&2
exit 1
EOF
cat >"$fake_bin/sccache" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --version)
    echo "sccache 0.16.0"
    ;;
  --zero-stats)
    exit 0
    ;;
  --show-stats)
    cat <<'STATS'
Compile requests                      3
Compile requests executed             1
Cache hits                            2
Cache misses                          1
STATS
    ;;
  *)
    echo "unexpected sccache invocation: $*" >&2
    exit 1
    ;;
esac
EOF
cat >"$fake_bin/apt-get" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FAIL_APT:-0}" == "1" ]]; then
  echo "apt-get fixture failure" >&2
  exit 1
fi
echo "apt-get update fixture ok"
EOF
chmod +x "$fake_bin/"*

sources_list="$TMP/sources.list"
kubernetes_list="$TMP/kubernetes.list"
cat >"$sources_list" <<'EOF'
deb https://apt.releases.hashicorp.com focal main
EOF
cat >"$kubernetes_list" <<'EOF'
deb https://apt.kubernetes.io/ kubernetes-xenial main
EOF

PATH="$fake_bin:$PATH" \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-pass" \
  --run-id fixture-pass \
  --command "printf remote-ok" \
  --local-artifact-dir "$TMP/artifacts-pass" \
  >"$TMP/pass.json"

assert_file "$TMP/artifacts-pass/summary.json"
assert_file "$TMP/artifacts-pass/run-logs.tar.gz"
python3 - <<'PY' "$TMP/artifacts-pass/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["schema_version"] == "adl.remote_validation_run.v1"
assert summary["runner"] == "nessus"
assert summary["status"] == "passed"
assert summary["resolved_commit"] != "unknown"
assert summary["command"] == "printf remote-ok"
assert summary["logs"]["command"].endswith("command.log")
PY

grep -F "apt.releases.hashicorp.com" "$sources_list" >/dev/null
assert_file "$kubernetes_list"

cat >"$fake_bin/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  image)
    [[ "${2:-}" == "inspect" ]]
    exit 0
    ;;
  pull)
    exit 0
    ;;
  run)
    shift
    while [[ $# -gt 0 ]]; do
      case "$1" in
        --rm)
          shift
          ;;
        -v|-e|-w)
          shift 2
          ;;
        *)
          break
          ;;
      esac
    done
    image="${1:-}"
    command="${2:-}"
    if [[ "$image" != "example.invalid/adl-builder:test" ]]; then
      echo "unexpected image: $image" >&2
      exit 1
    fi
    case "$command" in
      "rustc --version")
        echo "rustc 1.96.0 (builder fixture)"
        ;;
      "cargo --version")
        echo "cargo 1.96.0 (builder fixture)"
        ;;
      "sccache --version")
        echo "sccache 0.16.0"
        ;;
      "sccache --zero-stats"* )
        exit 0
        ;;
      "sccache --show-stats")
        cat <<'STATS'
Compile requests                      5
Compile requests executed             1
Cache hits                            4
Cache misses                          1
STATS
        ;;
      "printf builder-ok")
        printf builder-ok
        ;;
      *)
        echo "unexpected builder command: $command" >&2
        exit 1
        ;;
    esac
    ;;
  *)
    echo "unexpected docker invocation: $*" >&2
    exit 1
    ;;
esac
EOF
chmod +x "$fake_bin/docker"

PATH="$fake_bin:$PATH" \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-builder" \
  --run-id fixture-builder \
  --command "printf builder-ok" \
  --builder-image "example.invalid/adl-builder:test" \
  --local-artifact-dir "$TMP/artifacts-builder" \
  >"$TMP/builder.json"

assert_file "$TMP/artifacts-builder/summary.json"
python3 - <<'PY' "$TMP/artifacts-builder/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "passed"
assert summary["builder_image"] == "example.invalid/adl-builder:test"
assert summary["builder_runtime"] == "auto"
assert summary["resolved_builder_runtime"] == "docker"
assert summary["cache_status"]["cache_hits"] == "4"
PY

PATH="$fake_bin:$PATH" \
FAIL_APT=1 \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-builder-apt-skip" \
  --run-id fixture-builder-apt-skip \
  --command "printf builder-ok" \
  --builder-image "example.invalid/adl-builder:test" \
  --local-artifact-dir "$TMP/artifacts-builder-apt-skip" \
  >"$TMP/builder-apt-skip.json"

assert_file "$TMP/artifacts-builder-apt-skip/summary.json"
tar -xzf "$TMP/artifacts-builder-apt-skip/run-logs.tar.gz" -C "$TMP/artifacts-builder-apt-skip"
grep -F "skipped: builder image mode uses container toolchain" "$TMP/artifacts-builder-apt-skip/apt-update.log" >/dev/null

if PATH="$fake_bin:$PATH" \
  FAIL_APT=1 \
  ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
  ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
  bash "$SCRIPT" \
    --executor local \
    --repo-url "$origin_bare" \
    --git-ref origin/main \
    --remote-root "$TMP/remote-root-fail" \
    --run-id fixture-fail \
    --command "printf should-not-run" \
    --local-artifact-dir "$TMP/artifacts-fail" \
    >"$TMP/fail.json" 2>"$TMP/fail.err"; then
  echo "expected apt failure path to fail closed" >&2
  exit 1
fi

assert_file "$TMP/artifacts-fail/summary.json"
python3 - <<'PY' "$TMP/artifacts-fail/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "failed"
assert summary["exit_code"] != 0
assert summary["command"] == "printf should-not-run"
PY

cat >"$fake_bin/ssh-fail" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "ssh fixture failure" >&2
exit 255
EOF
chmod +x "$fake_bin/ssh-fail"

if SSH_BIN="$fake_bin/ssh-fail" \
  bash "$SCRIPT" \
    --executor ssh \
    --host fixture.invalid \
    --ssh-user fixture \
    --run-id fixture-transport-fail \
    --command "printf no-transport" \
    --local-artifact-dir "$TMP/artifacts-transport-fail" \
    >"$TMP/transport-fail.json" 2>"$TMP/transport-fail.err"; then
  echo "expected transport failure path to fail closed" >&2
  exit 1
fi

assert_file "$TMP/artifacts-transport-fail/summary.json"
python3 - <<'PY' "$TMP/artifacts-transport-fail/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "failed"
assert summary["transport_failure"]["summary_fetch_failed"] is True
assert summary["transport_failure"]["executor"] == "ssh"
assert summary["command"] == "printf no-transport"
PY
grep -F "fallback summary written locally" "$TMP/transport-fail.err" >/dev/null

echo "PASS test_run_nessus_remote_validation"
