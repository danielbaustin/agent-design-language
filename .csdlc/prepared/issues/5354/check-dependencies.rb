#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = 5384
MERGE_SHA = "72fbf30c74a5193ea41f042c76c5986a48e59d6c"
LEDGER = ROOT.join(".csdlc/evidence/5384/platform-acceptance-ledger.v1.json")

def fail_gate(message)
  warn("#5354 WP-14A gate: #{message}")
  exit 1
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

begin
  head = git("rev-parse", "HEAD")
  merge = git("rev-parse", "#{MERGE_SHA}^{commit}")
  fail_gate("WP-14A merge identity drifted") unless merge == MERGE_SHA

  message = git("show", "-s", "--format=%B", MERGE_SHA)
  fail_gate("WP-14A merge commit does not identify PR #5726") unless message.include?("(#5726)")

  _out, status = Open3.capture2e(
    "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", MERGE_SHA, head
  )
  fail_gate("WP-14A merge #{MERGE_SHA} is not ancestral to #{head}") unless status.success?

  fail_gate("missing accepted WP-14A ledger") unless LEDGER.file?
  ledger = JSON.parse(LEDGER.read)
  fail_gate("WP-14A ledger schema rejected") unless ledger["schema"] == "adl.wp14a.platform_acceptance_ledger.v1"
  fail_gate("WP-14A ledger does not pass") unless ledger["issue"] == ISSUE && ledger["status"] == "pass"

  issue_json, issue_status = Open3.capture2e(
    "gh", "issue", "view", ISSUE.to_s,
    "--repo", "danielbaustin/agent-design-language",
    "--json", "number,state,closed,closedAt,url"
  )
  fail_gate("live GitHub issue ##{ISSUE} lookup failed: #{issue_json.strip}") unless issue_status.success?
  issue = JSON.parse(issue_json)
  fail_gate("live GitHub issue ##{ISSUE} is not closed") unless
    issue["number"] == ISSUE &&
    issue["state"] == "CLOSED" &&
    issue["closed"] == true &&
    issue["closedAt"].to_s.match?(/\A\d{4}-\d{2}-\d{2}T/)

  puts JSON.generate(
    status: "pass",
    issue: 5354,
    dependency: ISSUE,
    dependency_merge_sha: MERGE_SHA,
    dependency_pr: 5726,
    dependency_issue_state: issue["state"],
    dependency_closed_at: issue["closedAt"],
    dependency_url: issue["url"],
    revision: head,
    closeout_gate: "not_required"
  )
rescue JSON::ParserError, KeyError => e
  fail_gate("invalid accepted ledger: #{e.message}")
end
