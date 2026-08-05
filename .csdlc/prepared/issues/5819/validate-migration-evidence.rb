#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5819")
REPOS = %w[cognitive-sdlc-paper godel-hadamard-bayes-paper general-intelligence-paper-private universal-tool-schema agent-design-language].freeze
SURFACES = %w[
  visibility history issues pull_requests assignees collaborators teams oidc
  webhooks apps rulesets releases actions pages packages lfs secrets variables
].freeze
NAME_ONLY_SURFACES = %w[secrets variables].freeze
SHA256 = /\A[0-9a-f]{64}\z/

def canonical(value)
  case value
  when Hash then value.keys.sort.to_h { |key| [key, canonical(value.fetch(key))] }
  when Array then value.map { |item| canonical(item) }.sort_by { |item| JSON.generate(item) }
  else value
  end
end

def sha256_json(value)
  Digest::SHA256.hexdigest(JSON.generate(canonical(value)))
end

def load_manifest(root, relative, expected_digest, label)
  path = root.join(relative.to_s).cleanpath
  abort "#{label} manifest escapes repository" unless path.to_s.start_with?(root.to_s + File::SEPARATOR)
  abort "missing #{label} manifest: #{relative}" unless path.file? && !path.zero?
  abort "invalid #{label} manifest digest" unless expected_digest.to_s.match?(SHA256)
  abort "#{label} manifest digest mismatch" unless Digest::SHA256.file(path).hexdigest == expected_digest
  JSON.parse(path.read)
end

def timestamp!(value, label)
  Time.iso8601(value.to_s)
rescue ArgumentError
  abort "invalid #{label} timestamp"
end

def names_only!(value, label)
  rows = Array(value)
  rows.each do |row|
    abort "#{label} entry must be an object" unless row.is_a?(Hash)
    abort "#{label} entry missing name" if row["name"].to_s.empty?
    abort "#{label} entry exposes a value" if row.keys.any? { |key| key.match?(/value|plaintext|ciphertext|secret_value/i) }
    allowed = %w[name scope visibility selected_repositories created_at updated_at]
    abort "#{label} entry has non-name metadata: #{(row.keys - allowed).join(', ')}" unless (row.keys - allowed).empty?
  end
end

report_path = EVIDENCE.join("migration-report.json")
abort "missing migration report" unless report_path.file? && !report_path.zero?
report = JSON.parse(report_path.read)
rows = Array(report["repositories"])
abort "repository order or scope mismatch" unless rows.map { |row| row["name"] } == REPOS
abort "transfer sequence mismatch" unless rows.map { |row| row["transfer_sequence"] } == (1..REPOS.length).to_a

