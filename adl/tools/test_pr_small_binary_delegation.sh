#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
OBS_SRC="$ROOT_DIR/adl/tools/observability.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl" "$repo/bin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git config commit.gpgsign false
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

doctor_bin="$repo/bin/adl-pr-doctor"
cat >"$doctor_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'doctor:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$doctor_bin"

run_bin="$repo/bin/adl-pr-run"
cat >"$run_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'run:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$run_bin"

broad_bin="$repo/bin/adl-broad"
cat >"$broad_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'broad:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$broad_bin"

doctor_log="$tmpdir/doctor.log"
(
  cd "$repo"
  ADL_TEST_LOG="$doctor_log" \
    ADL_PR_DOCTOR_BIN="$doctor_bin" \
    "$BASH_BIN" adl/tools/pr.sh doctor 3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full >/dev/null
)
grep -Fqx 'doctor:3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full' "$doctor_log" || {
  echo "assertion failed: doctor should delegate directly to adl-pr-doctor without broad 'pr doctor' argv" >&2
  exit 1
}

run_log="$tmpdir/run.log"
(
  cd "$repo"
  ADL_TEST_LOG="$run_log" \
    ADL_PR_RUN_BIN="$run_bin" \
    "$BASH_BIN" adl/tools/pr.sh run 3838 --slug demo --no-fetch-issue --version v0.91.5 >/dev/null
)
grep -Fqx 'run:3838 --slug demo --no-fetch-issue --version v0.91.5' "$run_log" || {
  echo "assertion failed: issue-mode run should delegate directly to adl-pr-run without broad 'pr start' argv" >&2
  exit 1
}

broad_log="$tmpdir/broad.log"
(
  cd "$repo"
  ADL_TEST_LOG="$broad_log" \
    ADL_PR_RUST_BIN="$broad_bin" \
    ADL_PR_DOCTOR_BIN="$doctor_bin" \
    "$BASH_BIN" adl/tools/pr.sh doctor 3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full >/dev/null
)
grep -Fqx 'broad:pr doctor 3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full' "$broad_log" || {
  echo "assertion failed: ADL_PR_RUST_BIN must retain precedence over direct small-binary overrides" >&2
  exit 1
}

echo "pr.sh small-binary delegation: ok"
