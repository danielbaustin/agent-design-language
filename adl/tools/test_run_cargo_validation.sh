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
