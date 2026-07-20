#!/usr/bin/env bash
set -euo pipefail

repo_root=${1:-.}
out_dir="$repo_root/docs/milestones/v0.91.7/review"
json_out="$out_dir/V0917_CLOSED_ISSUE_CLOSEOUT_REGISTER.json"
md_out="$out_dir/V0917_CLOSED_ISSUE_CLOSEOUT_REGISTER.md"
mkdir -p "$out_dir"

issues_json=$(mktemp)
trap 'rm -f "$issues_json"' EXIT
gh issue list --state closed --label version:v0.91.7 --limit 1000 \
  --json number,title,state,closedAt,closedByPullRequestsReferences,labels,url > "$issues_json"

tmp_rows=$(mktemp)
trap 'rm -f "$issues_json" "$tmp_rows"' EXIT
printf '[]' > "$tmp_rows"

while IFS= read -r issue; do
  number=$(jq -r '.number' <<<"$issue")
  typed=false
  record_path=""
  if [[ -f "$repo_root/.csdlc/issues/$number/index.json" ]]; then
    typed=true
    record_path=".csdlc/issues/$number/index.json"
  fi
receipt_path=".git/csdlc-v2/closeout/$number.json"
  valid_receipt=false
  common_git_dir=$(git -C "$repo_root" rev-parse --git-common-dir)
  receipt_file="$common_git_dir/csdlc-v2/closeout/$number.json"
  if [[ -f "$receipt_file" ]] && jq -e --argjson issue "$number" '
    .schema == "csdlc.terminal_receipt.v1" and
    .issue == $issue and
    .record.schema == "csdlc.issue.index.v1" and
    .record.issue == $issue and
    .record.phase == "closed_out" and
    (.record.terminal != null) and
    (.record.claim == null)
  ' "$receipt_file" >/dev/null 2>&1; then
    valid_receipt=true
  fi

  prs='[]'
  while IFS= read -r pr; do
    [[ -z "$pr" ]] && continue
    pr_json=$(gh pr view "$pr" --json number,state,mergedAt,url,headRefOid,baseRefName 2>/dev/null || printf '{"number":%s,"state":"unknown"}' "$pr")
    prs=$(jq --argjson item "$pr_json" '. + [$item]' <<<"$prs")
  done < <(jq -r '.closedByPullRequestsReferences[]?.number' <<<"$issue")

  if [[ "$typed" == true ]]; then
    class="typed_projection"
    disposition="typed_receipt_required"
    evidence="tracked typed projection exists; receipt and terminal parity must be checked"
    if [[ "$valid_receipt" == true ]]; then
      disposition="typed_receipt_present"
      evidence="tracked typed projection and retained shared terminal receipt exist"
    fi
  elif [[ "$valid_receipt" == true ]]; then
    class="orphaned_typed_receipt"
    disposition="orphaned_typed_receipt"
    evidence="valid csdlc.terminal_receipt.v1 exists with closed_out record, but no tracked projection exists; no cards are fabricated"
  elif jq -e 'any(.[]; .state == "MERGED" and .mergedAt != null)' >/dev/null <<<"$prs"; then
    class="legacy"
    disposition="legacy_merged_pr"
    evidence="GitHub closedByPullRequestsReferences includes a merged PR"
  elif [[ $(jq 'length' <<<"$prs") -gt 0 ]]; then
    class="legacy"
    disposition="legacy_linked_pr_not_merged"
    evidence="GitHub closedByPullRequestsReferences exists, but no linked PR is observed merged"
  else
    class="legacy"
    disposition="legacy_closed_without_linked_pr"
    evidence="GitHub issue has no closedByPullRequestsReferences; no typed lifecycle record is fabricated"
  fi

  row=$(jq -n \
    --argjson issue "$issue" \
    --argjson typed "$typed" \
    --arg record_path "$record_path" \
    --arg receipt_path "$receipt_path" \
    --argjson receipt "$valid_receipt" \
    --arg class "$class" \
    --arg disposition "$disposition" \
    --arg evidence "$evidence" \
    --argjson prs "$prs" \
    '$issue + {class:$class,typed_projection:$typed,tracked_record:$record_path,shared_receipt:$receipt,receipt_path:$receipt_path,linked_prs:$prs,disposition:$disposition,evidence:$evidence}')
  jq --argjson row "$row" '. + [$row]' "$tmp_rows" > "$tmp_rows.next"
  mv "$tmp_rows.next" "$tmp_rows"
done < <(jq -c '.[]' "$issues_json")

jq '{schema:"adl.v0.91.7.closed_issue_closeout_register.v1",generated_at:(now|todateiso8601),source:{issue_label:"version:v0.91.7",issue_state:"closed"},summary:{total:length,typed_projection:(map(select(.class=="typed_projection"))|length),orphaned_typed_receipt:(map(select(.class=="orphaned_typed_receipt"))|length),legacy:(map(select(.class=="legacy"))|length),valid_receipts:(map(select(.shared_receipt))|length)},issues:.}' "$tmp_rows" > "$json_out"

{
  echo '# v0.91.7 Closed-Issue Closeout Register'
  echo
  echo 'Generated from live GitHub closed issues labeled `version:v0.91.7` and local typed C-SDLC v2 projections/receipts. Legacy issues remain legacy; this register does not fabricate typed cards.'
  echo
  jq -r '.summary | "- Total closed issues: \(.total)\n- Typed projections: \(.typed_projection)\n- Orphaned valid typed receipts: \(.orphaned_typed_receipt)\n- Legacy/pre-v2 closures: \(.legacy)\n- Valid shared receipts: \(.valid_receipts)"' "$json_out"
  echo
  echo '| Issue | Class | Disposition | Evidence | Linked PRs |'
  echo '|---:|---|---|---|---|'
  jq -r '.issues[] | "| [#\(.number)](\(.url)) | \(.class) | \(.disposition) | \(.evidence) | \([.linked_prs[]?.number] | map("#"+tostring) | join(", ")) |"' "$json_out"
} > "$md_out"

echo "$json_out"
echo "$md_out"
