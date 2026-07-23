#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

def capture!(*argv, chdir: nil)
  stdout, stderr, status = Open3.capture3(*argv, chdir: chdir)
  abort("command failed: #{argv.join(' ')}\n#{stderr}#{stdout}") unless status.success?
  stdout
end

root = File.realpath(File.expand_path("../../../..", __dir__))
issue_root = File.join(root, ".csdlc/issues/5340")
prepared = File.join(root, ".csdlc/prepared/issues/5340")
required = %w[sip stp spp vpp srp sor].flat_map do |card|
  [File.join(issue_root, "cards/#{card}.md"), File.join(issue_root, "cards/#{card}.values.json")]
end
required += %w[
  design.md diagram.mmd preparation-validation.json validate-preparation.sh
  validate-preparation.rb validate-engine.sh validate-engine-lane.sh
  validate-post-merge.rb validate-post-merge-lane.sh fetch-dependency.sh
  check-dependency.sh verify-dependency.rb verify-scope.rb validate-cots.rb
  validate-source-authority.rb validate-source-authority-fixtures.rb measure-engine.rb
  warm-source-authority-validator.sh
  source-authority-validator/Cargo.toml source-authority-validator/Cargo.lock
  source-authority-validator/src/main.rs
].map { |name| File.join(prepared, name) }
required += Dir.glob(File.join(prepared, "pvf/*.json"))
missing = required.reject { |path| File.file?(path) }
abort("missing canonical preparation artifacts: #{missing.join(', ')}") unless missing.empty?
abort("missing future typed engine PVF requests") unless Dir.glob(File.join(prepared, "pvf/*.json")).length == 6

record = JSON.parse(File.read(File.join(issue_root, "index.json")))
abort("typed lifecycle is not bound") unless record.fetch("phase") == "bound"
review = record.fetch("design_review")
approved = review.is_a?(Hash) ? review["approved"] : nil
abort("typed design review is not approved") unless approved.is_a?(Hash) && !approved.fetch("reviewer").empty? && !approved.fetch("revision").empty?

registry = JSON.parse(File.read(File.join(root, "docs/templates/prompts/current.json")))
abort("current prompt registry is not active") unless registry.fetch("status") == "active"
native = registry.dig("generations", "csdlc_v2_native")
abort("current prompt registry lacks native v2 authority") unless native.is_a?(Hash)
template_version = native.fetch("template_set")
%w[sip stp spp vpp srp sor].each do |kind|
  card = JSON.parse(File.read(File.join(issue_root, "cards/#{kind}.values.json")))
  identity = card.fetch("identity")
  abort("#{kind} is not rendered from current native registry") unless identity.fetch("template_version") == template_version
  abort("#{kind} generation drift") unless identity.fetch("generation") == record.fetch("generation")
end

scope = JSON.parse(capture!("ruby", ".csdlc/prepared/issues/5340/verify-scope.rb", chdir: root))
abort("scope proof did not pass") unless scope.fetch("outcome") == "passed"
authority_negative = JSON.parse(capture!("ruby", ".csdlc/prepared/issues/5340/validate-source-authority-fixtures.rb", chdir: root))
abort("source-authority negative proof did not pass") unless authority_negative.fetch("outcome") == "passed" && authority_negative.fetch("fixtures").length >= 6 && authority_negative.fetch("fixtures").all? { |item| item.fetch("rejected") == true }

common = capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir", chdir: root).strip
primary = File.dirname(common)
root_branch = capture!("git", "branch", "--show-current", chdir: primary).strip
root_status = capture!("git", "status", "--short", chdir: primary).strip
abort("primary checkout is not clean main") unless root_branch == "main" && root_status.empty?
abort("#5340 canonical state leaked into primary checkout") if File.exist?(File.join(primary, ".csdlc/issues/5340"))

design_text = File.read(File.join(prepared, "design.md"))
diagram_text = File.read(File.join(prepared, "diagram.mmd"))
stp = JSON.parse(File.read(File.join(issue_root, "cards/stp.values.json"))).dig("content", "values")
spp = JSON.parse(File.read(File.join(issue_root, "cards/spp.values.json"))).dig("content", "values")
vpp = JSON.parse(File.read(File.join(issue_root, "cards/vpp.values.json"))).dig("content", "values")
dependencies = stp.fetch("dependencies")
repo_inputs = stp.fetch("repo_inputs")
affected_areas = spp.fetch("affected_areas")
replan_triggers = spp.fetch("replan_triggers")
steps = spp.fetch("steps").to_h { |step| [step.fetch("id"), step.fetch("status")] }
vpp_lanes = vpp.fetch("lanes").map { |lane| lane.fetch("lane") }
contract_text = [design_text, diagram_text].join("\n")
dependency_source = File.read(File.join(prepared, "verify-dependency.rb"))
scope_source = File.read(File.join(prepared, "verify-scope.rb"))
fetch_source = File.read(File.join(prepared, "fetch-dependency.sh"))
engine_source = File.read(File.join(prepared, "validate-engine-lane.sh"))
postmerge_source = File.read(File.join(prepared, "validate-post-merge.rb")) + File.read(File.join(prepared, "validate-post-merge-lane.sh"))
authority_wrapper = File.read(File.join(prepared, "validate-source-authority.rb"))
authority_source = File.read(File.join(prepared, "source-authority-validator/src/main.rs"))
authority_manifest = File.read(File.join(prepared, "source-authority-validator/Cargo.toml"))
preparation_request = JSON.parse(File.read(File.join(prepared, "preparation-validation.json")))
pvf = Dir.glob(File.join(prepared, "pvf/*.json")).to_h do |path|
  [File.basename(path, ".json"), JSON.parse(File.read(path)).dig("manifest", "lanes", 0)]
