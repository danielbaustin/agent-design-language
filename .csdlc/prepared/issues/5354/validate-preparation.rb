#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE_DIR = ROOT.join(".csdlc/issues/5354")
INDEX = ISSUE_DIR.join("index.json")
PREP = ROOT.join(".csdlc/prepared/issues/5354")
REQUIRED_CARDS = %w[sip stp spp vpp srp sor].freeze
PREP_FILES = %w[design.md diagram.mmd bootstrap-request.json bind-request.json check-dependencies.rb validate-cards.rb validate-preparation.rb run-validation-lane.rb validation-request.json].freeze
EXPECTED_PATHS = [
  ".csdlc/issues/5354",
  ".csdlc/locks/5354.lock",
  ".csdlc/prepared/issues/5354",
  ".csdlc/evidence/5354"
].freeze

def assert(condition, message)
  raise message unless condition
end

def installed_binary(name)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
  assert(status.success?, "cannot resolve shared Git directory")
  common = Pathname.new(out.strip)
  common = ROOT.join(common) unless common.absolute?
  binary = common.parent.join(".adl/bin/csdlc-v2", name)
  assert(binary.file? && binary.executable?, "missing installed typed binary #{name}")
  binary.to_s
end

assert(INDEX.file?, "missing typed index .csdlc/issues/5354/index.json")
index = JSON.parse(INDEX.read)
assert(%w[initialized bound].include?(index.fetch("phase")), "unexpected phase")
assert(index.fetch("issue") == 5354, "wrong issue identity")
claim = index.fetch("claim")
assert(claim.fetch("id") == "claim-5354-v0918-wp15-demo-preparation", "wrong claim")
assert(claim.fetch("protected_paths") == EXPECTED_PATHS, "claim is not exact preparation-only scope")
assert(claim.fetch("purpose").include?("#5384"), "claim purpose omits WP-14A gate")

branch = `git -C #{ROOT} branch --show-current`.strip
top = `git -C #{ROOT} rev-parse --show-toplevel`.strip
common = Pathname.new(`git -C #{ROOT} rev-parse --git-common-dir`.strip)
common = ROOT.join(common) unless common.absolute?
primary = common.parent.to_s
assert(branch == "codex/5354-v0918-preparation", "wrong preparation branch")
assert(branch != "main", "preparation cannot run on main")
assert(top != primary, "preparation cannot run in the primary checkout")
assert(claim.fetch("worktree") == ".", "claim must bind the current dedicated worktree")

PREP_FILES.each { |name| assert(PREP.join(name).file?, "missing #{name}") }
PREP_FILES.each do |name|
  bytes = PREP.join(name).binread
  assert(bytes.end_with?("\n"), "#{name} lacks final newline")
  assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{name} contains trailing whitespace")
end
assert(index.fetch("design_path") == ".csdlc/prepared/issues/5354/design.md", "wrong design path")
assert(index.fetch("diagram_path") == ".csdlc/prepared/issues/5354/diagram.mmd", "wrong diagram path")

cards = index.fetch("cards")
REQUIRED_CARDS.each do |name|
  metadata = cards.fetch(name)
  card = ISSUE_DIR.join("cards/#{name}.md")
  values_path = ISSUE_DIR.join("cards/#{name}.values.json")
  assert(card.file? && values_path.file?, "missing #{name} card/value pair")
  values = JSON.parse(values_path.read)
  assert(values.dig("identity", "issue") == 5354, "wrong #{name} issue")
  assert(values.dig("content", "card_kind") == name, "wrong #{name} kind")
  digests = metadata.values_at("values_digest", "rendered_digest", "ast_digest")
  assert(digests.all? { |digest| digest.match?(/\A[0-9a-f]{64}\z/) }, "invalid #{name} digest")
end

