#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ISSUE = 5341
BASE = Pathname(".csdlc/prepared/issues/#{ISSUE}")
ISSUE_DIR = Pathname(".csdlc/issues/#{ISSUE}")
CARD_NAMES = %w[sip stp spp vpp srp sor].freeze
EXPECTED_DEPENDENCIES = [
  "WP-06 #5340 GitHub merged, typed closed_out, receipt-retained, and merged-SHA ancestral to current origin/main",
  "WP-07 #5342 GitHub merged, typed closed_out, receipt-retained, and merged-SHA ancestral to current origin/main",
  "#5591 canonical Runtime v3 ingress GitHub merged, typed closed_out, receipt-retained, and merged-SHA ancestral to current origin/main"
].freeze
PREPARATION_PATHS = [
  ".csdlc/issues/5341",
  ".csdlc/locks/5341.lock",
  ".csdlc/prepared/issues/5341",
  ".csdlc/evidence/5341"
].freeze

errors = []

required_files = [
  BASE.join("bootstrap-request.json"),
  BASE.join("design.md"),
  BASE.join("diagram.mmd"),
  BASE.join("dependency_gate.rb"),
  BASE.join("validate_budget.rb")
] + CARD_NAMES.flat_map do |card|
  [ISSUE_DIR.join("cards", "#{card}.md"), ISSUE_DIR.join("cards", "#{card}.values.json")]
end + [ISSUE_DIR.join("index.json"), ISSUE_DIR.join("audit.jsonl")]

required_files.each do |path|
  errors << "missing #{path}" unless path.file?
end

begin
  request = JSON.parse(BASE.join("bootstrap-request.json").read)
  errors << "wrong issue" unless request["issue"] == ISSUE
  errors << "design must be approved" unless request["design_approved"] == true
  errors << "dependencies drift" unless request.dig("initial", "dependencies") == EXPECTED_DEPENDENCIES
  errors << "preparation claim paths drift" unless request.dig("claim", "protected_paths") == PREPARATION_PATHS
  errors << "preparation claim must not protect product paths" if request.dig("claim", "protected_paths").any? { |path| path.start_with?("adl-v2/", "adl-runtime", "adl/src") }
  errors << "review scope missing" if request.dig("initial", "review_scope").to_s.strip.empty?
  errors << "operator constraints missing" if request.dig("initial", "operator_constraints").to_a.empty?

  acceptance_ids = request.dig("initial", "acceptance_criteria").to_a.map { |entry| entry[/\AAC-\d+/] }.compact
  step_ids = request.dig("initial", "steps").to_a.flat_map { |step| step.fetch("acceptance_ids", []) }.uniq
  lane_ids = request.dig("initial", "validation_lanes").to_a.flat_map { |lane| lane.fetch("acceptance_ids", []) }.uniq
  errors << "plan does not cover every acceptance id" unless (acceptance_ids - step_ids).empty?
  errors << "validation does not cover every acceptance id" unless (acceptance_ids - lane_ids).empty?
  errors << "validation contains deferred lane" if request.dig("initial", "validation_lanes").to_a.any? { |lane| !lane["defer_reason"].nil? }
  planned_seconds = request.dig("initial", "validation_lanes").to_a.sum { |lane| lane.fetch("budget_seconds") }
  errors << "validation seconds must total exactly 2400" unless planned_seconds == 2_400
rescue JSON::ParserError, Errno::ENOENT, KeyError => error
  errors << "bootstrap request invalid: #{error.message}"
end

if BASE.join("design.md").file?
  design = BASE.join("design.md").read
  %w[
    "Dependency Gate"
    "Authority Boundary"
    "COTS Inventory"
    "Source And Test Budgets"
    "No-Deferral Acceptance Matrix"
    "Negative Authority Proof"
    "Rollback"
    "Stop Conditions"
  ].each do |heading|
    errors << "design missing #{heading}" unless design.include?(heading)
  end
  errors << "design does not forbid Runtime v2 edits" unless design.include?("Runtime v2")
end

if BASE.join("diagram.mmd").file?
  diagram = BASE.join("diagram.mmd").read
  %w["#5341 owned boundary" "Canonical typed ingress" "Runtime v2" "C-SDLC v2 lifecycle"].each do |label|
    errors << "diagram missing #{label}" unless diagram.include?(label)
  end
end

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5341_preparation_validation.v1",
  "status" => errors.empty? ? "passed" : "failed",
  "errors" => errors
)
exit(errors.empty? ? 0 : 1)
