#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
closeout_completed_issue_wave.sh is retired.
Wave closeout must move to a Rust/octocrab-backed C-SDLC lane before it can be
used again. Use `adl/tools/pr.sh closeout <issue>` for explicit issue closeout.
MSG
exit 2
