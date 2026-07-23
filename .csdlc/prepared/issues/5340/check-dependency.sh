#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
source "${repo_root}/.csdlc/prepared/issues/5340/fetch-dependency.sh"
ruby "${repo_root}/.csdlc/prepared/issues/5340/verify-dependency.rb"
