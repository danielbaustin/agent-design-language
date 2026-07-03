#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
PR_DELEGATE_SRC="$ROOT_DIR/adl/tools/pr_delegate.sh"
PR_USAGE_SRC="$ROOT_DIR/adl/tools/pr_usage.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
OBS_SRC="$ROOT_DIR/adl/tools/observability.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

unset ADL_PR_RUST_BIN

repo="$tmpdir/repo"
pathbin="$tmpdir/pathbin"
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$pathbin" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

cat >"$pathbin/adl-pr-doctor" <<'EOF_DOCTOR'
#!/usr/bin/env bash
set -euo pipefail
printf 'path-doctor:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF_DOCTOR
chmod +x "$pathbin/adl-pr-doctor"

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${ADL_TEST_CARGO_ARGS}"
exit 97
EOF_CARGO
chmod +x "$mockbin/cargo"

doctor_log="$tmpdir/doctor.log"
cargo_args="$tmpdir/cargo.args"
: >"$cargo_args"
(
  cd "$repo"
  PATH="$pathbin:$mockbin:$PATH" \
    ADL_TEST_LOG="$doctor_log" \
    ADL_TEST_CARGO_ARGS="$cargo_args" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4590 --slug path-bin --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)
grep -Fqx 'path-doctor:4590 --slug path-bin --no-fetch-issue --version v0.91.6 --mode full' "$doctor_log" || {
  echo "assertion failed: PATH owner binary should receive direct small-binary argv" >&2
  cat "$doctor_log" >&2
  exit 1
}
[[ ! -s "$cargo_args" ]] || {
  echo "assertion failed: cargo should not run when a PATH owner binary exists" >&2
  cat "$cargo_args" >&2
  exit 1
}

echo "pr.sh prefers PATH owner binary: ok"

generic_adl_log="$tmpdir/generic-adl.log"
mkdir -p "$repo/adl/target/debug"
cat >"$repo/adl/target/debug/adl" <<'EOF_GENERIC_ADL'
#!/usr/bin/env bash
set -euo pipefail
printf 'generic-adl:%s\n' "$*" >"${ADL_TEST_GENERIC_ADL_LOG}"
EOF_GENERIC_ADL
chmod +x "$repo/adl/target/debug/adl"
sleep 1
touch "$repo/adl/Cargo.toml"
: >"$cargo_args"

(
  cd "$repo"
  PATH="$pathbin:$mockbin:$PATH" \
    ADL_TEST_GENERIC_ADL_LOG="$generic_adl_log" \
    ADL_TEST_CARGO_ARGS="$cargo_args" \
    "$BASH_BIN" adl/tools/pr.sh watch 4829 --json >/dev/null
)
grep -Fqx 'generic-adl:pr watch 4829 --json' "$generic_adl_log" || {
  echo "assertion failed: generic repo-local adl binary should be the no-cargo fallback for watch" >&2
  cat "$generic_adl_log" >&2
  exit 1
}
[[ ! -s "$cargo_args" ]] || {
  echo "assertion failed: cargo should not run when generic repo-local adl exists for watch" >&2
  cat "$cargo_args" >&2
  exit 1
}

echo "pr.sh uses generic repo-local adl for watch without cargo fallback: ok"

repo_bin_log="$tmpdir/repo-bin-issue.log"
path_issue_log="$tmpdir/path-issue.log"
mkdir -p "$repo/adl/target/debug"
cat >"$repo/adl/target/debug/adl-issue" <<'EOF_ISSUE'
#!/usr/bin/env bash
set -euo pipefail
printf 'repo-issue:%s\n' "$*" >"${ADL_TEST_REPO_ISSUE_LOG}"
EOF_ISSUE
chmod +x "$repo/adl/target/debug/adl-issue"
cat >"$pathbin/adl-issue" <<'EOF_PATH_ISSUE'
#!/usr/bin/env bash
set -euo pipefail
printf 'path-issue:%s\n' "$*" >"${ADL_TEST_PATH_ISSUE_LOG}"
EOF_PATH_ISSUE
chmod +x "$pathbin/adl-issue"
sleep 1
touch "$repo/adl/Cargo.toml"
: >"$cargo_args"

(
  cd "$repo"
  PATH="$pathbin:$mockbin:$PATH" \
    ADL_TEST_REPO_ISSUE_LOG="$repo_bin_log" \
    ADL_TEST_PATH_ISSUE_LOG="$path_issue_log" \
    ADL_TEST_CARGO_ARGS="$cargo_args" \
    "$BASH_BIN" adl/tools/pr.sh issue search "owner binary" --json >/dev/null
)
grep -Fqx 'path-issue:search owner binary --json' "$path_issue_log" || {
  echo "assertion failed: fresh PATH adl-issue should win over stale repo-local adl-issue" >&2
  cat "$path_issue_log" >&2
  exit 1
}
[[ ! -s "$repo_bin_log" ]] || {
  echo "assertion failed: stale repo-local adl-issue should not beat PATH adl-issue" >&2
  cat "$repo_bin_log" >&2
  exit 1
}
[[ ! -s "$cargo_args" ]] || {
  echo "assertion failed: cargo should not run when PATH adl-issue exists" >&2
  cat "$cargo_args" >&2
  exit 1
}

echo "pr.sh prefers PATH adl-issue before stale repo-local owner binary: ok"

rm -f "$pathbin/adl-issue"
: >"$repo_bin_log"
: >"$path_issue_log"
: >"$cargo_args"

(
  cd "$repo"
  PATH="$pathbin:$mockbin:$PATH" \
    ADL_TEST_REPO_ISSUE_LOG="$repo_bin_log" \
    ADL_TEST_PATH_ISSUE_LOG="$path_issue_log" \
    ADL_TEST_CARGO_ARGS="$cargo_args" \
    "$BASH_BIN" adl/tools/pr.sh issue search "owner binary" --json >/dev/null
)
grep -Fqx 'repo-issue:search owner binary --json' "$repo_bin_log" || {
  echo "assertion failed: repo-local adl-issue owner binary should be last-resort issue fallback" >&2
  cat "$repo_bin_log" >&2
  exit 1
}
[[ ! -s "$cargo_args" ]] || {
  echo "assertion failed: cargo should not run when repo-local adl-issue exists" >&2
  cat "$cargo_args" >&2
  exit 1
}

echo "pr.sh prefers repo-local adl-issue owner binary: ok"
