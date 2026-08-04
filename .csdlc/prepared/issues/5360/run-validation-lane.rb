#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "date"
require "yaml"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
allowed = %w[focused-doc-alignment complete post-merge-exact].freeze
abort("unknown validation lane: #{lane}") unless allowed.include?(lane)

root = Pathname.new(__dir__).join("../../../..").expand_path

def run!(*command, chdir:)
  output, status = Open3.capture2e(*command, chdir: chdir.to_s)
  abort("#{command.join(' ')} failed: #{output}") unless status.success?
  output
end

run!("ruby", ".csdlc/prepared/issues/5360/check-dependencies.rb", chdir: root)
index = JSON.parse(root.join(".csdlc/issues/5360/index.json").read)
claim = index.fetch("claim")
protected_paths = claim.fetch("protected_paths")

committed = run!("git", "diff", "--name-only", "origin/main...HEAD", chdir: root).lines.map(&:strip)
status = run!("git", "status", "--porcelain", chdir: root).lines.map { |line| line[3..]&.strip }.compact
changed = (committed + status).reject(&:empty?).uniq.sort
unexpected = changed.reject do |path|
  protected_paths.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") }
end
abort("changed paths outside #5360 claim: #{unexpected.join(', ')}") unless unexpected.empty?

docs = changed.select do |path|
  path.end_with?(".md", ".yaml", ".json") &&
    (path.start_with?("docs/") || %w[README.md REVIEW.md CHANGELOG.md].include?(path))
end
host_paths = %r{/(?:Users|Volumes|private/tmp)/}
secret_markers = /(BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY|gh[pousr]_[A-Za-z0-9_]{20,})/
docs.each do |path|
  text = root.join(path).read
  abort("host-absolute path retained in #{path}") if text.match?(host_paths)
  abort("secret marker retained in #{path}") if text.match?(secret_markers)
end

changed.grep(/\.ya?ml\z/).each do |path|
  YAML.safe_load(root.join(path).read, permitted_classes: [Date], aliases: true)
end
changed.grep(/\.json\z/).each { |path| JSON.parse(root.join(path).read) }

diff_check = run!("git", "diff", "--check", chdir: root)
abort(diff_check) unless diff_check.empty?

if lane != "focused-doc-alignment"
  numstat = run!("git", "diff", "--numstat", "origin/main...HEAD", "--", *docs, chdir: root)
  changed_lines = numstat.lines.sum do |line|
    added, deleted, = line.split("\t", 3)
    added.to_i + deleted.to_i
  end
  abort("documentation delta exceeds 2500 changed lines: #{changed_lines}") if changed_lines > 2500
end

puts JSON.generate(
  status: "pass",
  issue: 5360,
  lane: lane,
  revision: run!("git", "rev-parse", "HEAD", chdir: root).strip,
  changed_paths: changed.length,
  documentation_paths: docs.length,
  dependency: "wp16-merge-and-quality-pass"
)
