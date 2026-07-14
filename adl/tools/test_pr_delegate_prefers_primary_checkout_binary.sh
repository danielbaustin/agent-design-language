#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
PR_DELEGATE_SRC="$ROOT_DIR/adl/tools/pr_delegate.sh"
PR_USAGE_SRC="$ROOT_DIR/adl/tools/pr_usage.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
OBS_SRC="$ROOT_DIR/adl/tools/observability.sh"
OWNER_RESOLUTION_SRC="$ROOT_DIR/adl/tools/owner_binary_resolution.sh"

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
cp "$OWNER_RESOLUTION_SRC" "$repo/adl/tools/owner_binary_resolution.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"
mkdir -p "$repo/adl/src"
printf 'pub fn primary_seed() {}\n' >"$repo/adl/src/lib.rs"
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
  git add README.md adl/tools/pr.sh adl/tools/pr_delegate.sh adl/tools/pr_usage.sh adl/tools/card_paths.sh adl/tools/observability.sh adl/tools/owner_binary_resolution.sh adl/Cargo.toml adl/src/lib.rs
  git commit -q -m "init"
  git worktree add -q -b codex/4413 "$worktree" HEAD
)

sleep 1
touch "$repo/adl/target/debug/adl-pr-doctor" "$repo/adl/target/debug/adl-pr-finish" "$repo/adl/target/debug/adl-pr-validation"

TMP_ADL_ARGS="$tmpdir/adl_args.txt"
TMP_CARGO_ARGS="$tmpdir/cargo_args.txt"
export TMP_ADL_ARGS
export TMP_CARGO_ARGS
export ADL_OBSERVABILITY=0
export PATH="$mockbin:$PATH"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
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
    ./adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
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
    ./adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
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
    ./adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
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

rm -f "$repo/adl/target/debug/adl-pr-doctor"
mkdir -p "$repo/.adl/bin/.provenance"
cat >"$repo/.adl/bin/adl-pr-doctor" <<'EOF_STABLE_DOCTOR'
#!/usr/bin/env bash
set -euo pipefail
printf 'stable:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_STABLE_DOCTOR
chmod +x "$repo/.adl/bin/adl-pr-doctor"
primary_source_hash="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  adl_owner_source_hash "$repo"
)"
printf '%s\n' "$primary_source_hash" >"$repo/.adl/bin/.provenance/adl-pr-doctor.sha256"
rm -f "$worktree/adl/src/untracked_owner_input.rs"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == *"stable:4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full"* ]] || {
  echo "assertion failed: drift-free worktree should reuse fresh primary stable owner binary" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when the primary stable owner binary is fresh for the worktree" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

printf 'pub fn stable_primary_masking_probe() {}\n' >"$worktree/adl/src/stable_primary_masking_probe.rs"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ./adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: worktree Rust drift should block reuse of primary stable owner binary" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: worktree Rust drift should force cargo fallback instead of primary stable owner binary" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}
rm -f "$worktree/adl/src/stable_primary_masking_probe.rs"

cat >"$repo/.adl/bin/adl-pr-finish" <<'EOF_STABLE_FINISH'
#!/usr/bin/env bash
set -euo pipefail
printf 'stable-finish:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_STABLE_FINISH
chmod +x "$repo/.adl/bin/adl-pr-finish"
primary_source_hash="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  adl_owner_source_hash "$repo"
)"
printf '%s\n' "$primary_source_hash" >"$repo/.adl/bin/.provenance/adl-pr-finish.sha256"
sleep 1
touch "$repo/adl/target/debug/adl-pr-finish"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

stable_finish_log="$tmpdir/stable-finish.log"
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh finish 4413 --title "stable finish" --output-card out.md >"$stable_finish_log" 2>&1
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "stable-finish:4413 --title stable finish --output-card out.md" ]] || {
  echo "assertion failed: finish should prefer fresh installed owner binary over stale primary target fallback" >&2
  echo "$args" >&2
  cat "$stable_finish_log" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when fresh installed finish owner binary exists" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

printf 'stale-installed-owner-binary\n' >"$repo/.adl/bin/.provenance/adl-pr-finish.sha256"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"
stale_stable_finish_log="$tmpdir/stale-stable-finish.log"

set +e
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh finish 4413 --title "stale installed finish" --output-card out.md >"$stale_stable_finish_log" 2>&1
)
stale_stable_finish_status="$?"
set -e

[[ "$stale_stable_finish_status" == "75" ]] || {
  echo "assertion failed: stale installed finish owner binary should fail closed instead of using stale target fallback" >&2
  cat "$stale_stable_finish_log" >&2
  exit 1
}
[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: stale primary finish target must not run while installed owner binary is stale" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F "installed ADL PR owner binary for subcommand 'finish' is present but not current" "$stale_stable_finish_log" >/dev/null || {
  echo "assertion failed: stale installed finish owner binary should emit freshness diagnostic" >&2
  cat "$stale_stable_finish_log" >&2
  exit 1
}
printf '%s\n' "$primary_source_hash" >"$repo/.adl/bin/.provenance/adl-pr-finish.sha256"

mkdir -p "$worktree/adl/target/debug"
cat >"$worktree/adl/target/debug/adl-pr-finish" <<'EOF_WORKTREE_FINISH'
#!/usr/bin/env bash
set -euo pipefail
printf 'worktree-finish:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_WORKTREE_FINISH
chmod +x "$worktree/adl/target/debug/adl-pr-finish"
sleep 1
touch "$worktree/adl/target/debug/adl-pr-finish"
printf 'stale-installed-owner-binary\n' >"$repo/.adl/bin/.provenance/adl-pr-finish.sha256"

