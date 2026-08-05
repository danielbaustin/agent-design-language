#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

VALIDATORS = {
  "quality_matrix" => %w[ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb matrix],
  "quality_negative" => %w[ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb negative],
  "docs_release_truth" => %w[ruby .csdlc/prepared/issues/5843/validate-doc-release-truth.rb],
  "internal_review" => %w[ruby .csdlc/prepared/issues/5846/validate-internal-review.rb],
  "external_review" => %w[ruby .csdlc/prepared/issues/5847/validate-external-review.rb],
  "release_evidence" => %w[ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb]
}.freeze

manifest = JSON.parse(File.read(ARGV.fetch(0, "docs/reviews/v0.92/remediation-5848/regression-manifest.json")))
head = `git rev-parse HEAD`.strip
abort "target SHA is not HEAD" unless manifest["target_sha"] == head
rows = manifest["affected_wp22_rows"]
claims = manifest["release_claims"]
abort "affected WP-22 rows missing" unless rows.is_a?(Array) && !rows.empty?
abort "release claim dispositions missing" unless claims.is_a?(Array)

rows.each do |entry|
  abort "WP-22 row validator not derived" unless %w[quality_matrix quality_negative].include?(entry["validator_id"])
end
claims.each do |entry|
  abort "release-claim validator not derived" unless %w[docs_release_truth release_evidence].include?(entry["validator_id"])
end
(rows + claims).each do |entry|
  %w[id evidence_ref evidence_sha256 validator_id target_sha].each do |field|
    abort "#{field} missing" unless entry[field].is_a?(String) && !entry[field].strip.empty?
  end
  abort "entry target mismatch" unless entry["target_sha"] == head
  abort "evidence missing" unless File.file?(entry["evidence_ref"])
  abort "evidence digest mismatch" unless Digest::SHA256.file(entry["evidence_ref"]).hexdigest == entry["evidence_sha256"]
  argv = VALIDATORS[entry["validator_id"]]
  abort "validator not allowlisted: #{entry['validator_id']}" unless argv
  stdout, stderr, status = Open3.capture3(*argv)
  abort "regression failed for #{entry['id']}: #{stdout}\n#{stderr}" unless status.success?
end
abort "release impact not dispositioned" unless manifest["release_impact"] == "none" || !claims.empty?

puts "PASS: allowlisted affected-row and release-claim validators passed at HEAD"
