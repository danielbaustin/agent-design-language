#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5819")
REPOS = %w[cognitive-sdlc-paper godel-hadamard-bayes-paper general-intelligence-paper-private universal-tool-schema agent-design-language].freeze
SURFACES = %w[issues pull_requests assignees rulesets releases actions pages packages lfs integrations].freeze

def canonical(value)
  case value
  when Hash
    value.keys.sort.to_h { |key| [key, canonical(value.fetch(key))] }
  when Array
    value.map { |item| canonical(item) }
  else
    value
  end
end

def sha256_json(value)
  Digest::SHA256.hexdigest(JSON.generate(canonical(value)))
end

def load_manifest(root, relative, expected_digest, label)
  path = root.join(relative.to_s).cleanpath
  abort "#{label} manifest escapes repository" unless path.to_s.start_with?(root.to_s + File::SEPARATOR)
  abort "missing #{label} manifest: #{relative}" unless path.file? && !path.zero?
  abort "invalid #{label} manifest digest" unless expected_digest.to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "#{label} manifest digest mismatch" unless Digest::SHA256.file(path).hexdigest == expected_digest
  JSON.parse(path.read)
end

report_path = EVIDENCE.join("migration-report.json")
abort "missing migration report" unless report_path.file? && !report_path.zero?
report = JSON.parse(report_path.read)
rows = Array(report["repositories"])
abort "repository order or scope mismatch" unless rows.map { |row| row["name"] } == REPOS

rows.each do |row|
  name = row.fetch("name")
  abort "wrong destination for #{name}" unless row["source"] == "danielbaustin/#{name}" && row["destination"] == "agent-logic/#{name}"
  before = load_manifest(ROOT, row["before_manifest_path"], row["before_manifest_sha256"], "#{name} before")
  after = load_manifest(ROOT, row["after_manifest_path"], row["after_manifest_sha256"], "#{name} after")
  [before, after].each do |manifest|
    abort "#{name} manifest repository mismatch" unless manifest["repository"].to_s.end_with?("/#{name}")
    abort "#{name} manifest lacks exact head" unless manifest["exact_head"].to_s.match?(/\A[0-9a-f]{40}\z/)
    missing = SURFACES - manifest.fetch("surfaces", {}).keys
    abort "#{name} manifest omits surfaces: #{missing.join(', ')}" unless missing.empty?
  end
  abort "#{name} history head changed" unless before["exact_head"] == after["exact_head"] && after["exact_head"] == row["exact_head"]

  dispositions = row.fetch("surface_dispositions", {})
  SURFACES.each do |surface|
    before_digest = sha256_json(before.fetch("surfaces").fetch(surface))
    after_digest = sha256_json(after.fetch("surfaces").fetch(surface))
    disposition = dispositions.fetch(surface, {})
    if before_digest == after_digest
      abort "#{name} #{surface} must be recorded preserved" unless disposition["status"] == "preserved"
    else
      abort "#{name} #{surface} drift lacks verified disposition" unless disposition["status"] == "verified_difference"
      %w[reason evidence_path evidence_sha256].each do |field|
        abort "#{name} #{surface} disposition missing #{field}" if disposition[field].to_s.empty?
      end
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
  abort "#{key} lacks observation receipt" if row["observed_at"].to_s.empty? || row["head_or_repository_id"].to_s.empty?
end

site = report.fetch("agent_logic_ai_cutover", {})
abort "website repository mismatch" unless site["repository"] == "agent-logic/agent-logic.ai"
abort "website ownership missing" if site["owner"].to_s.empty? || site["publication_receipt"].to_s.empty?
abort "website files mismatch" unless Array(site["paths"]).sort == %w[site/beta/index.html site/index.html]
abort "expected four ADL links" unless site["old_adl_link_count"] == 4 && site["remaining_old_adl_link_count"] == 0

serialized = JSON.generate(report)
abort "secret-like value retained" if serialized.match?(/(ghp_|github_pat_|AKIA[0-9A-Z]{16}|-----BEGIN [A-Z ]*PRIVATE KEY-----)/)

puts "WP-02 migration evidence valid: five digest-bound before/after manifests, ten compared surfaces, two negative controls"
