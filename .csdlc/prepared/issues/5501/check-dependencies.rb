#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
EXPECTED_MERGES = {
  "5349" => "79c7dccf12540863f6c038e1fd7ef45e2357a55e",
  "5499" => "d8f02c5b77099552c376436acd695f2bf8922de6",
  "5498" => "7d6095acd0da8fe9e1a622387a229a02ecd824dc",
  "5500" => "fa49c2d0f32147547f0aafdca8bfbc841c49258a",
  "5502" => "1cbbf4eb5531814f7b4f0fdc9edeaa1df78410cd"
}.freeze

def fail_closed(message)
  warn(message)
  exit 2
end

common_dir, status = Open3.capture2("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
fail_closed("cannot resolve shared Git directory") unless status.success?
common = Pathname.new(common_dir.strip)
common = ROOT.join(common).cleanpath unless common.absolute?

def audit_receipt(common, issue)
  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  return { present: false } unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || receipt
  terminal = record["terminal"] || {}
  {
    present: true,
    phase: record["phase"],
    claim_active: !record["claim"].nil?,
    disposition: terminal["disposition"],
    observed_sha: terminal["observed_sha"] || record["observed_sha"] || record["merge_sha"]
  }
rescue JSON::ParserError => e
  { present: true, malformed: e.message }
end

def audit_projection(issue)
  index_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  return { present: false } unless index_path.file?

  index = JSON.parse(index_path.read)
  {
    present: true,
    phase: index["phase"],
    claim_active: !index["claim"].nil?
  }
rescue JSON::ParserError => e
  { present: true, malformed: e.message }
end

origin_main = "origin/main"
unless system("git", "-C", ROOT.to_s, "rev-parse", "--verify", origin_main, out: File::NULL, err: File::NULL)
  fail_closed("origin/main is unavailable; refresh live repository state before dependency admission")
end

results = []
blockers = []
EXPECTED_MERGES.each do |issue, merge_sha|
  exists = system("git", "-C", ROOT.to_s, "cat-file", "-e", "#{merge_sha}^{commit}",
                  out: File::NULL, err: File::NULL)
  origin_ancestral = exists && system(
    "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, origin_main,
    out: File::NULL, err: File::NULL
  )
  head_ancestral = origin_ancestral && system(
    "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, "HEAD",
    out: File::NULL, err: File::NULL
  )
  unless head_ancestral
    blockers << {
      issue: issue.to_i,
      live_merge_sha: merge_sha,
      commit_exists: exists,
      ancestral_to_origin_main: origin_ancestral,
      ancestral_to_5501_head: head_ancestral,
      reason: "exact_live_merge_sha_missing_or_not_ancestral"
    }
    next
  end

  results << {
    issue: issue.to_i,
    live_merge_sha: merge_sha,
    ancestral_to_origin_main: true,
    ancestral_to_head: true,
    receipt_audit: audit_receipt(common, issue),
    projection_audit: audit_projection(issue)
  }
end

status = blockers.empty? ? "ready" : "blocked"
puts JSON.pretty_generate(
  status: status,
  dependency_rule: "live_merge_plus_ancestry",
  audit_only: ["typed_closeout", "retained_receipt", "claim_release_projection"],
  blockers: blockers,
  dependencies: results
)
exit 2 unless blockers.empty?
