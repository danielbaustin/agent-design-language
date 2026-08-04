#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INSTALL_SRC="$ROOT_DIR/adl/tools/install_owner_binaries.sh"
RESOLUTION_SRC="$ROOT_DIR/adl/tools/owner_binary_resolution.sh"
VALIDATION_SRC="$ROOT_DIR/adl/tools/run_cargo_validation.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

mtime_seconds() {
  if stat -f %m "$1" >/dev/null 2>&1; then
    stat -f %m "$1"
    return 0
  fi
  stat -c %Y "$1"
}

repo="$tmpdir/repo"
source_bin_dir="$tmpdir/source-bins"
mkdir -p "$repo/adl/tools" "$repo/adl/src" "$source_bin_dir"
cp "$INSTALL_SRC" "$repo/adl/tools/install_owner_binaries.sh"
cp "$RESOLUTION_SRC" "$repo/adl/tools/owner_binary_resolution.sh"
cp "$VALIDATION_SRC" "$repo/adl/tools/run_cargo_validation.sh"
chmod +x "$repo/adl/tools/install_owner_binaries.sh"
cat >"$repo/adl/Cargo.toml" <<'EOF_CARGO'
[package]
name = "adl"
version = "0.0.0"
edition = "2021"
EOF_CARGO
printf 'pub fn seed() {}\n' >"$repo/adl/src/lib.rs"
printf '# fixture lock\n' >"$repo/adl/Cargo.lock"
printf 'fn main() {}\n' >"$repo/adl/tools/adl_provider_adapter.rs"
cat >"$source_bin_dir/adl-pr-closeout" <<'EOF_BIN'
#!/usr/bin/env bash
printf 'closeout-v1:%s\n' "$*"
EOF_BIN
chmod +x "$source_bin_dir/adl-pr-closeout"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add adl/Cargo.toml adl/Cargo.lock adl/src/lib.rs adl/tools/adl_provider_adapter.rs adl/tools/install_owner_binaries.sh adl/tools/owner_binary_resolution.sh adl/tools/run_cargo_validation.sh
  git commit -q -m "init"
)

stable_bin="$repo/.adl/bin/adl-pr-closeout"
provenance="$repo/.adl/bin/.provenance/adl-pr-closeout.sha256"

(
  cd "$repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$source_bin_dir" \
    --no-build >/dev/null
)
[[ -x "$stable_bin" ]] || {
  echo "assertion failed: stable owner binary was not installed outside target" >&2
  exit 1
}
[[ -f "$provenance" ]] || {
  echo "assertion failed: stable owner binary provenance was not recorded" >&2
  exit 1
}
[[ "$stable_bin" != *"/target/"* ]] || {
  echo "assertion failed: stable owner binary must not live under target" >&2
  exit 1
}

mtime_before="$(mtime_seconds "$stable_bin")"
sleep 1
(
  cd "$repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$source_bin_dir" \
    --no-build >/dev/null
)
mtime_after_noop="$(mtime_seconds "$stable_bin")"
[[ "$mtime_before" == "$mtime_after_noop" ]] || {
  echo "assertion failed: no-op reinstall replaced an unchanged stable binary" >&2
  exit 1
}

resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
[[ "$resolved" == "$stable_bin" ]] || {
  echo "assertion failed: resolver did not select fresh stable owner binary" >&2
  echo "resolved=$resolved" >&2
  exit 1
}

printf 'pub fn seed() { let _ = 1; }\n' >"$repo/adl/src/lib.rs"
set +e
stale_resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
stale_status=$?
set -e
[[ "$stale_status" -ne 0 && -z "$stale_resolved" ]] || {
  echo "assertion failed: resolver accepted stale stable owner binary after source changed" >&2
  exit 1
}

cat >"$source_bin_dir/adl-pr-closeout" <<'EOF_BIN'
#!/usr/bin/env bash
printf 'closeout-v2:%s\n' "$*"
EOF_BIN
chmod +x "$source_bin_dir/adl-pr-closeout"
sleep 1
(
  cd "$repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$source_bin_dir" \
    --no-build >/dev/null
)
mtime_after_update="$(mtime_seconds "$stable_bin")"
[[ "$mtime_after_update" -gt "$mtime_after_noop" ]] || {
  echo "assertion failed: changed source did not intentionally replace stable binary" >&2
  exit 1
}
grep -Fq 'closeout-v2' "$stable_bin" || {
  echo "assertion failed: stable binary content was not updated after source change" >&2
  exit 1
}

printf 'pub fn untracked_owner_input() {}\n' >"$repo/adl/src/untracked_owner_input.rs"
set +e
untracked_resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
untracked_status=$?
set -e
[[ "$untracked_status" -ne 0 && -z "$untracked_resolved" ]] || {
  echo "assertion failed: resolver accepted stale stable owner binary after untracked source was added" >&2
  exit 1
}
rm -f "$repo/adl/src/untracked_owner_input.rs"

