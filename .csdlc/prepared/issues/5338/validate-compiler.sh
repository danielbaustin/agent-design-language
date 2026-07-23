#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
repo_root="$(git rev-parse --show-toplevel)"
manifest="${repo_root}/adl-v2/crates/adl-compiler/Cargo.toml"
crate_root="${repo_root}/adl-v2/crates/adl-compiler"

if [[ ! -f "${manifest}" ]]; then
  echo "BLOCKED: #5338 compiler manifest does not exist; implementation remains gated on #5339 merged plus typed closed_out" >&2
  exit 20
fi

ruby "${repo_root}/.csdlc/prepared/issues/5338/verify-dependency.rb"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/Volumes/FastWork/adl-wp-5338-target}"
case "${CARGO_TARGET_DIR}" in
  /Volumes/FastWork/*) ;;
  *) echo "CARGO_TARGET_DIR must be under /Volumes/FastWork" >&2; exit 26 ;;
esac

run_with_budget() {
  local budget="$1"
  shift
  local start elapsed status
  start="$(date +%s)"
  set +e
  "$@"
  status="$?"
  set -e
  elapsed="$(( $(date +%s) - start ))"
  if (( elapsed > budget )); then
    echo "validation elapsed ${elapsed}s exceeds ${budget}s" >&2
    return 27
  fi
  return "${status}"
}

case "${mode}" in
  focused)
    run_with_budget 120 cargo test --manifest-path "${manifest}" --all-targets
    ;;
  quality)
    run_with_budget 120 cargo clippy --manifest-path "${manifest}" --all-targets -- -D warnings
    ;;
  determinism)
    run_with_budget 300 cargo test --manifest-path "${manifest}" --test deterministic_replay --test stable_identity
    ;;
  budgets)
    start="$(date +%s)"
    implementation_lines="$(find "${crate_root}" -type f \( -path "${crate_root}/src/*" -o -path "${crate_root}/examples/*" -o -path "${crate_root}/build.rs" \) -print0 | sort -z | xargs -0 wc -l | awk 'END {print $1 + 0}')"
    test_lines="$(find "${crate_root}" -type f \( -path "${crate_root}/tests/*" -o -path "${crate_root}/fixtures/*" -o -path "${crate_root}/benches/*" -o -path "${crate_root}/scripts/*" \) -print0 | sort -z | xargs -0 wc -l | awk 'END {print $1 + 0}')"
    unbudgeted_code="$(find "${crate_root}" -type f \( -name '*.rs' -o -name '*.sh' -o -name '*.rb' -o -name '*.py' \) ! -path "${crate_root}/src/*" ! -path "${crate_root}/examples/*" ! -path "${crate_root}/tests/*" ! -path "${crate_root}/fixtures/*" ! -path "${crate_root}/benches/*" ! -path "${crate_root}/scripts/*" ! -path "${crate_root}/build.rs" -print)"
    if [[ -n "${unbudgeted_code}" ]]; then
      echo "unbudgeted code surface detected" >&2
      printf '%s\n' "${unbudgeted_code}" >&2
      exit 28
    fi
    if (( implementation_lines > 3500 )); then
      echo "implementation LoC ${implementation_lines} exceeds 3500" >&2
      exit 21
    fi
    if (( test_lines > 3500 )); then
      echo "test/fixture LoC ${test_lines} exceeds 3500" >&2
      exit 22
    fi
    direct_dependencies="$(cargo metadata --manifest-path "${manifest}" --no-deps --format-version 1 | jq -r '.packages[0].dependencies[].name' | sort -u)"
    reviewed_dependencies="$(printf '%s\n' adl-language hex serde serde_json sha2 | sort)"
    if [[ "${direct_dependencies}" != "${reviewed_dependencies}" ]]; then
      echo "direct dependencies do not exactly match the reviewed COTS set" >&2
      printf 'observed:\n%s\nrequired:\n%s\n' "${direct_dependencies}" "${reviewed_dependencies}" >&2
      exit 23
    fi
    dependencies="$(cargo tree --manifest-path "${manifest}" --edges normal,build,dev --prefix none | tail -n +2)"
    if grep -Eiq '(^|[-_])(runtime|csdlc|tokio|async-std|reqwest|hyper|aws|sqlx|diesel|petgraph|rand)([-_]|$)' <<<"${dependencies}"; then
      echo "forbidden dependency family detected" >&2
      printf '%s\n' "${dependencies}" >&2
      exit 24
    fi
    cargo test --manifest-path "${manifest}" --all-targets
    elapsed="$(( $(date +%s) - start ))"
    if (( elapsed > 600 )); then
      echo "deterministic validation ${elapsed}s exceeds 600s" >&2
      exit 25
    fi
    printf '{"implementation_lines":%s,"test_fixture_lines":%s,"full_validation_seconds":%s}\n' "${implementation_lines}" "${test_lines}" "${elapsed}"
    ;;
  *)
    echo "usage: $0 focused|quality|determinism|budgets" >&2
    exit 64
    ;;
esac
