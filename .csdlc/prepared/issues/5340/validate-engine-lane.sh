#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
repo_root="$(git rev-parse --show-toplevel)"
crate_root="${repo_root}/adl-v2/crates/adl-engine"
manifest="${crate_root}/Cargo.toml"
authority_manifest="${repo_root}/.csdlc/prepared/issues/5340/source-authority-validator/Cargo.toml"
fast_root="${ADL_WP5340_FAST_ROOT:-/Volumes/FastWork/adl-wp-5340}"

mkdir -p "${fast_root}/target" "${fast_root}/cargo-home" "${fast_root}/sccache" "${fast_root}/tmp"
fast_root="$(ruby -e 'puts File.realpath(ARGV.fetch(0))' "${fast_root}")"
case "${fast_root}" in
  /Volumes/FastWork/*) ;;
  *) echo "canonical FastWork root escaped /Volumes/FastWork: ${fast_root}" >&2; exit 26 ;;
esac

export CARGO_TARGET_DIR="${fast_root}/target"
export CARGO_HOME="${fast_root}/cargo-home"
export SCCACHE_DIR="${fast_root}/sccache"
export TMPDIR="${fast_root}/tmp"
export CARGO_INCREMENTAL=0
for variable in CARGO_TARGET_DIR CARGO_HOME SCCACHE_DIR TMPDIR; do
  value="${!variable}"
  canonical="$(ruby -e 'puts File.realpath(ARGV.fetch(0))' "${value}")"
  case "${canonical}" in
    /Volumes/FastWork/*) ;;
    *) echo "${variable} escaped /Volumes/FastWork after canonicalization: ${canonical}" >&2; exit 26 ;;
  esac
done

ruby "${repo_root}/.csdlc/prepared/issues/5340/verify-dependency.rb"
if [[ ! -f "${manifest}" ]]; then
  echo "BLOCKED: #5340 engine manifest does not exist; product implementation remains gated" >&2
  exit 20
fi

case "${mode}" in
  warm-cache)
    export CARGO_NET_OFFLINE=false
    cargo fetch --locked --manifest-path "${manifest}"
    cargo fetch --locked --manifest-path "${authority_manifest}"
    ;;
  focused)
    export CARGO_NET_OFFLINE=true
    cargo test --offline --locked --manifest-path "${manifest}" --all-targets
    ;;
  quality)
    export CARGO_NET_OFFLINE=true
    cargo fmt --manifest-path "${manifest}" -- --check
    ruby "${repo_root}/.csdlc/prepared/issues/5340/validate-source-authority.rb"
    cargo clippy --offline --locked --manifest-path "${manifest}" --all-targets -- -D warnings
    ;;
  determinism)
    export CARGO_NET_OFFLINE=true
    cargo test --offline --locked --manifest-path "${manifest}" \
      --test bounded_schedule --test failure_resume --test port_contracts
    build_events="$(mktemp "${TMPDIR}/fresh-process-build.XXXXXX")"
    cargo test --offline --locked --manifest-path "${manifest}" \
      --test fresh_process_driver --no-run --message-format=json-render-diagnostics >"${build_events}"
    driver="$(jq -r 'select(.reason == "compiler-artifact" and .target.name == "fresh_process_driver" and .profile.test == true and .executable != null) | .executable' "${build_events}" | tail -n 1)"
    if [[ -z "${driver}" || ! -x "${driver}" ]]; then
      echo "fresh_process_driver executable was not produced" >&2
      exit 30
    fi
    first="$(mktemp "${TMPDIR}/fresh-process-first.XXXXXX")"
    second="$(mktemp "${TMPDIR}/fresh-process-second.XXXXXX")"
    "${driver}" >"${first}"
    "${driver}" >"${second}"
    if ! cmp -s "${first}" "${second}"; then
      echo "fresh-process canonical artifacts differ" >&2
      exit 31
    fi
    digest="$(shasum -a 256 "${first}" | awk '{print $1}')"
    printf '{"fresh_process_runs":2,"byte_identical":true,"artifact_sha256":"%s"}\n' "${digest}"
    ;;
  budgets)
    export CARGO_NET_OFFLINE=true
    ruby "${repo_root}/.csdlc/prepared/issues/5340/verify-scope.rb"
    ruby "${repo_root}/.csdlc/prepared/issues/5340/validate-source-authority.rb"
    measurement="$(ruby "${repo_root}/.csdlc/prepared/issues/5340/measure-engine.rb")"
    printf '%s\n' "${measurement}"
    metadata="$(mktemp "${TMPDIR}/cargo-metadata.XXXXXX")"
    cargo metadata --offline --locked --manifest-path "${manifest}" --format-version 1 >"${metadata}"
    ruby "${repo_root}/.csdlc/prepared/issues/5340/validate-cots.rb" "${metadata}"
    cargo test --offline --locked --manifest-path "${manifest}" --all-targets
    ;;
  *)
    echo "unknown engine validation lane: ${mode}" >&2
    exit 64
    ;;
esac
