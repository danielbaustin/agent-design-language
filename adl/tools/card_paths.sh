#!/usr/bin/env bash
set -euo pipefail

card_primary_checkout_root() {
  local common top current_top current_base current_parent
  current_top="$(git rev-parse --show-toplevel 2>/dev/null || pwd -P)"
  current_base="$(basename "$current_top")"
  current_parent="$(basename "$(dirname "$current_top")")"

  # Repo-local execution clones/worktrees under .worktrees/adl-wp-* should keep
  # their local ADL artifact state inside that execution surface, not spill back
  # into the shared primary checkout.
  if [[ "$current_parent" == ".worktrees" && "$current_base" == adl-wp-* ]]; then
    printf '%s\n' "$current_top"
    return 0
  fi

  common="$(git rev-parse --git-common-dir 2>/dev/null || true)"
  if [[ -z "$common" ]]; then
    printf '%s\n' "$current_top"
    return 0
  fi

  if [[ "$common" != /* ]]; then
    top="$current_top"
    common="$(cd "$top/$common" && pwd -P)"
  fi

  # For any non-primary git worktree, keep local ADL artifact state inside the
  # active worktree rather than redirecting it back into the shared checkout.
  if [[ "$current_top" != "$(cd "$common/.." && pwd -P)" ]]; then
    printf '%s\n' "$current_top"
    return 0
  fi

  cd "$common/.." && pwd -P
}

cards_root_resolve() {
  local root
  root="$(card_primary_checkout_root)"
  if [[ -n "${ADL_CARDS_ROOT:-}" ]]; then
    if [[ "${ADL_CARDS_ROOT}" == /* ]]; then
      echo "${ADL_CARDS_ROOT}"
    else
      echo "$root/${ADL_CARDS_ROOT}"
    fi
    return 0
  fi
  echo "$root/.adl/cards"
}

task_bundles_root_resolve() {
  local scope="${1:-}"
  local root
  root="$(card_primary_checkout_root)"
  [[ -n "$scope" ]] || { echo "missing scope" >&2; return 1; }
  echo "$root/.adl/$scope/tasks"
}

card_issue_normalize() {
  local raw="$1"
  if [[ ! "$raw" =~ ^[0-9]+$ ]]; then
    echo "invalid issue number: $raw" >&2
    return 1
  fi
  echo "$((10#$raw))"
}

card_issue_pad() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  printf '%04d' "$issue"
}

task_issue_id() {
  local issue
  issue="$(card_issue_pad "$1")" || return 1
  echo "issue-$issue"
}

task_bundle_dir_name() {
  local issue="$1" slug="$2"
  [[ -n "$slug" ]] || { echo "missing slug" >&2; return 1; }
  echo "$(task_issue_id "$issue")__${slug}"
}

task_bundle_dir_path() {
  local issue="$1" scope="$2" slug="$3"
  echo "$(task_bundles_root_resolve "$scope")/$(task_bundle_dir_name "$issue" "$slug")"
}

task_bundle_first_dir() {
  local issue="$1" scope="${2:-}"
  local issue_id root pattern
  local -a matches=()
  issue_id="$(task_issue_id "$issue")" || return 1
  root="$(card_primary_checkout_root)"

  shopt -s nullglob
  if [[ -n "$scope" ]]; then
    pattern="$root/.adl/$scope/tasks/${issue_id}__*"
    matches=($pattern)
  else
    matches=("$root"/.adl/*/tasks/"${issue_id}"__*)
  fi
  shopt -u nullglob

  if [[ ${#matches[@]} -gt 0 ]]; then
    printf '%s\n' "${matches[@]}" | LC_ALL=C sort | head -n1
    return 0
  fi

  return 1
}

task_bundle_input_path() {
  local issue="$1" scope="$2" slug="$3"
  echo "$(task_bundle_dir_path "$issue" "$scope" "$slug")/sip.md"
}

task_bundle_output_path() {
  local issue="$1" scope="$2" slug="$3"
  echo "$(task_bundle_dir_path "$issue" "$scope" "$slug")/sor.md"
}

card_dir_path() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  echo "$(cards_root_resolve)/${issue}"
}

card_input_path() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  echo "$(cards_root_resolve)/${issue}/input_${issue}.md"
}

card_output_path() {
  local issue
  issue="$(card_issue_normalize "$1")" || return 1
  echo "$(cards_root_resolve)/${issue}/output_${issue}.md"
}

card_legacy_input_path() {
  local issue="$1" ver="${2:-v0.2}"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo "$(cards_root_resolve)/issue-${pad}__input__${ver}.md"
}

card_legacy_output_path() {
  local issue="$1" ver="${2:-v0.2}"
  local pad
  pad="$(card_issue_pad "$issue")"
  echo "$(cards_root_resolve)/issue-${pad}__output__${ver}.md"
}

card_first_legacy_path() {
  local kind="$1" issue="$2" ver="${3:-v0.2}"
  local preferred pattern
  local -a matches=()
  local pad
  pad="$(card_issue_pad "$issue")" || return 1

  case "$kind" in
    input)
      preferred="$(card_legacy_input_path "$issue" "$ver")"
      pattern="$(cards_root_resolve)/issue-${pad}__input__v*.md"
      ;;
    output)
      preferred="$(card_legacy_output_path "$issue" "$ver")"
      pattern="$(cards_root_resolve)/issue-${pad}__output__v*.md"
      ;;
    *)
      echo "invalid card kind: $kind" >&2
      return 1
      ;;
  esac

  if [[ -e "$preferred" || -L "$preferred" ]]; then
    echo "$preferred"
    return 0
  fi

  shopt -s nullglob
  matches=($pattern)
  shopt -u nullglob
  if [[ ${#matches[@]} -gt 0 ]]; then
    echo "${matches[0]}"
    return 0
  fi

  return 1
}

next_migration_path() {
  local legacy="$1"
  local base out i
  base="$(basename "$legacy")"
  out="$(cards_root_resolve)/_legacy_migrated/${base}"
  i=0
  while [[ -e "$out" || -L "$out" ]]; do
    i=$((i + 1))
    out="$(cards_root_resolve)/_legacy_migrated/${base}.${i}"
  done
  echo "$out"
}

ensure_canonical_card_from_legacy() {
  local canonical="$1" legacy="$2"
  local migrated

  mkdir -p "$(dirname "$canonical")"

  if [[ ! -e "$canonical" && ! -L "$canonical" && -f "$legacy" ]]; then
    cp -f "$legacy" "$canonical"
    echo "warning: seeded canonical card from legacy content: $canonical" >&2
  fi

  if [[ -L "$legacy" ]]; then
    rm -f "$legacy"
    echo "warning: removed legacy symlink card path: $legacy" >&2
    return 0
  fi

  if [[ -f "$legacy" ]]; then
    mkdir -p "$(cards_root_resolve)/_legacy_migrated"
    migrated="$(next_migration_path "$legacy")"
    mv "$legacy" "$migrated"
    echo "warning: migrated legacy root card: $legacy -> $migrated" >&2
  fi
}

legacy_compat_link_path() {
  local kind="$1" issue="$2"
  case "$kind" in
    input) card_input_path "$issue" ;;
    output) card_output_path "$issue" ;;
    *) echo "invalid card kind: $kind" >&2; return 1 ;;
  esac
}

