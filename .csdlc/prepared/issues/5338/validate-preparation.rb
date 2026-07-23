#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue_root = File.join(root, ".csdlc/issues/5338")
prepared = File.join(root, ".csdlc/prepared/issues/5338")
required = %w[sip stp spp vpp srp sor].flat_map do |card|
  [File.join(issue_root, "cards/#{card}.md"), File.join(issue_root, "cards/#{card}.values.json")]
end
required += [File.join(issue_root, "index.json"), File.join(prepared, "design.md"), File.join(prepared, "diagram.mmd"), File.join(prepared, "validate-compiler.sh")]
missing = required.reject { |path| File.file?(path) }
abort("missing canonical preparation artifacts: #{missing.join(', ')}") unless missing.empty?

record = JSON.parse(File.read(File.join(issue_root, "index.json")))
claim = record.fetch("claim")
expected_paths = [
  ".csdlc/issues/5338",
  ".csdlc/locks/5338.lock",
  ".csdlc/prepared/issues/5338",
  ".csdlc/evidence/5338",
  "adl-v2/crates/adl-compiler"
]
abort("protected paths drift") unless claim.fetch("protected_paths").sort == expected_paths.sort

branch = `git branch --show-current`.strip
abort("preparation is not running on the dedicated #5338 branch") unless branch == "codex/5338-v0918-wp05-deterministic-compiler"
common = `git rev-parse --path-format=absolute --git-common-dir`.strip
primary = File.dirname(common)
abort("cannot resolve primary checkout") unless File.directory?(primary)
root_branch = `git -C #{primary.shellescape} branch --show-current`.strip
root_status = `git -C #{primary.shellescape} status --short`.strip
abort("primary checkout is not clean main") unless root_branch == "main" && root_status.empty?
abort("#5338 canonical state leaked into primary checkout") if File.exist?(File.join(primary, ".csdlc/issues/5338"))

text = required.grep(/\.(?:md|mmd)$/).map { |path| File.read(path) }.join("\n")
checks = {
  "dependency gate" => text.include?("#5339") && text.include?("closed_out"),
  "implementation budget" => text.include?("3,500") || text.include?("3500"),
  "test budget" => text.scan(/3,500|3500/).length >= 2,
  "FastWork build boundary" => text.include?("/Volumes/FastWork"),
  "no Runtime v2 authority" => text.include?("Runtime v2"),
  "stable identity" => text.include?("stable node identity") || text.include?("Stable node identity"),
  "COTS decision" => text.include?("COTS")
}
failed = checks.reject { |_name, passed| passed }.keys
abort("preparation contract checks failed: #{failed.join(', ')}") unless failed.empty?

puts JSON.generate(schema: "adl.csdlc.preparation-proof.v1", issue: 5338, checks: checks.keys, outcome: "passed")
