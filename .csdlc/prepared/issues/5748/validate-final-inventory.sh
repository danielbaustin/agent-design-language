#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
common_dir="$(git rev-parse --git-common-dir)"
if [[ "$common_dir" != /* ]]; then
  common_dir="$repo_root/$common_dir"
fi

doctor="$repo_root/.adl/bin/csdlc-v2/csdlc-doctor"
installer="$repo_root/.adl/bin/csdlc-v2/csdlc-install"
inventory="$repo_root/csdlc-v2/operator/coexistence.json"
register="$repo_root/.csdlc/prepared/issues/5748/fail-closed-exceptions.md"
universe="$repo_root/.csdlc/evidence/5748/v0918-closed-issue-universe.json"
remote_audit="$repo_root/.csdlc/evidence/5748/v0918-remote-terminal-audit.json"
closeout_prune_results="$repo_root/.csdlc/evidence/5748/v0918-closeout-prune-results.json"

fail() {
  printf 'v0.91.8 terminal inventory FAIL: %s\n' "$1" >&2
  exit 1
}

require_no_symlink_components() {
  local root="$1"
  local path="$2"
  local current="$path"
  case "$path" in
    "$root"|"$root"/*) ;;
    *) fail "governed path escapes its declared root: $path" ;;
  esac
  while [[ "$current" != "$root" ]]; do
    [[ ! -L "$current" ]] || fail "governed path contains a symlink: $current"
    current="${current%/*}"
  done
  [[ ! -L "$root" ]] || fail "governed root is a symlink: $root"
}

require_file() {
  require_no_symlink_components "$1" "$2"
  [[ -f "$2" && ! -L "$2" ]] || fail "missing canonical regular file: $2"
}

require_absent() {
  require_no_symlink_components "$1" "$2"
  [[ ! -e "$2" && ! -L "$2" ]] || fail "unexpected path exists: $2"
}

require_eq() {
  [[ "$1" == "$2" ]] || fail "$3 (expected $2, observed $1)"
}

path_guard_self_test() {
  local scratch="$repo_root/.csdlc/evidence/5748/.validator-path-guard-self-test"
  local target="$scratch/real/target"
  local file_link="$scratch/file-link"
  local dir_link="$scratch/dir-link"
  local dangling="$scratch/dangling"
  path_guard_cleanup() {
    unlink "$file_link" 2>/dev/null || true
    unlink "$dir_link" 2>/dev/null || true
    unlink "$dangling" 2>/dev/null || true
    unlink "$target" 2>/dev/null || true
    rmdir "$scratch/real" 2>/dev/null || true
    rmdir "$scratch" 2>/dev/null || true
  }
  trap path_guard_cleanup EXIT
  require_absent "$repo_root" "$scratch"
  mkdir -p "$scratch/real"
  printf 'canonical\n' >"$target"
  ln -s "$target" "$file_link"
  ln -s "$scratch/real" "$dir_link"
  ln -s "$scratch/missing" "$dangling"
  require_file "$repo_root" "$target"
  if (require_file "$repo_root" "$file_link") 2>/dev/null; then
    fail "path guard accepted a final file symlink"
  fi
  if (require_file "$repo_root" "$dir_link/target") 2>/dev/null; then
    fail "path guard accepted a symlinked parent component"
  fi
  if (require_absent "$repo_root" "$dangling") 2>/dev/null; then
    fail "path guard treated a dangling symlink as absent"
  fi
  path_guard_cleanup
  trap - EXIT
  printf 'v0.91.8 inventory path-guard self-test PASS\n'
}

verify_live_universe() {
  command -v gh >/dev/null 2>&1 || fail "gh is required for live universe verification"
  local observed
  observed="$(gh issue list --repo danielbaustin/agent-design-language \
    --state closed --label version:v0.91.8 --limit 1000 \
    --json number,state,closedAt,stateReason,title,url,closedByPullRequestsReferences)" ||
    fail "live GitHub closed-issue observation failed"
  local observed_shape retained_shape
  observed_shape="$(jq -S 'sort_by(.number) | map({
    number, state, state_reason:.stateReason, closed_at:.closedAt, title, url,
    closing_pull_requests:(.closedByPullRequestsReferences |
      map({number,url}) | sort_by(.number))
  })' <<<"$observed")"
  retained_shape="$(jq -S '.issues | sort_by(.number) | map({
    number, state, state_reason, closed_at, title, url, closing_pull_requests
  })' "$universe")"
  require_eq "$observed_shape" "$retained_shape" \
    "live GitHub closed-issue universe differs from the retained audit universe"
  printf 'v0.91.8 live terminal universe PASS: %s closed issues match retained evidence\n' \
    "$(jq 'length' <<<"$observed")"
}

if [[ "${1:-}" == "--self-test-path-guards" ]]; then
  path_guard_self_test
  exit 0
fi
if [[ "${1:-}" == "--verify-live" ]]; then
  require_file "$repo_root" "$universe"
  verify_live_universe
  exit 0
fi

terminal_issues=()
not_planned_terminal_issues=(5335)

require_file "$repo_root" "$register"
require_file "$repo_root" "$universe"
require_file "$repo_root" "$remote_audit"
require_file "$repo_root" "$closeout_prune_results"
require_file "$repo_root" "$installer"
require_file "$repo_root" "$doctor"
require_file "$repo_root" "$inventory"
while IFS= read -r issue; do
  [[ -n "$issue" ]] && terminal_issues+=("$issue")
done < <(jq -r '.issues[].number' "$universe" | sort -n)
terminal_count="${#terminal_issues[@]}"
not_planned_terminal_count="${#not_planned_terminal_issues[@]}"
closed_count="$terminal_count"
"$installer" verify --repo "$repo_root" --bin-dir .adl/bin/csdlc-v2 \
  --inventory "$inventory" >/dev/null || fail "owner-binary provenance is stale"

declared_completed="$(
  printf '%s\n' "${terminal_issues[@]}" |
    grep -vxF -f <(printf '%s\n' "${not_planned_terminal_issues[@]}") |
    sort -n | tr '\n' ' '
)"
observed_completed="$(
  jq -r '.issues[] | select(.state == "CLOSED" and .state_reason == "COMPLETED") |
    .number' "$universe" | sort -n | tr '\n' ' '
)"
require_eq "$observed_completed" "$declared_completed" \
  "retained live completed-issue universe differs from the declared partition"
require_eq "$(printf '%s\n' "${terminal_issues[@]}" | sort -nu | wc -l | tr -d ' ')" \
  "$closed_count" "declared closed-issue partition contains duplicates"
require_eq "$(jq -r '[.issues[] | select(.state == "CLOSED" and .state_reason == "NOT_PLANNED") | .number] | sort | @csv' "$universe")" \
  "$(IFS=,; printf '%s' "${not_planned_terminal_issues[*]}")" "retained terminal NOT_PLANNED issue universe mismatch"
jq -e --argjson closed_count "$closed_count" '.schema == "adl.v0918.closed_issue_universe.v1" and
  .repository == "danielbaustin/agent-design-language" and
  .label == "version:v0.91.8" and .state == "closed" and
  (.observed_at | type == "string" and length > 0) and
  .source == "github issue list read-only observation" and
  (.issues | length) == $closed_count and
  ([.issues[].number] | length) == ([.issues[].number] | unique | length)' \
  "$universe" >/dev/null || fail "retained closed-issue universe metadata is invalid"
require_eq "$(jq -S '[.issues[].number]' "$remote_audit")" \
  "$(jq -S '[.issues[].number]' "$universe")" \
  "remote terminal audit issue universe differs from retained closed universe"
jq -e --argjson closed_count "$closed_count" \
  '.schema == "adl.v0918.remote_terminal_audit.v1" and
   .repository == "danielbaustin/agent-design-language" and
   .label == "version:v0.91.8" and
   (.observed_at | type == "string" and length > 0) and
   .source == "GitHub closed issues and merged PRs joined to retained typed terminal projections" and
   (.issues | length) == $closed_count and
   all(.issues[];
     .state == "CLOSED" and
     .terminal.phase == "closed_out" and
     .terminal.claim_free == true and
     .checks.projection_terminal == true and
     .checks.remote_disposition_valid == true and
     .checks.linked_issue_matches == true and
     .checks.observed_head_matches == true and
     .checks.no_pr_consistent == true)' \
  "$remote_audit" >/dev/null || fail "remote terminal disposition audit is invalid"
require_eq "$(jq -S '[.issues[].number]' "$closeout_prune_results")" \
  "$(jq -S '[.issues[].number]' "$universe")" \
  "closeout/prune report issue universe differs from retained closed universe"
jq -e --argjson closed_count "$closed_count" \
  '.schema == "adl.v0918.closeout_prune_results.v1" and
   .repository == "danielbaustin/agent-design-language" and
   .label == "version:v0.91.8" and
   (.observed_at | type == "string" and length > 0) and
   .source == "typed C-SDLC v2 closeout verification and validate-prune observation" and
   (.issues | length) == $closed_count and
   all(.issues[];
     .closeout.phase == "closed_out" and
     .closeout.claim_free == true and
     .closeout.doctor_pass == true and
     .closeout.receipt_equal == true and
     .closeout.cards_equal == true and
     (.prune.status == "eligible" or
      .prune.status == "blocked" or
      .prune.status == "not_registered") and
     .prune.pruned == false and
     (if .prune.status == "eligible" then
        .prune.eligible == true
      else
        .prune.eligible == false and
        (.prune.reason | type == "string" and length > 0)
      end))' \
  "$closeout_prune_results" >/dev/null || fail "closeout/prune result report is invalid"
observed_register_issues="$(
  sed -n 's/^## #\([0-9][0-9]*\) —.*/\1/p' "$register" | sort -n | tr '\n' ' '
)"
require_eq "$observed_register_issues" "" \
  "final exception register must not retain an unresolved issue heading"

for issue in "${terminal_issues[@]}"; do
  index="$repo_root/.csdlc/issues/$issue/index.json"
  receipt="$common_dir/csdlc-v2/closeout/$issue.json"
  require_file "$repo_root" "$index"
  require_file "$common_dir" "$receipt"
  require_eq "$(jq -r '.phase' "$index")" closed_out \
    "terminal issue #$issue phase mismatch"
  require_eq "$(jq -r '.issue' "$index")" "$issue" \
    "terminal issue #$issue projection namespace mismatch"
  jq -e --argjson issue "$issue" \
    '.issue == $issue and .record.issue == $issue and
     .receipt_ref == ("csdlc-v2/closeout/" + ($issue | tostring) + ".json")' \
    "$receipt" >/dev/null || fail "terminal issue #$issue receipt namespace mismatch"
  require_eq "$(jq -r '.claim == null' "$index")" true \
    "terminal issue #$issue retained an active claim"
  "$doctor" --repo "$repo_root" --issue "$issue" >/dev/null || \
    fail "terminal issue #$issue failed doctor"
  jq -e --slurpfile receipt "$receipt" '. == $receipt[0].record' "$index" \
    >/dev/null || fail "terminal issue #$issue index differs from receipt"
  for card in sip stp spp vpp srp sor; do
    require_file "$repo_root" \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json"
    jq -e --arg card "$card" --slurpfile receipt "$receipt" \
      '. == $receipt[0].cards[$card]' \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json" >/dev/null || \
      fail "terminal issue #$issue $card values differ from receipt"
    require_eq "$(jq -r '.identity.issue' \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json")" "$issue" \
      "terminal issue #$issue $card namespace mismatch"
  done
done

require_eq "$(git status --porcelain -- .csdlc/locks .csdlc/requests)" "" \
  "generated lock or request state dirties the publication worktree"

printf 'v0.91.8 terminal inventory PASS: %s terminal (%s closed NOT_PLANNED), zero fail-closed exceptions\n' \
  "$terminal_count" "$not_planned_terminal_count"
