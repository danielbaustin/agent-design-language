#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

manifest = JSON.parse(File.read(ARGV.fetch(0, ".csdlc/evidence/5852/release-evidence-manifest.json")))
rows = manifest["rows"]
abort "release evidence rows missing" unless rows.is_a?(Array) && !rows.empty?
required = %w[claim implementation_ref validation_ref review_ref merge_sha terminal_ref artifact_path artifact_sha256 residual_risk_ref non_claim_ref]
rows.each do |row|
  required.each do |field|
    abort "#{field} missing" unless row[field].is_a?(String) && !row[field].strip.empty?
  end
  abort "invalid merge SHA" unless row["merge_sha"].match?(/\A[0-9a-f]{40}\z/)
  %w[implementation_ref validation_ref review_ref terminal_ref residual_risk_ref non_claim_ref artifact_path].each do |field|
    abort "#{field} file missing" unless File.file?(row[field])
  end
  abort "artifact digest mismatch" unless Digest::SHA256.file(row["artifact_path"]).hexdigest == row["artifact_sha256"]
end
%w[release_notes_ref checklist_ref handoff_ref residual_risk_summary_ref non_claim_summary_ref].each do |field|
  value = manifest[field]
  abort "#{field} missing" unless value.is_a?(String) && !value.strip.empty? && File.file?(value)
end

puts "PASS: release claims, artifact hashes, residual risks, and non-claims"
