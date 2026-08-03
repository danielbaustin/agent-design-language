#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
WRAPPER="$ROOT_DIR/adl/tools/run_cargo_validation.sh"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

fake="$tmp_dir/fake command"
cat >"$fake" <<'EOF'
#!/usr/bin/env bash
printf '%s\n%s\n%s\n%s\n%s\n%s\n' \
  "$CARGO_HOME" "$CARGO_TARGET_DIR" \
  "$GIT_AUTHOR_NAME" "$GIT_AUTHOR_EMAIL" \
  "$GIT_COMMITTER_NAME" "$GIT_COMMITTER_EMAIL"
EOF
chmod +x "$fake"

explicit="$tmp_dir/explicit root"
mkdir -p "$explicit"
explicit="$(cd "$explicit" && pwd -P)"
output="$(ADL_CARGO_BUILD_ROOT="$explicit" CARGO_HOME=/bad/home CARGO_TARGET_DIR=/bad/target bash "$WRAPPER" "$fake")"
grep -Fx "$explicit/cargo-home" <<<"$output" >/dev/null
grep -Fx "$explicit/cargo-target" <<<"$output" >/dev/null
grep -Fx "ADL Validation" <<<"$output" >/dev/null
grep -Fx "validation@agent-logic.invalid" <<<"$output" >/dev/null

output="$(
  ADL_CARGO_BUILD_ROOT="$explicit" \
  GIT_AUTHOR_NAME="Caller Author" \
  GIT_AUTHOR_EMAIL="caller-author@example.invalid" \
  GIT_COMMITTER_NAME="Caller Committer" \
  GIT_COMMITTER_EMAIL="caller-committer@example.invalid" \
  bash "$WRAPPER" "$fake"
)"
grep -Fx "Caller Author" <<<"$output" >/dev/null
grep -Fx "caller-author@example.invalid" <<<"$output" >/dev/null
grep -Fx "Caller Committer" <<<"$output" >/dev/null
grep -Fx "caller-committer@example.invalid" <<<"$output" >/dev/null

fallback="$tmp_dir/fast work"
mkdir -p "$fallback"
fallback="$(cd "$fallback" && pwd -P)"
output="$(ADL_FASTWORK_ROOT="$fallback" bash "$WRAPPER" "$fake")"
grep -Fx "$fallback/cargo-home" <<<"$output" >/dev/null
grep -Fx "$fallback/cargo-target" <<<"$output" >/dev/null

ADL_CARGO_BUILD_ROOT="$explicit" bash "$WRAPPER" "$fake" --manifest-path csdlc-v2/Cargo.toml >/dev/null
[[ ! -e "$ROOT_DIR/csdlc-v2/target" ]]
[[ ! -L "$ROOT_DIR/csdlc-v2/target" ]]

if ADL_CARGO_BUILD_ROOT="$ROOT_DIR" bash "$WRAPPER" "$fake" >/dev/null 2>&1; then
  echo "repo-local build root was accepted" >&2
  exit 1
fi
if ADL_CARGO_BUILD_ROOT="$ROOT_DIR/local-target" bash "$WRAPPER" "$fake" >/dev/null 2>&1; then
  echo "repo-descendant build root was accepted" >&2
  exit 1
fi
if ADL_CARGO_BUILD_ROOT="$tmp_dir/missing" bash "$WRAPPER" "$fake" >/dev/null 2>&1; then
  echo "missing build root was accepted" >&2
  exit 1
fi
if ADL_FASTWORK_ROOT="$tmp_dir/missing-fallback" bash "$WRAPPER" "$fake" >/dev/null 2>&1; then
  echo "missing fallback root was accepted" >&2
  exit 1
fi

guard_repo="$tmp_dir/guard-repo"
guard_build="$tmp_dir/guard-build"
mkdir -p "$guard_repo/adl/tools" "$guard_repo/adl" "$guard_build"
cp "$WRAPPER" "$guard_repo/adl/tools/run_cargo_validation.sh"
chmod +x "$guard_repo/adl/tools/run_cargo_validation.sh"
printf 'tracked lock bytes\n' >"$guard_repo/adl/Cargo.lock"
(
  cd "$guard_repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.invalid"
  git add adl/Cargo.lock adl/tools/run_cargo_validation.sh
  git commit -q -m init
)
mkdir -p "$guard_repo/fake-bin"
cat >"$guard_repo/fake-bin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
printf 'cargo ran\n' >"$FAKE_RAN"
printf 'command-created drift\n' >"$TEST_LOCK_PATH"
exit "${FAKE_STATUS:-0}"
EOF_CARGO
chmod +x "$guard_repo/fake-bin/cargo"

