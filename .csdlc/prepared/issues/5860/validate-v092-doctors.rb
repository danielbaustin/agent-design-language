#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"
require "yaml"

DOCTOR = "csdlc-v2/target/debug/csdlc-doctor"
OUTPUT = ".csdlc/evidence/5860/V092_TYPED_DOCTOR_REPORTS.json"
WAVE_PATH = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
ISSUES = (Array(wave["work_packages"]) + Array(wave["supporting_issues"]))
  .map { |row| row["issue"] if row["issue"].is_a?(Integer) && row["issue"] != 5817 }.compact
  .uniq.sort.freeze

abort "missing #{DOCTOR}" unless File.executable?(DOCTOR)

reports = ISSUES.map do |issue|
  stdout, stderr, status = Open3.capture3(DOCTOR, "--repo", ".", "--issue", issue.to_s)
  abort "doctor failed for ##{issue}: #{stderr}" unless [0, 2].include?(status.exitstatus)

  report = JSON.parse(stdout)
  expected_finding = [{ "code" => "claim_dormant", "message" => "nonterminal issue has no active writer claim" }]
  abort "##{issue}: wrong report identity" unless report["issue"] == issue
  abort "##{issue}: wrong handoff status" unless report["status"] == "block" && report["ready"] == false
  abort "##{issue}: wrong phase" unless report["phase"] == "bound"
  abort "##{issue}: unexpected doctor findings #{report['findings'].inspect}" unless report["findings"] == expected_finding
  abort "##{issue}: wrong next operation" unless report["next_operation"] == "reacquire_claim"
  report
end

payload = {
  "schema" => "adl.v092.child-doctor-reports.v1",
  "interpretation" => "design-ready handoff; execution remains dependency-gated and requires just-in-time claim reacquisition",
  "reports" => reports
}
rendered = JSON.pretty_generate(payload) + "\n"
if ARGV.include?("--write")
  File.write(OUTPUT, rendered)
else
  abort "missing pinned doctor evidence; run with --write explicitly" unless File.file?(OUTPUT)
  expected = File.read(OUTPUT)
  abort "typed doctor evidence drift: expected #{Digest::SHA256.hexdigest(expected)}, live #{Digest::SHA256.hexdigest(rendered)}" unless expected == rendered
end
puts "v0.92 typed doctors: PASS (#{reports.length} claim-null handoffs)"
