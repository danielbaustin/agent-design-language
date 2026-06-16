#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RETIRED_PR_NAME="codex_pr.sh"
RETIRED_CODEXW_NAME="codexw.sh"
RETIRED_PR_SH="$ROOT_DIR/adl/tools/$RETIRED_PR_NAME"
RETIRED_CODEXW_SH="$ROOT_DIR/adl/tools/$RETIRED_CODEXW_NAME"
BATCHED_SRC="$ROOT_DIR/adl/tools/batched_checks.sh"

if grep -Fq "$RETIRED_PR_NAME" "$ROOT_DIR/adl/tools/batched_checks.sh"; then
  echo "assertion failed: batched_checks.sh still references codex_pr.sh" >&2
  exit 1
fi

if grep -Fq "$RETIRED_CODEXW_NAME" "$ROOT_DIR/adl/tools/batched_checks.sh"; then
  echo "assertion failed: batched_checks.sh still references codexw.sh" >&2
  exit 1
fi

set +e
codex_pr_out="$(sh "$RETIRED_PR_SH" --help 2>&1)"
codex_pr_rc=$?
codexw_out="$(sh "$RETIRED_CODEXW_SH" --help 2>&1)"
codexw_rc=$?
set -e

[[ "$codex_pr_rc" -eq 2 ]] || {
  echo "assertion failed: codex_pr.sh should fail closed with exit 2" >&2
  printf '%s\n' "$codex_pr_out" >&2
  exit 1
}

[[ "$codexw_rc" -eq 2 ]] || {
  echo "assertion failed: codexw.sh should fail closed with exit 2" >&2
  printf '%s\n' "$codexw_out" >&2
  exit 1
}

grep -Fq "retired and now fails closed" <<<"$codex_pr_out" || {
  echo "assertion failed: codex_pr.sh missing fail-closed guidance" >&2
  printf '%s\n' "$codex_pr_out" >&2
  exit 1
}

grep -Fq "retired and now fails closed" <<<"$codexw_out" || {
  echo "assertion failed: codexw.sh missing fail-closed guidance" >&2
  printf '%s\n' "$codexw_out" >&2
  exit 1
}

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools"
cp "$BATCHED_SRC" "$repo/adl/tools/batched_checks.sh"
cat > "$repo/adl/tools/pr.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--help" ]]; then
  echo "fixture pr help"
  exit 0
fi
exit 0
SH

python3 - "$BATCHED_SRC" "$repo" <<'PY'
from pathlib import Path
import re
import sys

source = Path(sys.argv[1]).read_text()
repo = Path(sys.argv[2])
for rel in sorted(set(re.findall(r'adl/tools/([A-Za-z0-9_.-]+\.sh)', source))):
    if rel in {"batched_checks.sh", "pr.sh"}:
        continue
    target = repo / "adl" / "tools" / rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("#!/usr/bin/env bash\nexit 0\n")
    target.chmod(0o755)
PY
chmod +x "$repo/adl/tools/batched_checks.sh" "$repo/adl/tools/pr.sh"

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

if grep -Fq "$RETIRED_PR_NAME" "$out"; then
  echo "assertion failed: batched checks output still mentions codex_pr.sh" >&2
  cat "$out" >&2
  exit 1
fi

if grep -Fq "$RETIRED_CODEXW_NAME" "$out"; then
  echo "assertion failed: batched checks output still mentions codexw.sh" >&2
  cat "$out" >&2
  exit 1
fi

echo "retired codex wrappers fail closed and batched checks no longer depend on them: ok"
