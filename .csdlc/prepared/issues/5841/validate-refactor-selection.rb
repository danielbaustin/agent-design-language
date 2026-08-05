#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

OWNED_SOURCE = %w[
  adl-runtime-kernel/src/control.rs
  adl-runtime-kernel/tests/control.rs
  adl-runtime-kernel/src/observability.rs
  adl-runtime-kernel/tests/observability.rs
].freeze

def run!(*argv)
  out, err, status = Open3.capture3(*argv)
  abort "#{argv.join(' ')} failed: #{out}\n#{err}" unless status.success?
end

def read_json!(path, label)
  abort "missing #{label}: #{path}" unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  abort "invalid #{label}: #{error.message}"
end

manifest = read_json!(ARGV.fetch(0, ".csdlc/evidence/5841/refactor-selection.json"), "refactor selection")
head = `git rev-parse HEAD`.strip
abort "target SHA is not HEAD" unless manifest["target_sha"] == head
selections = manifest["selections"]
abort "selection missing" unless selections.is_a?(Array) && !selections.empty?
selected = selections.flat_map { |row| row.fetch("paths") }.uniq.sort
abort "selected source escapes exact ownership" unless (selected - OWNED_SOURCE).empty?
abort "selected source omits implementation and characterization paths" unless selected.any? { |path| path.include?("/src/") } && selected.any? { |path| path.include?("/tests/") }

selections.each do |row|
  %w[owner invariant rollback_note before_owner after_owner].each do |field|
    abort "selection #{field} missing" if row[field].to_s.strip.empty?
  end
  row.fetch("paths").each { |path| abort "selected path missing: #{path}" unless File.file?(path) }
end

run!("cargo", "fmt", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--", "--check")
run!("cargo", "clippy", "--locked", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--all-targets", "--", "-D", "warnings")
run!("cargo", "test", "--locked", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--test", "control")
run!("cargo", "test", "--locked", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--test", "observability")

metrics = read_json!(manifest.fetch("metrics_path"), "refactor metrics")
required = %w[before_loc after_loc before_duplication after_duplication ownership_before ownership_after]
abort "metrics incomplete" unless required.all? { |key| !metrics[key].nil? && metrics[key] != "" }
abort "refactor increased LoC without disposition" if metrics["after_loc"] > metrics["before_loc"] && metrics["loc_increase_justification"].to_s.empty?
abort "refactor increased duplication" if metrics["after_duplication"].to_f > metrics["before_duplication"].to_f

platforms = manifest["platform_evidence"]
abort "macOS and Linux evidence required" unless platforms.is_a?(Array) && platforms.map { |row| row["os"] }.sort == %w[linux macos]
platforms.each do |row|
  abort "platform target mismatch" unless row["head_sha"] == head && row["conclusion"] == "success"
  abort "platform artifact missing" unless File.file?(row["artifact_path"])
  digest = Digest::SHA256.file(row["artifact_path"]).hexdigest
  abort "platform proof digest mismatch" unless digest == row["artifact_sha256"] && digest == row["output_sha256"]
end

puts "PASS: exact-source refactor selection, characterization, metrics, and native proof"
