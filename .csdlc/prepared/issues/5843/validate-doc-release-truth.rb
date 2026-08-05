#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "yaml"

EXACT_ROOT_FILES = %w[README.md CHANGELOG.md AGENTS.md REVIEW.md docs/README.md docs/planning/ADL_FEATURE_LIST.md csdlc-v2/AGENTS.md].freeze
OWNED_TREES = %w[docs/milestones/v0.92 csdlc-v2/operator/skills].freeze

def git(*argv)
  out, err, status = Open3.capture3("git", *argv)
  abort "git #{argv.join(' ')} failed: #{err}" unless status.success?
  out
end

expected = (EXACT_ROOT_FILES + OWNED_TREES.flat_map { |root| git("ls-files", "--", root).lines.map(&:strip) }).uniq.sort
abort "canonical docs denominator is empty" if expected.empty?
manifest = JSON.parse(File.read(ARGV.fetch(0, ".csdlc/evidence/5843/canonical-doc-inventory.json")))
rows = manifest["rows"]
abort "canonical inventory missing" unless rows.is_a?(Array) && !rows.empty?
abort "canonical document universe mismatch" unless rows.map { |row| row["path"] }.sort == expected
abort "duplicate canonical rows" unless rows.map { |row| row["path"] }.uniq.length == rows.length

rows.each do |row|
  %w[path owner wp_owner status version disposition evidence_path evidence_sha256].each do |field|
    abort "#{row['path']} #{field} missing" if row[field].to_s.strip.empty?
  end
  abort "wrong version" unless row["version"] == "v0.92"
  abort "document missing: #{row['path']}" unless File.file?(row["path"])
  abort "claim evidence missing" unless File.file?(row["evidence_path"])
  abort "claim evidence digest mismatch" unless Digest::SHA256.file(row["evidence_path"]).hexdigest == row["evidence_sha256"]
  case File.extname(row["path"])
  when ".json" then JSON.parse(File.read(row["path"]))
  when ".yaml", ".yml" then YAML.safe_load(File.read(row["path"]), aliases: true)
  when ".md"
    File.read(row["path"]).scan(/\[[^\]]+\]\(([^)]+)\)/).flatten.each do |target|
      next if target.start_with?("http://", "https://", "mailto:", "#")
      relative = target.split("#", 2).first
      next if relative.empty?
      abort "broken link #{target} in #{row['path']}" unless File.exist?(File.expand_path(relative, File.dirname(row["path"])))
    end
  end
  text = File.read(row["path"])
  abort "machine-local path leaked" if text.match?(%r{/(Users|home)/[^/\s]+/})
  abort "credential-like text leaked" if text.match?(/(?:ghp_|github_pat_|AKIA)[A-Za-z0-9_\-]{12,}/)
end

checks = manifest["command_checks"]
abort "command checks missing" unless checks.is_a?(Array) && !checks.empty?
checks.each do |check|
  argv = check["argv"]
  abort "command argv missing" unless argv.is_a?(Array) && !argv.empty?
  stdout, stderr, status = Open3.capture3(*argv)
  abort "command check failed: #{argv.join(' ')}\n#{stdout}\n#{stderr}" unless status.success?
  abort "command output digest missing" unless check["expected_output_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "command output drift" unless Digest::SHA256.hexdigest(stdout + stderr) == check["expected_output_sha256"]
end

puts "PASS: complete changelog, feature, ADR, release, skill, guidance, and milestone universe"