if (
  cd "$guard_repo"
  PATH="$guard_repo/fake-bin:$PATH" \
    FAKE_RAN="$tmp_dir/missing-locked-ran" \
    TEST_LOCK_PATH="$guard_repo/adl/Cargo.lock" \
    ADL_CARGO_BUILD_ROOT="$guard_build" \
    bash adl/tools/run_cargo_validation.sh cargo check
) >/dev/null 2>&1; then
  echo "Cargo validation accepted cargo without --locked" >&2
  exit 1
fi
[[ ! -e "$tmp_dir/missing-locked-ran" ]] || {
  echo "Cargo validation executed cargo before rejecting missing --locked" >&2
  exit 1
}

cp "$guard_repo/adl/Cargo.lock" "$tmp_dir/guard-lock.before"
guard_log="$tmp_dir/guard.log"
set +e
(
  cd "$guard_repo"
  PATH="$guard_repo/fake-bin:$PATH" \
    FAKE_RAN="$tmp_dir/guard-ran" \
    TEST_LOCK_PATH="$guard_repo/adl/Cargo.lock" \
    ADL_CARGO_BUILD_ROOT="$guard_build" \
    bash adl/tools/run_cargo_validation.sh cargo check --locked
) >"$guard_log" 2>&1
guard_status=$?
set -e
[[ "$guard_status" -ne 0 ]] || {
  echo "Cargo validation accepted invocation-created lock drift" >&2
  exit 1
}
cmp -s "$tmp_dir/guard-lock.before" "$guard_repo/adl/Cargo.lock" || {
  echo "Cargo validation did not restore exact lock bytes" >&2
  exit 1
}
grep -Fq 'Cargo validation restored invocation-created lockfile drift: adl/Cargo.lock' "$guard_log" || {
  echo "Cargo validation did not report the exact drift path" >&2
  cat "$guard_log" >&2
  exit 1
}

printf 'pre-existing user lock bytes\n' >"$guard_repo/adl/Cargo.lock"
cp "$guard_repo/adl/Cargo.lock" "$tmp_dir/guard-user-lock.before"
set +e
(
  cd "$guard_repo"
  PATH="$guard_repo/fake-bin:$PATH" \
    FAKE_RAN="$tmp_dir/guard-user-ran" \
    TEST_LOCK_PATH="$guard_repo/adl/Cargo.lock" \
    FAKE_STATUS=73 \
    ADL_CARGO_BUILD_ROOT="$guard_build" \
    bash adl/tools/run_cargo_validation.sh cargo check --locked
) >/dev/null 2>&1
guard_user_status=$?
set -e
[[ "$guard_user_status" -ne 0 ]]
cmp -s "$tmp_dir/guard-user-lock.before" "$guard_repo/adl/Cargo.lock" || {
  echo "Cargo validation overwrote pre-existing user lock bytes" >&2
  exit 1
}

escaped="$tmp_dir/escaped"
mkdir -p "$escaped"
symlink_root="$tmp_dir/symlink-root"
mkdir -p "$symlink_root"
ln -s "$escaped" "$symlink_root/cargo-home"
if ADL_CARGO_BUILD_ROOT="$symlink_root" bash "$WRAPPER" "$fake" >/dev/null 2>&1; then
  echo "symlinked Cargo home escaped the selected build root" >&2
  exit 1
fi
rm -f "$symlink_root/cargo-home"
ln -s "$escaped" "$symlink_root/cargo-target"
if ADL_CARGO_BUILD_ROOT="$symlink_root" bash "$WRAPPER" "$fake" >/dev/null 2>&1; then
  echo "symlinked Cargo target escaped the selected build root" >&2
  exit 1
fi

echo "portable Cargo validation wrapper contract: pass"
