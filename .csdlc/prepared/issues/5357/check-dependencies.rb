#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
FIRST_PASS_ISSUE = 5356
FIRST_PASS_MERGE = "9e5745cdaad6f0753b22f1ef3ea7843573352c0d"
FINAL_PASS_ISSUE = 5791
FINAL_PASS_PR = 5799
FINAL_PACKET = "docs/reviews/v0.91.8/internal-review-5791/README.md"
FINAL_FINDINGS = "docs/reviews/v0.91.8/internal-review-5791/FINDINGS_REGISTER.md"
FINAL_PUBLICATION = ".csdlc/publication/5791.intent.json"
HEX40 = /\A[0-9a-f]{40}\z/

def fail_gate(message)
  warn("#5357 WP-18 gate: #{message}")
  exit 1
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

begin
  head = git("rev-parse", "HEAD")
  _out, first_ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", FIRST_PASS_MERGE, head)
  fail_gate("first-pass ##{FIRST_PASS_ISSUE} merge is not ancestral to #{head}") unless first_ancestry.success?

  [FINAL_PACKET, FINAL_FINDINGS, FINAL_PUBLICATION].each do |path|
    fail_gate("final-pass ##{FINAL_PASS_ISSUE} artifact is absent: #{path}") unless ROOT.join(path).file?
  end
  packet = ROOT.join(FINAL_PACKET).read
  review_head = packet[/Review head: `([0-9a-f]{40})`/, 1]
  fail_gate("final-pass packet has no exact review head") unless review_head&.match?(HEX40)

  publication = JSON.parse(ROOT.join(FINAL_PUBLICATION).read)
  fail_gate("final-pass publication does not close ##{FINAL_PASS_ISSUE}") unless publication.fetch("body").include?("Closes ##{FINAL_PASS_ISSUE}")
  merge_sha = git("log", "--format=%H", "--fixed-strings", "--grep=(##{FINAL_PASS_PR})", "-n", "1", "HEAD")
  fail_gate("cannot resolve merged final-pass PR ##{FINAL_PASS_PR} from target history") unless merge_sha.match?(HEX40)

  puts JSON.generate(
    status: "pass",
    issue: 5357,
    first_pass_issue: FIRST_PASS_ISSUE,
    first_pass_merge_sha: FIRST_PASS_MERGE,
    final_pass_issue: FINAL_PASS_ISSUE,
    final_pass_review_sha: review_head,
    final_pass_merge_sha: merge_sha,
    target_sha: head
  )
rescue JSON::ParserError, KeyError => e
  fail_gate("invalid final-pass review evidence: #{e.message}")
end
