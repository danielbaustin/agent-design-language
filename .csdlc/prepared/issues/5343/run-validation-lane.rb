#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[transaction-fault-matrix fresh-install-override rollback-window-evidence cutover-budgets post-merge-exact]
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

root = Pathname.new(__dir__).join("../../../..").expand_path
report = root.join("docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json")

case lane
when "transaction-fault-matrix"
  v1 = ENV.fetch("ADL_WP12_V1_BINARY") do
    abort("ADL_WP12_V1_BINARY must name the installed v1 executable")
  end
  command = [
    "bash",
    ".csdlc/prepared/issues/5343/run-cutover-proof.sh",
    "--v1-binary",
    v1
  ]
  stdout, stderr, status = Open3.capture3(*command, chdir: root.to_s)
  warn(stderr) unless stderr.empty?
  abort("#{lane} failed") unless status.success?
  result = JSON.parse(stdout)
  puts JSON.pretty_generate(status: "pass", lane: lane, report: result)
when "fresh-install-override"
  abort("cutover report is absent") unless report.file?
  value = JSON.parse(report.read)
  abort("fresh v2 installation was not executed") unless value.dig("v2", "fresh_install") == true
  abort("v2 is not the final default") unless value["final_default"] == "adl-v2"
  abort("v1 was not retained as previous") unless value["retained_previous"] == "adl-v1"
  puts JSON.pretty_generate(status: "pass", lane: lane, report: report.relative_path_from(root).to_s)
when "rollback-window-evidence"
  abort("cutover report is absent") unless report.file?
  value = JSON.parse(report.read)
  abort("v1 rollback is not exact") unless value["exact_prior_bytes_restored"] == true
  abort("v1 was not executed after rollback") unless value.dig("v1", "executed_after_rollback") == true
  abort("rollback window is not 14 days") unless value.dig("rollback_window", "duration_days") == 14
  abort("legacy deletion was authorized") unless value.dig("rollback_window", "deletion_authorized") == false
  puts JSON.pretty_generate(status: "pass", lane: lane, report: report.relative_path_from(root).to_s)
when "cutover-budgets"
  scripts = [
    ".csdlc/prepared/issues/5343/check-dependencies.rb",
    ".csdlc/prepared/issues/5343/run-validation-lane.rb",
    ".csdlc/prepared/issues/5343/run-cutover-proof.sh"
  ]
  lines = scripts.to_h do |path|
    count = root.join(path).each_line.count { |line| !line.strip.empty? }
    [path, count]
  end
  abort("cutover orchestration exceeds 500 nonblank lines") if lines.values.sum > 500
  abort("one cutover module exceeds 400 nonblank lines") if lines.values.any? { |count| count >= 400 }
  puts JSON.pretty_generate(status: "pass", lane: lane, nonblank_lines: lines, total: lines.values.sum)
when "post-merge-exact"
  abort("cutover report is absent") unless report.file?
  value = JSON.parse(report.read)
  abort("cutover report is not accepted") unless value["status"] == "pass"
  abort("v1 rollback is not exact") unless value["exact_prior_bytes_restored"] == true
  abort("v2 is not the final default") unless value["final_default"] == "adl-v2"
  abort("legacy deletion was recorded") unless value["legacy_deleted"] == false
  puts JSON.pretty_generate(status: "pass", lane: lane, report: report.relative_path_from(root).to_s)
end
