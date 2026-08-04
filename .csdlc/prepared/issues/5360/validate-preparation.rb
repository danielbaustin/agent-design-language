#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = 5360
BASE_REVISION = "fbf96beac1cb61c85bf7889e9c08729916c0796b"
ISSUE_DIR = ROOT.join(".csdlc/issues/#{ISSUE}")
INDEX = ISSUE_DIR.join("index.json")
PREP = ROOT.join(".csdlc/prepared/issues/#{ISSUE}")
REQUIRED_CARDS = %w[sip stp spp vpp srp sor].freeze
PREP_FILES = %w[
  design.md diagram.mmd bootstrap-request.json bind-request.json
  approve-design-request.json
  check-dependencies.rb validate-preparation.rb run-validation-lane.rb
  validate-card-integrity.rb card-integrity-request.json validation-request.json
  preparation-review.md preparation-review-findings.md
].freeze
EXPECTED_PATHS = [
  ".csdlc/issues/5360",
  ".csdlc/locks/5360.lock",
  ".csdlc/prepared/issues/5360",
  ".csdlc/evidence/5360"
].freeze
FUTURE_PATHS = %w[
  README.md
  docs/planning/ADL_FEATURE_LIST.md
  docs/milestones/v0.91.8/WBS_v0.91.8.md
  docs/milestones/v0.91.8/SPRINT_v0.91.8.md
  docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md
  docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
  docs/milestones/v0.91.8/MILESTONE_CHECKLIST_v0.91.8.md
  docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
  docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md
  docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md
  docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md
  docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
  docs/milestones/v0.91.8/FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md
  docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
  docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md
  docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md
  docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md
].freeze

def assert(condition, message)
  raise message unless condition
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  assert(status.success?, "git #{args.join(' ')} failed: #{out.strip}")
  out.strip
end

def installed_binary(name)
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  binary = common.parent.join(".adl/bin/csdlc-v2", name)
  assert(binary.file? && binary.executable?, "missing installed typed binary #{name}")
  binary.to_s
end

assert(INDEX.file?, "missing typed index")
index = JSON.parse(INDEX.read)
assert(index.fetch("issue") == ISSUE, "wrong issue identity")
assert(index.fetch("phase") == "bound", "preparation must finish bound")
design_review = index.fetch("design_review").fetch("approved")
assert(design_review.fetch("reviewer") == "subagent:5360-preparation-review", "wrong design reviewer")
claim = index.fetch("claim")
assert(claim.fetch("id") == "claim-5360-v0918-wp17-documentation-preparation", "wrong claim")
assert(claim.fetch("protected_paths") == EXPECTED_PATHS, "claim is not exact preparation-only scope")
assert(claim.fetch("purpose").include?("#5351"), "claim purpose omits WP-16 gate")
assert(claim.fetch("worktree") == ".", "claim must bind the current worktree")

assert(git("branch", "--show-current") == "codex/5360-v0918-preparation", "wrong branch")
assert(git("branch", "--show-current") != "main", "preparation cannot run on main")
common = Pathname.new(git("rev-parse", "--git-common-dir"))
common = ROOT.join(common) unless common.absolute?
assert(git("rev-parse", "--show-toplevel") != common.parent.to_s, "preparation cannot run in primary checkout")

PREP_FILES.each { |name| assert(PREP.join(name).file?, "missing #{name}") }
PREP_FILES.each do |name|
  bytes = PREP.join(name).binread
  assert(bytes.end_with?("\n"), "#{name} lacks final newline")
  assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{name} contains trailing whitespace")
end

assert(index.fetch("design_path") == ".csdlc/prepared/issues/5360/design.md", "wrong design path")
assert(index.fetch("diagram_path") == ".csdlc/prepared/issues/5360/diagram.mmd", "wrong diagram path")
cards = index.fetch("cards")
REQUIRED_CARDS.each do |name|
  metadata = cards.fetch(name)
  card = ISSUE_DIR.join("cards/#{name}.md")
  values_path = ISSUE_DIR.join("cards/#{name}.values.json")
  assert(card.file? && values_path.file?, "missing #{name} card/value pair")
  values = JSON.parse(values_path.read)
  assert(values.dig("identity", "issue") == ISSUE, "wrong #{name} issue")
  assert(values.dig("content", "card_kind") == name, "wrong #{name} kind")
  digests = metadata.values_at("values_digest", "rendered_digest", "ast_digest")
  assert(digests.all? { |digest| digest.match?(/\A[0-9a-f]{64}\z/) }, "invalid #{name} digest")
