#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "yaml"

manifest = JSON.parse(File.read(ARGV.fetch(0, ".csdlc/evidence/5843/canonical-doc-inventory.json")))
rows = manifest["rows"]
abort "canonical inventory missing" unless rows.is_a?(Array) && !rows.empty?

rows.each do |row|
  %w[path owner wp_owner status version disposition].each do |field|
    abort "#{field} missing" unless row[field].is_a?(String) && !row[field].strip.empty?
  end
  abort "wrong version" unless row["version"] == "v0.92"
  abort "document missing: #{row['path']}" unless File.file?(row["path"])
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
end

puts "PASS: docs, links, YAML/JSON, commands, ownership, version, and redaction"
