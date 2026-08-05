#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
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

def number!(value, label)
  number = Float(value)
  abort "#{label} must be finite and nonnegative" unless number.finite? && number >= 0
  number
rescue ArgumentError, TypeError
  abort "#{label} must be numeric"
end

def scalar!(value, label)
  number = Float(value)
  abort "#{label} must be finite" unless number.finite?
  number
rescue ArgumentError, TypeError
  abort "#{label} must be numeric"
end

def percentile(values, fraction)
  sorted = values.sort
  return sorted.first if sorted.length == 1
  rank = fraction * (sorted.length - 1)
  lower = sorted[rank.floor]
  upper = sorted[rank.ceil]
  lower + ((upper - lower) * (rank - rank.floor))
end

def close!(reported, computed, label)
  actual = scalar!(reported, label)
  tolerance = [computed.abs * 0.001, 0.001].max
  abort "#{label} inconsistent: #{actual} != #{computed}" if (actual - computed).abs > tolerance
end

REQUIRED_FILES.each do |name|
  path = EVIDENCE.join(name)
  abort "missing evidence #{path.relative_path_from(ROOT)}" unless path.file? && !path.zero?
end

eligibility = read_json(EVIDENCE.join("eligibility.json"))
%w[
  migration_gate ci_reliability_gate organization_plan_ready owner_budget_approved
  budget_alerts_configured spend_alerts_configured selected_repository_access
  concurrency_one rollback_verified
].each do |gate|
  abort "eligibility gate not proven: #{gate}" unless eligibility[gate] == true
end
%w[organization_plan_receipt budget_approval_receipt budget_alert_receipt spend_alert_receipt].each do |field|
  abort "eligibility missing #{field}" if eligibility[field].to_s.empty?
end
approved_max_total_cost = number!(eligibility["approved_max_total_cost"], "approved maximum total cost")
abort "approved maximum total cost must be positive" unless approved_max_total_cost.positive?

manifest = read_json(EVIDENCE.join("frozen-manifest.json"))
%w[commit_sha workflow_revision rust_toolchain lockfile_digest cache_design permissions required_checks workloads].each do |field|
  abort "frozen manifest missing #{field}" if manifest[field].nil? || manifest[field] == ""
end
thresholds = manifest["adoption_thresholds"] || abort("frozen manifest missing adoption_thresholds")
required_thresholds = %w[
  minimum_critical_path_improvement_fraction
  minimum_reliability
  maximum_candidate_p95_regression_fraction
  maximum_queue_increase_seconds
  maximum_cost_per_minute_saved
]
required_thresholds.each { |field| number!(thresholds[field], "threshold #{field}") }
abort "minimum reliability exceeds one" if thresholds["minimum_reliability"].to_f > 1

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
    metrics = REQUIRED_METRICS.to_h do |metric|
      abort "trial missing #{metric}" unless sample.key?(metric)
      [metric, number!(sample[metric], "#{platform}/#{workload} #{metric}")]
    end
    components = %w[queue_seconds setup_seconds cache_seconds compile_link_seconds execution_seconds artifact_seconds]
    component_total = components.sum { |metric| metrics.fetch(metric) }
    abort "trial total is smaller than measured components for #{platform}/#{workload}" if metrics["total_seconds"] + 0.001 < component_total
    abort "critical path exceeds total for #{platform}/#{workload}" if metrics["critical_path_seconds"] > metrics["total_seconds"] + 0.001
    if cache_state == "warm"
      abort "warm trial lacks cache-hit evidence" unless sample["cache_hit_evidence"] == true
    end
    abort "invalid trial outcome" unless %w[passed failed cancelled retried].include?(sample["outcome"])
    abort "trial must explicitly classify outlier status" unless [true, false].include?(sample["outlier"])
    if sample["outlier"]
      abort "outlier lacks justification" if sample["outlier_reason"].to_s.empty?
    elsif !sample["outlier_reason"].nil? && !sample["outlier_reason"].to_s.empty?
      abort "non-outlier carries an outlier justification"
    end
    abort "trial was dropped from statistics" unless sample["included_in_statistics"] == true
    abort "trial must explicitly classify canary status" unless [true, false].include?(sample["canary"])
  end
end
total_cost = trials.sum { |trial| number!(trial["cost"], "trial cost") }
abort "experiment exceeded approved total cost" if total_cost > approved_max_total_cost
canary_trials = trials.select { |trial| trial["canary"] == true }
abort "experiment must contain exactly one total canary" unless canary_trials.length == 1

parity = read_json(EVIDENCE.join("parity.json"))
%w[result artifact validation required_check].each do |kind|
  abort "parity failed: #{kind}" unless parity[kind] == true
end

