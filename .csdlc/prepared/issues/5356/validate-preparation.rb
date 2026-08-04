#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE_DIR = ROOT.join(".csdlc/issues/5356")
PREP = ROOT.join(".csdlc/prepared/issues/5356")
CARDS = %w[sip stp spp vpp srp sor].freeze
EXPECTED_PATHS = [
  ".csdlc/evidence/5356",
  ".csdlc/issues/5356",
  ".csdlc/locks/5356.lock",
  ".csdlc/prepared/issues/5356",
  "adl-runtime/src/runtime_api.rs",
  "adl-runtime/tests/runtime_api_wss.rs",
  "docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md",
  "docs/milestones/v0.91.8/README.md",
  "docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md",
  "docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md",
  "docs/milestones/v0.91.8/review/README.md",
  "docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md",
  "docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_5356.md",
  "docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_PLAN_5356.md",
  "docs/reviews/v0.91.8/internal-review-5356"
].freeze
REQUIRED_PREP = %w[
  bootstrap-request.json check-dependencies.rb design.md diagram.mmd
  bind-request.json replace-operator-constraints.json
  review-corpus.json specialist-lanes.json run-validation-lane.rb
  validate-cards.rb validate-preparation.rb validation-request.json
].freeze
SPECIALISTS = %w[code security tests docs architecture evidence].freeze
WP14A_CHILDREN = [5352, 4758, 4759, 4760, 4761, 4762, 4763, 5007, 4739, 4741, 5332, 5107].freeze
CSDLC_DEFECTS = [5540, 5541, 5548, 5558].freeze

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

index = JSON.parse(ISSUE_DIR.join("index.json").read)
assert(index.fetch("issue") == 5356, "wrong issue identity")
allowed_phases = %w[bound implemented reviewed published merge_ready].freeze
assert(allowed_phases.include?(index.fetch("phase")), "unexpected review-execution phase")
claim = index.fetch("claim")
assert(claim.fetch("id") == "claim-5356-v0918-wp18-review-execution-20260802", "wrong claim")
assert(claim.fetch("protected_paths") == EXPECTED_PATHS, "claim is not exact review-execution scope")
assert(claim.fetch("purpose").include?("Execute WP-18 #5356 internal milestone review"), "claim purpose omits WP-18 execution")

branch = `git -C #{ROOT} branch --show-current`.strip
common = Pathname.new(`git -C #{ROOT} rev-parse --git-common-dir`.strip)
common = ROOT.join(common) unless common.absolute?
assert(branch == "codex/5356-v0918-preparation", "wrong preparation branch")
assert(ROOT.to_s != common.parent.to_s, "preparation is in primary checkout")
assert(claim.fetch("worktree") == ".", "claim is not bound to dedicated worktree")

REQUIRED_PREP.each { |name| assert(PREP.join(name).file?, "missing #{name}") }
REQUIRED_PREP.each do |name|
  bytes = PREP.join(name).binread
  assert(bytes.end_with?("\n"), "#{name} lacks final newline")
  assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{name} has trailing whitespace")
end

retained_paths = [ISSUE_DIR, PREP, ROOT.join(".csdlc/evidence/5356")]
retained_paths.each do |root|
  next unless root.exist?
  root.find do |path|
    next unless path.file?
    if path.basename.to_s == "finalize-review-execution-20260802.json"
      request = JSON.parse(path.read)
      assert(request.dig("execution", "root") == ROOT.to_s, "finalize request root does not match current worktree")
      assert(
        request.dig("execution", "evidence_dir") == ROOT.join(".csdlc/evidence/5356").to_s,
        "finalize request evidence_dir does not match issue-owned evidence directory"
      )
      next
    end
    text = path.binread
    assert(!text.match?(%r{/(?:Users|Volumes|home)/}), "host-absolute retained path in #{path.relative_path_from(ROOT)}")
  end
end
assert(index.fetch("design_path") == ".csdlc/prepared/issues/5356/design.md", "wrong design path")
assert(index.fetch("diagram_path") == ".csdlc/prepared/issues/5356/diagram.mmd", "wrong diagram path")

cards = index.fetch("cards")
CARDS.each do |name|
  assert(cards.key?(name), "missing typed #{name} metadata")
  values = JSON.parse(ISSUE_DIR.join("cards/#{name}.values.json").read)
  assert(values.dig("identity", "issue") == 5356, "wrong #{name} issue")
