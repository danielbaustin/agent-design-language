#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
check_pr_closing_linkage.sh is retired.
PR closing-linkage checks are now owned by the Rust C-SDLC control plane and
its octocrab-backed PR inspection path. Use `adl/tools/pr.sh finish <issue> ...`
or `adl/tools/pr.sh doctor <issue>` instead of this shell helper.
MSG
exit 2
