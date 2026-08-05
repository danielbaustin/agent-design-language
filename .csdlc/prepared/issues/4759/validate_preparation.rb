#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = "4759"
root = File.expand_path("../../../..", __dir__)
base = File.join(root, ".csdlc", "issues", issue)
prepared = File.join(root, ".csdlc", "prepared", "issues", issue)

required = [
  File.join(base, "index.json"),
  File.join(base, "cards", "sip.md"),
  File.join(base, "cards", "sip.values.json"),
  File.join(base, "cards", "stp.md"),
  File.join(base, "cards", "stp.values.json"),
  File.join(base, "cards", "spp.md"),
  File.join(base, "cards", "spp.values.json"),
  File.join(base, "cards", "vpp.md"),
  File.join(base, "cards", "vpp.values.json"),
  File.join(base, "cards", "srp.md"),
  File.join(base, "cards", "srp.values.json"),
  File.join(base, "cards", "sor.md"),
  File.join(base, "cards", "sor.values.json"),
  File.join(prepared, "design.md"),
  File.join(prepared, "diagram.mmd"),
  File.join(prepared, "preparation-contract.json"),
  File.join(prepared, "preparation-review.md")
]

missing = required.reject { |path| File.file?(path) }
abort("missing preparation files: #{missing.join(", ")}") unless missing.empty?

index = JSON.parse(File.read(File.join(base, "index.json")))
abort("wrong issue") unless index["issue"] == issue.to_i
abort("unexpected phase") unless %w[initialized ready bound].include?(index["phase"])
abort("implementation evidence present") unless index["publication"].nil? && index["terminal"].nil?

claim = index["claim"]
if claim
  issue_prefixes = [
    ".csdlc/issues/#{issue}",
    ".csdlc/locks/#{issue}.lock",
    ".csdlc/prepared/issues/#{issue}",
    ".csdlc/evidence/#{issue}"
  ]
  protected_paths = claim.fetch("protected_paths", [])
  abort("claim has non-issue-local paths") unless protected_paths.all? { |path| issue_prefixes.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") } }
end

contract = JSON.parse(File.read(File.join(prepared, "preparation-contract.json")))
abort("wrong preparation contract issue") unless contract["issue"] == issue.to_i
abort("claim acquisition must be deferred") unless contract["execution_claim"] == "deferred_to_execution"
abort("preparation must have one concern") unless contract.fetch("concerns", []).length == 1
abort("wrong preparation concern") unless contract["concerns"].first == "live #5384 merge plus current origin/main ancestry before execution"
abort("preparation review fixes incomplete") unless contract.dig("review", "status") == "findings_resolved"

text = required.grep(/\.md$/).map { |path| File.read(path) }.join("\n")
%w[#5384 #5335 activation origin/main ancestry non-blocking].each do |needle|
  abort("missing required gate text: #{needle}") unless text.include?(needle)
end

puts "issue #{issue} preparation packet OK"