previous_completed = nil
rows.each do |row|
  name = row.fetch("name")
  abort "wrong destination for #{name}" unless row["source"] == "danielbaustin/#{name}" && row["destination"] == "agent-logic/#{name}"
  started = timestamp!(row["transfer_started_at"], "#{name} transfer start")
  completed = timestamp!(row["transfer_completed_at"], "#{name} transfer completion")
  abort "#{name} transfer completes before it starts" unless completed >= started
  abort "#{name} transfer overlaps or is not strictly after prior repository" if previous_completed && started <= previous_completed
  previous_completed = completed

  before = load_manifest(ROOT, row["before_manifest_path"], row["before_manifest_sha256"], "#{name} before")
  after = load_manifest(ROOT, row["after_manifest_path"], row["after_manifest_sha256"], "#{name} after")
  [before, after].each do |manifest|
    abort "#{name} manifest repository mismatch" unless manifest["repository"].to_s.end_with?("/#{name}")
    abort "#{name} manifest lacks exact head" unless manifest["exact_head"].to_s.match?(/\A[0-9a-f]{40}\z/)
    abort "#{name} manifest lacks default branch" if manifest["default_branch"].to_s.empty?
    missing = SURFACES - manifest.fetch("surfaces", {}).keys
    extra = manifest.fetch("surfaces", {}).keys - SURFACES
    abort "#{name} manifest surface mismatch; missing=#{missing.join(', ')} extra=#{extra.join(', ')}" unless missing.empty? && extra.empty?
    NAME_ONLY_SURFACES.each { |surface| names_only!(manifest.fetch("surfaces").fetch(surface), "#{name} #{surface}") }
  end
  abort "#{name} history head changed" unless before["exact_head"] == after["exact_head"] && after["exact_head"] == row["exact_head"]
  abort "#{name} default branch changed" unless before["default_branch"] == after["default_branch"] && after["default_branch"] == row["default_branch"]
  abort "#{name} history surface disagrees with exact head" unless after.dig("surfaces", "history", "default_head") == row["exact_head"]
  abort "#{name} visibility changed" unless before.dig("surfaces", "visibility") == after.dig("surfaces", "visibility")

  dispositions = row.fetch("surface_dispositions", {})
  abort "#{name} disposition denominator mismatch" unless dispositions.keys.sort == SURFACES.sort
  SURFACES.each do |surface|
    before_digest = sha256_json(before.fetch("surfaces").fetch(surface))
    after_digest = sha256_json(after.fetch("surfaces").fetch(surface))
    disposition = dispositions.fetch(surface)
    if before_digest == after_digest
      abort "#{name} #{surface} must be recorded preserved" unless disposition["status"] == "preserved"
    else
      abort "#{name} #{surface} drift lacks verified disposition" unless disposition["status"] == "verified_difference"
      %w[reason evidence_path evidence_sha256].each { |field| abort "#{name} #{surface} disposition missing #{field}" if disposition[field].to_s.empty? }
      evidence = ROOT.join(disposition["evidence_path"]).cleanpath
      abort "#{name} #{surface} disposition evidence escapes repository" unless evidence.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
      abort "#{name} #{surface} disposition evidence missing" unless evidence.file? && !evidence.zero?
      abort "#{name} #{surface} disposition digest mismatch" unless Digest::SHA256.file(evidence).hexdigest == disposition["evidence_sha256"]
    end
  end

  abort "#{name} destination verification failed" unless row["destination_verified"] == true
  abort "#{name} lacks transfer receipt" if row["transfer_receipt"].to_s.empty? || row["destination_verified_at"].to_s.empty?
end

controls = report.fetch("negative_controls", {})
{"asksifu" => "danielbaustin/asksifu", "Horust" => "danielbaustin/Horust"}.each do |key, expected|
  row = controls[key] || abort("missing negative control #{key}")
  abort "#{key} identity mismatch" unless row["repository"] == expected
  abort "#{key} was mutated" unless row["transferred"] == false && row["settings_mutated"] == false
  abort "#{key} lacks repository id" if row["repository_id"].to_s.empty?
  abort "#{key} lacks exact head" unless row["exact_head"].to_s.match?(/\A[0-9a-f]{40}\z/)
  %w[before_snapshot_sha256 after_snapshot_sha256].each { |field| abort "#{key} lacks #{field}" unless row[field].to_s.match?(SHA256) }
  abort "#{key} negative-control snapshot drifted" unless row["before_snapshot_sha256"] == row["after_snapshot_sha256"]
  abort "#{key} lacks two-sided observation" if row["observed_before_at"].to_s.empty? || row["observed_after_at"].to_s.empty?
end

site = report.fetch("agent_logic_ai_cutover", {})
abort "website repository mismatch" unless site["repository"] == "agent-logic/agent-logic.ai"
abort "website ownership missing" if site["owner"].to_s.empty? || site["publication_receipt"].to_s.empty?
abort "website files mismatch" unless Array(site["paths"]).sort == %w[site/beta/index.html site/index.html]
abort "expected four ADL links" unless site["old_adl_link_count"] == 4 && site["remaining_old_adl_link_count"] == 0

serialized = JSON.generate(report)
abort "secret-like value retained" if serialized.match?(/(ghp_|github_pat_|AKIA[0-9A-Z]{16}|-----BEGIN [A-Z ]*PRIVATE KEY-----)/)

puts "WP-02 migration evidence valid: five serial transfers, #{SURFACES.length} digest-bound surfaces, two negative controls"
