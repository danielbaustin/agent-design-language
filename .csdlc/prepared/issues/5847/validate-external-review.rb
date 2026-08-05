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
elsif mode != "packet"
  abort "usage: #{$PROGRAM_NAME} packet|report [review-root]"
end

puts "PASS: external review #{mode} identity"
