#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
root="$(git rev-parse --show-toplevel)"
manifest="$root/adl-v2/crates/adl-cli/Cargo.toml"
target="${CARGO_TARGET_DIR:-/Volumes/FastWork/adl-5345/target}"

if [[ ! -f "$manifest" ]]; then
  printf 'WP-10 implementation is dependency-gated; missing %s\n' "$manifest" >&2
  exit 78
fi

export CARGO_TARGET_DIR="$target"
case "$mode" in
  focused)
    cargo test --locked --manifest-path "$manifest" --all-targets
    ;;
  quality)
    cargo fmt --manifest-path "$manifest" --all -- --check
    cargo clippy --locked --manifest-path "$manifest" --all-targets -- -D warnings
    ;;
  budgets)
    ruby "$root/.csdlc/prepared/issues/5345/check-dependencies.rb"
    ruby "$root/.csdlc/prepared/issues/5345/validate-implementation.rb"
    ;;
  post-merge)
    [[ -n "${ADL_WP10_CI_EVIDENCE:-}" && -f "$ADL_WP10_CI_EVIDENCE" ]] || {
      printf 'ADL_WP10_CI_EVIDENCE must name retained exact-head CI JSON\n' >&2
      exit 78
    }
    ruby "$root/.csdlc/prepared/issues/5345/check-dependencies.rb"
    ruby "$root/.csdlc/prepared/issues/5345/validate-implementation.rb"
    ruby -rjson -e 'v=JSON.parse(File.read(ARGV.fetch(0))); abort("CI head mismatch") unless v["head_sha"]==ENV.fetch("ADL_WP10_EXPECTED_HEAD"); required=v.fetch("checks").select{|c| c["requirement"]=="required"}; abort("required CI not green") if required.empty? || required.any?{|c| c["conclusion"]!="success"}' "$ADL_WP10_CI_EVIDENCE"
    ;;
  install-selector)
    install_root="$target/install-root"
    rm -rf "$install_root"
    bash "$root/adl-v2/tools/install-adl-v2.sh" --test-root "$install_root"
    first_digest="$(shasum -a 256 "$install_root/bin/adl-v2" | awk '{print $1}')"
    test -x "$install_root/bin/adl-v2"
    test "$(jq -r .sha256 "$install_root/receipts/adl-v2.json")" = "$first_digest"
    bash "$root/adl-v2/tools/install-adl-v2.sh" --test-root "$install_root" >/dev/null
    test "$(shasum -a 256 "$install_root/bin/adl-v2" | awk '{print $1}')" = "$first_digest"
    "$install_root/bin/adl-v2" select adl-v2 --root "$install_root" >/dev/null
    "$install_root/bin/adl-v2" inspect --root "$install_root" | jq -e '.result.current.generation == "adl-v2"' >/dev/null
    cargo test --locked --manifest-path "$manifest" --test cli selector_select_inspect_and_rollback_are_transactional
    ;;
  *)
    printf 'usage: %s focused|quality|budgets|install-selector|post-merge\n' "$0" >&2
    exit 64
    ;;
esac
