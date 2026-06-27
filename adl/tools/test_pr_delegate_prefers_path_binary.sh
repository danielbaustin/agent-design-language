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
