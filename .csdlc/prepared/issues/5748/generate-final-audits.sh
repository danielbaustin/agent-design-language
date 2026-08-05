#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
common_dir="$(git rev-parse --path-format=absolute --git-common-dir)"
universe="$repo_root/.csdlc/evidence/5748/v0918-closed-issue-universe.json"
remote_audit="$repo_root/.csdlc/evidence/5748/v0918-remote-terminal-audit.json"
prune_audit="$repo_root/.csdlc/evidence/5748/v0918-closeout-prune-results.json"
pr_state="$repo_root/.adl/bin/csdlc-v2/csdlc-pr-state"
doctor="$repo_root/.adl/bin/csdlc-v2/csdlc-doctor"
closeout="$repo_root/.adl/bin/csdlc-v2/csdlc-closeout"
token_file="/Users/daniel/keys/github.token"
parallelism="${CSDLC_V2_AUDIT_PARALLELISM:-8}"
run_id="$(date -u +%Y%m%dT%H%M%SZ)"
run_root="$common_dir/csdlc-v2/audits/5748-$run_id"
request_root="$run_root/requests"
packet_root="$run_root/packets"
remote_rows="$run_root/remote.ndjson"
prune_rows="$run_root/prune.ndjson"
worktree_map="$run_root/worktrees.tsv"

fail() {
  printf 'v0.91.8 final audit generation FAIL: %s\n' "$1" >&2
  exit 1
}

for required in "$pr_state" "$doctor" "$closeout" "$token_file"; do
  [[ -f "$required" ]] || fail "missing required input: $required"
done
mkdir -p "$request_root" "$packet_root"
: >"$remote_rows"
: >"$prune_rows"

live_issue_packet="$run_root/live-closed-issues.json"
gh issue list --repo danielbaustin/agent-design-language \
  --state closed --label version:v0.91.8 --limit 1000 \
  --json number,state,closedAt,stateReason,title,url,closedByPullRequestsReferences \
  >"$live_issue_packet" || fail "live GitHub closed-issue observation failed"
observed_universe_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
jq --arg observed_at "$observed_universe_at" '
  {schema:"adl.v0918.closed_issue_universe.v1",
   repository:"danielbaustin/agent-design-language",
   label:"version:v0.91.8",state:"closed",observed_at:$observed_at,
   source:"github issue list read-only observation",
   issues:(sort_by(.number) | map({
     number,
     state,
     state_reason:.stateReason,
     closed_at:.closedAt,
     title,
     url,
     closing_pull_requests:(.closedByPullRequestsReferences |
       map({number,url}) | sort_by(.number))
   }))}' "$live_issue_packet" >"$universe.tmp"
mv "$universe.tmp" "$universe"

while IFS= read -r issue; do
  index="$repo_root/.csdlc/issues/$issue/index.json"
  [[ -f "$index" ]] || fail "missing terminal projection for issue #$issue"
  pr="$(jq -r '.terminal.pull_request // empty' "$index")"
  [[ -n "$pr" ]] || continue
  jq -e --argjson issue "$issue" --argjson pr "$pr" '
    .issues[] | select(.number == $issue) |
    any(.closing_pull_requests[]?; .number == $pr)' "$universe" >/dev/null ||
    fail "terminal PR #$pr is not a closing PR for issue #$issue"
  jq -n --argjson pr "$pr" --argjson linked_issue "$issue" \
    --arg token_file "$token_file" \
    '{repository:"danielbaustin/agent-design-language",pull_request:$pr,
      required_checks:[],require_review:false,token_file:$token_file,
      linked_issue:$linked_issue}' >"$request_root/$issue-$pr.json"
done < <(jq -r '.issues[].number' "$universe")

export packet_root pr_state
find "$request_root" -type f -name '*.json' -print | sort | \
  xargs -P "$parallelism" -I{} bash -c '
    set -euo pipefail
    request="$1"
    pr="$(basename "$request" .json)"
    "$pr_state" --request "$request" >"$packet_root/$pr.json"
  ' _ {}

