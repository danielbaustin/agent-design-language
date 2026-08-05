#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

mode = ARGV.fetch(0)
expected_features = %w[
  ACP_COGNITIVE_PROFILES_v0.92.md
  ADAPTIVE_LEARNING_DAG_v0.92.md
  ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
  DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
  CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md
  FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md
  IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
  MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
  MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md
  OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md
  PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md
  RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md
  FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
].sort

def present?(value)
  value.is_a?(String) ? !value.strip.empty? : !value.nil?
end

case mode
when "matrix"
  packet = JSON.parse(File.read(ARGV.fetch(1, ".csdlc/evidence/5842/feature-completion-matrix.json")))
  rows = packet["rows"]
  abort "exactly 13 feature rows required" unless rows.is_a?(Array) && rows.length == expected_features.length
  abort "feature universe mismatch" unless rows.map { |row| row["feature"] }.sort == expected_features
  required = %w[feature owner_issue implementation_paths reviewed_head pr merge_sha validation_ref negative_ref integration_ref platform_ref terminal_ref disposition]
  rows.each do |row|
    abort "incomplete #{row['feature']} row" unless required.all? { |key| present?(row[key]) }
    abort "implementation paths missing" unless row["implementation_paths"].is_a?(Array) && !row["implementation_paths"].empty?
    abort "feature not accepted" unless row["disposition"] == "accepted"
    %w[validation_ref negative_ref integration_ref platform_ref terminal_ref].each do |key|
      abort "missing #{key} file" unless File.file?(row[key])
    end
  end
when "negative"
  packet = JSON.parse(File.read(ARGV.fetch(1, ".csdlc/evidence/5842/negative-cases.json")))
  cases = packet["cases"]
  required_classes = %w[fixture receipt_only demo_mode synthetic provider_substitution stale_review missing_ancestry unsupported_platform].sort
  abort "negative class universe mismatch" unless cases.is_a?(Array) && cases.map { |row| row["class"] }.sort == required_classes
  cases.each do |row|
    argv = row["gate_argv"]
    abort "negative argv missing" unless argv.is_a?(Array) && !argv.empty?
    stdout, stderr, status = Open3.capture3(*argv)
    abort "negative case escaped: #{row['class']}" if status.success?
    abort "negative evidence digest mismatch" unless Digest::SHA256.hexdigest(stdout + stderr) == row["observed_sha256"]
  end
else
  abort "usage: #{$PROGRAM_NAME} matrix|negative [evidence.json]"
end

puts "PASS: quality-gate #{mode} proof"
