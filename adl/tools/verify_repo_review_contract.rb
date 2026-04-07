#!/usr/bin/env ruby
# frozen_string_literal: true

require "optparse"

REQUIRED_HEADINGS = [
  "Metadata",
  "Scope",
  "Findings",
  "System-Level Assessment",
  "Recommended Action Plan",
  "Follow-ups / Deferred Work",
  "Final Assessment"
].freeze

FINDING_RE = /^\d+\.\s+\[(P[1-5])\]\s+.+$/
ABSOLUTE_HOST_PATH_RE = %r{(?:^|[\s`'"])/(Users|home|tmp|var/folders)/}.freeze

def parse_args
  options = {}
  OptionParser.new do |opts|
    opts.banner = "Usage: verify_repo_review_contract.rb --review <markdown>"
    opts.on("--review PATH", "Review markdown path") { |value| options[:review] = value }
  end.parse!

  abort("missing --review") unless options[:review]
  options
end

def assert!(condition, message)
  abort(message) unless condition
end

def section_body(text, heading)
  match = text.match(/^## #{Regexp.escape(heading)}\n(.*?)(?=^## |\z)/m)
  match && match[1]
end

options = parse_args
review_path = File.expand_path(options[:review], Dir.pwd)
assert!(File.file?(review_path), "review artifact not found: #{review_path}")
text = File.read(review_path)

assert!(!text.match?(ABSOLUTE_HOST_PATH_RE), "review artifact contains absolute host path")

headings = text.scan(/^## (.+)$/).flatten
assert!(headings == REQUIRED_HEADINGS, "review headings must match canonical order exactly")

metadata = section_body(text, "Metadata")
scope = section_body(text, "Scope")
findings = section_body(text, "Findings")
assessment = section_body(text, "System-Level Assessment")
action_plan = section_body(text, "Recommended Action Plan")
follow_ups = section_body(text, "Follow-ups / Deferred Work")
final_assessment = section_body(text, "Final Assessment")

assert!(metadata && metadata.include?("Review Type:"), "Metadata must include Review Type")
assert!(metadata.include?("Subject:"), "Metadata must include Subject")
assert!(metadata.include?("Reviewer:"), "Metadata must include Reviewer")
assert!(scope && scope.include?("Reviewed:"), "Scope must include Reviewed")
assert!(scope.include?("Not Reviewed:"), "Scope must include Not Reviewed")
assert!(scope.include?("Review Mode:"), "Scope must include Review Mode")
assert!(scope.include?("Gate:"), "Scope must include Gate")

findings_lines = findings.to_s.lines.map(&:rstrip)
has_explicit_no_findings = findings_lines.any? { |line| line.strip == "No material findings." }
finding_titles = findings_lines.select { |line| line.match?(FINDING_RE) }
assert!(has_explicit_no_findings || !finding_titles.empty?, "Findings must contain explicit findings or 'No material findings.'")

unless finding_titles.empty?
  expected_order = finding_titles.sort_by do |line|
    match = line.match(FINDING_RE)
    [match[1].delete("P").to_i, line]
  end
  assert!(finding_titles == expected_order, "Findings must be ordered by severity and stable title ordering")
end

assert!(assessment && !assessment.strip.empty?, "System-Level Assessment must not be empty")
assert!(action_plan && action_plan.include?("Fix now:"), "Recommended Action Plan must include Fix now")
assert!(action_plan.include?("Fix before milestone closeout:"), "Recommended Action Plan must include Fix before milestone closeout")
assert!(action_plan.include?("Defer:"), "Recommended Action Plan must include Defer")
assert!(follow_ups && !follow_ups.strip.empty?, "Follow-ups / Deferred Work must not be empty")
assert!(final_assessment && !final_assessment.strip.empty?, "Final Assessment must not be empty")

puts "PASS: repo review contract valid for #{review_path}"
