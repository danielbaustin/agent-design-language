#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "yaml"
require "digest"

ISSUE = 5350
KINDS = %w[sip stp spp vpp srp sor].freeze
ROOT = File.expand_path("../../../..", __dir__)
Dir.chdir(ROOT)

def read_json(path)
  JSON.parse(File.read(path))
rescue StandardError => e
  abort "cannot read #{path}: #{e.message}"
end

index = read_json(".csdlc/issues/#{ISSUE}/index.json")
abort "wrong issue" unless index.fetch("issue") == ISSUE
abort "not bound" unless index.fetch("phase") == "bound"

cards = {}
KINDS.each do |kind|
  values = ".csdlc/issues/#{ISSUE}/cards/#{kind}.values.json"
  rendered = ".csdlc/issues/#{ISSUE}/cards/#{kind}.md"
  abort "missing #{values}" unless File.file?(values)
  abort "missing #{rendered}" unless File.file?(rendered)
  cards[kind] = read_json(values).fetch("content").fetch("values")
end

claim = index.fetch("claim")
expected_paths = [
  ".csdlc/issues/5350",
  ".csdlc/locks/5350.lock",
  ".csdlc/prepared/issues/5350"
]
abort "claim is not preparation-only" unless claim.fetch("protected_paths") == expected_paths
abort "claim purpose lacks execution stop" unless claim.fetch("purpose").include?("execution remains prohibited")

inventory = read_json(".csdlc/prepared/issues/5350/source-inventory.json")
abort "inventory is not preparation-only" unless inventory.fetch("status") == "preparation_only"
v1 = inventory.fetch("adl_v1")
abort "v1 revision drift" unless v1.fetch("source_revision") == "19c2b6e2ad18bddc75db9231643a54b2a446ce72"
abort "v1 binary drift" unless v1.fetch("binary_sha256") == "f558fa2111474e2fab540f8d0244be82cdb727ebbaa15aee758d8a7d57d0969c"
abort "corpus counts drift" unless v1.values_at("cases", "observations", "behaviors", "equivalence_groups", "difference_groups") == [25, 75, 23, 2, 1]
abort "v2 is falsely pinned during preparation" unless inventory.dig("adl_v2", "revision").nil? && inventory.dig("adl_v2", "binary_sha256").nil?
required_v2_fields = %w[revision binary_sha256 selector_identity selector_generation cargo_lock_sha256 command_contract_sha256 corpus_bundle_sha256 current_main_revision current_main_ancestry_verified dependency_receipts]
abort "v2 identity schema incomplete" unless required_v2_fields.all? { |field| inventory.fetch("adl_v2").key?(field) }
abort "v2 preparation identity is not fail-closed" unless inventory.dig("adl_v2", "current_main_ancestry_verified") == false && inventory.dig("adl_v2", "dependency_receipts") == []
abort "runtime proof count drift" unless inventory.dig("runtime_v3", "proof_group_count") == 10
abort "#5361 direction drift" unless inventory.dig("runtime_v3", "acceptance_is_downstream") == true
abort "execution falsely authorized" unless inventory.fetch("product_execution_authorized") == false
abort "future runner falsely implemented" unless inventory.dig("future_runner", "implemented") == false && inventory.dig("future_runner", "execution_credit_authorized") == false

expected_cases = %w[cli-help cli-version six-primitives-plan graph-json prompt-projection fork-join-ordering map-order-a map-order-b branch-order-a branch-order-b sequential-order-a sequential-order-b invalid-argument malformed-yaml schema-error unknown-provider unknown-agent unknown-task unknown-tool unknown-workflow unsupported-run-field missing-state dependency-cycle local-mock-run ed25519-sign-verify-tamper]
expected_behaviors = %w[cli-help cli-version six-primitives-plan graph-json prompt-projection fork-join-ordering map-reorder-equivalence branch-reorder-equivalence sequential-reorder-difference invalid-argument malformed-yaml schema-error unknown-provider-reference unknown-agent-reference unknown-task-reference unknown-tool-reference unknown-workflow-reference unsupported-run-field missing-state-reference dependency-cycle repeated-byte-stability local-mock-run ed25519-sign-verify-tamper]
abort "case identity drift" unless v1.fetch("case_ids") == expected_cases
abort "behavior identity drift" unless v1.fetch("behavior_ids") == expected_behaviors
abort "group identity drift" unless v1.fetch("equivalence_group_ids") == %w[map-declaration-order branch-declaration-order] && v1.fetch("difference_group_ids") == %w[sequential-step-order]
abort "observation inventory drift" unless v1.values_at("raw_observation_count", "normalized_observation_count") == [75, 75]

