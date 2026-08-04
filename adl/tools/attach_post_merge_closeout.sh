#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
attach_post_merge_closeout.sh is retired.
Post-merge closeout watching must move to a Rust/octocrab-backed C-SDLC lane
before automatic attachment is re-enabled. Resolve the selected generation with
`csdlc-install resolve`, then submit explicit typed `csdlc-finish` and
`csdlc-clean cleanup` requests after terminal GitHub truth is available.
MSG
exit 2
