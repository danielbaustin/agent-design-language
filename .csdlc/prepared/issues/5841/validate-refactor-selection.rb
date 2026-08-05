#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

manifest_path = ARGV.fetch(0, ".csdlc/evidence/5841/refactor-selection.json")
manifest = JSON.parse(File.read(manifest_path))
abort "target_sha missing" unless manifest["target_sha"].is_a?(String) && manifest["target_sha"].match?(/\A[0-9a-f]{40}\z/)

allowed = %w[adl-v2 adl-runtime-kernel csdlc-v2]
selections = manifest["selections"]
abort "selection missing" unless selections.is_a?(Array) && !selections.empty?
abort "unknown owner" unless selections.all? { |row| allowed.include?(row["owner"]) }

selections.each do |row|
  paths = row["paths"]
  abort "selected paths missing" unless paths.is_a?(Array) && !paths.empty? && paths.all? { |path| File.exist?(path) }
  %w[test_argv clippy_argv fmt_argv].each do |field|
    argv = row[field]
    abort "#{field} missing for #{row['owner']}" unless argv.is_a?(Array) && !argv.empty? && argv.all? { |part| part.is_a?(String) && !part.empty? }
    stdout, stderr, status = Open3.capture3(*argv)
    abort "#{field} failed for #{row['owner']}: #{stdout}\n#{stderr}" unless status.success?
  end
end

metrics_path = manifest["metrics_path"]
abort "metrics missing" unless metrics_path.is_a?(String) && File.file?(metrics_path)
metrics = JSON.parse(File.read(metrics_path))
required_metrics = %w[before_loc after_loc before_duplication after_duplication ownership_before ownership_after]
abort "metrics incomplete" unless required_metrics.all? { |key| !metrics[key].nil? && metrics[key] != "" }
abort "invalid LoC metrics" unless metrics.values_at("before_loc", "after_loc").all? { |value| value.is_a?(Integer) && value >= 0 }

platforms = manifest["platform_evidence"]
abort "macOS and Linux evidence required" unless platforms.is_a?(Array) && platforms.map { |row| row["os"] }.sort == %w[linux macos]
platforms.each do |row|
  %w[head_sha run_url artifact_path artifact_sha256].each do |field|
    abort "#{row['os']} #{field} missing" unless row[field].is_a?(String) && !row[field].empty?
  end
  abort "platform head mismatch" unless row["head_sha"] == manifest["target_sha"]
  abort "platform conclusion not successful" unless row["conclusion"] == "success"
  abort "platform artifact missing" unless File.file?(row["artifact_path"])
  abort "platform artifact digest mismatch" unless Digest::SHA256.file(row["artifact_path"]).hexdigest == row["artifact_sha256"]
end

puts "PASS: selected owners, owner-specific commands, metrics, and native macOS/Linux evidence"
