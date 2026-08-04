#!/usr/bin/env bash
set -euo pipefail

cat >&2 <<'EOF'
ERROR: codex_pr.sh has been retired and now fails closed.

Use the independent C-SDLC v2 route instead:
  csdlc-install resolve
  csdlc-init --root <worktree> --request <bootstrap-request.json>
  csdlc-bind --root <worktree> --request <bind-request.json>
  csdlc-validate --root <worktree> finalize --request <finalize-request.json>
  csdlc-review record --request <review-request.json>

This legacy wrapper depended on deprecated pre-run behavior and is kept only to
emit migration guidance.
EOF

exit 2
