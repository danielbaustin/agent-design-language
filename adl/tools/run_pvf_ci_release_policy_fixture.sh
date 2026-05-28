#!/usr/bin/env bash
set -euo pipefail

case "${PVF_LANE_ID:-}" in
  docs_only_pr)
    echo "lane pass"
    ;;
  docs_only_reuse_candidate)
    echo "PVF_STATUS=reused"
    ;;
  runtime_pr_fast)
    echo "lane pass"
    ;;
  authoritative_release_gate)
    echo "lane pass"
    ;;
  *)
    echo "unknown PVF policy fixture lane: ${PVF_LANE_ID:-unset}" >&2
    exit 2
    ;;
esac
