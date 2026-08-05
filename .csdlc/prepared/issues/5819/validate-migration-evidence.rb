#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5819")
REPOS = %w[cognitive-sdlc-paper godel-hadamard-bayes-paper general-intelligence-paper-private universal-tool-schema agent-design-language].freeze

report_path = EVIDENCE.join("migration-report.json")
abort "missing migration report" unless report_path.file? && !report_path.zero?
report = JSON.parse(report_path.read)
rows = Array(report["repositories"])
abort "repository order or scope mismatch" unless rows.map { |row| row["name"] } == REPOS

rows.each do |row|
  name = row["name"]
  abort "wrong destination for #{name}" unless row["source"] == "danielbaustin/#{name}" && row["destination"] == "agent-logic/#{name}"
  %w[before_manifest_digest after_manifest_digest exact_head transfer_receipt destination_verified_at].each do |field|
    abort "#{name} missing #{field}" if row[field].to_s.empty?
  end
  abort "#{name} has unexplained drift" unless Array(row["unexplained_drift"]).empty?
  abort "#{name} destination verification failed" unless row["destination_verified"] == true
end

controls = report.fetch("negative_controls", {})
{
  "asksifu" => "danielbaustin/asksifu",
  "Horust" => "danielbaustin/Horust"
}.each do |key, expected|
  row = controls[key] || abort("missing negative control #{key}")
  abort "#{key} identity mismatch" unless row["repository"] == expected
  abort "#{key} was mutated" unless row["transferred"] == false && row["settings_mutated"] == false
  abort "#{key} lacks observation receipt" if row["observed_at"].to_s.empty? || row["head_or_repository_id"].to_s.empty?
end

site = report.fetch("agent_logic_ai_cutover", {})
abort "website repository mismatch" unless site["repository"] == "agent-logic/agent-logic.ai"
abort "website ownership missing" if site["owner"] .to_s.empty? || site["publication_receipt"].to_s.empty?
abort "website files mismatch" unless Array(site["paths"]).sort == %w[site/beta/index.html site/index.html]
abort "expected four ADL links" unless site["old_adl_link_count"] == 4 && site["remaining_old_adl_link_count"] == 0

serialized = JSON.generate(report)
abort "secret-like value retained" if serialized.match?(/(ghp_|github_pat_|AKIA[0-9A-Z]{16}|-----BEGIN [A-Z ]*PRIVATE KEY-----)/)

puts "WP-02 migration evidence valid: five destinations, two negative controls, production/beta cutover"
