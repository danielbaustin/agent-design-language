#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.pwd.realpath
BASELINE_LOC = 355_675
MINIMUM_REDUCTION = 0.80
REFERENCE_ROOTS = %w[adl/Cargo.toml .github/workflows adl/tools docs README.md].freeze

def tracked_files(path)
  out, err, status = Open3.capture3("git", "ls-files", "--", path)
  abort "git ls-files failed: #{err}" unless status.success?
  out.lines.map(&:strip).reject(&:empty?)
end

manifest = JSON.parse(File.read(ARGV.fetch(0, ".csdlc/evidence/5786/deletion-manifest.json")))
denominator = JSON.parse(File.read(ARGV.fetch(1, ".csdlc/evidence/5786/repository-denominator.json")))
platform = JSON.parse(File.read(ARGV.fetch(2, ".csdlc/evidence/5786/platform-proof.json")))

expected_source = tracked_files("adl/src")
rows = manifest.fetch("rows")
abort "duplicate inventory paths" unless rows.map { |r| r["path"] }.uniq.length == rows.length
abort "source denominator mismatch" unless rows.map { |r| r["path"] }.sort == expected_source.sort
allowed = %w[replaced adapter retained delete]
rows.each do |row|
  %w[path disposition owner reason].each { |key| abort "#{key} missing" if row[key].to_s.strip.empty? }
  abort "invalid disposition" unless allowed.include?(row["disposition"])
  if row["disposition"] == "retained"
    abort "retained row missing expiry" unless row["expires_at"].to_s.match?(/\A\d{4}-\d{2}-\d{2}T/)
  end
end

reference_files = REFERENCE_ROOTS.flat_map { |path| tracked_files(path) }.uniq
expected_references = reference_files.flat_map do |path|
  next [] unless File.file?(path)
  File.readlines(path, chomp: true).each_with_index.filter_map do |line, index|
    { "path" => path, "line" => index + 1, "text_sha256" => Digest::SHA256.hexdigest(line) } if line.match?(/adl\/src|runtime_v2|target\/debug\/adl\b/)
  end
end
actual_references = denominator.fetch("references")
abort "reference denominator mismatch" unless actual_references == expected_references
abort "baseline LoC mismatch" unless denominator["baseline_loc"] == BASELINE_LOC
after_loc = denominator["after_loc"]
abort "after_loc invalid" unless after_loc.is_a?(Integer) && after_loc >= 0
reduction = (BASELINE_LOC - after_loc).fdiv(BASELINE_LOC)
abort "reported reduction mismatch" unless (denominator["reduction_fraction"].to_f - reduction).abs < 0.000001
abort "minimum 80% reduction not met" if reduction < MINIMUM_REDUCTION

receipts = platform.fetch("receipts")
abort "native macOS and Linux receipts required" unless receipts.map { |r| r["os"] }.sort == %w[linux macos]
head = `git rev-parse HEAD`.strip
receipts.each do |receipt|
  abort "platform head mismatch" unless receipt["head_sha"] == head
  abort "invalid native run URL" unless receipt["run_url"].to_s.match?(%r{\Ahttps://github\.com/.+/actions/runs/\d+\z})
  abort "runner identity missing" if receipt["runner_identity"].to_s.strip.empty?
  artifact = receipt["artifact_path"].to_s
  abort "platform artifact missing" unless File.file?(artifact)
  abort "platform digest mismatch" unless Digest::SHA256.file(artifact).hexdigest == receipt["artifact_sha256"]
  abort "platform proof failed" unless receipt["conclusion"] == "success"
end

stale = expected_references.reject { |ref| manifest.fetch("resolved_references").include?(ref) }
abort "stale references remain: #{stale.first(5).inspect}" unless stale.empty?

stdout, stderr, status = Open3.capture3("bash", "adl/tools/install_owner_binaries.sh")
abort "clean install failed: #{stdout}\n#{stderr}" unless status.success?
puts "PASS: exhaustive denominator, #{(reduction * 100).round(2)}% reduction, native proof, and stale-reference absence"
