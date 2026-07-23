#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
git -C "${repo_root}" fetch origin refs/heads/main:refs/remotes/origin/main
export ADL_WP5340_EXPECTED_ORIGIN_MAIN_SHA
ADL_WP5340_EXPECTED_ORIGIN_MAIN_SHA="$(git -C "${repo_root}" rev-parse origin/main^{commit})"
export ADL_WP5340_FETCHED_UNIX_SECONDS
ADL_WP5340_FETCHED_UNIX_SECONDS="$(date +%s)"
