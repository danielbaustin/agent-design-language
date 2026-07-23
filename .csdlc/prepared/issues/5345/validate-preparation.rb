#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"
require "shellwords"

ISSUE = "5345"
ROOT = File.expand_path("../../../..", __dir__)
COMMON_DIR = File.expand_path(`git rev-parse --git-common-dir`.strip, ROOT)
OWNER_ROOT = File.dirname(COMMON_DIR)
ISSUE_DIR = File.join(ROOT, ".csdlc", "issues", ISSUE)
PREP_DIR = File.join(ROOT, ".csdlc", "prepared", "issues", ISSUE)
CARDS = %w[sip stp spp vpp srp sor].freeze
REQUIRED_PATHS = [
  File.join(ISSUE_DIR, "index.json"),
  File.join(ISSUE_DIR, "audit.jsonl"),
  File.join(PREP_DIR, "design.md"),
  File.join(PREP_DIR, "diagram.mmd"),
  File.join(PREP_DIR, "check-dependencies.rb"),
  File.join(PREP_DIR, "cots-lock-baseline.json"),
  File.join(PREP_DIR, "bootstrap-request.json"),
  File.join(PREP_DIR, "bind-request.json"),
  File.join(PREP_DIR, "validate-preparation.rb"),
  File.join(PREP_DIR, "validate-cli.sh"),
  File.join(PREP_DIR, "validate-implementation.rb")
].freeze

failures = []
REQUIRED_PATHS.each { |path| failures << "missing #{path}" unless File.file?(path) }
CARDS.each do |card|
  failures << "missing #{card}.md" unless File.file?(File.join(ISSUE_DIR, "cards", "#{card}.md"))
  failures << "missing #{card}.values.json" unless File.file?(File.join(ISSUE_DIR, "cards", "#{card}.values.json"))
end

index = File.file?(File.join(ISSUE_DIR, "index.json")) ? JSON.parse(File.read(File.join(ISSUE_DIR, "index.json"))) : {}
claim = index["claim"] || {}
expected = [
  ".csdlc/issues/5345",
  ".csdlc/locks/5345.lock",
  ".csdlc/prepared/issues/5345",
  ".csdlc/evidence/5345",
  "adl-v2/crates/adl-cli",
  "adl-v2/crates/adl-cli/src",
  "adl-v2/crates/adl-cli/tests",
  "adl-v2/Cargo.toml",
  "adl-v2/Cargo.lock",
  "adl-v2/tools",
  "adl-v2/tools/install-adl-v2.sh"
]
actual = claim["protected_paths"] || []
if index["phase"] == "initialized"
  pending = expected - ["adl-v2/Cargo.toml", "adl-v2/Cargo.lock"]
  failures << "pre-bind protected paths differ: #{actual.inspect}" unless actual.sort == pending.sort
  plan_path = File.join(PREP_DIR, "amend-claim-scope-plan.json")
  failures << "typed post-bind scope amendment plan is missing" unless File.file?(plan_path)
  if File.file?(plan_path)
    plan = JSON.parse(File.read(plan_path))
    failures << "scope amendment is not first post-bind mutation" unless plan["timing"] == "first_typed_mutation_after_bind"
    failures << "scope amendment does not require live CAS" unless plan["cas_source"].to_s.include?("current generation and digest after csdlc-bind")
    failures << "scope amendment paths differ" unless plan["add_protected_paths"] == ["adl-v2/Cargo.toml", "adl-v2/Cargo.lock"]
  end
else
  failures << "protected paths differ: #{actual.inspect}" unless actual.sort == expected.sort
end

design = File.file?(File.join(PREP_DIR, "design.md")) ? File.read(File.join(PREP_DIR, "design.md")) : ""
%w[5339 5338 5340 5342 5341 5349 5343 5344].each do |dependency|
  failures << "design omits ##{dependency}" unless design.include?("##{dependency}")
end
failures << "design omits no-hard-coded-address boundary" unless design.include?("hard-coded address")
failures << "design omits Runtime v2 exclusion" unless design.include?("Runtime v2")
failures << "design omits 2500 implementation LoC budget" unless design.include?("2,500 Rust implementation lines")
failures << "design omits 600-second complete budget" unless design.include?("600-second ceiling")
failures << "design overclaims final COTS closure" unless design.include?("not a claim that the future")
failures << "design omits retained/current projection parity" unless design.include?("current typed\nprojection to equal the retained")

diagram = File.file?(File.join(PREP_DIR, "diagram.mmd")) ? File.read(File.join(PREP_DIR, "diagram.mmd")) : ""
failures << "diagram is not a Mermaid flowchart" unless diagram.start_with?("flowchart ")
failures << "diagram omits receipt and ancestry gate" unless diagram.include?("receipt-backed, and ancestral")

bootstrap = File.file?(File.join(PREP_DIR, "bootstrap-request.json")) ? JSON.parse(File.read(File.join(PREP_DIR, "bootstrap-request.json"))) : {}
initial = bootstrap.fetch("initial", {})
acceptance = initial.fetch("acceptance_criteria", []).join("\n")
dependencies = initial.fetch("dependencies", []).join("\n")
lanes = initial.fetch("validation_lanes", [])
cots = %w[clap serde serde_json tempfile fs2 sha2]
cots.each { |name| failures << "cards omit COTS #{name}" unless acceptance.include?(name) }
%w[5339 5338 5340 5342 5341 5349].each do |dependency|
  failures << "typed dependencies omit ##{dependency}" unless dependencies.include?("##{dependency}")
