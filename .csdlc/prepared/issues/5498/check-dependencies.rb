#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

DEPENDENCIES = {
  5499 => "origin/codex/5499-v0918-wp10a-conductor",
  5349 => "origin/codex/5349-v0918-wp09-provider-tool-adapters"
}.freeze
REPOSITORY = "danielbaustin/agent-design-language"

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

root = Pathname.new(capture!("git", "rev-parse", "--show-toplevel"))
common = Pathname.new(capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir"))
path_inventory = JSON.parse(root.join(".csdlc", "prepared", "issues", "5498", "planned-path-sets.json").read)
confirmations = path_inventory.fetch("confirmations")

audit_receipts = {}
merge_results = {}
failures = DEPENDENCIES.map do |issue, remote_ref|
  _ref_out, _ref_err, ref_status = Open3.capture3("git", "rev-parse", "--verify", "#{remote_ref}^{commit}", chdir: root.to_s)
  unless ref_status.success?
    merge_results[issue.to_s] = { "remote_ref" => remote_ref, "status" => "missing_remote_ref" }
    next "##{issue}: missing remote dependency ref #{remote_ref}; run git fetch before evaluating live ancestry"
  end

  dependency_sha = capture!("git", "rev-parse", "#{remote_ref}^{commit}")
  _out, _err, status = Open3.capture3("git", "merge-base", "--is-ancestor", dependency_sha, "origin/main", chdir: root.to_s)
  if status.success?
    merge_results[issue.to_s] = {
      "remote_ref" => remote_ref,
      "dependency_sha" => dependency_sha,
      "base_ref" => "origin/main",
      "status" => "merged_ancestral"
    }
  else
    merge_results[issue.to_s] = {
      "remote_ref" => remote_ref,
      "dependency_sha" => dependency_sha,
      "base_ref" => "origin/main",
      "status" => "waiting"
    }
    next "##{issue}: #{remote_ref} at #{dependency_sha} is not ancestral to origin/main"
  end

  receipt_path = common.join("csdlc-v2", "closeout", "#{issue}.json")
  unless receipt_path.file?
    audit_receipts[issue.to_s] = { "status" => "missing_audit_receipt" }
    next nil
  end

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || {}
  phase = receipt.dig("record", "phase") || receipt["phase"]
  terminal = receipt.dig("record", "terminal") || receipt["terminal"] || {}
  disposition = terminal["disposition"] || receipt.dig("terminal", "disposition")
  merged_sha = terminal["observed_sha"] || receipt.dig("terminal", "observed_sha")

  expected_ref = "csdlc-v2/closeout/#{issue}.json"
  receipt_ok = receipt["schema"] == "csdlc.terminal_receipt.v1" &&
               receipt["issue"] == issue &&
               record["issue"] == issue &&
               receipt["repository"] == REPOSITORY &&
               record["repository"] == REPOSITORY &&
               receipt["receipt_ref"] == expected_ref &&
               terminal["receipt_path"] == expected_ref &&
               !receipt["initialization_digest"].to_s.empty? &&
               receipt["initialization_digest"].to_s == record["initialization_digest"].to_s &&
               !record["digest"].to_s.empty? &&
               record.key?("claim") &&
               record["claim"].nil? &&
               phase == "closed_out" &&
               disposition == "merged" &&
               !merged_sha.to_s.empty?
  audit_receipts[issue.to_s] = {
    "status" => receipt_ok ? "present_valid_audit" : "present_audit_mismatch",
    "phase" => phase,
    "disposition" => disposition,
    "observed_sha" => merged_sha
  }

  nil
end.compact

confirmations.each do |issue, state|
  failures << "##{issue}: adjacent planned-path owner confirmation is #{state.inspect}, expected confirmed" unless state == "confirmed"
end

if failures.empty?
  puts JSON.generate(status: "ready", issues: DEPENDENCIES.keys, conductor_gate: 5499, interface_gate: 5349, merge_results: merge_results, audit_receipts: audit_receipts, path_confirmations: confirmations)
  exit 0
end

puts JSON.generate(status: "waiting", issues: DEPENDENCIES.keys, conductor_gate: 5499, interface_gate: 5349, merge_results: merge_results, audit_receipts: audit_receipts, path_confirmations: confirmations, blockers: failures)
exit 3
