#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "rbconfig"
require "yaml"

FEATURE_DIR = "docs/milestones/v0.92/features"
EXPECTED_FEATURES = %w[
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
].sort.freeze
EXPECTED_CRITICAL = %w[WP-04 WP-05 WP-06 WP-07 WP-13A WP-20 WP-21 WP-21A].sort.freeze
EVIDENCE_FIELDS = %w[validation negative integration platform terminal].freeze
EVIDENCE_KIND = {
  "validation" => "validation_result",
  "negative" => "negative_result",
  "integration" => "integration_result",
  "platform" => "platform_result",
  "terminal" => "terminal_result"
}.freeze

def present?(value)
  value.is_a?(String) && !value.strip.empty?
end

def git(*argv)
  out, err, status = Open3.capture3("git", *argv)
  abort "git #{argv.join(' ')} failed: #{err}" unless status.success?
  out.strip
end

def github_pr(number)
  out, err, status = Open3.capture3("gh", "pr", "view", number.to_s, "--json", "number,headRefOid,mergeCommit,state")
  abort "cannot read PR ##{number}: #{err}" unless status.success?
  JSON.parse(out)
end

def verify_evidence_ref(ref, row, field, gate_sha)
  abort "evidence ref must contain path and sha256" unless ref.is_a?(Hash)
  path = ref["path"]
  digest = ref["sha256"]
  abort "evidence path missing" unless present?(path) && File.file?(path)
  abort "evidence digest malformed" unless digest.to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "evidence digest mismatch for #{path}" unless Digest::SHA256.file(path).hexdigest == digest
  evidence = JSON.parse(File.read(path))
  abort "#{field} schema mismatch" unless evidence["schema"] == EVIDENCE_KIND.fetch(field)
  abort "#{field} issue identity mismatch" unless evidence["issue"] == Integer(row["owner_issue"])
  abort "#{field} PR identity mismatch" unless evidence["pr"] == Integer(row["pr"])
  abort "#{field} reviewed SHA mismatch" unless evidence["reviewed_sha"] == row["reviewed_head"]
  abort "#{field} result did not pass" unless evidence["result"] == "passed"
  abort "#{field} produced after gate SHA" unless system("git", "merge-base", "--is-ancestor", evidence["reviewed_sha"], gate_sha)
  case field
  when "validation", "negative"
    abort "#{field} commands empty" unless evidence["commands"].is_a?(Array) && !evidence["commands"].empty?
    abort "#{field} contains nonzero exit" unless evidence["commands"].all? { |command| command["exit_code"] == 0 && present?(command["output_sha256"]) }
  when "integration"
    abort "integration merge mismatch" unless evidence["merge_sha"] == row["merge_sha"]
    abort "integration ancestry not proved" unless evidence["ancestral_to_gate"] == true
  when "platform"
    platforms = evidence["platforms"]
    abort "platform universe must be native macOS and Linux" unless platforms.is_a?(Array) && platforms.map { |item| item["os"] }.sort == %w[linux macos]
    abort "platform run failed or unbound" unless platforms.all? { |item| item["conclusion"] == "success" && item["head_sha"] == row["reviewed_head"] && present?(item["run_url"]) }
  when "terminal"
    abort "terminal phase not closed_out" unless evidence["phase"] == "closed_out"
    abort "terminal claim remains active" unless evidence["claim_released"] == true
    abort "terminal receipt missing" unless present?(evidence["receipt_sha256"])
  end
end

mode = ARGV.fetch(0)
case mode
when "matrix"
  packet = JSON.parse(File.read(ARGV.fetch(1, ".csdlc/evidence/5842/feature-completion-matrix.json")))
  abort "gate SHA is not HEAD" unless packet["gate_sha"] == git("rev-parse", "HEAD")
  rows = packet["rows"]
  abort "rows missing" unless rows.is_a?(Array)
  features = rows.select { |row| row["kind"] == "feature" }
  critical = rows.select { |row| row["kind"] == "critical_path" }
  abort "feature universe mismatch" unless features.map { |row| row["id"] }.sort == EXPECTED_FEATURES
  abort "critical-path universe mismatch" unless critical.map { |row| row["id"] }.sort == EXPECTED_CRITICAL
  EXPECTED_FEATURES.each { |name| abort "indexed feature missing" unless File.file?(File.join(FEATURE_DIR, name)) }

  rows.each do |row|
    %w[id kind owner_issue reviewed_head pr merge_sha disposition].each do |key|
      abort "#{row['id']} missing #{key}" unless present?(row[key].to_s)
    end
    paths = row["implementation_paths"]
    abort "#{row['id']} implementation paths missing" unless paths.is_a?(Array) && !paths.empty?
    paths.each { |path| abort "#{row['id']} missing implementation path #{path}" unless File.exist?(path) }
    EVIDENCE_FIELDS.each { |field| verify_evidence_ref(row.fetch("#{field}_evidence"), row, field, packet["gate_sha"]) }
    abort "#{row['id']} not accepted" unless row["disposition"] == "accepted"

    pr = github_pr(Integer(row["pr"].to_s, 10))
    abort "#{row['id']} PR not merged" unless pr["state"] == "MERGED"
    abort "#{row['id']} reviewed head mismatch" unless pr["headRefOid"] == row["reviewed_head"]
    abort "#{row['id']} merge SHA mismatch" unless pr.dig("mergeCommit", "oid") == row["merge_sha"]
    abort "#{row['id']} merge not ancestral" unless system("git", "merge-base", "--is-ancestor", row["merge_sha"], packet["gate_sha"])
  end
when "negative"
  packet = JSON.parse(File.read(ARGV.fetch(1, ".csdlc/evidence/5842/negative-cases.json")))
  required = %w[fixture receipt_only demo_mode synthetic provider_substitution stale_review missing_ancestry unsupported_platform].sort
  cases = packet["cases"]
  abort "negative class universe mismatch" unless cases.is_a?(Array) && cases.map { |row| row["class"] }.sort == required
  cases.each do |row|
    fixture = row["matrix_path"]
    abort "negative fixture missing" unless File.file?(fixture)
    out, err, status = Open3.capture3(RbConfig.ruby, __FILE__, "matrix", fixture)
    abort "negative case escaped: #{row['class']}" if status.success?
    abort "negative output digest mismatch" unless Digest::SHA256.hexdigest(out + err) == row["observed_sha256"]
  end
else
  abort "usage: #{$PROGRAM_NAME} matrix|negative [evidence.json]"
end

puts "PASS: quality-gate #{mode} proof"
