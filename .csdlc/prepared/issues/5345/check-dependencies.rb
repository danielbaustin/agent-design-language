#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ISSUES = [5339, 5338, 5340, 5342, 5341, 5349].freeze
ROOT = File.expand_path("../../../..", __dir__)

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

common_dir = File.expand_path(capture!("git", "rev-parse", "--git-common-dir"))
head = capture!("git", "rev-parse", "HEAD")

observations = []
ISSUES.each do |issue|
  path = File.join(common_dir, "csdlc-v2", "closeout", "#{issue}.json")
  unless File.file?(path)
    observations << { issue: issue, status: "receipt_unavailable" }
    next
  end

  receipt = JSON.parse(File.read(path))
  record = receipt.fetch("record")
  phase = record["phase"]
  publication = record.fetch("publication")
  readiness = record.fetch("readiness")
  terminal = record.fetch("terminal")
  disposition = terminal["disposition"]
  merged_sha = terminal["observed_sha"]
  tracked_path = File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")

  observations << {
    issue: issue,
    phase: phase,
    disposition: disposition,
    merged_sha: merged_sha,
    ancestral: merged_sha.is_a?(String) && system("git", "merge-base", "--is-ancestor", merged_sha, head, out: File::NULL, err: File::NULL)
  }
end

puts JSON.generate(
  schema: "adl.v0918.wp10_dependency_gate.v1",
  status: "observational",
  head: head,
  dependencies: observations,
  note: "Dependency receipts and ancestry are evidence only; they do not block WP-10 implementation."
)
