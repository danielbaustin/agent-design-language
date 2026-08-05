#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

manifest_path = ARGV.fetch(0, ".csdlc/evidence/5841/refactor-selection.json")
manifest = JSON.parse(File.read(manifest_path))
abort "target_sha missing" unless manifest["target_sha"].is_a?(String) && manifest["target_sha"].match?(/\A[0-9a-f]{40}\z/)
head = `git rev-parse HEAD`.strip
abort "target SHA is not HEAD" unless manifest["target_sha"] == head

allowed = %w[adl-v2 adl-runtime-kernel csdlc-v2]
owner_commands = {
  "adl-v2" => {
    "test" => %w[cargo test --locked --manifest-path adl-v2/Cargo.toml],
    "clippy" => %w[cargo clippy --locked --manifest-path adl-v2/Cargo.toml --all-targets -- -D warnings],
    "fmt" => %w[cargo fmt --manifest-path adl-v2/Cargo.toml -- --check]
  },
  "adl-runtime-kernel" => {
    "test" => %w[cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml],
    "clippy" => %w[cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings],
    "fmt" => %w[cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check]
  },
  "csdlc-v2" => {
    "test" => %w[cargo test --locked --manifest-path csdlc-v2/Cargo.toml],
    "clippy" => %w[cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings],
    "fmt" => %w[cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check]
  }
}.freeze
selections = manifest["selections"]
abort "selection missing" unless selections.is_a?(Array) && !selections.empty?
abort "unknown owner" unless selections.all? { |row| allowed.include?(row["owner"]) }

selections.each do |row|
  paths = row["paths"]
  abort "selected paths missing" unless paths.is_a?(Array) && !paths.empty? && paths.all? { |path| File.exist?(path) }
  owner_prefix = row["owner"] + "/"
  abort "selected path escapes #{row['owner']}" unless paths.all? { |path| path == row["owner"] || path.start_with?(owner_prefix) }
  owner_commands.fetch(row["owner"]).each do |kind, argv|
    stdout, stderr, status = Open3.capture3(*argv)
    abort "#{kind} failed for #{row['owner']}: #{stdout}\n#{stderr}" unless status.success?
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
  abort "invalid native run URL" unless row["run_url"].match?(%r{\Ahttps://github\.com/.+/actions/runs/\d+\z})
  abort "runner identity missing" unless row["runner_identity"].is_a?(String) && !row["runner_identity"].strip.empty?
  abort "output digest missing" unless row["output_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "platform conclusion not successful" unless row["conclusion"] == "success"
  abort "platform artifact missing" unless File.file?(row["artifact_path"])
  abort "platform artifact digest mismatch" unless Digest::SHA256.file(row["artifact_path"]).hexdigest == row["artifact_sha256"]
  abort "output digest is not artifact-bound" unless row["output_sha256"] == row["artifact_sha256"]
end

puts "PASS: HEAD-bound selection, derived owner commands, metrics, and native macOS/Linux evidence"
