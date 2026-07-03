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
worktree="$repo/.worktrees/issue-4413"
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$repo/adl/target/debug" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"
sleep 1

cat >"$repo/adl/target/debug/adl-pr-doctor" <<'EOF_ADL'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL
chmod +x "$repo/adl/target/debug/adl-pr-doctor"

cat >"$repo/adl/target/debug/adl-pr-finish" <<'EOF_ADL_FINISH'
#!/usr/bin/env bash
set -euo pipefail
printf 'finish:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL_FINISH
chmod +x "$repo/adl/target/debug/adl-pr-finish"

cat >"$repo/adl/target/debug/adl-pr-validation" <<'EOF_ADL_VALIDATION'
#!/usr/bin/env bash
set -euo pipefail
printf 'validation:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL_VALIDATION
chmod +x "$repo/adl/target/debug/adl-pr-validation"

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${TMP_CARGO_ARGS}"
exit 0
EOF_CARGO
chmod +x "$mockbin/cargo"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md adl/tools/pr.sh adl/tools/pr_delegate.sh adl/tools/pr_usage.sh adl/tools/card_paths.sh adl/tools/observability.sh adl/Cargo.toml
  git commit -q -m "init"
  git worktree add -q -b codex/4413 "$worktree" HEAD
)

sleep 1
touch "$repo/adl/target/debug/adl-pr-doctor" "$repo/adl/target/debug/adl-pr-finish" "$repo/adl/target/debug/adl-pr-validation"

TMP_ADL_ARGS="$tmpdir/adl_args.txt"
TMP_CARGO_ARGS="$tmpdir/cargo_args.txt"
export TMP_ADL_ARGS
export TMP_CARGO_ARGS
export PATH="$mockbin:$PATH"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == *"4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full"* ]] || {
  echo "assertion failed: expected worktree doctor delegation through the primary checkout direct binary" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when the primary checkout binary is fresh for the worktree" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

sleep 1
mkdir -p "$worktree/adl/src/cli/pr_cmd/lifecycle"
touch "$worktree/adl/src/cli/pr_cmd/lifecycle/tests.rs"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == *"4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full"* ]] || {
  echo "assertion failed: test-only Rust sources should not block reuse of the primary checkout direct binary" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run for test-only Rust source drift" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

sleep 1
printf '\n# worktree-only manifest drift\n' >>"$worktree/adl/Cargo.toml"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: content-drifted worktree Cargo.toml should block reuse of the primary checkout direct binary" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: content-drifted worktree Cargo.toml should force cargo fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

echo "pr.sh worktree prefers primary checkout built binary: ok"

cp "$repo/adl/Cargo.toml" "$worktree/adl/Cargo.toml"
mkdir -p "$worktree/adl/src"
printf 'pub fn untracked_worktree_input() {}\n' >"$worktree/adl/src/untracked_owner_input.rs"
sleep 1
touch "$repo/adl/target/debug/adl-pr-doctor"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: untracked worktree Rust input should block reuse of the primary checkout direct binary" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: untracked worktree Rust input should force cargo fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}
rm -f "$worktree/adl/src/untracked_owner_input.rs"

: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh finish 4413 --title "worktree finish" --output-card out.md >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "finish:4413 --title worktree finish --output-card out.md" ]] || {
  echo "assertion failed: worktree finish should delegate through the primary checkout adl-pr-finish binary without override" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when the primary checkout finish binary is fresh for the worktree" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh validation 4772 --json >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "validation:4772 --json" ]] || {
  echo "assertion failed: worktree validation should delegate through the primary checkout adl-pr-validation binary without override" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when the primary checkout validation binary is fresh for the worktree" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

echo "pr.sh worktree prefers primary checkout finish/validation owner binaries: ok"

rm -f "$repo/adl/target/debug/adl-pr-doctor"
cat >"$repo/adl/target/debug/adl" <<'EOF_ADL_GENERIC'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL_GENERIC
chmod +x "$repo/adl/target/debug/adl"
sleep 1
touch "$repo/adl/target/debug/adl"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

set +e
shepherd_output="$(
  (
    cd "$worktree" && \
    ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh shepherd 4413 --slug rust-start --version v0.91.6 --json
  ) 2>&1
)"
shepherd_status="$?"
set -e

if [[ "$shepherd_status" -eq 0 ]]; then
  echo "assertion failed: shepherd should not silently fall through to the generic primary checkout adl binary" >&2
  exit 1
fi
[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: generic primary checkout adl binary should not be used for shepherd when the dedicated owner binary is missing" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F "missing dedicated ADL PR owner binary for subcommand 'shepherd'" <<<"$shepherd_output" >/dev/null || {
  echo "assertion failed: expected missing dedicated owner binary guidance for shepherd" >&2
  echo "$shepherd_output" >&2
  exit 1
}

echo "pr.sh shepherd requires dedicated owner binary when generic adl is stale: ok"