printf 'fn main() { let _provider_source_changed = true; }\n' >"$repo/adl/tools/adl_provider_adapter.rs"
set +e
provider_source_resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
provider_source_status=$?
set -e
[[ "$provider_source_status" -ne 0 && -z "$provider_source_resolved" ]] || {
  echo "assertion failed: resolver accepted stale stable owner binary after provider adapter source changed" >&2
  exit 1
}
git -C "$repo" checkout -- adl/tools/adl_provider_adapter.rs

nogit="$tmpdir/nogit"
mkdir -p "$nogit/adl/tools" "$nogit/adl/src" "$tmpdir/nogit-source-bins"
cp "$INSTALL_SRC" "$nogit/adl/tools/install_owner_binaries.sh"
cp "$RESOLUTION_SRC" "$nogit/adl/tools/owner_binary_resolution.sh"
chmod +x "$nogit/adl/tools/install_owner_binaries.sh"
cp "$repo/adl/Cargo.toml" "$nogit/adl/Cargo.toml"
printf 'pub fn nongit_seed() {}\n' >"$nogit/adl/src/lib.rs"
cat >"$tmpdir/nogit-source-bins/adl-pr-closeout" <<'EOF_BIN'
#!/usr/bin/env bash
printf 'closeout-nongit:%s\n' "$*"
EOF_BIN
chmod +x "$tmpdir/nogit-source-bins/adl-pr-closeout"
(
  cd "$nogit"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$tmpdir/nogit-source-bins" \
    --no-build >/dev/null
)
"$nogit/.adl/bin/adl-pr-closeout" | grep -Fq 'closeout-nongit:' || {
  echo "assertion failed: non-git stable owner binary install did not produce runnable binary" >&2
  exit 1
}

inventory_help="$("$BASH_BIN" "$INSTALL_SRC" --help 2>&1 || true)"
[[ "$inventory_help" == *"install_owner_binaries.sh"* ]] || {
  echo "assertion failed: installer help is not available" >&2
  exit 1
}

default_repo="$tmpdir/default-repo"
default_source_bin_dir="$tmpdir/default-source-bins"
mkdir -p "$default_repo/adl/tools" "$default_repo/adl/src" "$default_source_bin_dir"
cp "$INSTALL_SRC" "$default_repo/adl/tools/install_owner_binaries.sh"
cp "$RESOLUTION_SRC" "$default_repo/adl/tools/owner_binary_resolution.sh"
chmod +x "$default_repo/adl/tools/install_owner_binaries.sh"
cat >"$default_repo/adl/tools/install_vector_component.sh" <<'EOF_VECTOR'
#!/usr/bin/env bash
exit 0
EOF_VECTOR
chmod +x "$default_repo/adl/tools/install_vector_component.sh"
cp "$repo/adl/Cargo.toml" "$default_repo/adl/Cargo.toml"
printf 'pub fn default_seed() {}\n' >"$default_repo/adl/src/lib.rs"
default_bins=(
  adl adl-runtime csm csmctl adl-review adl-process adl-remote
  adl-aws-remote-validation adl-provider-adapter
)
for bin in "${default_bins[@]}"; do
  cat >"$default_source_bin_dir/$bin" <<EOF_BIN
#!/usr/bin/env bash
printf '${bin}-default:%s\n' "\$*"
EOF_BIN
  chmod +x "$default_source_bin_dir/$bin"
done
(
  cd "$default_repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --source-bin-dir "$default_source_bin_dir" \
    --no-build >/dev/null
)
[[ -x "$default_repo/.adl/bin/csm" ]] || {
  echo "assertion failed: default stable owner binary install omitted csm" >&2
  exit 1
}
"$default_repo/.adl/bin/csm" | grep -Fq 'csm-default:' || {
  echo "assertion failed: default stable csm binary install did not produce runnable csm" >&2
  exit 1
}

incomplete_repo="$tmpdir/incomplete-default-repo"
incomplete_source_bins="$tmpdir/incomplete-default-source-bins"
mkdir -p "$incomplete_repo/adl/tools" "$incomplete_repo/adl/src" "$incomplete_source_bins"
cp "$INSTALL_SRC" "$incomplete_repo/adl/tools/install_owner_binaries.sh"
chmod +x "$incomplete_repo/adl/tools/install_owner_binaries.sh"
cat >"$incomplete_repo/adl/tools/install_vector_component.sh" <<'EOF_VECTOR'
#!/usr/bin/env bash
exit 0
EOF_VECTOR
chmod +x "$incomplete_repo/adl/tools/install_vector_component.sh"
cp "$repo/adl/Cargo.toml" "$incomplete_repo/adl/Cargo.toml"
printf 'pub fn incomplete_default_seed() {}\n' >"$incomplete_repo/adl/src/lib.rs"
for bin in adl csm csmctl adl-remote adl-aws-remote-validation adl-provider-adapter; do
  cat >"$incomplete_source_bins/$bin" <<EOF_BIN
