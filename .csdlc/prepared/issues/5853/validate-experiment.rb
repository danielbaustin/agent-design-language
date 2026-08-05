#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5853")
REQUIRED_FILES = %w[
  eligibility.json
  frozen-manifest.json
  trials.jsonl
  parity.json
  decision.json
  final-state.json
].freeze
REQUIRED_METRICS = %w[
  queue_seconds
  setup_seconds
  cache_seconds
  compile_link_seconds
  execution_seconds
  artifact_seconds
  total_seconds
  critical_path_seconds
  cost
].freeze

def read_json(path)
  JSON.parse(path.read)
rescue JSON::ParserError => e
  abort "invalid JSON #{path.relative_path_from(ROOT)}: #{e.message}"
end

REQUIRED_FILES.each do |name|
  path = EVIDENCE.join(name)
  abort "missing evidence #{path.relative_path_from(ROOT)}" unless path.file? && !path.zero?
end

eligibility = read_json(EVIDENCE.join("eligibility.json"))
%w[migration_gate ci_reliability_gate owner_budget_approved selected_repository_access concurrency_one rollback_verified].each do |gate|
  abort "eligibility gate not proven: #{gate}" unless eligibility[gate] == true
end

manifest = read_json(EVIDENCE.join("frozen-manifest.json"))
%w[commit_sha workflow_revision rust_toolchain lockfile_digest cache_design permissions required_checks workloads].each do |field|
  abort "frozen manifest missing #{field}" if manifest[field].nil? || manifest[field] == ""
end

trials = EVIDENCE.join("trials.jsonl").each_line.filter_map do |line|
  next if line.strip.empty?
  JSON.parse(line)
rescue JSON::ParserError => e
  abort "invalid trial JSON: #{e.message}"
end
abort "no trials retained" if trials.empty?

groups = trials.group_by { |trial| [trial["platform"], trial["workload"], trial["cache_state"]] }
platforms = %w[ubuntu-latest github-hosted-ubuntu-16-core]
workloads = Array(manifest["workloads"])
platforms.product(workloads, %w[cold warm]).each do |platform, workload, cache_state|
  samples = groups.fetch([platform, workload, cache_state], [])
  minimum = cache_state == "cold" ? 5 : 10
  abort "insufficient #{cache_state} samples for #{platform}/#{workload}: #{samples.length}/#{minimum}" if samples.length < minimum
  samples.each do |sample|
    abort "trial revision drift for #{platform}/#{workload}" unless sample["commit_sha"] == manifest["commit_sha"]
    REQUIRED_METRICS.each { |metric| abort "trial missing #{metric}" unless sample.key?(metric) }
    if cache_state == "warm"
      abort "warm trial lacks cache-hit evidence" unless sample["cache_hit_evidence"] == true
    end
    abort "trial outcome missing" if sample["outcome"].to_s.empty?
  end
end

parity = read_json(EVIDENCE.join("parity.json"))
%w[result artifact validation required_check].each do |kind|
  abort "parity failed: #{kind}" unless parity[kind] == true
end

decision = read_json(EVIDENCE.join("decision.json"))
rows = Array(decision["lanes"])
abort "decision table is empty" if rows.empty?
rows.each do |row|
  %w[lane control_p50 control_p95 candidate_p50 candidate_p95 queue_delta critical_path_delta cost_per_run cost_per_minute_saved proof_parity decision].each do |field|
    abort "decision row missing #{field}" if row[field].nil? || row[field] == ""
  end
  abort "invalid decision for #{row['lane']}" unless %w[adopt reject defer].include?(row["decision"])
  abort "adopted lane lacks successful canary" if row["decision"] == "adopt" && row["canary_passed"] != true
end

final_state = read_json(EVIDENCE.join("final-state.json"))
abort "standard-runner fallback not retained" unless final_state["standard_runner_fallback"] == true
abort "required-check identity changed" unless final_state["required_check_identity_preserved"] == true
if rows.any? { |row| row["decision"] == "adopt" }
  abort "adopted routing lacks ten-run observation" unless final_state["representative_observation_runs"].to_i >= 10
else
  abort "rejected or deferred configuration was not cleaned up" unless final_state["experimental_configuration_removed"] == true
end

puts "WP-02B experiment evidence valid: #{trials.length} trials, #{rows.length} lane decisions"
