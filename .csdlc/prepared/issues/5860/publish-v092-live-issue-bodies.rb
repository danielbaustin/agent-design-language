#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "yaml"

WAVE_PATH = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"
BODY_ROOT = ".csdlc/evidence/5860/live-issue-bodies"

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
issues = (Array(wave["work_packages"]) + Array(wave["supporting_issues"]))
  .map { |row| row["issue"] if row["issue"].is_a?(Integer) && row["issue"] != 5817 }
  .compact.uniq.sort
selected = if (index = ARGV.index("--issues"))
             ARGV.fetch(index + 1).split(",").map(&:to_i)
           else
             issues
           end
apply = ARGV.include?("--apply")

unknown = selected - issues
abort "unknown v0.92 execution issues: #{unknown.join(', ')}" unless unknown.empty?

drift = []
list_out, list_err, list_status = Open3.capture3(
  "gh", "issue", "list", "--state", "open", "--limit", "200", "--json", "number,body,state"
)
abort "gh issue list failed: #{list_err.strip}" unless list_status.success?
live = JSON.parse(list_out).to_h { |row| [row.fetch("number"), row] }

selected.each do |issue|
  body_path = "#{BODY_ROOT}/#{issue}.md"
  abort "missing #{body_path}" unless File.file?(body_path)
  expected = File.read(body_path)
  observed = live[issue] || abort("##{issue}: missing from open issue inventory")
  abort "##{issue}: expected open issue" unless observed.fetch("state") == "OPEN"
  next if observed.fetch("body") == expected

  drift << issue
  next unless apply

  _edit_out, edit_err, edit_status = Open3.capture3("gh", "issue", "edit", issue.to_s, "--body-file", body_path)
  abort "gh issue edit #{issue} failed: #{edit_err.strip}" unless edit_status.success?
end

if apply
  puts "v0.92 live issue body publish: PASS (#{drift.length} updated, #{selected.length} checked)"
elsif drift.empty?
  puts "v0.92 live issue body parity: PASS (#{selected.length} issues)"
else
  abort "live issue body drift: #{drift.map { |issue| "##{issue}" }.join(', ')}"
end