#!/usr/bin/env bash
printf '$bin:%s\n' "\$*"
EOF_BIN
  chmod +x "$incomplete_source_bins/$bin"
done
default_install_log="$tmpdir/default-install.log"
set +e
(
  cd "$incomplete_repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --source-bin-dir "$incomplete_source_bins" \
    --no-build >"$default_install_log" 2>&1
)
default_install_status=$?
set -e
[[ "$default_install_status" -ne 0 ]] || {
  echo "assertion failed: incomplete default no-build install should return nonzero" >&2
  cat "$default_install_log" >&2
  exit 1
}
for bin in adl csm csmctl adl-remote adl-aws-remote-validation adl-provider-adapter; do
  [[ -x "$incomplete_repo/.adl/bin/$bin" ]] || {
    echo "assertion failed: default no-build install did not install current owner binary $bin" >&2
    cat "$default_install_log" >&2
    exit 1
  }
done
grep -Fq "owner-binary source missing; skipped in default --no-build install" "$default_install_log" || {
  echo "assertion failed: default no-build install should report skipped missing default binaries" >&2
  cat "$default_install_log" >&2
  exit 1
}
grep -Fq "install_owner_binaries: incomplete default --no-build install" "$default_install_log" || {
  echo "assertion failed: default no-build install should report incomplete install summary" >&2
  cat "$default_install_log" >&2
  exit 1
}

explicit_missing_log="$tmpdir/explicit-missing.log"
set +e
(
  cd "$incomplete_repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin definitely-missing-owner-binary \
    --source-bin-dir "$incomplete_source_bins" \
    --no-build >"$explicit_missing_log" 2>&1
)
explicit_missing_status=$?
set -e
[[ "$explicit_missing_status" -ne 0 ]] || {
  echo "assertion failed: explicit missing no-build binary should fail closed" >&2
  cat "$explicit_missing_log" >&2
  exit 1
}
grep -Fq "install_owner_binaries: missing built source binary" "$explicit_missing_log" || {
  echo "assertion failed: explicit missing no-build binary should report missing source" >&2
  cat "$explicit_missing_log" >&2
  exit 1
}

fake_cargo_dir="$tmpdir/fake-cargo-bin"
owner_build_root="$tmpdir/owner-build"
mkdir -p "$fake_cargo_dir" "$owner_build_root"
cat >"$fake_cargo_dir/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"$FAKE_CARGO_LOG"
printf 'invocation-created lock drift\n' >"$TEST_LOCK_PATH"
echo "error: no bin target named removed-owner-target" >&2
exit "${FAKE_CARGO_STATUS:-101}"
EOF_CARGO
chmod +x "$fake_cargo_dir/cargo"

assert_lock_restored_after_build() {
  local expected_file="$1"
  local fake_status="$2"
  local output_file="$3"
  set +e
  (
    cd "$repo"
    PATH="$fake_cargo_dir:$PATH" \
      TEST_LOCK_PATH="$repo/adl/Cargo.lock" \
      FAKE_CARGO_LOG="$tmpdir/fake-cargo.log" \
      FAKE_CARGO_STATUS="$fake_status" \
      ADL_OWNER_BUILD_ROOT="$owner_build_root" \
      "$BASH_BIN" adl/tools/install_owner_binaries.sh --bin adl >"$output_file" 2>&1
  )
  local status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: lock-mutating build unexpectedly passed" >&2
    cat "$output_file" >&2
    exit 1
  }
  cmp -s "$expected_file" "$repo/adl/Cargo.lock" || {
    echo "assertion failed: build did not restore exact pre-invocation Cargo.lock bytes" >&2
    exit 1
  }
  grep -Fq "Cargo validation restored invocation-created lockfile drift: adl/Cargo.lock" "$output_file" || {
    echo "assertion failed: lock drift was not reported with its exact path" >&2
    cat "$output_file" >&2
    exit 1
  }
}

cp "$repo/adl/Cargo.lock" "$tmpdir/clean-lock.before"
assert_lock_restored_after_build "$tmpdir/clean-lock.before" 101 "$tmpdir/removed-target.log"
grep -Fq -- '--locked' "$tmpdir/fake-cargo.log" || {
  echo "assertion failed: owner installer cargo build omitted --locked" >&2
  exit 1
}
grep -Fq -- '--bin adl' "$tmpdir/fake-cargo.log" || {
  echo "assertion failed: focused owner installer did not request the selected current target" >&2
  exit 1
}

printf 'user-owned pre-existing lock bytes\n' >"$repo/adl/Cargo.lock"
cp "$repo/adl/Cargo.lock" "$tmpdir/user-lock.before"
assert_lock_restored_after_build "$tmpdir/user-lock.before" 0 "$tmpdir/dependency-drift.log"

echo "owner binary stable install: ok"
