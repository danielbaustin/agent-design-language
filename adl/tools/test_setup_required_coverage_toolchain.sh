#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/setup_required_coverage_toolchain.sh"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
mkdir -p "$tmp_dir/bin" "$tmp_dir/home"

cat > "$tmp_dir/bin/sudo" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [ "${1:-}" = "find" ]; then
  exit 0
fi
if [ "${1:-}" = "apt-get" ]; then
  shift
  case "${1:-}" in
    update)
      exit 0
      ;;
    install)
      cat > "$FAKE_BIN/ld.lld" <<'LLD'
#!/usr/bin/env bash
echo "LLD 17.0.0 fake installed"
LLD
      chmod +x "$FAKE_BIN/ld.lld"
      exit 0
      ;;
  esac
fi
exec "$@"
SH
cat > "$tmp_dir/bin/sccache" <<'SH'
#!/usr/bin/env bash
case "$1" in
  --version) echo "sccache 0.8.0 fake" ;;
  --start-server|--zero-stats|--show-stats) exit 0 ;;
  *) exit 2 ;;
esac
SH
cat > "$tmp_dir/bin/rustc" <<'SH'
#!/usr/bin/env bash
echo "rustc fake"
SH
cat > "$tmp_dir/bin/cargo" <<'SH'
#!/usr/bin/env bash
case "$1" in
  --version) echo "cargo fake" ;;
  llvm-cov) echo "cargo-llvm-cov fake" ;;
  nextest) echo "cargo-nextest fake" ;;
  *) exit 2 ;;
esac
SH
chmod +x "$tmp_dir/bin"/*

env_file="$tmp_dir/github-env"
FAKE_BIN="$tmp_dir/bin" PATH="$tmp_dir/bin:$PATH" HOME="$tmp_dir/home" "$SCRIPT" install-lld >/dev/null
test -x "$tmp_dir/bin/ld.lld"
PATH="$tmp_dir/bin:$PATH" HOME="$tmp_dir/home" "$SCRIPT" configure "$env_file" >/dev/null
PATH="$tmp_dir/bin:$PATH" HOME="$tmp_dir/home" "$SCRIPT" verify >/dev/null
PATH="$tmp_dir/bin:$PATH" HOME="$tmp_dir/home" RUST_LINK_ACCEL=lld "$SCRIPT" stats >/dev/null

grep -Fx 'RUSTC_WRAPPER=sccache' "$env_file" >/dev/null
grep -Fx 'RUSTFLAGS=-C link-arg=-fuse-ld=lld' "$env_file" >/dev/null
grep -Fx 'RUST_LINK_ACCEL=lld' "$env_file" >/dev/null

if PATH="/usr/bin:/bin" HOME="$tmp_dir/home" "$SCRIPT" verify >/dev/null 2>&1; then
  echo "expected verify to fail without required fake tools" >&2
  exit 1
fi

echo "PASS test_setup_required_coverage_toolchain"