end

corpus = JSON.parse(PREP.join("review-corpus.json").read)
assert(corpus.fetch("product_changes") == 0, "product_changes must be exactly zero")
assert(corpus.dig("owner_groups", "wp14a_children") == WP14A_CHILDREN, "WP-14A child corpus is incomplete")
assert((CSDLC_DEFECTS - corpus.dig("owner_groups", "csdlc_v2")).empty?, "C-SDLC defect corpus is incomplete")
assert(corpus.fetch("canonical_documents").include?("docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md"), "canonical review handoff missing")
corpus.fetch("canonical_documents").each { |path| assert(ROOT.join(path).file?, "missing canonical document #{path}") }

matrix = JSON.parse(PREP.join("specialist-lanes.json").read)
assert(matrix.fetch("lanes").map { |lane| lane.fetch("id") } == SPECIALISTS, "specialist lane matrix mismatch")
assert(matrix.fetch("lanes").all? { |lane| lane["required"] == true }, "all six specialist lanes must be mandatory")
assert(matrix.fetch("finding_severities") == %w[P0 P1 P2 P3], "severity contract mismatch")
assert(matrix.fetch("publication_blockers").include?("missing_lane"), "missing-lane blocker absent")

bootstrap = JSON.parse(PREP.join("bootstrap-request.json").read)
initial = bootstrap.fetch("initial")
assert(initial.fetch("dependencies") == ["WP-17 #5360 merged, typed closed_out, claim-free, backed by a retained merged receipt, and ancestral to the exact #5356 execution revision"], "dependency drift")
assert(initial.fetch("acceptance_criteria").length == 8, "acceptance criteria count mismatch")
assert(initial.fetch("non_goals").any? { |item| item.include?("performing internal review") }, "review-execution non-goal missing")
assert(initial.fetch("non_goals").any? { |item| item.include?("Runtime v2") && item.include?("AWS") }, "forbidden execution boundary missing")

vpp = JSON.parse(ISSUE_DIR.join("cards/vpp.values.json").read).dig("content", "values")
expected_lanes = %w[preparation-contract wp17-terminal-gate specialist-review synthesis-review-quality complete post-merge-exact]
assert(vpp.fetch("lanes").map { |lane| lane.fetch("lane") } == expected_lanes, "PVF lane inventory mismatch")
assert(vpp.fetch("lanes").all? { |lane| lane["deterministic"] == true && lane.fetch("budget_seconds").positive? }, "invalid PVF metadata")
assert(vpp.fetch("lanes").first["defer_reason"].nil?, "preparation lane cannot be deferred")

status_out, status = Open3.capture2("git", "-C", ROOT.to_s, "status", "--porcelain")
assert(status.success?, "cannot inspect worktree")
changed = status_out.lines.map { |line| line[3..]&.strip }.compact
out_of_scope = changed.reject do |path|
  normalized = path.delete_suffix("/")
  EXPECTED_PATHS.any? do |prefix|
    normalized == prefix ||
      normalized.start_with?("#{prefix}/") ||
      prefix.start_with?("#{normalized}/")
  end
end
assert(out_of_scope.empty?, "out-of-scope preparation change: #{out_of_scope.join(', ')}")

authored = REQUIRED_PREP + %w[design-review.md preparation-review-first-pass.md preparation-review-final.md]
counts = authored.select { |name| PREP.join(name).file? }.to_h do |name|
  [name, PREP.join(name).read.lines.count { |line| !line.strip.empty? }]
end
assert(counts.values.all? { |count| count < 500 }, "preparation file exceeds 500 nonblank lines")
assert(counts.values.sum <= 1400, "preparation packet exceeds 1400 nonblank lines")

out, doctor_status = Open3.capture2e(installed_binary("csdlc-doctor"), "--repo", ".", "--issue", "5356", chdir: ROOT.to_s)
assert(doctor_status.success?, "typed doctor failed: #{out.strip}")
doctor = JSON.parse(out)
assert(doctor["status"] == "pass" && Array(doctor["findings"]).empty?, "typed doctor not clean")

puts JSON.generate(status: "pass", issue: 5356, phase: index.fetch("phase"), cards: 6,
                   specialists: SPECIALISTS.length, product_changes: corpus.fetch("product_changes"),
                   typed_doctor: "pass", authored_nonblank_lines: counts.values.sum)
