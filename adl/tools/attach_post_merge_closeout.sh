#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
attach_post_merge_closeout.sh is retired.
Post-merge closeout watching must move to a Rust/octocrab-backed C-SDLC lane
before automatic attachment is re-enabled. For now, run explicit closeout with:
  adl/tools/pr.sh closeout <issue>
MSG
exit 2
