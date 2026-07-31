#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ISSUE = 5349
BASE = Pathname(".csdlc/prepared/issues/#{ISSUE}")
ISSUE_DIR = Pathname(".csdlc/issues/#{ISSUE}")
CARD_NAMES = %w[sip stp spp vpp srp sor].freeze
EXPECTED_DEPENDENCIES = [
  "WP-06 #5340 GitHub merged, typed closed_out, receipt-retained, and merged-SHA ancestral to current origin/main",
  "WP-08 #5341 GitHub merged, typed closed_out, receipt-retained, and merged-SHA ancestral to current origin/main",
  "WP-07 #5342 records/trust and #5591 Runtime v3 ingress are consumed transitively through terminal #5341 rather than added as conflicting direct wave dependencies",
  "#5526 is a downstream WP-09 provider-expansion child and does not block parent interface implementation or deterministic acceptance"
].freeze
PREPARATION_PATHS = [
  ".csdlc/issues/5349",
  ".csdlc/locks/5349.lock",
  ".csdlc/prepared/issues/5349",
  ".csdlc/evidence/5349"
].freeze
EXPECTED_COTS = [
  "reqwest 0.13.4",
  "secrecy 0.10.3",
  "url 2.5.8",
  "serde 1.0.229",
  "serde_json 1.0.151",
  "tokio 1.53.1",
  "wiremock 0.6.5"
].freeze

errors = []

required_files = [
  BASE.join("bootstrap-request.json"),
  BASE.join("design.md"),
  BASE.join("diagram.mmd"),
  BASE.join("dependency_gate.rb"),
  BASE.join("validate_budget.rb"),
  BASE.join("run_validation_lane.rb")
] + CARD_NAMES.flat_map do |card|
  [ISSUE_DIR.join("cards", "#{card}.md"), ISSUE_DIR.join("cards", "#{card}.values.json")]
end + [ISSUE_DIR.join("index.json"), ISSUE_DIR.join("audit.jsonl")]

required_files.each do |path|
  errors << "missing #{path}" unless path.file?
end

begin
  request = JSON.parse(BASE.join("bootstrap-request.json").read)
  errors << "wrong issue" unless request["issue"] == ISSUE
  errors << "dependencies drift" unless request.dig("initial", "dependencies") == EXPECTED_DEPENDENCIES
  errors << "preparation claim paths drift" unless request.dig("claim", "protected_paths") == PREPARATION_PATHS
  errors << "preparation claim includes product path" if request.dig("claim", "protected_paths").any? { |path| path.start_with?("adl-v2/", "adl-runtime", "adl/src") }
  errors << "future product path missing" unless request.dig("initial", "declared_scope").join("\n").include?("adl-v2/crates/adl-adapters")
  errors << "review scope missing" if request.dig("initial", "review_scope").to_s.strip.empty?
  errors << "operator constraints missing" if request.dig("initial", "operator_constraints").to_a.empty?

  cots_text = [request.dig("initial", "deliverables"), request.dig("initial", "acceptance_criteria")].flatten.join("\n")
  EXPECTED_COTS.each { |entry| errors << "COTS pin missing: #{entry}" unless cots_text.include?(entry) }

  acceptance_ids = request.dig("initial", "acceptance_criteria").to_a.map { |entry| entry[/\AAC-\d+/] }.compact
  step_ids = request.dig("initial", "steps").to_a.flat_map { |step| step.fetch("acceptance_ids", []) }.uniq
  lane_ids = request.dig("initial", "validation_lanes").to_a.flat_map { |lane| lane.fetch("acceptance_ids", []) }.uniq
  errors << "plan does not cover every acceptance id" unless (acceptance_ids - step_ids).empty?
  errors << "validation does not cover every acceptance id" unless (acceptance_ids - lane_ids).empty?
  errors << "validation contains deferred lane" if request.dig("initial", "validation_lanes").to_a.any? { |lane| !lane["defer_reason"].nil? }
  planned_seconds = request.dig("initial", "validation_lanes").to_a.sum { |lane| lane.fetch("budget_seconds") }
  errors << "validation lane allocation must total exactly 3600 seconds" unless planned_seconds == 3_600
rescue JSON::ParserError, Errno::ENOENT, KeyError => error
  errors << "bootstrap request invalid: #{error.message}"
end

if ISSUE_DIR.join("index.json").file?
  begin
    index = JSON.parse(ISSUE_DIR.join("index.json").read)
    errors << "typed issue mismatch" unless index["issue"] == ISSUE
    unless %w[initialized bound].include?(index["phase"])
      errors << "typed phase must be initialized or bound during preparation"
    end
    errors << "typed claim missing" unless index["claim"].is_a?(Hash)
    errors << "typed protected paths drift" unless index.dig("claim", "protected_paths") == PREPARATION_PATHS
    errors << "typed purpose is not preparation-only" unless index.dig("claim", "purpose").to_s.include?("without product implementation")
  rescue JSON::ParserError => error
    errors << "typed index invalid: #{error.message}"
  end
end

vpp_values_path = ISSUE_DIR.join("cards", "vpp.values.json")
if vpp_values_path.file?
  begin
    vpp_values = JSON.parse(vpp_values_path.read)
    unless vpp_values.dig("content", "values", "planned_validation_seconds") == 7_200
      errors << "planning profile hard validation ceiling must be 7200 seconds"
    end
  rescue JSON::ParserError => error
    errors << "VPP values invalid: #{error.message}"
  end
end

if BASE.join("design.md").file?
  design = BASE.join("design.md").read
  [
    "Dependency Gate",
    "Authority Boundary",
    "Design-By-Contract Adapter Surface",
    "COTS Inventory",
    "Source And Test Budgets",
    "No-Deferral Acceptance Matrix",
    "No-Credential Live-Claim Gate",
    "Rollback",
    "Stop Conditions"
  ].each do |heading|
    errors << "design missing #{heading}" unless design.include?(heading)
  end
  errors << "design does not forbid Runtime v2" unless design.include?("Runtime v2")
  errors << "design does not forbid AWS" unless design.include?("No AWS") || design.include?("AWS")
end

if BASE.join("diagram.mmd").file?
  diagram = BASE.join("diagram.mmd").read
  ["#5349 owned boundary", "Bounded HTTPS adapter", "Governed-tool adapter", "Runtime v2", "AWS", "C-SDLC v2"].each do |label|
    errors << "diagram missing #{label}" unless diagram.include?(label)
  end
end

errors << "product crate must not exist during preparation" if Pathname("adl-v2/crates/adl-adapters").exist?

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5349_preparation_validation.v1",
  "status" => errors.empty? ? "passed" : "failed",
  "errors" => errors
)
exit(errors.empty? ? 0 : 1)