end
failures << "typed cards omit explicit rollback contract" unless acceptance.include?("Rollback is explicit")
failures << "typed cards omit no-deferral contract" unless acceptance.include?("no deferred implementation or validation claim")
failures << "typed cards omit 2500 implementation budget" unless acceptance.include?("2500 Rust implementation lines")
failures << "typed cards omit 2500 test budget" unless acceptance.include?("2500 test/fixture lines")
failures << "typed cards omit module stop" unless acceptance.include?("modules stay below 1000 lines")
expected_lanes = %w[preparation-contract cli-focused cli-quality selector-installer cli-budgets post-merge-exact]
failures << "validation lanes differ: #{lanes.map { |lane| lane['lane'] }.inspect}" unless lanes.map { |lane| lane["lane"] } == expected_lanes
failures << "preparation lane is deferred" unless lanes.first && lanes.first["defer_reason"].nil?
failures << "future lanes lack truthful defer reasons" unless lanes.drop(1).all? { |lane| lane["defer_reason"].is_a?(String) && !lane["defer_reason"].empty? }
failures << "validation budgets differ" unless lanes.map { |lane| lane["budget_seconds"] } == [120, 120, 120, 300, 600, 600]

baseline_path = File.join(PREP_DIR, "cots-lock-baseline.json")
if File.file?(baseline_path)
  baseline = JSON.parse(File.read(baseline_path))
  failures << "COTS baseline revision differs from preparation base" unless baseline["revision"] == `git merge-base HEAD origin/main`.strip
  baseline.fetch("locks", {}).each do |relative, expected_digest|
    path = File.join(ROOT, relative)
    failures << "missing COTS evidence lock #{relative}" unless File.file?(path)
    failures << "COTS evidence digest drift for #{relative}" if File.file?(path) && Digest::SHA256.file(path).hexdigest != expected_digest
  end
  expected_pins = {"clap"=>"4.6.4", "serde"=>"1.0.229", "serde_json"=>"1.0.151", "tempfile"=>"3.27.0", "fs2"=>"0.4.3", "sha2"=>"0.10.9"}
  observed_pins = baseline.fetch("packages", {}).transform_values { |entry| entry["version"] }
  failures << "COTS pin baseline differs: #{observed_pins.inspect}" unless observed_pins == expected_pins
end

baseline_docs = {
  "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml" => ["issue: 5345", "depends_on:", "WP-09"],
  "docs/milestones/v0.91.8/WBS_v0.91.8.md" => ["WP-10", "#5345"],
  "docs/milestones/v0.91.8/DESIGN_v0.91.8.md" => ["thin CLI", "generation selector"],
  "docs/milestones/v0.91.8/features/ADL_V2_CORE_v0.91.8.md" => ["selector"]
}
baseline_docs.each do |relative, needles|
  path = File.join(ROOT, relative)
  body = File.file?(path) ? File.read(path) : ""
  failures << "baseline missing #{relative}" unless File.file?(path)
  needles.each { |needle| failures << "#{relative} omits #{needle}" unless body.include?(needle) }
end

selector = JSON.parse(File.read(File.join(ROOT, "csdlc-v2/operator/generation-selector.json")))
failures << "typed selector does not resolve v2 by default" unless selector["default_generation"] == "v2"

branch = `git branch --show-current`.strip
failures << "wrong issue branch #{branch.inspect}" unless branch == "codex/5345-v0918-wp10-thin-cli-selector"
base_revision = JSON.parse(File.read(baseline_path))["revision"]
failures << "reviewed preparation base is not ancestral to HEAD" unless system("git", "merge-base", "--is-ancestor", base_revision, "HEAD")
primary = `git worktree list --porcelain`.split("\n\n").find { |entry| entry.include?("branch refs/heads/main") }
if primary
  primary_path = primary.lines.first.to_s.sub("worktree ", "").strip
  primary_status = `git -C #{primary_path.shellescape} status --short --branch`
  failures << "primary main is not clean: #{primary_status}" unless primary_status.lines.drop(1).empty?
else
  failures << "primary main worktree not found"
end

{
  "check-dependencies.rb" => ["ruby", "-c"],
  "validate-preparation.rb" => ["ruby", "-c"],
  "validate-implementation.rb" => ["ruby", "-c"],
  "validate-cli.sh" => ["bash", "-n"]
}.each do |file, argv|
  _out, err, ok = Open3.capture3(*argv, File.join(PREP_DIR, file))
  failures << "syntax validation failed for #{file}: #{err}" unless ok.success?
end

if index.any?
  registry = JSON.parse(File.read(File.join(ROOT, "docs/templates/prompts/current.json")))
  native_version = registry.dig("generations", "csdlc_v2_native", "template_set")
  CARDS.each do |card|
    values_path = File.join(ISSUE_DIR, "cards", "#{card}.values.json")
    next unless File.file?(values_path)
    values = JSON.parse(File.read(values_path))
    failures << "#{card} template version drift" unless values.dig("identity", "template_version") == native_version
    failures << "#{card} generation drift" unless values.dig("identity", "generation") == index["generation"]
  end
end

stdout, stderr, status = Open3.capture3(
  File.join(OWNER_ROOT, ".adl", "bin", "csdlc-v2", "csdlc-doctor"),
  "--repo", ROOT,
  "--issue", ISSUE
)
unless status.success?
  report = JSON.parse(stdout) rescue {}
  expected_preapproval_stale = index["phase"] == "initialized" &&
    report["status"] == "corrupt" &&
    report.fetch("findings", []).map { |finding| finding["message"] } == ["design/diagram references are stale"]
  failures << "typed doctor failed: #{stderr}\n#{stdout}" unless expected_preapproval_stale
end

if failures.empty?
  puts JSON.generate(schema: "adl.v0918.wp10_preparation.v1", status: "pass", issue: 5345)
  exit 0
end

warn JSON.pretty_generate(schema: "adl.v0918.wp10_preparation.v1", status: "fail", failures: failures)
exit 1
