#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BATCHED_SRC="$ROOT_DIR/swarm/tools/batched_checks.sh"
CODEX_PR_SRC="$ROOT_DIR/swarm/tools/codex_pr.sh"
CODEXW_SRC="$ROOT_DIR/swarm/tools/codexw.sh"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/swarm/tools" "$repo/swarm"
cp "$BATCHED_SRC" "$repo/swarm/tools/batched_checks.sh"
cp "$CODEX_PR_SRC" "$repo/swarm/tools/codex_pr.sh"
cat > "$repo/swarm/tools/codexw.sh" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$repo/swarm/tools/batched_checks.sh" "$repo/swarm/tools/codex_pr.sh" "$repo/swarm/tools/codexw.sh"

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
  PATH="$fakebin:$PATH" bash swarm/tools/batched_checks.sh >"$out" 2>&1
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
