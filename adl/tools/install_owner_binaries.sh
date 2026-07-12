#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
STABLE_BIN_DIR="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}"
SOURCE_BIN_DIR=""
NO_BUILD=0
EXPLICIT_BINS=0
BINS=()
INSTALL_VECTOR_COMPONENT=0

usage() {
  cat <<'EOF' >&2
Usage:
  adl/tools/install_owner_binaries.sh [--bin <name>]... [--stable-bin-dir <dir>] [--source-bin-dir <dir>] [--no-build]

Installs ADL owner binaries into a stable repo-local generated directory outside
Cargo target. Re-running without relevant source changes is a no-op and does
not replace binaries.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      EXPLICIT_BINS=1
      BINS+=("$2")
      shift 2
      ;;
    --stable-bin-dir)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      STABLE_BIN_DIR="$2"
      shift 2
      ;;
    --source-bin-dir)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      SOURCE_BIN_DIR="$2"
      shift 2
      ;;
    --no-build)
      NO_BUILD=1
      shift
      ;;
    -h|--help|help)
      usage
      exit 0
      ;;
    *)
      usage
      echo "install_owner_binaries: unsupported argument '$1'" >&2
      exit 2
      ;;
  esac
done

if [[ "${#BINS[@]}" -eq 0 ]]; then
  BINS=(
    adl csdlc adl-csdlc adl-runtime csm adl-review
    adl-validate-structured-prompt adl-lint-prompt-spec adl-prompt-template
    adl-pr-create adl-pr-init adl-pr-repair-issue-body
    adl-pr-run adl-pr-doctor adl-pr-ready adl-pr-preflight
    adl-pr-finish adl-pr-validation adl-pr-inventory
    adl-pr-shepherd adl-pr-closing-linkage adl-issue adl-pr-closeout
    adl-session adl-process adl-remote adl-aws-remote-validation
    adl-provider-adapter
  )
fi

for bin in "${BINS[@]}"; do
  if [[ "$bin" == "csm" ]]; then
    INSTALL_VECTOR_COMPONENT=1
    break
  fi
done

install_vector_component() {
  if [[ "$INSTALL_VECTOR_COMPONENT" == "1" ]]; then
    ADL_VECTOR_INSTALL_ROOT="$(dirname "$STABLE_BIN_DIR")" \
      bash "$ROOT_DIR/adl/tools/install_vector_component.sh"
  fi
}

source_hash() {
  if git -C "$ROOT_DIR" rev-parse --show-toplevel >/dev/null 2>&1; then
    (
      cd "$ROOT_DIR"
      git ls-files --cached --others --exclude-standard -- adl/Cargo.toml adl/Cargo.lock adl/build.rs adl/src adl/tools/adl_provider_adapter.rs |
        grep -Ev '(^adl/src/cli/tests/|/tests\.rs$|/tests/)' |
        LC_ALL=C sort |
        while IFS= read -r path; do
          [[ -f "$path" ]] || continue
          shasum -a 256 "$path"
        done |
        shasum -a 256 |
        awk '{print $1}'
    )
    return 0
  fi
  (
    cd "$ROOT_DIR"
    find adl -type f \( -path 'adl/src/*' -o -path 'adl/tools/adl_provider_adapter.rs' -o -name Cargo.toml -o -name Cargo.lock -o -name build.rs \) -print 2>/dev/null |
      grep -Ev '(^adl/src/cli/tests/|/tests\.rs$|/tests/)' |
      LC_ALL=C sort |
      while IFS= read -r path; do
        [[ -f "$path" ]] || continue
        shasum -a 256 "$path"
      done |
      shasum -a 256 |
      awk '{print $1}'
  )
}

SOURCE_HASH="$(source_hash)"
BUILD_BINS=()
for bin in "${BINS[@]}"; do
  target="$STABLE_BIN_DIR/$bin"
  provenance="$STABLE_BIN_DIR/.provenance/$bin.sha256"
  if [[ -x "$target" && -f "$provenance" && "$(cat "$provenance" 2>/dev/null || true)" == "$SOURCE_HASH" ]]; then
    echo "owner-binary unchanged: $bin"
    continue
  fi
  BUILD_BINS+=("$bin")
done

if [[ "${#BUILD_BINS[@]}" -eq 0 ]]; then
  install_vector_component
  echo "owner-binary install: all requested binaries are current"
  exit 0
fi

if [[ -z "$SOURCE_BIN_DIR" ]]; then
  SOURCE_BIN_DIR="${CARGO_TARGET_DIR:-$ROOT_DIR/adl/target}/debug"
fi

if [[ "$NO_BUILD" != "1" ]]; then
  cargo_args=(cargo build --quiet --manifest-path "$MANIFEST")
  for bin in "${BUILD_BINS[@]}"; do
    cargo_args+=(--bin "$bin")
  done
  "${cargo_args[@]}"
fi

mkdir -p "$STABLE_BIN_DIR/.provenance"

MISSING_BINS=()
for bin in "${BUILD_BINS[@]}"; do
  src="$SOURCE_BIN_DIR/$bin"
  dest="$STABLE_BIN_DIR/$bin"
  [[ -x "$src" ]] || {
    if [[ "$NO_BUILD" == "1" && "$EXPLICIT_BINS" != "1" ]]; then
      echo "owner-binary source missing; skipped in default --no-build install: $src" >&2
      MISSING_BINS+=("$bin")
      continue
    fi
    echo "install_owner_binaries: missing built source binary: $src" >&2
    exit 1
  }
  tmp="$(mktemp "$STABLE_BIN_DIR/.${bin}.XXXXXX")"
  cp "$src" "$tmp"
  chmod 0755 "$tmp"
  mv "$tmp" "$dest"
  printf '%s\n' "$SOURCE_HASH" >"$STABLE_BIN_DIR/.provenance/$bin.sha256"
  cat >"$STABLE_BIN_DIR/.provenance/$bin.json" <<EOF
{"binary":"$bin","source_hash":"$SOURCE_HASH","build_profile":"debug","platform":"$(uname -s)-$(uname -m)","installed_path":"$dest"}
EOF
  echo "owner-binary installed: $bin -> $dest"
done

if [[ "${#MISSING_BINS[@]}" -gt 0 ]]; then
  echo "install_owner_binaries: incomplete default --no-build install; missing source binaries: ${MISSING_BINS[*]}" >&2
  exit 1
fi

install_vector_component
