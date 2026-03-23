#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BATCHED_SRC="$ROOT_DIR/adl/tools/batched_checks.sh"
CODEX_PR_SRC="$ROOT_DIR/adl/tools/codex_pr.sh"
CODEXW_SRC="$ROOT_DIR/adl/tools/codexw.sh"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl"
cp "$BATCHED_SRC" "$repo/adl/tools/batched_checks.sh"
cp "$CODEX_PR_SRC" "$repo/adl/tools/codex_pr.sh"
cat > "$repo/adl/tools/codexw.sh" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$repo/adl/tools/batched_checks.sh" "$repo/adl/tools/codex_pr.sh" "$repo/adl/tools/codexw.sh"

fakebin="$tmpdir/fakebin"
mkdir -p "$fakebin"
cat > "$fakebin/cargo" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$fakebin/cargo"

out="$tmpdir/out.log"
(
  cd "$repo"
  PATH="$fakebin:$PATH" bash adl/tools/batched_checks.sh >"$out" 2>&1
)

grep -Fxq "Skipping codex_pr sanity check (no --paths configured)." "$out" || {
  echo "assertion failed: missing skip informational line" >&2
  cat "$out" >&2
  exit 1
}

count="$(grep -Fxc "Skipping codex_pr sanity check (no --paths configured)." "$out")"
[[ "$count" == "1" ]] || {
  echo "assertion failed: expected exactly one skip informational line, got $count" >&2
  cat "$out" >&2
  exit 1
}

if grep -Eq 'Usage:|--paths is required|Missing required --paths' "$out"; then
  echo "assertion failed: codex_pr usage banner leaked into batched checks output" >&2
  cat "$out" >&2
  exit 1
fi

echo "batched checks codex_pr no-paths banner suppression: ok"
