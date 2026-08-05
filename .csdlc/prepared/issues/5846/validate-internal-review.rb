#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

root = ARGV.fetch(0, "docs/reviews/v0.92/internal-review-5846")
manifest = JSON.parse(File.read(File.join(root, "packet-manifest.json")))
findings = JSON.parse(File.read(File.join(root, "findings.json")))
required_roster = %w[
  architecture code dependencies docs security tests lifecycle demos release_publication
].sort

abort "target SHA missing" unless manifest["target_sha"].is_a?(String) && manifest["target_sha"].match?(/\A[0-9a-f]{40}\z/)
paths = manifest["paths"]
abort "packet corpus missing" unless paths.is_a?(Array) && !paths.empty? && paths.all? { |path| File.file?(path) }
normalized = paths.sort.map { |path| "#{path}\0#{Digest::SHA256.file(path).hexdigest}" }.join("\n")
abort "packet digest mismatch" unless manifest["packet_sha256"] == Digest::SHA256.hexdigest(normalized)
reports = findings["specialists"]
abort "specialist roster mismatch" unless reports.is_a?(Array) && reports.map { |row| row["lane"] }.sort == required_roster
reports.each do |row|
  %w[reviewer_identity report_path report_sha256 target_sha].each do |field|
    abort "#{row['lane']} #{field} missing" unless row[field].is_a?(String) && !row[field].strip.empty?
  end
  abort "specialist target mismatch" unless row["target_sha"] == manifest["target_sha"]
  abort "specialist report missing" unless File.file?(row["report_path"])
  abort "specialist report digest mismatch" unless Digest::SHA256.file(row["report_path"]).hexdigest == row["report_sha256"]
  count = row["finding_count"]
  abort "invalid finding count" unless count.is_a?(Integer) && count >= 0
  if count.zero?
    abort "zero-finding rationale missing" unless row["zero_findings_rationale"].is_a?(String) && !row["zero_findings_rationale"].strip.empty?
    abort "zero-finding coverage missing" unless row["coverage_refs"].is_a?(Array) && !row["coverage_refs"].empty?
  end
end
all_findings = findings["findings"]
abort "findings array missing" unless all_findings.is_a?(Array)
abort "specialist counts do not reconcile" unless reports.sum { |row| row["finding_count"] } == all_findings.length
required = %w[id severity evidence invariant reproduction_or_proof_gap recommendation owner disposition source_lane]
allowed_severity = %w[P0 P1 P2 P3].freeze
allowed_disposition = %w[open disputed accepted_risk duplicate resolved].freeze
ids = all_findings.map { |row| row["id"] }
abort "duplicate finding IDs" unless ids.uniq.length == ids.length
all_findings.each do |row|
  abort "bad finding" unless required.all? { |key| row[key].is_a?(String) && !row[key].strip.empty? }
  abort "invalid severity" unless allowed_severity.include?(row["severity"])
  abort "invalid disposition" unless allowed_disposition.include?(row["disposition"])
  evidence_path = row["evidence"].split(":", 2).first
  abort "finding evidence path missing" unless File.exist?(evidence_path)
  abort "accepted risk lacks authority" if row["disposition"] == "accepted_risk" && row["authority"].to_s.strip.empty?
end

duplicates = findings.fetch("duplicates", [])
duplicates.each do |row|
  abort "duplicate references unknown finding" unless ids.include?(row["canonical_id"]) && row.fetch("duplicate_ids").all? { |id| ids.include?(id) }
end
findings.fetch("disagreements", []).each do |row|
  abort "disagreement finding missing" unless row.fetch("finding_ids").all? { |id| ids.include?(id) }
  abort "disagreement rationale missing" if row["rationale"].to_s.strip.empty?
end

puts "PASS: explicit specialist roster, report identities, and defensible zero-findings"
