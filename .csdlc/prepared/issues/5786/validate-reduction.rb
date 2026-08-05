#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

MINIMUM_REDUCTION = 0.80
ALLOWED_DISPOSITIONS = %w[replaced adapter retained delete].freeze
REFERENCE_ROOTS = %w[Cargo.toml adl/Cargo.toml .github/workflows adl/tools docs README.md].freeze

def git(*argv)
  out, err, status = Open3.capture3("git", *argv)
  abort "git #{argv.join(' ')} failed: #{err}" unless status.success?
  out
end

def read_json!(path, label)
  abort "missing #{label}: #{path}" unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  abort "invalid #{label}: #{error.message}"
end

def baseline_files(sha)
  git("ls-tree", "-r", "--name-only", sha, "--", "adl/src").lines.map(&:strip).reject(&:empty?).sort
end

def baseline_loc(sha, paths)
  paths.sum do |path|
    blob = git("show", "#{sha}:#{path}")
    blob.lines.length
  end
end

def current_references
  tracked = REFERENCE_ROOTS.flat_map do |root|
    git("ls-files", "--", root).lines.map(&:strip).reject(&:empty?)
  end.uniq.sort
  tracked.flat_map do |path|
    next [] unless File.file?(path)
    File.readlines(path, chomp: true).each_with_index.filter_map do |line, index|
      if line.match?(/adl\/src|runtime_v2|target\/debug\/adl\b/)
        { "path" => path, "line" => index + 1, "text_sha256" => Digest::SHA256.hexdigest(line) }
      end
    end
  end
end

manifest = read_json!(ARGV.fetch(0, ".csdlc/evidence/5786/deletion-manifest.json"), "deletion manifest")
baseline = read_json!(ARGV.fetch(1, ".csdlc/evidence/5786/pre-change-denominator.json"), "pre-change denominator")
platform = read_json!(ARGV.fetch(2, ".csdlc/evidence/5786/platform-proof.json"), "platform proof")

baseline_sha = baseline["baseline_head_sha"]
abort "baseline SHA missing" unless baseline_sha.to_s.match?(/\A[0-9a-f]{40}\z/)
abort "baseline is not ancestral to HEAD" unless system("git", "merge-base", "--is-ancestor", baseline_sha, "HEAD")
expected_files = baseline_files(baseline_sha)
abort "pre-change source denominator empty" if expected_files.empty?
abort "baseline file denominator mismatch" unless baseline["paths"] == expected_files
expected_loc = baseline_loc(baseline_sha, expected_files)
abort "baseline LoC mismatch" unless baseline["baseline_loc"] == expected_loc
abort "baseline file count mismatch" unless baseline["baseline_file_count"] == expected_files.length
expected_blobs = expected_files.to_h { |path| [path, git("rev-parse", "#{baseline_sha}:#{path}").strip] }
abort "baseline blob identities mismatch" unless baseline["blob_oids"] == expected_blobs

rows = manifest.fetch("rows")
abort "deletion inventory is empty" unless rows.is_a?(Array) && !rows.empty?
abort "duplicate inventory paths" unless rows.map { |row| row["path"] }.uniq.length == rows.length
abort "inventory does not cover pre-change denominator" unless rows.map { |row| row["path"] }.sort == expected_files
rows.each do |row|
  %w[path disposition owner reason].each { |key| abort "#{key} missing" if row[key].to_s.strip.empty? }
  abort "invalid disposition" unless ALLOWED_DISPOSITIONS.include?(row["disposition"])
  abort "retained row missing expiry" if row["disposition"] == "retained" && !row["expires_at"].to_s.match?(/\A\d{4}-\d{2}-\d{2}T/)
end

current_files = git("ls-files", "--", "adl/src").lines.map(&:strip).reject(&:empty?).sort
after_loc = current_files.sum { |path| File.readlines(path).length }
reduction = (expected_loc - after_loc).fdiv(expected_loc)
abort "reported after_loc mismatch" unless manifest["after_loc"] == after_loc
abort "reported reduction mismatch" unless (manifest["reduction_fraction"].to_f - reduction).abs < 0.000001
abort "minimum 80% reduction not met" if reduction < MINIMUM_REDUCTION

references = current_references
abort "reference denominator mismatch" unless manifest["reference_denominator"] == references
abort "unresolved stale references" unless manifest["resolved_references"] == references

head = git("rev-parse", "HEAD").strip
receipts = platform.fetch("receipts")
abort "native macOS and Linux receipts required" unless receipts.is_a?(Array) && receipts.map { |row| row["os"] }.sort == %w[linux macos]
receipts.each do |receipt|
  %w[head_sha run_url runner_identity artifact_path artifact_sha256 output_sha256 conclusion].each do |field|
    abort "#{receipt['os']} #{field} missing" if receipt[field].to_s.strip.empty?
  end
  abort "platform head mismatch" unless receipt["head_sha"] == head
  abort "platform proof failed" unless receipt["conclusion"] == "success"
  abort "platform artifact missing" unless File.file?(receipt["artifact_path"])
  digest = Digest::SHA256.file(receipt["artifact_path"]).hexdigest
  abort "platform artifact digest mismatch" unless digest == receipt["artifact_sha256"] && digest == receipt["output_sha256"]
end

stdout, stderr, status = Open3.capture3("bash", "adl/tools/install_owner_binaries.sh")
abort "clean install failed: #{stdout}\n#{stderr}" unless status.success?
puts "PASS: pinned pre-change denominator, #{expected_files.length} files, #{(reduction * 100).round(2)}% reduction"
