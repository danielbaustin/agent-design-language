#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION="0.56.0"
INSTALL_ROOT="${ADL_VECTOR_INSTALL_ROOT:-$ROOT_DIR/.adl}"
BIN_DIR="$INSTALL_ROOT/bin"
DOWNLOAD_DIR="$INSTALL_ROOT/downloads/vector"
COMPONENT_DIR="$INSTALL_ROOT/components/vector"
PROVENANCE_DIR="$BIN_DIR/.provenance"

case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)
    ARCHIVE_ARCH="arm64-apple-darwin"
    ARCHIVE_SHA256="9aa8b6772d7c887734d38c84eb721d3a067e08a4aa4dc0dcc809365da242ec16"
    ;;
  Linux-aarch64|Linux-arm64)
    ARCHIVE_ARCH="aarch64-unknown-linux-musl"
    ARCHIVE_SHA256="afa383a264e7ab373dac68281cd86fb808f8447bb3813c08b5b0baaae0314a05"
    ;;
  Linux-x86_64)
    ARCHIVE_ARCH="x86_64-unknown-linux-musl"
    ARCHIVE_SHA256="8c114c5e9fd9646516f014d5d837690447cf0d4f43ba4a3746713bc0612b039b"
    ;;
  *)
    echo "install_vector_component: unsupported platform $(uname -s)-$(uname -m)" >&2
    exit 2
    ;;
esac

ARCHIVE="vector-${VERSION}-${ARCHIVE_ARCH}.tar.gz"
URL="https://github.com/vectordotdev/vector/releases/download/v${VERSION}/${ARCHIVE}"
ARCHIVE_PATH="$DOWNLOAD_DIR/$ARCHIVE"
TARGET="$BIN_DIR/vector"
PROVENANCE="$PROVENANCE_DIR/vector.json"

if [[ -x "$TARGET" && -f "$PROVENANCE" ]] &&
  "$TARGET" --version 2>/dev/null | grep -q "vector ${VERSION}" &&
  grep -q "\"archive_sha256\":\"${ARCHIVE_SHA256}\"" "$PROVENANCE" &&
  [[ "$(shasum -a 256 "$TARGET" | awk '{print $1}')" == "$(sed -n 's/.*"binary_sha256":"\([^"]*\)".*/\1/p' "$PROVENANCE")" ]]; then
  echo "vector component unchanged: $TARGET"
  exit 0
fi

mkdir -p "$BIN_DIR" "$DOWNLOAD_DIR" "$COMPONENT_DIR" "$PROVENANCE_DIR"
LOCK_ROOT="$INSTALL_ROOT/locks"
LOCK_DIR="$LOCK_ROOT/vector-install.lock"
mkdir -p "$LOCK_ROOT"
if ! mkdir "$LOCK_DIR" 2>/dev/null; then
  echo "install_vector_component: another verified Vector installation is active" >&2
  exit 75
fi
STAGE=""
cleanup() {
  [[ -z "$STAGE" ]] || rm -rf "$STAGE"
  rmdir "$LOCK_DIR" 2>/dev/null || true
}
trap cleanup EXIT

if [[ ! -f "$ARCHIVE_PATH" ]] ||
  [[ "$(shasum -a 256 "$ARCHIVE_PATH" | awk '{print $1}')" != "$ARCHIVE_SHA256" ]]; then
  rm -f "$ARCHIVE_PATH"
  curl --proto '=https' --tlsv1.2 -sSfL "$URL" -o "$ARCHIVE_PATH"
fi

ACTUAL_SHA256="$(shasum -a 256 "$ARCHIVE_PATH" | awk '{print $1}')"
if [[ "$ACTUAL_SHA256" != "$ARCHIVE_SHA256" ]]; then
  echo "install_vector_component: checksum mismatch for $ARCHIVE" >&2
  exit 1
fi

STAGE="$(mktemp -d "$COMPONENT_DIR/install.XXXXXX")"
tar -xzf "$ARCHIVE_PATH" -C "$STAGE"
SOURCE="$(find "$STAGE" -type f -path '*/bin/vector' -print -quit)"
[[ -n "$SOURCE" && -x "$SOURCE" ]] || {
  echo "install_vector_component: archive does not contain an executable vector binary" >&2
  exit 1
}

NEW_BINARY="$(mktemp "$BIN_DIR/.vector.XXXXXX")"
cp "$SOURCE" "$NEW_BINARY"
chmod 0755 "$NEW_BINARY"
"$NEW_BINARY" --version | grep -q "vector ${VERSION}"
BINARY_SHA256="$(shasum -a 256 "$NEW_BINARY" | awk '{print $1}')"
mv "$NEW_BINARY" "$TARGET"

LICENSE_SOURCE="$(find "$STAGE" -type f -name LICENSE -print -quit)"
if [[ -n "$LICENSE_SOURCE" ]]; then
  mkdir -p "$INSTALL_ROOT/share/vector"
  cp "$LICENSE_SOURCE" "$INSTALL_ROOT/share/vector/LICENSE"
fi

cat >"$PROVENANCE" <<EOF
{"schema":"adl.component.provenance.v1","component":"vector","version":"${VERSION}","platform":"$(uname -s)-$(uname -m)","archive":"${ARCHIVE}","archive_sha256":"${ARCHIVE_SHA256}","binary_sha256":"${BINARY_SHA256}","source":"${URL}","license":"MPL-2.0","installed_ref":".adl/bin/vector"}
EOF

echo "vector component installed: $TARGET"
