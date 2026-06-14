#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
check_issue_metadata_parity.sh is retired.
GitHub issue metadata parity is now owned by the Rust C-SDLC control plane and
its octocrab-backed GitHub transport. Use `adl/tools/pr.sh doctor <issue>` for
issue-local readiness or route milestone-wide parity through a new Rust/PVF lane.
MSG
exit 2
