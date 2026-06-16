#!/usr/bin/env bash
set -euo pipefail

cat >&2 <<'EOF'
ERROR: codex_pr.sh has been retired and now fails closed.

Use the repo-native issue workflow instead:
  bash adl/tools/pr.sh init <issue> --slug <slug>
  bash adl/tools/pr.sh doctor <issue> --mode full
  bash adl/tools/pr.sh run <issue>
  bash adl/tools/pr.sh finish <issue> --title "<title>" --paths "<paths>"

This legacy wrapper depended on deprecated pre-run behavior and is kept only to
emit migration guidance.
EOF

exit 2