decision = read_json(EVIDENCE.join("decision.json"))
rows = Array(decision["lanes"])
abort "decision table is empty" if rows.empty?
abort "decision lanes do not exactly match frozen workloads" unless rows.map { |row| row["lane"] }.sort == workloads.sort
rows.each do |row|
  %w[lane control_mean control_p50 control_p95 candidate_mean candidate_p50 candidate_p95 queue_delta critical_path_delta reliability cost_per_run cost_per_minute_saved proof_parity threshold_disposition decision canary_selected canary_passed].each do |field|
    abort "decision row missing #{field}" if row[field].nil? || row[field] == ""
  end
  abort "invalid decision for #{row['lane']}" unless %w[adopt reject defer].include?(row["decision"])
  lane = row["lane"]
  control = trials.select { |trial| trial["platform"] == "ubuntu-latest" && trial["workload"] == lane }
  candidate = trials.select { |trial| trial["platform"] == "github-hosted-ubuntu-16-core" && trial["workload"] == lane }
  abort "decision lane lacks raw samples: #{lane}" if control.empty? || candidate.empty?

  control_totals = control.map { |trial| number!(trial["total_seconds"], "#{lane} control total") }
  candidate_totals = candidate.map { |trial| number!(trial["total_seconds"], "#{lane} candidate total") }
  control_mean = control_totals.sum.fdiv(control_totals.length)
  candidate_mean = candidate_totals.sum.fdiv(candidate_totals.length)
  control_critical = control.sum { |trial| number!(trial["critical_path_seconds"], "#{lane} control critical") } / control.length
  candidate_critical = candidate.sum { |trial| number!(trial["critical_path_seconds"], "#{lane} candidate critical") } / candidate.length
  queue_delta = candidate.sum { |trial| number!(trial["queue_seconds"], "#{lane} candidate queue") } / candidate.length -
    control.sum { |trial| number!(trial["queue_seconds"], "#{lane} control queue") } / control.length
  critical_delta = candidate_critical - control_critical
  reliability = candidate.count { |trial| trial["outcome"] == "passed" }.fdiv(candidate.length)
  cost_per_run = candidate.sum { |trial| number!(trial["cost"], "#{lane} candidate cost") } / candidate.length
  minutes_saved = [(-critical_delta) / 60.0, 0.0].max
  cost_per_minute_saved = minutes_saved.positive? ? cost_per_run / minutes_saved : Float::INFINITY

  close!(row["control_mean"], control_mean, "#{lane} control_mean")
  close!(row["control_p50"], percentile(control_totals, 0.50), "#{lane} control_p50")
  close!(row["control_p95"], percentile(control_totals, 0.95), "#{lane} control_p95")
  close!(row["candidate_mean"], candidate_mean, "#{lane} candidate_mean")
  close!(row["candidate_p50"], percentile(candidate_totals, 0.50), "#{lane} candidate_p50")
  close!(row["candidate_p95"], percentile(candidate_totals, 0.95), "#{lane} candidate_p95")
  close!(row["queue_delta"], queue_delta, "#{lane} queue_delta")
  close!(row["critical_path_delta"], critical_delta, "#{lane} critical_path_delta")
  close!(row["reliability"], reliability, "#{lane} reliability")
  close!(row["cost_per_run"], cost_per_run, "#{lane} cost_per_run")
  if cost_per_minute_saved.finite?
    close!(row["cost_per_minute_saved"], cost_per_minute_saved, "#{lane} cost_per_minute_saved")
  else
    abort "#{lane} cost_per_minute_saved must be not_applicable without savings" unless row["cost_per_minute_saved"] == "not_applicable"
  end

  control_p95 = percentile(control_totals, 0.95)
  candidate_p95 = percentile(candidate_totals, 0.95)
  improvement = control_critical.zero? ? 0.0 : -critical_delta / control_critical
  p95_regression = control_p95.zero? ? 0.0 : (candidate_p95 - control_p95) / control_p95
  gates = {
    "critical_path" => improvement >= thresholds["minimum_critical_path_improvement_fraction"].to_f,
    "reliability" => reliability >= thresholds["minimum_reliability"].to_f,
    "p95" => p95_regression <= thresholds["maximum_candidate_p95_regression_fraction"].to_f,
    "queue" => queue_delta <= thresholds["maximum_queue_increase_seconds"].to_f,
    "cost" => cost_per_minute_saved.finite? && cost_per_minute_saved <= thresholds["maximum_cost_per_minute_saved"].to_f,
    "proof_parity" => row["proof_parity"] == true
  }
  expected_disposition = gates.values.all? ? "meets_all_thresholds" : "fails_thresholds"
  abort "#{lane} threshold disposition mismatch" unless row["threshold_disposition"] == expected_disposition
  abort "#{lane} adopted below predeclared thresholds: #{gates.reject { |_k, value| value }.keys.join(', ')}" if row["decision"] == "adopt" && !gates.values.all?
  if row["canary_selected"]
    canary = canary_trials.first
    abort "#{lane} selected canary does not match retained trial" unless canary["workload"] == lane && canary["platform"] == "github-hosted-ubuntu-16-core"
    abort "#{lane} canary outcome mismatch" unless row["canary_passed"] == (canary["outcome"] == "passed")
  else
    abort "#{lane} reports canary pass without selection" unless row["canary_passed"] == false
  end
  abort "adopted lane lacks the single successful canary" if row["decision"] == "adopt" && (row["canary_selected"] != true || row["canary_passed"] != true)
end
abort "decision table must select exactly one canary lane" unless rows.count { |row| row["canary_selected"] == true } == 1

final_state = read_json(EVIDENCE.join("final-state.json"))
abort "standard-runner fallback not retained" unless final_state["standard_runner_fallback"] == true
abort "required-check identity changed" unless final_state["required_check_identity_preserved"] == true
abort "experiment cleanup is incomplete" unless final_state["cleanup_completed"] == true
abort "runner-group concurrency changed" unless final_state["runner_group_concurrency"] == 1
abort "experimental resources remain" unless final_state["experimental_resources_remaining"] == 0
close!(final_state["total_cost"], total_cost, "final total_cost")
abort "final state lost approved budget ceiling" unless final_state["approved_max_total_cost"].to_f == approved_max_total_cost
if rows.any? { |row| row["decision"] == "adopt" }
  abort "adopted routing lacks ten-run observation" unless final_state["representative_observation_runs"].to_i >= 10
else
  abort "rejected or deferred configuration was not cleaned up" unless final_state["experimental_configuration_removed"] == true
end

puts "WP-02B experiment evidence valid: #{trials.length} trials, #{rows.length} lane decisions"
