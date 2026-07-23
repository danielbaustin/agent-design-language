#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

DEPENDENCIES = {
  5340 => 5623,
  5342 => 5628,
  5591 => 5608
}.freeze
EXPECTED_REPOSITORY = "danielbaustin/agent-design-language"

def git(*argv)
  stdout, stderr, status = Open3.capture3("git", *argv)
  [stdout.strip, stderr.strip, status]
end

common_dir_text, common_dir_error, common_dir_status = git(
  "rev-parse", "--path-format=absolute", "--git-common-dir"
)
unless common_dir_status.success?
  warn common_dir_error
  exit 3
end

common_dir = Pathname(common_dir_text)
results = DEPENDENCIES.map do |issue, pull_request|
  receipt_path = common_dir.join("csdlc-v2", "closeout", "#{issue}.json")
  result = {
    "issue" => issue,
    "receipt" => receipt_path.to_s,
    "github_merged" => false,
    "typed_closed_out" => false,
    "receipt_retained" => false,
    "merged_sha_ancestral" => false,
    "closeout_pending" => false,
    "reasons" => []
  }

  unless receipt_path.file?
    merge_sha, _merge_error, merge_status = git(
      "log", "origin/main", "--format=%H", "--grep=(##{pull_request})", "-1"
    )
    if merge_status.success? && merge_sha.match?(/\A[0-9a-f]{40}\z/)
      result["github_merged"] = true
      result["merged_sha"] = merge_sha
      _stdout, _stderr, ancestry_status = git(
        "merge-base", "--is-ancestor", merge_sha, "origin/main"
      )
      result["merged_sha_ancestral"] = ancestry_status.success?
      result["closeout_pending"] = true
      result["reasons"] << "typed_closeout_pending_non_blocking"
    else
      result["reasons"] << "merged_commit_not_found"
    end
    next result
  end

  begin
    receipt = JSON.parse(receipt_path.read)
  rescue JSON::ParserError => error
    result["reasons"] << "malformed_terminal_receipt:#{error.message}"
    next result
  end

  record = receipt.fetch("record", {})
  terminal = record.fetch("terminal", {})
  result["receipt_retained"] =
    receipt["schema"] == "csdlc.terminal_receipt.v1" &&
    receipt["issue"] == issue &&
    receipt["repository"] == EXPECTED_REPOSITORY &&
    receipt["receipt_ref"] == "csdlc-v2/closeout/#{issue}.json"
  result["typed_closed_out"] = record["phase"] == "closed_out"
  result["github_merged"] =
    terminal["disposition"] == "merged" &&
    terminal["observed_state"] == "merged"

  sha = terminal["observed_sha"]
  if sha.is_a?(String) && sha.match?(/\A[0-9a-f]{40}\z/)
    _stdout, _stderr, ancestry_status = git(
      "merge-base", "--is-ancestor", sha, "origin/main"
    )
    result["merged_sha"] = sha
    result["merged_sha_ancestral"] = ancestry_status.success?
  else
    result["reasons"] << "missing_or_invalid_merged_sha"
  end

  result["reasons"] << "receipt_identity_mismatch" unless result["receipt_retained"]
  result["reasons"] << "typed_phase_not_closed_out" unless result["typed_closed_out"]
  result["reasons"] << "github_terminal_state_not_merged" unless result["github_merged"]
  result["reasons"] << "merged_sha_not_ancestral_to_origin_main" unless result["merged_sha_ancestral"]
  result
end

ready = results.all? do |result|
  result.values_at("github_merged", "merged_sha_ancestral").all?
end

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5341_dependency_gate.v1",
  "status" => ready ? "ready" : "waiting",
  "origin_main" => git("rev-parse", "origin/main").first,
  "results" => results
)
exit(ready ? 0 : 2)
