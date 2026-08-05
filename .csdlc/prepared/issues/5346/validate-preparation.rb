#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE_DIR = ROOT.join(".csdlc/issues/5346")
INDEX = ISSUE_DIR.join("index.json")
PREP = ROOT.join(".csdlc/prepared/issues/5346")
REQUIRED = %w[sip stp spp vpp srp sor].freeze
PREP_FILES = %w[design.md diagram.mmd bootstrap-request.json check-dependencies.rb validate-preparation.rb run-validation-lane.rb write-deletion-evidence.rb reacquire-preparation-claim-20260731.json transition-active-execution-claim.json revoke-lockfile-overclaim.json reacquire-clean-execution-claim.json].freeze
PROHIBITED_PRODUCT_PREFIXES = %w[adl/src adl-v2 adl-runtime adl-runtime-kernel].freeze
EXPECTED_DELETION_PREFIXES = %w[adl/src/cli/tooling_cmd adl/src/cli/tests/pr_cmd_inline].freeze

def assert(condition, message)
  raise message unless condition
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

def installed_binary(name)
  common, status = Open3.capture2e("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
  assert(status.success?, "cannot resolve shared Git directory")
  common = Pathname.new(common.strip)
  common = ROOT.join(common) unless common.absolute?
  binary = common.parent.join(".adl/bin/csdlc-v2", name)
  assert(binary.file? && binary.executable?, "missing installed typed binary #{name}")
  binary.to_s
end

index = JSON.parse(INDEX.read)
assert(index.fetch("phase") == "bound", "unexpected phase")
assert(index.fetch("issue") == 5346 && index.fetch("repository") == "danielbaustin/agent-design-language", "wrong issue identity")
claim = index.fetch("claim")
assert(claim.fetch("id") == "claim-5346-v0918-wp13-deletion-preparation-current", "wrong claim")
assert(claim.fetch("branch") == "codex/5346-v0918-wp13-final-adl-deletion" && claim.fetch("worktree") == ".", "claim is not bound to this checkout")
assert(claim.fetch("purpose").include?("Execute #5346 WP-13") && claim.fetch("purpose").include?("do not touch #5347 external-band paths, Runtime v2, or unrelated lockfiles"), "claim is not execution-scoped")

PREP_FILES.each { |name| assert(PREP.join(name).file?, "missing preparation artifact #{name}") }
assert(index.fetch("design_path") == ".csdlc/prepared/issues/5346/design.md", "wrong design path")
assert(index.fetch("diagram_path") == ".csdlc/prepared/issues/5346/diagram.mmd", "wrong diagram path")

cards = index.fetch("cards")
REQUIRED.each do |name|
  metadata = cards.fetch(name)
  card_path = ISSUE_DIR.join("cards/#{name}.md")
  values_path = ISSUE_DIR.join("cards/#{name}.values.json")
  assert(card_path.file? && values_path.file?, "missing #{name} card/value pair")
  values = JSON.parse(values_path.read)
  assert(values.dig("identity", "issue") == 5346 && values.dig("content", "card_kind") == name, "wrong #{name} identity")
  assert(metadata.values_at("values_digest", "rendered_digest", "ast_digest").all? { |value| value.match?(/\A[0-9a-f]{64}\z/) }, "invalid #{name} typed digest")
end

vpp = JSON.parse(ISSUE_DIR.join("cards/vpp.values.json").read).fetch("content").fetch("values")
lanes = vpp.fetch("lanes")
expected_lanes = %w[preparation-contract terminal-and-manifest-gate eligibility-before-deletion complete-post-deletion post-merge-exact]
assert(lanes.map { |item| item.fetch("lane") } == expected_lanes, "validation lane inventory mismatch")
assert(lanes.all? { |item| item["deterministic"] == true && item.fetch("budget_seconds").positive? && item.fetch("argv").is_a?(Array) && !item["argv"].empty? }, "invalid PVF lane metadata")
assert(lanes.first["defer_reason"].nil?, "preparation proof cannot be deferred")

bootstrap = JSON.parse(PREP.join("bootstrap-request.json").read)
initial = bootstrap.fetch("initial")
assert(initial.fetch("acceptance_criteria").length == 10, "acceptance criteria count mismatch")
assert(initial.fetch("validation_lanes").map { |item| item["lane"] } == expected_lanes, "bootstrap/VPP lane drift")
assert(initial.fetch("invariants").include?("Runtime v2 is categorically outside #5346 ownership and may not be edited or deleted by this issue"), "Runtime v2 prohibition is not categorical")
assert(initial.fetch("non_goals").any? { |item| item.include?("Runtime v2") }, "Runtime v2 non-goal missing")
assert(JSON.parse(ISSUE_DIR.join("cards/stp.values.json").read).dig("content", "values", "dependencies").any? { |item| item.include?("#5352") }, "current STP dependency truth does not include #5352")

text = [PREP.join("design.md").read, PREP.join("diagram.mmd").read, *REQUIRED.map { |name| ISSUE_DIR.join("cards/#{name}.md").read }].join("\n")
%w[#5344 #5343 #5358 #5361 #5347 #5384 #5354 #5352 WP-14A WP-15 WP-21 closed_out receipt ancestry disjoint 80 90].each do |term|
  assert(text.include?(term), "missing preparation term #{term}")
end
%w[Cargo symlink generated rollback retained owner COTS PVF 800 1200 3600].each do |term|
  assert(text.downcase.include?(term.downcase), "missing contract term #{term}")
end

protected = claim.fetch("protected_paths")
assert(protected.uniq == protected, "duplicate protected path")
EXPECTED_DELETION_PREFIXES.each do |path|
  assert(protected.include?(path), "missing execution protected path #{path}")
end
assert(!protected.include?("adl/Cargo.lock"), "lockfile must not remain in #5346 deletion claim")
assert(protected.all? { |path| path.start_with?(".csdlc/issues/5346", ".csdlc/locks/5346", ".csdlc/prepared/issues/5346", ".csdlc/evidence/5346", "docs/milestones/v0.91.8/evidence/wp13/5346-", *EXPECTED_DELETION_PREFIXES) }, "claim includes out-of-scope path")

status_out, status = Open3.capture2("git", "-C", ROOT.to_s, "status", "--porcelain")
assert(status.success?, "cannot inspect worktree status")
changed = status_out.lines.map { |line| line[3..]&.strip }.compact
assert(changed.none? { |path| path.include?("runtime_v2") }, "Runtime v2 path changed")
assert(changed.all? { |path| !PROHIBITED_PRODUCT_PREFIXES.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") } || EXPECTED_DELETION_PREFIXES.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") } }, "changed product path is outside #5346 execution scope")

line_counts = %w[check-dependencies.rb validate-preparation.rb run-validation-lane.rb].to_h { |name| [name, PREP.join(name).read.lines.count { |line| !line.strip.empty? }] }
assert(line_counts.values.all? { |count| count < 500 }, "preparation module exceeds 500 lines")
assert(line_counts.values.sum <= 800, "preparation orchestration exceeds 800 lines")

doctor_out, doctor_status = Open3.capture2e(installed_binary("csdlc-doctor"), "--repo", ROOT.to_s, "--issue", "5346")
assert(doctor_status.success?, "typed card/index/schema authentication failed: #{doctor_out.strip}")
doctor = JSON.parse(doctor_out)
assert(doctor["status"] == "pass" && Array(doctor["findings"]).empty?, "typed doctor did not pass cleanly")

puts JSON.generate(status: "pass", issue: 5346, phase: index.fetch("phase"), cards: REQUIRED.length, typed_doctor: "pass", deletion_prefixes: EXPECTED_DELETION_PREFIXES, preparation_sha256: PREP_FILES.to_h { |name| [name, sha256(PREP.join(name))] }, lines: line_counts)
