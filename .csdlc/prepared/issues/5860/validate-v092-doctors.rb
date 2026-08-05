#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ISSUES = [
  5786, 5795, 5800, 5801, 5812, 5818, 5819, 5820, 5821, 5822, 5823,
  5824, 5825, 5826, 5827, 5828, 5829, 5830, 5831, 5832, 5833, 5834,
  5835, 5836, 5837, 5838, 5839, 5840, 5841, 5842, 5843, 5844, 5845,
  5846, 5847, 5848, 5849, 5850, 5851, 5852, 5853
].freeze

DOCTOR = "csdlc-v2/target/debug/csdlc-doctor"
OUTPUT = ".csdlc/evidence/5860/V092_TYPED_DOCTOR_REPORTS.json"

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
File.write(OUTPUT, JSON.pretty_generate(payload) + "\n")
puts "v0.92 typed doctors: PASS (#{reports.length} claim-null handoffs)"
