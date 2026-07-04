#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_validation_manager_nessus_lane.sh"
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
mkdir -p "$origin_src/adl/tools"
cat >"$origin_src/adl/tools/run_pr_fast_test_lane.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cargo test --manifest-path adl/Cargo.toml provider_communication -- --nocapture
EOF
chmod +x "$origin_src/adl/tools/run_pr_fast_test_lane.sh"
git -C "$origin_src" add adl/tools/run_pr_fast_test_lane.sh
git -C "$origin_src" -c user.name=Codex -c user.email=codex@example.com commit -q -m "fixture"
git clone -q --bare "$origin_src" "$origin_bare"

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "rustc 1.96.0 (fixture)"
else
  echo "rustc fixture"
fi
EOF
cat >"$fake_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "cargo 1.96.0 (fixture)"
else
  echo "cargo fixture remote lane command ok"
fi
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
Compile requests                      7
Compile requests executed             3
Cache hits                            4
Cache misses                          3
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
echo "apt-get update fixture ok"
EOF
chmod +x "$fake_bin/"*

sources="$TMP/sources.list"
kubernetes="$TMP/kubernetes.list"
cat >"$sources" <<'EOF'
deb https://apt.releases.hashicorp.com focal main
EOF
cat >"$kubernetes" <<'EOF'
deb https://apt.kubernetes.io/ kubernetes-xenial main
EOF

changed="$TMP/changed-files.txt"
printf 'M\tadl/src/provider_communication.rs\n' >"$changed"

ADL_NESSUS_REMOTE_EXECUTOR=local \
ADL_NESSUS_REMOTE_ROOT="$TMP/remote-root" \
ADL_NESSUS_REMOTE_REPO_URL="$origin_bare" \
ADL_NESSUS_REMOTE_GIT_REF=origin/main \
ADL_NESSUS_APT_SOURCES_LIST="$sources" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes" \
PATH="$fake_bin:$PATH" \
bash "$SCRIPT" \
  --changed-files "$changed" \
  --remote-artifact-dir "$TMP/artifacts" \
  --remote-git-ref origin/main \
  --run \
  --json \
  --report-out "$TMP/report.json" >"$TMP/out.json"

assert_file "$TMP/report.json"
assert_file "$TMP/artifacts/summary.json"
python3 - "$TMP/report.json" "$TMP/artifacts/summary.json" "$changed" <<'PY'
import json
import sys

profile = json.load(open(sys.argv[1], encoding="utf-8"))
summary = json.load(open(sys.argv[2], encoding="utf-8"))
assert profile["run_status"] == "passed"
assert profile["remote_runner"]["requested"] == "nessus"
assert profile["remote_runner"]["decision"] == "selected"
assert profile["run"][0]["lane_id"] == "nessus_remote_validation"
assert profile["run"][0]["local_run"], "remote lane should retain consumed local lane evidence"
assert "run_pr_fast_test_lane.sh" in summary["command"]
assert ".adl/tmp/validation-manager-nessus-changed-files.txt" in summary["command"]
assert sys.argv[3] not in summary["command"], "remote command must not keep local temp changed-files path"
assert summary["git_ref"] == "origin/main"
assert summary["runner"] == "nessus"
assert summary["status"] == "passed"
PY

docs_only="$TMP/docs-only.txt"
printf 'M\tdocs/README.md\n' >"$docs_only"
if bash "$SCRIPT" \
  --changed-files "$docs_only" \
  --remote-command "printf no-remote-docs" \
  --run >"$TMP/docs.out" 2>"$TMP/docs.err"; then
  echo "expected docs-only Nessus lane request to fail closed" >&2
  exit 1
fi
grep -F "requested remote runner is not eligible" "$TMP/docs.err" >/dev/null

echo "PASS test_run_validation_manager_nessus_lane"
