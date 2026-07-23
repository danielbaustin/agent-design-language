#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = 5590
base = "6d0f6115632a06619544b8ad4792792e741f1f31"
reviewed_head = "2f26da4455efd4dfc7ab6c65df5d19327fe765c8"
substantive_repair_head = "f2eda77cf6a36d9a07d7d64041d8eae99b18239b"
kinds = %w[sip stp spp vpp srp sor]
cards = kinds.to_h do |kind|
  path = ".csdlc/issues/#{issue}/cards/#{kind}.values.json"
  [kind, JSON.parse(File.read(path)).fetch("content").fetch("values")]
end

abort "expected six cards" unless cards.length == 6
generations = kinds.map do |kind|
  JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{kind}.values.json")).dig("identity", "generation")
end
abort "card generation mismatch" unless generations.uniq.length == 1

expected = (1..8).map { |number| "AC-#{number}" }
acceptance = cards.fetch("stp").fetch("acceptance_criteria").map { |value| value[/AC-\d+/] }.uniq.sort
step_coverage = cards.fetch("spp").fetch("steps").flat_map { |step| step.fetch("acceptance_ids") }.uniq.sort
lane_coverage = cards.fetch("vpp").fetch("lanes").flat_map { |lane| lane.fetch("acceptance_ids") }.uniq.sort
abort "acceptance set incomplete" unless acceptance == expected
abort "SPP coverage incomplete" unless step_coverage == expected
abort "VPP coverage incomplete" unless lane_coverage == expected
abort "deferred validation lane" if cards.fetch("vpp").fetch("lanes").any? { |lane| lane["defer_reason"] }

filtered = cards.fetch("vpp").fetch("lanes").select do |lane|
  lane.fetch("argv").any? { |argument| argument.end_with?("run_filtered_test_lane.sh") }
end
abort "expected four guarded filtered lanes" unless filtered.length == 4
abort "filtered lane omits inventory guard" unless filtered.all? { |lane| lane.fetch("proof_role").include?("positive") && lane.fetch("proof_role").include?("zero matches") }

claim = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json")).fetch("claim")
expected_paths = [
  ".csdlc/evidence/5590",
  ".csdlc/issues/5590",
  ".csdlc/locks/5590.lock",
  ".csdlc/prepared/issues/5590"
]
abort "claim is not preparation-only" unless claim.fetch("protected_paths").sort == expected_paths.sort
abort "claim purpose omits implementation gate" unless claim.fetch("purpose").include?("without product edits")

abort "reviewed head does not descend from exact base" unless system("git", "merge-base", "--is-ancestor", base, reviewed_head)
abort "substantive repair head does not descend from reviewed head" unless system("git", "merge-base", "--is-ancestor", reviewed_head, substantive_repair_head)
abort "current head does not descend from substantive repair head" unless system("git", "merge-base", "--is-ancestor", substantive_repair_head, "HEAD")
binding = JSON.parse(File.read(".csdlc/prepared/issues/5590/revision-binding.json"))
abort "revision binding base drift" unless binding.fetch("exact_base") == base
abort "revision binding reviewed-head drift" unless binding.fetch("first_reviewed_head") == reviewed_head
abort "revision binding repair-head drift" unless binding.fetch("substantive_repair_head") == substantive_repair_head
range_proof = binding.fetch("range_proof")
abort "revision range drift" unless range_proof.fetch("range") == "#{base}..#{substantive_repair_head}"
abort "revision inventory drift" unless range_proof.fetch("changed_path_inventory") == ".csdlc/prepared/issues/5590/revision-scope-inventory.txt"
abort "revision proof is not fail closed" unless range_proof.fetch("requires_non_empty_range") && range_proof.fetch("requires_range_bound_diff_check")
abort "substantive/evidence boundary missing" unless binding.dig("boundaries", "substantive")&.include?("f2eda77") && binding.dig("boundaries", "evidence_only")&.include?("re-review target")
constraints = cards.fetch("sip").fetch("operator_constraints").join("\n")
dependencies = cards.fetch("stp").fetch("dependencies").join("\n")
abort "exact base missing" unless constraints.include?(base) && dependencies.include?(base)
abort "reviewed head missing" unless constraints.include?(reviewed_head) && dependencies.include?(reviewed_head)

design = File.read(".csdlc/prepared/issues/5590/design.md")
diagram = File.read(".csdlc/prepared/issues/5590/diagram.mmd")
matrix = File.read(".csdlc/prepared/issues/5590/security-acceptance-matrix.md")
%w[HTTPS WebSocket Observatory guardian Vector rollback 20997 Runtime\ v2].each do |term|
  normalized = term.tr("\\", "")
  abort "design missing #{normalized}" unless design.include?(normalized)
end
abort "diagram incomplete" unless %w[guardian https websocket observatory vector rollback].all? { |term| diagram.downcase.include?(term) }
abort "matrix incomplete" unless expected.all? { |id| matrix.include?(id) }
abort "rollback remains report-only" unless design.include?("report-only") && design.include?("authenticated HTTPS service health")
selector_lane = cards.fetch("vpp").fetch("lanes").find { |lane| lane.fetch("lane") == "runtime-v3-operational-selector-rollback" }
abort "operational selector lane missing" unless selector_lane
abort "selector lane is report-only" unless selector_lane.fetch("argv").any? { |argument| argument.end_with?("run_operational_selector_transition.sh") }
abort "guardian soak not independently executable" unless cards.fetch("vpp").fetch("lanes").any? { |lane| lane.fetch("lane") == "runtime-v3-guardian-soak" && lane.fetch("argv").include?("adl/tools/run_runtime_v3_guardian_soak.sh") }

title = "[v0.91.8][WP-14][runtime-v3][parity-d] Prove secure access, Observatory, guardian, and rollback"
abort "card title drift" unless kinds.all? do |kind|
  JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{kind}.values.json")).dig("identity", "title") == title
end
abort "S1 not complete" unless cards.fetch("spp").fetch("steps").find { |step| step.fetch("id") == "S1" }.fetch("status") == "completed"

forbidden = %w[adl-runtime adl-runtime-kernel infra/runtime-v3 demos/v0.91.7/html-observatory]
abort "product path protected prematurely" if claim.fetch("protected_paths").any? { |path| forbidden.include?(path) }
abort "guard contract test missing" unless File.file?(".csdlc/prepared/issues/5590/test_preparation_guards.sh")
abort "revision scope proof missing" unless File.file?(".csdlc/prepared/issues/5590/run_revision_scope_proof.sh")
abort "revision inventory missing" unless File.file?(".csdlc/prepared/issues/5590/revision-scope-inventory.txt")

puts "cards=6 generation=#{generations.first} acceptance=8 steps=complete lanes=complete deferrals=0 claim=preparation-only base=#{base} reviewed_head=#{reviewed_head} substantive_repair_head=#{substantive_repair_head}"
