#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
issue_dir = root.join(".csdlc", "issues", "5499")
prepared = root.join(".csdlc", "prepared", "issues", "5499")

required = %w[sip stp spp vpp srp sor].map { |card| issue_dir.join("cards", "#{card}.md") }
required += [prepared.join("design.md"), prepared.join("diagram.mmd"), prepared.join("check-dependencies.rb"), prepared.join("validate-conductor.sh")]
missing = required.reject(&:file?)
abort("missing preparation artifacts: #{missing.join(', ')}") unless missing.empty?

record = JSON.parse(issue_dir.join("index.json").read)
abort("wrong issue") unless record["issue"] == 5499
abort("preparation must remain bound") unless record["phase"] == "bound"
abort("preparation must not publish") unless record["publication"].nil?
abort("preparation must not close out") unless record["terminal"].nil?

claim = record.fetch("claim")
expected_paths = [".csdlc/issues/5499", ".csdlc/locks/5499.lock", ".csdlc/prepared/issues/5499"]
abort("unexpected protected paths") unless claim.fetch("protected_paths").sort == expected_paths.sort
abort("claim grants product authority") if claim.fetch("protected_paths").any? { |path| path.start_with?("adl-v2/") }

text = required.select { |path| %w[.md .mmd].include?(path.extname) }.map(&:read).join("\n")
%w[#5340 #5341 #5342 #5349 petgraph serde blake3 thiserror 3,000 600 audit-only].each do |needle|
  abort("missing preparation contract term #{needle}") unless text.include?(needle)
end

stdout, _stderr, status = Open3.capture3("ruby", prepared.join("check-dependencies.rb").to_s, chdir: root.to_s)
dependency = JSON.parse(stdout)
unless status.success? || (status.exitstatus == 3 && dependency["status"] == "waiting")
  abort("dependency gate did not return ready or truthful waiting")
end

changed = Open3.capture3("git", "diff", "--name-only", "origin/main...HEAD", chdir: root.to_s).first.lines.map(&:strip)
bad = changed.reject { |path| path.start_with?(".csdlc/issues/5499/", ".csdlc/prepared/issues/5499/") || path == ".csdlc/locks/5499.lock" }
abort("product or out-of-scope changes present: #{bad.join(', ')}") unless bad.empty?

puts JSON.pretty_generate(status: "pass", phase: record["phase"], cards: 6, dependency_status: dependency["status"], product_changes: 0)
