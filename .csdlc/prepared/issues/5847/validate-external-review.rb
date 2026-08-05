#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

mode = ARGV.fetch(0)
root = ARGV.fetch(1, "docs/reviews/v0.92/external-review-5847")
manifest = JSON.parse(File.read(File.join(root, "packet-manifest.json")))
paths = manifest["paths"]
abort "packet paths missing" unless paths.is_a?(Array) && !paths.empty? && paths.all? { |path| File.file?(path) }
normalized = paths.sort.map { |path| "#{path}\0#{Digest::SHA256.file(path).hexdigest}" }.join("\n")
packet_digest = Digest::SHA256.hexdigest(normalized)
abort "packet digest mismatch" unless packet_digest == manifest["packet_sha256"]
abort "packet target missing" unless manifest["target_sha"].is_a?(String) && manifest["target_sha"].match?(/\A[0-9a-f]{40}\z/)

if mode == "report"
  index = JSON.parse(File.read(File.join(root, "findings-index.json")))
  %w[reviewer_identity report_path report_sha256 packet_sha256 target_sha].each do |field|
    abort "#{field} missing" unless index[field].is_a?(String) && !index[field].strip.empty?
  end
  abort "report not received" unless index["report_received"] == true
  abort "report packet mismatch" unless index["packet_sha256"] == packet_digest
  abort "report target mismatch" unless index["target_sha"] == manifest["target_sha"]
  abort "reviewer report missing" unless File.file?(index["report_path"])
  abort "reviewer report digest mismatch" unless Digest::SHA256.file(index["report_path"]).hexdigest == index["report_sha256"]
  abort "findings array missing" unless index["findings"].is_a?(Array)
  abort "unindexed finding" unless index["source_count"].is_a?(Integer) && index["source_count"] == index["findings"].length
  abort "external reviewer authority missing" unless index["reviewer_authority"].is_a?(String) && !index["reviewer_authority"].strip.empty?
  required = %w[id severity evidence invariant reproduction_or_proof_gap recommendation owner disposition]
  allowed_severity = %w[P0 P1 P2 P3]
  allowed_disposition = %w[open disputed accepted_risk duplicate resolved]
  ids = index["findings"].map { |finding| finding["id"] }
  abort "duplicate finding IDs" unless ids.uniq.length == ids.length
  index["findings"].each do |finding|
    abort "finding schema incomplete" unless required.all? { |field| finding[field].is_a?(String) && !finding[field].strip.empty? }
    abort "invalid severity" unless allowed_severity.include?(finding["severity"])
    abort "invalid disposition" unless allowed_disposition.include?(finding["disposition"])
    evidence_path = finding["evidence"].split(":", 2).first
    abort "finding evidence path missing" unless File.exist?(evidence_path)
    if finding["disposition"] == "accepted_risk"
      abort "accepted risk lacks operator authority" unless finding["authority"].to_s.start_with?("operator:")
    end
    if finding["disposition"] == "duplicate"
      abort "duplicate target missing" unless ids.include?(finding["duplicate_of"])
    end
  end
elsif mode != "packet"
  abort "usage: #{$PROGRAM_NAME} packet|report [review-root]"
end

puts "PASS: external review #{mode} identity"