ensure_legacy_card_compat_link() {
  local kind="$1" issue="$2" canonical="$3"
  local compat existing migrated
  compat="$(legacy_compat_link_path "$kind" "$issue")" || return 1
  mkdir -p "$(dirname "$compat")"

  if [[ -L "$compat" ]]; then
    rm -f "$compat"
  elif [[ -e "$compat" ]]; then
    mkdir -p "$(cards_root_resolve)/_legacy_migrated"
    migrated="$(next_migration_path "$compat")"
    mv "$compat" "$migrated"
    echo "warning: migrated compatibility card path: $compat -> $migrated" >&2
  fi

  ln -s "$canonical" "$compat"
}

resolve_input_card_path() {
  local issue="$1" ver="${2:-}" slug="${3:-}"
  local p_new p_legacy bundle_dir compat
  compat="$(card_input_path "$issue")" || return 1

  if [[ -n "$slug" && -n "$ver" ]]; then
    p_new="$(task_bundle_input_path "$issue" "$ver" "$slug")"
    mkdir -p "$(dirname "$p_new")"
    if [[ ! -e "$p_new" && -f "$compat" && ! -L "$compat" ]]; then
      cp -f "$compat" "$p_new"
    fi
    ensure_legacy_card_compat_link input "$issue" "$p_new"
    echo "$p_new"
    return 0
  fi

  bundle_dir="$(task_bundle_first_dir "$issue" "$ver" || true)"
  if [[ -n "$bundle_dir" ]]; then
    ensure_legacy_card_compat_link input "$issue" "$bundle_dir/sip.md"
    echo "$bundle_dir/sip.md"
    return 0
  fi

  p_legacy="$(card_first_legacy_path input "$issue" "${ver:-v0.2}" || true)"
  if [[ -n "$p_legacy" ]]; then
    ensure_canonical_card_from_legacy "$compat" "$p_legacy"
  fi
  echo "$compat"
}

resolve_output_card_path() {
  local issue="$1" ver="${2:-}" slug="${3:-}"
  local p_new p_legacy bundle_dir compat
  compat="$(card_output_path "$issue")" || return 1

  if [[ -n "$slug" && -n "$ver" ]]; then
    p_new="$(task_bundle_output_path "$issue" "$ver" "$slug")"
    mkdir -p "$(dirname "$p_new")"
    if [[ ! -e "$p_new" && -f "$compat" && ! -L "$compat" ]]; then
      cp -f "$compat" "$p_new"
    fi
    ensure_legacy_card_compat_link output "$issue" "$p_new"
    echo "$p_new"
    return 0
  fi

  bundle_dir="$(task_bundle_first_dir "$issue" "$ver" || true)"
  if [[ -n "$bundle_dir" ]]; then
    ensure_legacy_card_compat_link output "$issue" "$bundle_dir/sor.md"
    echo "$bundle_dir/sor.md"
    return 0
  fi

  p_legacy="$(card_first_legacy_path output "$issue" "${ver:-v0.2}" || true)"
  if [[ -n "$p_legacy" ]]; then
    ensure_canonical_card_from_legacy "$compat" "$p_legacy"
  fi
  echo "$compat"
}