bootstrap = JSON.parse(PREP.join("bootstrap-request.json").read)
initial = bootstrap.fetch("initial")
assert(initial.fetch("acceptance_criteria").length == 8, "acceptance criteria count mismatch")
assert(initial.fetch("operator_constraints").any? { |item| item.include?("current-registry") }, "current-registry rule missing")
assert(initial.fetch("dependencies") == ["WP-14A #5384 merged, typed closed_out, claim-free, backed by a retained merged receipt, and ancestral to the exact #5354 execution revision"], "dependency drift")
assert(initial.fetch("non_goals").any? { |item| item.include?("Runtime v2") }, "Runtime v2 non-goal missing")
assert(initial.fetch("authority_boundary").any? { |item| item.include?("claims no demo") }, "preparation authority boundary missing")

registry = JSON.parse(ROOT.join("docs/templates/prompts/current.json").read)
assert(registry.fetch("status") == "active", "prompt registry is not active")
assert(registry.fetch("lifecycle").map(&:downcase) == REQUIRED_CARDS, "prompt registry lifecycle mismatch")
native = registry.dig("generations", "csdlc_v2_native")
assert(native.is_a?(Hash) && native.fetch("projection_family") == "compact_native", "current registry lacks native v2 projection")
assert(ROOT.join(native.fetch("shape_manifest_path")).file?, "native card shape manifest missing")

vpp = JSON.parse(ISSUE_DIR.join("cards/vpp.values.json").read).dig("content", "values")
expected_lanes = %w[preparation-contract wp14a-terminal-gate integrated-live-demo claim-boundary-matrix complete post-merge-exact]
lanes = vpp.fetch("lanes")
assert(lanes.map { |entry| entry.fetch("lane") } == expected_lanes, "PVF lane inventory mismatch")
assert(lanes.all? { |entry| entry["deterministic"] == true && entry.fetch("budget_seconds").positive? && Array(entry["argv"]).any? }, "invalid PVF metadata")
assert(lanes.first["defer_reason"].nil?, "preparation lane cannot be deferred")

text = [PREP.join("design.md").read, PREP.join("diagram.mmd").read, *REQUIRED_CARDS.map { |name| ISSUE_DIR.join("cards/#{name}.md").read }].join("\n")
%w[#5384 closed_out receipt ancestry ADL Runtime C-SDLC COTS PVF 1500 100 1800].each do |term|
  assert(text.downcase.include?(term.downcase), "missing contract term #{term}")
end

status_out, status = Open3.capture2("git", "-C", ROOT.to_s, "status", "--porcelain")
assert(status.success?, "cannot inspect worktree")
changed = status_out.lines.map { |line| line[3..]&.strip }.compact
assert(changed.all? { |path| EXPECTED_PATHS.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") } }, "preparation changed an out-of-scope path")

line_counts = PREP_FILES.to_h do |name|
  [name, PREP.join(name).read.lines.count { |line| !line.strip.empty? }]
end
assert(line_counts.values.all? { |count| count < 500 }, "preparation file exceeds 500 nonblank lines")
assert(line_counts.values.sum <= 800, "preparation orchestration exceeds 800 nonblank lines")

out, doctor_status = Open3.capture2e(installed_binary("csdlc-doctor"), "--repo", ".", "--issue", "5354", chdir: ROOT.to_s)
assert(doctor_status.success?, "typed doctor failed: #{out.strip}")
doctor = JSON.parse(out)
assert(doctor["status"] == "pass" && Array(doctor["findings"]).empty?, "typed doctor not clean")

validate_out, validate_status = Open3.capture2e(installed_binary("csdlc-validate"), "--request", ".csdlc/prepared/issues/5354/validation-request.json", chdir: ROOT.to_s)
assert(validate_status.success?, "typed PVF validation failed: #{validate_out.strip}")
validation = JSON.parse(validate_out)
assert(validation["disposition"] == "local_pass", "typed PVF did not report local_pass")

puts JSON.generate(status: "pass", issue: 5354, phase: index.fetch("phase"), cards: 6, product_changes: 0, typed_doctor: "pass", lines: line_counts)