worktree_finish_log="$tmpdir/worktree-finish.log"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh finish 4413 --title "worktree finish" --output-card out.md >"$worktree_finish_log" 2>&1
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "worktree-finish:4413 --title worktree finish --output-card out.md" ]] || {
  echo "assertion failed: finish should prefer the fresh bound-worktree owner binary over stale installed and primary checkout binaries" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when the bound-worktree finish binary is fresh" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}
printf '%s\n' "$primary_source_hash" >"$repo/.adl/bin/.provenance/adl-pr-finish.sha256"

printf 'pub fn stale_worktree_finish_probe() {}\n' >"$worktree/adl/src/stale_worktree_finish_probe.rs"
sleep 1
touch "$worktree/adl/src/stale_worktree_finish_probe.rs"
touch "$repo/adl/target/debug/adl-pr-finish"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"
stale_worktree_finish_log="$tmpdir/stale-worktree-finish.log"

set +e
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh finish 4413 --title "stale worktree finish" --output-card out.md >"$stale_worktree_finish_log" 2>&1
)
stale_worktree_finish_status="$?"
set -e

[[ "$stale_worktree_finish_status" == "75" ]] || {
  echo "assertion failed: stale bound-worktree finish binary should fail closed with exit 75" >&2
  cat "$stale_worktree_finish_log" >&2
  exit 1
}
[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: stale primary finish binary must not run for divergent worktree inputs" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F "bound worktree has finish-support changes not represented by a fresh adl-pr-finish binary" "$stale_worktree_finish_log" >/dev/null || {
  echo "assertion failed: stale bound-worktree finish failure should be classified" >&2
  cat "$stale_worktree_finish_log" >&2
  exit 1
}
grep -F "ADL_PR_FINISH_BIN=" "$stale_worktree_finish_log" >/dev/null || {
  echo "assertion failed: stale bound-worktree finish failure should explain the explicit repair override" >&2
  cat "$stale_worktree_finish_log" >&2
  exit 1
}

: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"
cat >"$repo/adl/target/debug/adl" <<'EOF_ADL_GENERIC_BEFORE_FINISH_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf 'generic-before-finish-cargo:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL_GENERIC_BEFORE_FINISH_CARGO
chmod +x "$repo/adl/target/debug/adl"
sleep 1
touch "$repo/adl/target/debug/adl"
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ./adl/tools/pr.sh finish 4413 --title "cargo fallback finish" --output-card out.md >/dev/null
)

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: explicit cargo fallback should not run a stale finish target" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-finish -- 4413 --title cargo fallback finish --output-card out.md" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: explicit cargo fallback should run bound-worktree adl-pr-finish" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}
rm -f "$repo/adl/target/debug/adl"
rm -f "$worktree/adl/src/stale_worktree_finish_probe.rs"

rm -f "$worktree/adl/target/debug/adl-pr-finish"

: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh validation 4772 --json >/dev/null
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

echo "pr.sh worktree prefers its current finish binary and reuses the primary validation binary: ok"

cat >"$repo/adl/target/debug/adl-issue" <<'EOF_ADL_ISSUE'
#!/usr/bin/env bash
set -euo pipefail
printf 'issue-target:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL_ISSUE
chmod +x "$repo/adl/target/debug/adl-issue"
cat >"$repo/.adl/bin/adl-issue" <<'EOF_STABLE_ISSUE'
#!/usr/bin/env bash
set -euo pipefail
printf 'stable-issue:%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_STABLE_ISSUE
chmod +x "$repo/.adl/bin/adl-issue"
printf '%s\n' "$primary_source_hash" >"$repo/.adl/bin/.provenance/adl-issue.sha256"
sleep 1
touch "$repo/adl/target/debug/adl-issue"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

issue_log="$tmpdir/stable-issue.log"
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh issue view 4413 --json >"$issue_log" 2>&1
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "stable-issue:view 4413 --json" ]] || {
  echo "assertion failed: issue command should prefer fresh installed owner binary over stale target fallback" >&2
  echo "$args" >&2
  cat "$issue_log" >&2
  exit 1
}
printf 'stale-installed-owner-binary\n' >"$repo/.adl/bin/.provenance/adl-issue.sha256"
: >"$TMP_ADL_ARGS"
stale_issue_log="$tmpdir/stale-issue.log"
set +e
(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    ./adl/tools/pr.sh issue view 4413 --json >"$stale_issue_log" 2>&1
)
stale_issue_status="$?"
set -e

[[ "$stale_issue_status" == "75" ]] || {
  echo "assertion failed: stale installed issue owner binary should fail closed instead of using stale target fallback" >&2
  cat "$stale_issue_log" >&2
  exit 1
}
[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: stale issue target must not run while installed owner binary is stale" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F "installed ADL PR owner binary for subcommand 'issue' is present but not current" "$stale_issue_log" >/dev/null || {
  echo "assertion failed: stale installed issue owner binary should emit freshness diagnostic" >&2
  cat "$stale_issue_log" >&2
  exit 1
}

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
    ./adl/tools/pr.sh shepherd 4413 --slug rust-start --version v0.91.6 --json >"$shepherd_log" 2>&1
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == "pr shepherd 4413 --slug rust-start --version v0.91.6 --json" ]] || {
  echo "assertion failed: shepherd should use repo-owned generic adl fallback when dedicated owner binary is missing" >&2
  echo "$args" >&2
  cat "$shepherd_log" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when shepherd uses generic primary checkout adl fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

echo "pr.sh shepherd uses generic primary checkout adl fallback when dedicated owner binary is missing: ok"
