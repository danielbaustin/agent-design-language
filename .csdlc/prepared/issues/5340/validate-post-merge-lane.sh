#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
reviewed_head="${ADL_WP5340_REVIEWED_HEAD_SHA:?typed reviewed PR head SHA is required}"
expected_sha="${ADL_WP5340_INTEGRATION_SHA:?captured current-main integration SHA is required}"
actual_sha="$(git -C "${repo_root}" rev-parse HEAD^{commit})"
[[ "${actual_sha}" == "${expected_sha}" ]] || { echo "detached checkout is not the captured integration SHA" >&2; exit 40; }
git -C "${repo_root}" merge-base --is-ancestor "${reviewed_head}" "${actual_sha}" || { echo "reviewed PR head is not integrated" >&2; exit 44; }
[[ -z "$(git -C "${repo_root}" status --short)" ]] || { echo "detached post-merge validation tree is dirty" >&2; exit 41; }

fast_root="${ADL_WP5340_FAST_ROOT:?post-merge FastWork build root is required}"
postmerge_cargo_home="${ADL_WP5340_CARGO_HOME:-/Volumes/FastWork/adl-wp-5340/cargo-home}"
mkdir -p "${fast_root}/target" "${fast_root}/sccache" "${fast_root}/tmp" "${postmerge_cargo_home}"
fast_root="$(ruby -e 'puts File.realpath(ARGV.fetch(0))' "${fast_root}")"
case "${fast_root}" in /Volumes/FastWork/*) ;; *) echo "post-merge build root escaped FastWork" >&2; exit 42 ;; esac
export CARGO_TARGET_DIR="${fast_root}/target"
export CARGO_HOME="$(ruby -e 'puts File.realpath(ARGV.fetch(0))' "${postmerge_cargo_home}")"
export SCCACHE_DIR="${fast_root}/sccache"
export TMPDIR="${fast_root}/tmp"
case "${CARGO_HOME}" in /Volumes/FastWork/*) ;; *) echo "post-merge Cargo home escaped FastWork" >&2; exit 43 ;; esac
export CARGO_INCREMENTAL=0
export CARGO_NET_OFFLINE=true

manifest="${repo_root}/adl-v2/crates/adl-engine/Cargo.toml"
metadata="$(mktemp "${TMPDIR}/postmerge-metadata.XXXXXX")"
cargo fmt --manifest-path "${manifest}" -- --check
ruby "${repo_root}/.csdlc/prepared/issues/5340/validate-source-authority.rb"
cargo clippy --offline --locked --manifest-path "${manifest}" --all-targets -- -D warnings
cargo test --offline --locked --manifest-path "${manifest}" --all-targets
ruby "${repo_root}/.csdlc/prepared/issues/5340/measure-engine.rb"
cargo metadata --offline --locked --manifest-path "${manifest}" --format-version 1 >"${metadata}"
ruby "${repo_root}/.csdlc/prepared/issues/5340/validate-cots.rb" "${metadata}"
printf '{"schema":"adl.wp06.post-merge-tree.v2","reviewed_head_sha":"%s","integration_sha":"%s","dirty":false,"outcome":"passed"}\n' "${reviewed_head}" "${actual_sha}"
