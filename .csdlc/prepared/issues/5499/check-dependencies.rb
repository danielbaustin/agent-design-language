#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

DEPENDENCIES = {
  5340 => {
    name: "WP-06 portable engine",
    merged_sha: "19601faec54a53e8bab90af484f745bc4972f116",
    evidence: "origin/main first-parent merge for PR #5623"
  },
  5341 => {
    name: "WP-08 Runtime v3 adapter",
    merged_sha: "713f9cb6e5b7f7b7674e601ce4f088134aa3c0a0",
    evidence: "origin/main squash merge for PR #5635"
  },
  5342 => {
    name: "WP-07 portable signed records",
    merged_sha: "34186ad77c84f50fe5e4ae097cfce14314b0e983",
    evidence: "origin/main squash merge for PR #5628"
  },
  5349 => {
    name: "WP-09 provider and governed-tool adapters",
    merged_sha: "79c7dccf12540863f6c038e1fd7ef45e2357a55e",
    evidence: "origin/main first-parent merge for PR #5636"
  }
}.freeze

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

def git_success?(root, *argv)
  _stdout, _stderr, status = Open3.capture3("git", *argv, chdir: root.to_s)
  status.success?
end

root = Pathname.new(capture!("git", "rev-parse", "--show-toplevel"))
common = Pathname.new(capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir"))

receipt_audit = {}
failures = DEPENDENCIES.map do |issue, dependency|
  receipt_path = common.join("csdlc-v2", "closeout", "#{issue}.json")
  if receipt_path.file?
    receipt = JSON.parse(receipt_path.read)
    terminal = receipt.dig("record", "terminal") || receipt["terminal"] || {}
    receipt_audit[issue] = {
      "present" => true,
      "phase" => receipt.dig("record", "phase") || receipt["phase"],
      "disposition" => terminal["disposition"] || receipt.dig("terminal", "disposition"),
      "observed_sha" => terminal["observed_sha"] || receipt.dig("terminal", "observed_sha")
    }
  else
    receipt_audit[issue] = {"present" => false}
  end

  merged_sha = dependency.fetch(:merged_sha)
  next "##{issue}: merged revision #{merged_sha} is missing locally" unless git_success?(root, "cat-file", "-e", "#{merged_sha}^{commit}")
  next "##{issue}: merged revision #{merged_sha} is not ancestral to origin/main" unless git_success?(root, "merge-base", "--is-ancestor", merged_sha, "origin/main")
  next "##{issue}: merged revision #{merged_sha} is not ancestral to HEAD" unless git_success?(root, "merge-base", "--is-ancestor", merged_sha, "HEAD")

  nil
end.compact

merged_revisions = DEPENDENCIES.transform_values do |dependency|
  {
    "name" => dependency.fetch(:name),
    "merged_sha" => dependency.fetch(:merged_sha),
    "evidence" => dependency.fetch(:evidence)
  }
end

if failures.empty?
  puts JSON.generate(status: "ready", issues: DEPENDENCIES.keys, final_gate: 5349, merged_revisions: merged_revisions, receipt_audit: receipt_audit)
  exit 0
end

puts JSON.generate(status: "waiting", issues: DEPENDENCIES.keys, final_gate: 5349, blockers: failures, merged_revisions: merged_revisions, receipt_audit: receipt_audit)
exit 3
