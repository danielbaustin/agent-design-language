#!/usr/bin/env bash
set -euo pipefail

cat >&2 <<'EOF'
ERROR: codexw.sh has been retired and now fails closed.

Use the typed v2 route to bind the issue worktree:
  csdlc-install resolve
  csdlc-bind --root <worktree> --request <bind-request.json>

Then run Codex directly from that worktree as needed. The old codexw wrapper is
no longer a supported workflow surface.
EOF

exit 2
