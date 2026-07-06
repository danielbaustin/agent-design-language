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

mkdir -p "$worktree/adl/src"
printf 'pub fn finish_owner_last_resort_probe() {}\n' >"$worktree/adl/src/finish_owner_last_resort_probe.rs"
sleep 1
touch "$repo/adl/target/debug/adl-pr-finish"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"
finish_last_resort_log="$tmpdir/finish-last-resort.log"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh finish 4413 --title "worktree finish stale last resort" --output-card out.md >"$finish_last_resort_log" 2>&1
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "finish:4413 --title worktree finish stale last resort --output-card out.md" ]] || {
  echo "assertion failed: worktree finish should use primary adl-pr-finish as dedicated last resort when cargo fallback is disabled" >&2
  echo "$args" >&2
  cat "$finish_last_resort_log" >&2
  exit 1
}
grep -F "freshness=stale_allowed_primary_owner_last_resort" "$finish_last_resort_log" >/dev/null || {
  echo "assertion failed: stale primary owner-binary last resort should be observable" >&2
  cat "$finish_last_resort_log" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when primary finish owner binary is used as disabled-fallback last resort" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}
rm -f "$worktree/adl/src/finish_owner_last_resort_probe.rs"

echo "pr.sh worktree uses primary finish owner binary as explicit disabled-fallback last resort: ok"

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

shepherd_log="$tmpdir/shepherd-generic-fallback.log"
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh shepherd 4413 --slug rust-start --version v0.91.6 --json >"$shepherd_log" 2>&1
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "pr shepherd 4413 --slug rust-start --version v0.91.6 --json" ]] || {
  echo "assertion failed: shepherd should use repo-owned generic adl fallback when dedicated owner binary is missing" >&2
  echo "$args" >&2
  cat "$shepherd_log" >&2
  exit 1
}
grep -F "stage=rust_delegate result=exec subcommand=shepherd" "$shepherd_log" >/dev/null || {
  echo "assertion failed: generic primary checkout adl delegation should be observable for shepherd" >&2
  cat "$shepherd_log" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when shepherd uses generic primary checkout adl fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

echo "pr.sh shepherd uses generic primary checkout adl fallback when dedicated owner binary is missing: ok"
