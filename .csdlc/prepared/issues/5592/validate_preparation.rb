#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ISSUE = 5592
KINDS = %w[sip stp spp vpp srp sor].freeze
EXPECTED_AC = (1..10).map { |number| "AC-#{number}" }.freeze
PREPARATION_PATHS = [
  ".csdlc/issues/5592",
  ".csdlc/locks/5592.lock",
  ".csdlc/prepared/issues/5592"
].freeze

def read_json(path)
  JSON.parse(File.read(path))
end

documents = KINDS.to_h do |kind|
  document = read_json(".csdlc/issues/#{ISSUE}/cards/#{kind}.values.json")
  abort "#{kind} is not the current v2 native projection" unless document.dig("identity", "template_version") == "1.0.0"
  [kind, document]
end
cards = documents.transform_values { |document| document.fetch("content").fetch("values") }

initial = read_json(".csdlc/prepared/issues/5592/bootstrap-request.json").fetch("initial")
source_authority = read_json(".csdlc/prepared/issues/5592/source-authority.json")
canonical_title = source_authority.fetch("canonical_title")
abort "operator-directed source authority is not canonical" unless source_authority.fetch("authority") == "operator_directive_2026-07-20"
abort "mutable snapshot is incorrectly authoritative" unless source_authority.fetch("snapshot_is_canonical_live_truth") == false
abort "bootstrap title is not the canonical WP-14 title" unless initial.fetch("title") == canonical_title && canonical_title.include?("[WP-14]")
abort "typed card title role is ambiguous" unless source_authority.fetch("card_identity_role") == "typed_regenerated_canonical_title"
abort "typed card regeneration path is unstated" unless source_authority.fetch("card_identity_update_path") == "csdlc-edit bootstrap followed by csdlc-bind"
abort "canonical WP-14 title is not synchronized across all six cards" unless documents.values.all? { |document| document.dig("identity", "title") == canonical_title }
{
  "goal" => cards.fetch("sip").fetch("goal"),
  "required_outcome" => cards.fetch("sip").fetch("required_outcome"),
  "declared_scope" => cards.fetch("sip").fetch("declared_scope"),
  "authority_boundary" => cards.fetch("sip").fetch("authority_boundary"),
  "operator_constraints" => cards.fetch("sip").fetch("operator_constraints"),
  "task_boundary" => cards.fetch("stp").fetch("task_boundary"),
  "deliverables" => cards.fetch("stp").fetch("deliverables"),
  "acceptance_criteria" => cards.fetch("stp").fetch("acceptance_criteria"),
  "dependencies" => cards.fetch("stp").fetch("dependencies"),
  "repo_inputs" => cards.fetch("stp").fetch("repo_inputs"),
  "non_goals" => cards.fetch("stp").fetch("non_goals"),
  "plan_summary" => cards.fetch("spp").fetch("summary"),
  "steps" => cards.fetch("spp").fetch("steps"),
  "invariants" => cards.fetch("spp").fetch("invariants"),
  "risks" => cards.fetch("spp").fetch("risks"),
  "stop_conditions" => cards.fetch("spp").fetch("stop_conditions"),
  "validation_lanes" => cards.fetch("vpp").fetch("lanes"),
  "failure_policy" => cards.fetch("vpp").fetch("failure_policy"),
  "review_prompts" => cards.fetch("srp").fetch("review_prompts"),
  "review_scope" => cards.fetch("srp").fetch("review_scope")
}.each do |field, canonical|
  abort "bootstrap/card drift for #{field}" unless initial.fetch(field) == canonical
end

generations = KINDS.map do |kind|
  read_json(".csdlc/issues/#{ISSUE}/cards/#{kind}.values.json").dig("identity", "generation")
end
abort "card generation mismatch" unless generations.uniq.length == 1

registry = read_json("docs/templates/prompts/current.json")
abort "active prompt registry is not 1.0.3" unless registry.fetch("csdlc_prompt_template_set") == "1.0.3"
abort "native projection authority is not 1.0.0" unless registry.dig("generations", "csdlc_v2_native", "template_set") == "1.0.0"