end
checks = {
  "sole direct dependency #5338" => dependencies.all? { |value| value.include?("#5338") || value.start_with?("Landed reviewed adl-compiler") } && dependencies.none? { |value| value.include?("#5336") } && repo_inputs.any? { |value| value.include?("#5336") && value.include?("transitive") },
  "exact protected affected areas" => affected_areas.sort == scope.fetch("protected_paths").sort,
  "explicit replan triggers" => replan_triggers.length >= 5 && replan_triggers.any? { |value| value.include?("#5338") } && replan_triggers.any? { |value| value.include?("COTS") },
  "preparation and watch step truth" => steps.fetch("S1") == "completed" && steps.fetch("S2") == "in_progress" && %w[S3 S4 S5].all? { |id| steps.fetch(id) == "pending" },
  "complete canonical VPP" => vpp_lanes == %w[preparation-contract engine-cache-warm engine-focused engine-quality ordering-resume engine-budgets post-merge-exact],
  "typed terminal receipt gate" => dependency_source.include?("retain-receipt") && dependency_source.include?("observed_state") && dependency_source.include?("pull_request") && dependency_source.include?("csdlc-doctor"),
  "current-main and merge ancestry" => fetch_source.include?("refs/heads/main:refs/remotes/origin/main") && dependency_source.include?("ADL_WP5340_EXPECTED_ORIGIN_MAIN_SHA") && dependency_source.include?("[origin_main, head"),
  "live sole-writer claim" => scope_source.include?("expires_unix_seconds") && scope_source.include?("heartbeat_unix_seconds") && dependency_source.include?("typed claim/record doctor"),
  "implementation budget" => contract_text.include?("4,000"),
  "test budget" => contract_text.include?("3,500"),
  "typed hard validation budgets" => pvf.fetch("focused").fetch("timeout_seconds") == 120 && pvf.fetch("quality").fetch("timeout_seconds") == 120 && pvf.fetch("determinism").fetch("timeout_seconds") == 300 && pvf.fetch("budgets").fetch("timeout_seconds") == 600 && pvf.fetch("postmerge").fetch("timeout_seconds") == 600,
  "FastWork cache boundary" => %w[CARGO_TARGET_DIR CARGO_HOME SCCACHE_DIR TMPDIR].all? { |value| engine_source.include?(value) } && engine_source.include?("File.realpath"),
  "no Runtime source authority" => contract_text.include?("Runtime v2") && contract_text.include?("Runtime v3"),
  "COTS source authority guard" => authority_manifest.include?('syn = { version = "=2.0.118"') && %w[fs io net process thread time env sync future task].all? { |token| authority_source.include?(token) } && %w[visit_signature visit_expr_async visit_expr_await visit_expr_unsafe visit_macro visit_attribute ALLOWED_ATTRIBUTES].all? { |method| authority_source.include?(method) } && %w[ADL_WP5340_CARGO_HOME SCCACHE_DIR RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER].all? { |name| authority_wrapper.include?(name) } && authority_wrapper.include?("/Volumes/FastWork/") && !authority_wrapper.include?("scan(") && engine_source.include?("validate-source-authority.rb") && postmerge_source.include?("validate-source-authority.rb"),
  "source authority negative fixtures" => authority_negative.fetch("fixtures").map { |item| item.fetch("fixture") }.sort == %w[async_fn.rs core_future.rs grouped_fs.rs native_export.rs path_module.rs stdout.rs],
  "engine-owned policy" => contract_text.include?("EnginePolicy") && contract_text.include?("engine-owned"),
  "deterministic cancellation" => contract_text.include?("cancelling") && contract_text.include?("late completion"),
  "fresh-process proof" => engine_source.include?("fresh_process_driver") && engine_source.include?("cmp -s") && engine_source.scan('"${driver}"').length >= 2,
  "exact provisional COTS" => %w[1.0.229 1.0.151 0.10.9 0.4.3].all? { |value| contract_text.include?(value) } && contract_text.include?("provisional"),
  "declared preparation tool cache" => preparation_request.dig("selection", "allow_network") == true && preparation_request.dig("manifest", "lanes").any? { |lane| lane.fetch("id") == "preparation-tool-cache" && lane.fetch("network") == "external" && lane.fetch("timeout_seconds") == 60 } && preparation_request.dig("manifest", "lanes").any? { |lane| lane.fetch("id") == "preparation-contract" && lane.fetch("network") == "denied" && lane.fetch("dependencies") == ["preparation-tool-cache"] },
  "offline required validation" => %w[focused quality determinism budgets postmerge].all? { |name| pvf.fetch(name).fetch("network") == "denied" } && engine_source.include?("CARGO_NET_OFFLINE=true") && engine_source.include?("--offline") && pvf.fetch("warm-cache").fetch("network") == "external",
  "detached typed post-merge" => postmerge_source.include?("checkout\", \"--detach") && postmerge_source.include?("csdlc-validate") && postmerge_source.include?("merge-base") && postmerge_source.include?("CARGO_NET_OFFLINE=true"),
  "PVF classification" => contract_text.include?("PVF and test classification"),
  "no-deferral matrix" => contract_text.include?("No-deferral acceptance matrix"),
  "rollback" => contract_text.include?("Failure and rollback")
}
failed = checks.reject { |_name, passed| passed }.keys
abort("preparation contract checks failed: #{failed.join(', ')}") unless failed.empty?

puts JSON.generate(
  schema: "adl.csdlc.preparation-contract-proof.v2",
  issue: 5340,
  typed_integrity_authority: "csdlc-doctor plus csdlc-validate wrapper",
  checks: checks.keys,
  protected_paths: scope.fetch("protected_paths"),
  changed_paths: scope.fetch("changed_paths"),
  outcome: "passed"
)