end

bootstrap = JSON.parse(PREP.join("bootstrap-request.json").read)
initial = bootstrap.fetch("initial")
assert(initial.fetch("acceptance_criteria").length == 8, "acceptance criteria count mismatch")
assert(initial.fetch("dependencies") == ["WP-16 #5351 merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral to the exact #5360 execution revision"], "dependency drift")
assert(initial.fetch("operator_constraints").any? { |item| item.include?("current registry") }, "current-registry rule missing")
assert(initial.fetch("non_goals").any? { |item| item.include?("Runtime v2") }, "Runtime v2 non-goal missing")
assert(initial.fetch("authority_boundary").any? { |item| item.include?("only four exact #5360") }, "preparation authority boundary missing")
FUTURE_PATHS.each do |path|
  assert(ROOT.join(path).file?, "future protected path does not exist: #{path}")
  assert(PREP.join("design.md").read.include?("`#{path}`"), "future protected path missing from design: #{path}")
end

registry = JSON.parse(ROOT.join("docs/templates/prompts/current.json").read)
assert(registry.fetch("status") == "active", "prompt registry is not active")
assert(registry.fetch("lifecycle").map(&:downcase) == REQUIRED_CARDS, "prompt registry lifecycle mismatch")
native = registry.dig("generations", "csdlc_v2_native")
assert(native.is_a?(Hash) && native.fetch("projection_family") == "compact_native", "native v2 projection missing")
shape_manifest_path = ROOT.join(native.fetch("shape_manifest_path"))
assert(shape_manifest_path.file?, "native shape manifest missing")
shape_manifest = JSON.parse(shape_manifest_path.read)
assert(shape_manifest.fetch("generation") == "csdlc_v2_native", "native shape generation mismatch")
assert(shape_manifest.fetch("template_set") == native.fetch("template_set"), "native shape template mismatch")
REQUIRED_CARDS.each do |name|
  rendered = ISSUE_DIR.join("cards/#{name}.md").read
  values = JSON.parse(ISSUE_DIR.join("cards/#{name}.values.json").read)
  identity = values.fetch("identity")
  assert(identity.fetch("template_version") == native.fetch("template_set"), "#{name} native template provenance mismatch")
  assert(identity.fetch("repository") == "danielbaustin/agent-design-language", "#{name} repository mismatch")
  assert(identity.fetch("version") == "v0.91.8", "#{name} version mismatch")
  assert(identity.fetch("title").include?("WP-17"), "#{name} title is not issue-specific")
  assert(identity.fetch("slug") == "v0918-wp17-documentation-release-truth-alignment", "#{name} slug mismatch")
  headings = rendered.lines.map { |line| line[/\A## (.+)\s*\z/, 1] }.compact
  assert(headings == shape_manifest.fetch("cards").fetch(name), "#{name} rendered structure mismatch")
end

vpp = JSON.parse(ISSUE_DIR.join("cards/vpp.values.json").read).dig("content", "values")
expected_lanes = %w[preparation-contract wp16-terminal-gate focused-doc-alignment complete post-merge-exact]
lanes = vpp.fetch("lanes")
assert(lanes.map { |entry| entry.fetch("lane") } == expected_lanes, "PVF lane inventory mismatch")
expected_lane_contracts = {
  "preparation-contract" => ["small", 120, ["AC-1", "AC-3", "AC-6", "AC-8"]],
  "wp16-terminal-gate" => ["small", 120, ["AC-1"]],
  "focused-doc-alignment" => ["medium", 600, ["AC-2", "AC-3", "AC-4", "AC-5", "AC-6", "AC-8"]],
  "complete" => ["large", 900, %w[AC-1 AC-2 AC-3 AC-4 AC-5 AC-6 AC-7 AC-8]],
  "post-merge-exact" => ["large", 900, %w[AC-1 AC-2 AC-3 AC-4 AC-5 AC-6 AC-7 AC-8]]
}.freeze
lanes.each do |entry|
  profile, seconds, acceptance_ids = expected_lane_contracts.fetch(entry.fetch("lane"))
  assert(entry["deterministic"] == true, "lane is not deterministic")
  assert(entry.fetch("resource_profile") == profile, "lane resource profile drift")
  assert(entry.fetch("budget_seconds") == seconds, "lane time budget drift")
  assert(entry.fetch("acceptance_ids") == acceptance_ids, "lane acceptance mapping drift")
  assert(Array(entry["argv"]).any?, "lane has no command")
end
assert(lanes.first["defer_reason"].nil?, "preparation lane cannot be deferred")
assert(lanes.drop(1).all? { |entry| !entry["defer_reason"].to_s.empty? }, "future lane lacks truthful preparation deferral")

text = [PREP.join("design.md").read, PREP.join("diagram.mmd").read, *REQUIRED_CARDS.map { |name| ISSUE_DIR.join("cards/#{name}.md").read }].join("\n")
[
  "#5351", "closed_out", "receipt", "ancestry", "COTS", "PVF",
  "Runtime v2", "2500", "1500", "150", "900", "product changes"
].each do |term|
  assert(text.downcase.include?(term.downcase), "missing contract term #{term}")
end

review = PREP.join("preparation-review.md").read
findings = PREP.join("preparation-review-findings.md").read
assert(review.include?("Result: PASS"), "bounded review did not pass")
assert(review.include?("Actionable findings remaining: 0"), "bounded review has remaining findings")
assert(findings.include?("Open actionable findings: 0"), "finding register is not clean")

integrity_log = ROOT.join(".csdlc/evidence/5360/card-integrity/current-registry-card-integrity.log")
assert(integrity_log.file?, "missing retained typed card-integrity evidence")
integrity = JSON.parse(integrity_log.read)
assert(integrity.fetch("status") == "pass", "typed card-integrity evidence did not pass")
assert(integrity.fetch("issue") == ISSUE && integrity.fetch("cards") == 6, "typed card-integrity evidence identity mismatch")
assert(integrity.fetch("phase") == "initialized" && integrity.fetch("generation") == 1, "typed card-integrity evidence is not tied to the post-approval pre-bind generation")
assert(integrity.fetch("typed_integrity_findings") == 0, "typed card-integrity evidence retained a finding")

_base_out, base_status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", BASE_REVISION, "HEAD")
assert(base_status.success?, "trusted preparation base is not ancestral to HEAD")
committed_or_tracked = git("diff", "--name-only", BASE_REVISION).lines.map(&:strip).reject(&:empty?)
status_out, status = Open3.capture2("git", "-C", ROOT.to_s, "status", "--porcelain")
assert(status.success?, "cannot inspect worktree")
uncommitted = status_out.lines.map { |line| line[3..]&.strip }.compact
changed = (committed_or_tracked + uncommitted).uniq
assert(changed.all? { |path| EXPECTED_PATHS.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") } }, "preparation changed an out-of-scope path")

line_counts = PREP_FILES.to_h do |name|
  [name, PREP.join(name).read.lines.count { |line| !line.strip.empty? }]
end
assert(line_counts.values.all? { |count| count < 500 }, "preparation file exceeds 500 nonblank lines")
assert(line_counts.values.sum <= 1500, "preparation orchestration exceeds 1500 nonblank lines")
assertion_count = Dir.glob(PREP.join("*.rb")).sum { |path| File.read(path).scan(/\bassert\s*\(/).length }
assert(assertion_count < 150, "preparation assertion budget exceeded")

doctor_out, doctor_status = Open3.capture2e(installed_binary("csdlc-doctor"), "--repo", ".", "--issue", ISSUE.to_s, chdir: ROOT.to_s)
assert(doctor_status.success?, "typed doctor failed: #{doctor_out.strip}")
doctor = JSON.parse(doctor_out)
assert(doctor["status"] == "pass" && doctor["phase"] == "bound" && Array(doctor["findings"]).empty?, "typed doctor not clean bound")

puts JSON.generate(status: "pass", issue: ISSUE, phase: index.fetch("phase"), generation: index.fetch("generation"), cards: 6, product_changes: 0, typed_doctor: "pass", review: "pass", lines: line_counts)