git worktree list --porcelain | awk '
  /^worktree / { path=substr($0,10) }
  /^branch refs\/heads\// { branch=substr($0,19) }
  /^$/ { if (path != "" && branch != "") print branch "\t" path; path=""; branch="" }
  END { if (path != "" && branch != "") print branch "\t" path }
' >"$worktree_map"

while IFS= read -r issue; do
  index="$repo_root/.csdlc/issues/$issue/index.json"
  receipt="$common_dir/csdlc-v2/closeout/$issue.json"
  [[ -f "$receipt" ]] || fail "missing retained receipt for issue #$issue"
  pr="$(jq -r '.terminal.pull_request // empty' "$index")"
  if [[ -n "$pr" ]]; then
    packet="$packet_root/$issue-$pr.json"
    [[ -s "$packet" ]] || fail "missing typed remote PR packet for issue #$issue PR #$pr"
    jq -n --argjson issue "$issue" --slurpfile universe "$universe" \
      --slurpfile index "$index" --slurpfile packet "$packet" '
      ($universe[0].issues[] | select(.number == $issue)) as $remote_issue |
      $index[0] as $record | $packet[0] as $pr |
      {
        number:$issue,
        state:$remote_issue.state,
        state_reason:$remote_issue.state_reason,
        closed_at:$remote_issue.closed_at,
        title:$remote_issue.title,
        url:$remote_issue.url,
        terminal:{
          phase:$record.phase,
          claim_free:($record.claim == null),
          disposition:$record.terminal.disposition,
          pull_request:$record.terminal.pull_request,
          observed_sha:$record.terminal.observed_sha,
          observed_state:$record.terminal.observed_state,
          receipt_path:$record.terminal.receipt_path
        },
        remote:{
          schema:$pr.schema,
          pull_request:$pr.pull_request,
          linked_issue:$pr.linked_issue,
          url:$pr.url,
          head_ref:$pr.head_ref,
          head_sha:$pr.head_sha,
          merged:$pr.merged,
          merge_commit_sha:$pr.merge_commit_sha
        },
        checks:{
          projection_terminal:($record.phase == "closed_out" and $record.claim == null),
          remote_disposition_valid:
            (if $record.terminal.disposition == "merged" then $pr.merged == true
             else $pr.merged == false end),
          linked_issue_matches:($pr.linked_issue == $issue),
          observed_head_matches:($record.terminal.observed_sha == $pr.head_sha),
          no_pr_consistent:true
        }
      }' >>"$remote_rows"
  else
    jq -n --argjson issue "$issue" --slurpfile universe "$universe" \
      --slurpfile index "$index" '
      ($universe[0].issues[] | select(.number == $issue)) as $remote_issue |
      $index[0] as $record |
      {
        number:$issue,
        state:$remote_issue.state,
        state_reason:$remote_issue.state_reason,
        closed_at:$remote_issue.closed_at,
        title:$remote_issue.title,
        url:$remote_issue.url,
        terminal:{
          phase:$record.phase,
          claim_free:($record.claim == null),
          disposition:$record.terminal.disposition,
          pull_request:null,
          observed_sha:$record.terminal.observed_sha,
          observed_state:$record.terminal.observed_state,
          receipt_path:$record.terminal.receipt_path
        },
        remote:null,
        checks:{
          projection_terminal:($record.phase == "closed_out" and $record.claim == null),
          remote_disposition_valid:
            ($record.terminal.disposition == "closed_no_pr" and
             $record.terminal.observed_state == "closed_no_pr"),
          linked_issue_matches:true,
          observed_head_matches:($record.terminal.observed_sha == null),
          no_pr_consistent:
            ($record.terminal.pull_request == null and
             ($remote_issue.closing_pull_requests | length) == 0)
        }
      }' >>"$remote_rows"
  fi

  phase="$(jq -r '.phase' "$index")"
  claim_free="$(jq -r '.claim == null' "$index")"
  doctor_pass=false
  receipt_equal=false
  cards_equal=true
  if "$doctor" --repo "$repo_root" --issue "$issue" >/dev/null; then
    doctor_pass=true
  fi
  if jq -e --slurpfile receipt "$receipt" '. == $receipt[0].record' "$index" >/dev/null; then
    receipt_equal=true
  fi
  for card in sip stp spp vpp srp sor; do
    values="$repo_root/.csdlc/issues/$issue/cards/$card.values.json"
    if [[ ! -f "$values" ]] ||
       ! jq -e --arg card "$card" --slurpfile receipt "$receipt" \
         '. == $receipt[0].cards[$card]' "$values" >/dev/null; then
      cards_equal=false
    fi
  done

  released_branch="$(jq -r '.terminal.released_branch' "$index")"
  wtpath="$(awk -F '\t' -v branch="$released_branch" '$1 == branch {print $2; exit}' "$worktree_map")"
  prune_status=not_registered
  prune_eligible=false
  prune_reason="released branch is not currently registered as a worktree"
  if [[ -n "$wtpath" ]]; then
    prune_packet="$run_root/prune-$issue.json"
    if "$closeout" --root "$wtpath" validate-prune --issue "$issue" >"$prune_packet"; then
      if jq -e '.eligible == true and .pruned == false' "$prune_packet" >/dev/null; then
        prune_status=eligible
        prune_eligible=true
        prune_reason=""
      else
        prune_status=blocked
        prune_reason="typed validate-prune returned a non-eligible packet"
      fi
    else
      prune_status=blocked
      prune_reason="$(jq -r '.message // "typed validate-prune failed closed"' "$prune_packet" 2>/dev/null || printf 'typed validate-prune failed closed')"
    fi
  fi

  jq -n --argjson issue "$issue" --arg phase "$phase" \
    --argjson claim_free "$claim_free" --argjson doctor_pass "$doctor_pass" \
    --argjson receipt_equal "$receipt_equal" --argjson cards_equal "$cards_equal" \
    --arg prune_status "$prune_status" --argjson prune_eligible "$prune_eligible" \
    --arg wtpath "$wtpath" --arg prune_reason "$prune_reason" '
    {
      number:$issue,
      closeout:{phase:$phase,claim_free:$claim_free,doctor_pass:$doctor_pass,
        receipt_equal:$receipt_equal,cards_equal:$cards_equal},
      prune:{status:$prune_status,eligible:$prune_eligible,pruned:false,
        worktree:(if $wtpath == "" then null else $wtpath end),
        reason:(if $prune_reason == "" then null else $prune_reason end)}
    }' >>"$prune_rows"
done < <(jq -r '.issues[].number' "$universe" | sort -n)

observed_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
jq -s --arg observed_at "$observed_at" '
  {schema:"adl.v0918.remote_terminal_audit.v1",
   repository:"danielbaustin/agent-design-language",label:"version:v0.91.8",
   observed_at:$observed_at,
   source:"GitHub closed issues and merged PRs joined to retained typed terminal projections",
   issues:sort_by(.number)}' "$remote_rows" >"$remote_audit"
jq -s --arg observed_at "$observed_at" '
  {schema:"adl.v0918.closeout_prune_results.v1",
   repository:"danielbaustin/agent-design-language",label:"version:v0.91.8",
   observed_at:$observed_at,
   source:"typed C-SDLC v2 closeout verification and validate-prune observation",
   issues:sort_by(.number)}' "$prune_rows" >"$prune_audit"

printf 'v0.91.8 final audits generated: %s issues, %s typed issue/PR packets (%s unique PRs)\n' \
  "$(jq '.issues | length' "$remote_audit")" \
  "$(find "$packet_root" -type f -name '*.json' | wc -l | tr -d ' ')" \
  "$(jq '[.issues[].terminal.pull_request | select(. != null)] | unique | length' "$remote_audit")"