git_common_dir, git_common_result = Open3.capture2("git", "rev-parse", "--git-common-dir")
abort "cannot resolve shared repository root" unless git_common_result.success?
shared_root = File.dirname(File.expand_path(git_common_dir.strip))
issue_source = read_json(File.join(shared_root, source_authority.fetch("retained_snapshot")))
issue = issue_source.find { |entry| (entry["number"] || entry["issue_number"]) == ISSUE }
abort "retained issue-body snapshot missing" unless issue
%w[reasoning graphs bounded loops adaptive learning governed cognition].each do |term|
  abort "retained issue-body snapshot missing #{term}" unless issue.fetch("body").downcase.include?(term)
end

inventory = read_json(".csdlc/prepared/issues/5592/future-live-test-inventory.json")
abort "future inventory issue mismatch" unless inventory.fetch("issue") == ISSUE
abort "future inventory is not live-kernel proof" unless inventory.fetch("proof_class") == "future_live_kernel"
abort "future inventory does not reject zero matches" unless inventory.fetch("zero_matches") == "fail"
abort "future inventory can credit mutable metadata" unless inventory.fetch("forbidden_credit").include?("metadata")
future_lanes = inventory.fetch("lanes")
abort "future inventory must contain seven exact lanes" unless future_lanes.length == 7
future_tests = future_lanes.flat_map { |lane| lane.fetch("exact_tests") }
abort "future inventory contains empty or duplicate exact identities" if future_tests.any?(&:empty?) || future_tests.uniq != future_tests
vpp_lanes = cards.fetch("vpp").fetch("lanes").to_h { |lane| [lane.fetch("lane"), lane] }
future_lanes.each do |lane|
  id = lane.fetch("id")
  argv = vpp_lanes.fetch(id).fetch("argv")
  abort "VPP lane #{id} is not bound to the exact runner" unless argv == ["ruby", ".csdlc/prepared/issues/5592/run_exact_live_test_lane.rb", "--lane", id]
end

by_number = ->(id) { id.delete_prefix("AC-").to_i }
acceptance = cards.fetch("stp").fetch("acceptance_criteria").map { |value| value[/AC-\d+/] }.compact.uniq.sort_by(&by_number)
step_coverage = cards.fetch("spp").fetch("steps").flat_map { |step| step.fetch("acceptance_ids") }.uniq.sort_by(&by_number)
lane_coverage = cards.fetch("vpp").fetch("lanes").flat_map { |lane| lane.fetch("acceptance_ids") }.uniq.sort_by(&by_number)
abort "acceptance set incomplete" unless acceptance == EXPECTED_AC
abort "SPP coverage incomplete" unless step_coverage == EXPECTED_AC
abort "VPP coverage incomplete" unless lane_coverage == EXPECTED_AC
abort "deferred product validation lane" if cards.fetch("vpp").fetch("lanes").any? { |lane| lane["defer_reason"] }

constraints = cards.fetch("sip").fetch("operator_constraints").join("\n")
dependencies = cards.fetch("stp").fetch("dependencies").join("\n")
non_goals = cards.fetch("stp").fetch("non_goals").join("\n")
review_scope = cards.fetch("srp").fetch("review_scope")
abort "#5591 clean-review stop missing" unless dependencies.include?("clean reviewed #5591") && dependencies.include?("before any product implementation")
abort "typed collision-free claim gate missing" unless dependencies.include?("typed active-claim ledger") && dependencies.include?("collision-free narrow Parity-B")
abort "#5341 downstream truth missing" unless dependencies.include?("#5341 is downstream") && dependencies.include?("grants no implementation authority")
repo_inputs = cards.fetch("stp").fetch("repo_inputs").join("\n")
abort "operator-directed title authority missing" unless repo_inputs.include?("source-authority.json operator-directed canonical title authority")
abort "mutable snapshot authority is ambiguous" unless repo_inputs.include?("mutable operator snapshot") && repo_inputs.include?("not canonical live truth")
abort "typed-only constraint missing" unless constraints.include?("typed C-SDLC v2")
abort "adversarial signal constraint missing" unless constraints.include?("untrusted signals") && constraints.include?("cannot create authority")
abort "safe affect non-claims missing" unless constraints.include?("affect") && constraints.include?("non-claims") && non_goals.include?("subjective affect")
abort "Runtime v2 non-reuse missing" unless non_goals.include?("Runtime v2 source reuse")
abort "no-publication boundary missing" unless constraints.include?("publication") && non_goals.include?("publication")
abort "full review scope missing" unless review_scope.include?("AC-1 through AC-10") && review_scope.include?("live-kernel proof")

