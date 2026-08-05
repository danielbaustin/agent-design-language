#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

manifest = JSON.parse(File.read(ARGV.fetch(0, "docs/reviews/v0.92/remediation-5848/regression-manifest.json")))
abort "target SHA missing" unless manifest["target_sha"].is_a?(String) && manifest["target_sha"].match?(/\A[0-9a-f]{40}\z/)
rows = manifest["affected_wp22_rows"]
abort "affected WP-22 rows missing" unless rows.is_a?(Array) && !rows.empty?
claims = manifest["release_claims"]
abort "release claim dispositions missing" unless claims.is_a?(Array)
(rows + claims).each do |entry|
  %w[id evidence_ref].each do |field|
    abort "#{field} missing" unless entry[field].is_a?(String) && !entry[field].strip.empty?
  end
  abort "evidence missing" unless File.file?(entry["evidence_ref"])
  argv = entry["validator_argv"]
  abort "validator argv missing" unless argv.is_a?(Array) && !argv.empty?
  stdout, stderr, status = Open3.capture3(*argv)
  abort "regression failed for #{entry['id']}: #{stdout}\n#{stderr}" unless status.success?
end
abort "release impact not dispositioned" unless manifest["release_impact"] == "none" || !claims.empty?

puts "PASS: all affected WP-22 rows and release claims revalidated"
