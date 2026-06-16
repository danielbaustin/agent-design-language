#!/usr/bin/env bash
set -euo pipefail

cat >&2 <<'EOF'
ERROR: codexw.sh has been retired and now fails closed.

Use the bound issue worktree from:
  bash adl/tools/pr.sh run <issue>

Then run Codex directly from that worktree as needed. The old codexw wrapper is
no longer a supported workflow surface.
EOF

exit 2