design = File.read(".csdlc/prepared/issues/5592/design.md")
diagram = File.read(".csdlc/prepared/issues/5592/diagram.mmd")
matrix = File.read(".csdlc/prepared/issues/5592/acceptance-matrix.md")
claim_doc = File.read(".csdlc/prepared/issues/5592/protected-path-claim.md")
%w[adl-runtime-kernel canonical ingress bounded loops one-shot rollback adversarial monotonic subjective Runtime v2].each do |term|
  abort "design missing #{term}" unless design.include?(term)
end
%w[ingress graph loop mutation affect curiosity freedom shutdown evidence].each do |term|
  abort "diagram missing #{term}" unless diagram.downcase.include?(term)
end
EXPECTED_AC.each { |id| abort "acceptance matrix missing #{id}" unless matrix.include?(id) }
{
  "AC-4" => "Safe affect reasoning-control",
  "AC-5" => "Curiosity and Theory-of-Mind boundaries",
  "AC-6" => "Governed cognition non-bypass",
  "AC-9" => "Exact live-kernel evidence",
  "AC-10" => "Budget, quality, and review truth"
}.each do |id, outcome|
  abort "acceptance matrix drift for #{id}" unless matrix.lines.any? { |line| line.include?("| #{id} | #{outcome} |") }
end
%w[Constructability Godel guild economics skill standard].each do |term|
  abort "feature matrix missing #{term}" unless matrix.downcase.include?(term.downcase)
end

index = read_json(".csdlc/issues/5592/index.json")
regeneration = read_json(".csdlc/prepared/issues/5592/typed-regeneration.json")
abort "typed regeneration result identity drifted" unless regeneration.fetch("result_generation") == index.fetch("generation") && regeneration.fetch("result_digest") == index.fetch("digest")
abort "typed regeneration did not retain preparation-only truth" unless regeneration.fetch("product_paths_changed") == false && regeneration.fetch("publication_authorized") == false
abort "typed regeneration route is incomplete" unless regeneration.fetch("typed_route") == ["csdlc-install resolve", "csdlc-edit bootstrap", "csdlc-bind"]
claim_paths = index.fetch("claim").fetch("protected_paths")
abort "preparation claim is not exact and disjoint" unless claim_paths == PREPARATION_PATHS
abort "claim document omits implementation gate" unless claim_doc.include?("#5591 has a clean reviewed")
abort "claim includes Runtime v2" if claim_paths.any? { |path| path.downcase.include?("runtime_v2") }
abort "claim includes product path" if claim_paths.any? { |path| path.start_with?("adl-runtime", "adl/") }

status, status_result = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all")
abort "git status failed" unless status_result.success?
changed_paths = status.lines.map { |line| line[3..].strip }.reject(&:empty?)
unexpected = changed_paths.reject do |path|
  path.start_with?(".csdlc/issues/5592/", ".csdlc/prepared/issues/5592/") || path == ".csdlc/locks/5592.lock"
end
abort "product or unrelated path changed: #{unexpected.join(', ')}" unless unexpected.empty?

puts "cards=6 generation=#{generations.first} acceptance=10 spp=complete vpp=complete deferrals=0 claim=preparation-only product_edits=0"
