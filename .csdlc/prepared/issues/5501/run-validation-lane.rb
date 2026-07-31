#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "tempfile"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[live-manifest live-two-shard baseline-comparison post-merge-exact].freeze
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

if lane == "live-manifest"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  manifest = root.join(".csdlc/evidence/5501/live-run-manifest.json")
  validator = Pathname.new(__dir__).join("validate-live-run-manifest.rb")
  system("ruby", validator.to_s, manifest.to_s) || exit(2)

  source = JSON.parse(manifest.read)
  mutations = {
    "digest mismatch" => lambda do |copy|
      copy["negative_case"]["evidence_digest"] = "0" * 64
    end,
    "parent traversal" => lambda do |copy|
      copy["negative_case"]["evidence_ref"] = "../outside.json"
    end,
    "absent evidence" => lambda do |copy|
      copy["dashboard"]["observation_ref"] = ".csdlc/evidence/5501/absent.json"
    end
  }
  mutations.each do |label, mutate|
    copy = Marshal.load(Marshal.dump(source))
    mutate.call(copy)
    Tempfile.create(["5501-manifest-negative", ".json"]) do |file|
      file.write(JSON.generate(copy))
      file.flush
      _stdout, _stderr, status = Open3.capture3("ruby", validator.to_s, file.path)
      abort("manifest validator accepted #{label}") if status.success?
    end
  end
  puts JSON.pretty_generate(status: "pass", lane: lane, negative_cases: mutations.keys)
  exit 0
end

if lane == "live-two-shard"
  validator = Pathname.new(__dir__).join("validate-retained-live-proof.rb")
  exec("ruby", validator.to_s)
end

if lane == "baseline-comparison"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  baseline = root.join(".csdlc/evidence/5501/single-agent-comparison.json")
  proof = root.join(".csdlc/evidence/5501/retained-live-proof.json")
  fail "single-agent comparison evidence is absent" unless baseline.file?
  fail "retained proof evidence is absent" unless proof.file?

  data = JSON.parse(baseline.read)
  unless data["baseline_status"] == "measured_lifecycle_window_comparison"
    abort("baseline comparison must retain measured lifecycle windows")
  end
  parallel_seconds = data.dig("parallel_workcell", "observed_window", "elapsed_seconds")
  serialized_seconds = data.dig("single_agent_baseline", "serialized_observed_window_seconds")
  unless parallel_seconds.is_a?(Integer) && parallel_seconds.positive?
    abort("parallel lifecycle window is not measured")
  end
  unless serialized_seconds.is_a?(Integer) && serialized_seconds >= parallel_seconds
    abort("serialized lifecycle window is not measured fairly")
  end
  unless data.dig("comparison", "fairness_result") == "measured_bounded_comparison_without_speedup_claim" &&
         data.dig("comparison", "numeric_speedup_claim") == false
    abort("baseline comparison must avoid numeric speedup claims")
  end
  puts JSON.pretty_generate(status: "pass", lane: lane, baseline: data["baseline_status"])
  exit 0
end

if lane == "post-merge-exact"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  dependency_gate = Pathname.new(__dir__).join("check-dependencies.rb")
  manifest = root.join(".csdlc/evidence/5501/live-run-manifest.json")
  manifest_validator = Pathname.new(__dir__).join("validate-live-run-manifest.rb")
  retained_validator = Pathname.new(__dir__).join("validate-retained-live-proof.rb")
  system("ruby", dependency_gate.to_s) || exit(2)
  system("ruby", manifest_validator.to_s, manifest.to_s) || exit(2)
  system("ruby", retained_validator.to_s) || exit(2)
  puts JSON.pretty_generate(status: "pass", lane: lane)
  exit 0
end
