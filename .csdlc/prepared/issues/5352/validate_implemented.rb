#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
issue_root = File.join(root, ".csdlc", "issues", "5352")
required = %w[sip stp spp vpp srp sor].map { |card| File.join(issue_root, "cards", "#{card}.md") }
required += [
  File.join(issue_root, "index.json"),
  File.join(root, "docs", "milestones", "v0.91.8", "handoff", "issue-5352-v092-consumption-handoff.md"),
  File.join(root, "docs", "milestones", "v0.91.8", "handoff", "WP21_SPRINT_REVIEW_5352.md"),
  File.join(__dir__, "design.md"),
  File.join(__dir__, "diagram.mmd"),
  File.join(__dir__, "validate_handoff.rb"),
  File.join(__dir__, "validate_dependency_ancestry.rb")
]
missing = required.reject { |path| File.file?(path) }
abort("missing implemented handoff files: #{missing.join(', ')}") unless missing.empty?

index = JSON.parse(File.read(File.join(issue_root, "index.json")))
abort("wrong issue") unless index["issue"] == 5352
abort("unexpected phase #{index['phase']}") unless %w[bound implemented reviewed published merge_ready].include?(index["phase"])
claim = index["claim"]
abort("missing active claim") unless claim.is_a?(Hash)
abort("wrong claim id") unless claim["id"] == "claim-5352-v0918-final-handoff"
abort("wrong claim branch") unless claim["branch"] == "codex/5352-v0918-final-handoff"
expected_paths = [
  ".csdlc/evidence/5352",
  ".csdlc/issues/5352",
  ".csdlc/locks/5352.lock",
  ".csdlc/prepared/issues/5352",
  "adl/src/csm_runtime_api.rs",
  "docs/milestones/v0.91.8/handoff"
]
abort("wrong protected paths") unless claim["protected_paths"] == expected_paths

card_text = %w[sip stp spp vpp srp sor].map { |card| File.read(File.join(issue_root, "cards", "#{card}.md")) }.join("\n")
%w[c34f0c9412495039a6374f7ce88fa39e34bb5042 #5558 #5749 #5352].each do |token|
  abort("missing current lifecycle truth: #{token}") unless card_text.include?(token)
end
%w[ab4e9e2217c152df47b1754b66b01febb4a59549 51bc5ae51b57c19dbab693af1c5a45142995f4e5].each do |token|
  abort("stale lifecycle truth remains: #{token}") if card_text.include?(token)
end

puts "PASS implemented_handoff_packet phase=#{index['phase']} generation=#{index['generation']}"