expected_owners = {"1"=>5591, "2"=>5591, "3"=>5591, "4"=>5591, "5"=>5592, "6"=>5589, "7"=>5589, "8"=>5589, "9"=>5590, "10"=>5590}
abort "runtime proof ownership drift" unless inventory.fetch("runtime_proof_group_owners") == expected_owners
abort "dependency receipt inventory drift" unless inventory.fetch("dependency_receipt_issues") == [5337, 5345, 5497, 5501, 5591, 5592, 5589, 5590, 5341, 5349]
required_cots = %w[serde serde_json serde_yaml jsonschema sha2 wait-timeout walkdir clap ed25519-dalek tempfile assert_cmd predicates]
abort "COTS reuse inventory drift" unless inventory.fetch("cots_reuse") == required_cots
budgets = inventory.fetch("budgets")
abort "time budget drift" unless budgets.values_at("preparation_seconds", "subject_verification_seconds", "comparison_seconds", "overlay_seconds", "complete_seconds", "lane_total_seconds", "lifecycle_ceiling_seconds") == [120, 120, 300, 120, 600, 1260, 7200]
abort "size budget drift" unless budgets.values_at("implementation_loc_ceiling", "test_fixture_loc_ceiling", "test_count_ceiling") == [1500, 2000, 120]

dependencies = cards.fetch("stp").fetch("dependencies").join("\n")
%w[#5337 #5345 #5497 #5501 #5591 #5592 #5589 #5590 #5341 #5349].each do |id|
  abort "missing dependency #{id}" unless dependencies.include?(id)
end
abort "#5361 downstream direction missing" unless dependencies.include?("#5361") && dependencies.include?("downstream")

acceptance = cards.fetch("stp").fetch("acceptance_criteria")
ids = acceptance.map { |item| item[/AC-\d+/] }.compact
abort "acceptance IDs incomplete" unless ids == (1..10).map { |n| "AC-#{n}" }

steps = cards.fetch("spp").fetch("steps")
step_ids = steps.flat_map { |step| step.fetch("acceptance_ids") }.uniq
abort "SPP acceptance coverage incomplete" unless (ids - step_ids).empty?

lanes = cards.fetch("vpp").fetch("lanes")
lane_ids = lanes.flat_map { |lane| lane.fetch("acceptance_ids") }.uniq
abort "VPP acceptance coverage incomplete" unless (ids - lane_ids).empty?
abort "preparation lane missing" unless lanes.any? { |lane| lane.fetch("lane") == "preparation-contract" && lane["defer_reason"].nil? }
execution_lanes = lanes.reject { |lane| lane.fetch("lane") == "preparation-contract" }
abort "execution lane lacks exact dependency gate" unless execution_lanes.all? { |lane| lane.fetch("defer_reason").to_s.include?("terminal") }
abort "future lanes falsely claim executable proof" unless execution_lanes.all? { |lane| lane.fetch("proof_role").include?("PLANNED-UNIMPLEMENTED") }
lane_seconds = lanes.to_h { |lane| [lane.fetch("lane"), lane.fetch("budget_seconds")] }
abort "lane budgets drift" unless lane_seconds == {"preparation-contract"=>120, "subject-and-corpus-verification"=>120, "exact-shadow-comparison"=>300, "runtime-workcell-overlay"=>120, "parity-complete"=>600}

design = File.read(".csdlc/prepared/issues/5350/design.md")
diagram = File.read(".csdlc/prepared/issues/5350/diagram.mmd")
%w[exact_match normalized_match approved_intentional_difference regression_blocker unsupported_blocker evidence_invalid].each do |term|
  abort "design missing disposition #{term}" unless design.include?(term)
end
abort "intentional difference lacks rollback impact" unless design.include?("reviewer identity, and rollback impact")
abort "future runner boundary missing" unless design.include?("not executable") && design.include?("proof") && design.include?("120 tests/fixture cases")
%w[WP-10 WP-10A 5497 5501 5591 5592 5589 5590 5341 5349 5361].each do |term|
  abort "design missing dependency #{term}" unless design.include?(term)
end
%w[exact_match normalized_match blocker Runtime v3 WP-10A].each do |term|
  abort "diagram missing #{term}" unless diagram.include?(term)
end

registry = read_json("docs/templates/prompts/current.json")
abort "native template authority drift" unless registry.dig("generations", "csdlc_v2_native", "template_set") == "1.0.0"

%w[design.md diagram.mmd].each do |name|
  path = ".csdlc/prepared/issues/5350/#{name}"
  abort "empty #{name}" if Digest::SHA256.file(path).hexdigest.empty?
end

status = `git status --porcelain=v1 --untracked-files=all`
abort "git status failed" unless $?.success?
unexpected = status.lines.map { |line| line[3..].strip }.reject do |path|
  path.start_with?(".csdlc/issues/5350/", ".csdlc/prepared/issues/5350/") || path == ".csdlc/locks/5350.lock"
end
abort "unrelated or product path changed: #{unexpected.join(', ')}" unless unexpected.empty?

puts "issue=5350 cards=6 phase=bound corpus=25/75/23 runtime_groups=10 product_edits=0 publication=forbidden"
