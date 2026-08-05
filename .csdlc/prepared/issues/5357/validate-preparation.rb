#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = 5357
BASE_REVISION = "e51768d63"
ISSUE_DIR = ROOT.join(".csdlc/issues/5357")
PREP = ROOT.join(".csdlc/prepared/issues/5357")
CARDS = %w[sip stp spp vpp srp sor].freeze
PATHS = [
  ".csdlc/evidence/5357",
  ".csdlc/issues/5357",
  ".csdlc/locks/5357.lock",
  ".csdlc/prepared/issues/5357",
  "CHANGELOG.md",
  "REVIEW.md",
  "docs/README.md",
  "docs/milestones/v0.91.8/MILESTONE_CHECKLIST_v0.91.8.md",
  "docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md",
  "docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md",
  "docs/milestones/v0.91.8/README.md",
  "docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md",
  "docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md",
  "docs/milestones/v0.91.8/WBS_v0.91.8.md",
  "docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md",
  "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml",
  "docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md",
  "docs/milestones/v0.91.8/review/README.md",
  "docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md",
  "docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_5356.md"
].freeze
MERGED_TRANSFER_PATHS = %w[
  README.md
  docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md
  docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
].freeze
FILES = %w[
  design.md diagram.mmd bootstrap-request.json bind-request.json approve-design-request.json
  amend-canonical-review-docs-scope.json
  amend-post-5791-canonical-paths.json
  replace-sip-constraints-for-final-review-gate.json
  replace-stp-dependencies-for-second-pass.json
  replace-stp-acceptance-for-canonical-sweep.json
  replace-stp-deliverables-for-review-sweep.json
  replace-spp-invariants-for-doc-readiness.json
  replace-spp-steps-for-doc-readiness.json
  replace-spp-stop-conditions-for-doc-readiness.json
  replace-vpp-for-final-review-gate.json
  check-dependencies.rb validate-preparation.rb run-validation-lane.rb
  validate-card-integrity.rb card-integrity-request.json validation-request.json execution-validation-request.json
  corpus-manifest.template.json dispatch-receipt.template.json dispatch-receipt.schema.json review-output.schema.json
  post-merge.schema.json post-merge.template.json
  preparation-review.md preparation-review-findings.md
].freeze

def assert(condition, message)
  raise message unless condition
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  assert(status.success?, "git #{args.join(' ')} failed: #{out.strip}")
  out.strip
end

def binary(name)
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  path = common.parent.join(".adl/bin/csdlc-v2", name)
  path = ROOT.join("csdlc-v2/target/debug", name) unless path.file? && path.executable?
  assert(path.file? && path.executable?, "missing typed binary #{name}")
  path.to_s
end

def validate_local_links(path)
  path.read.scan(/!?\[[^\]]*\]\(([^)]+)\)/).flatten.each do |raw|
    target = raw.strip.sub(/\A</, "").sub(/>\z/, "").split(/\s+[\"']/, 2).first
    next if target.empty? || target.start_with?("#", "http://", "https://", "mailto:", "data:")

    relative = target.split("#", 2).first
    next if relative.empty?

    resolved = path.dirname.join(relative).cleanpath
    assert(resolved.exist?, "broken local link in #{path.relative_path_from(ROOT)}: #{target}")
  end
end

index = JSON.parse(ISSUE_DIR.join("index.json").read)
assert(index.fetch("issue") == ISSUE, "wrong issue")
assert(index.fetch("phase") == "bound", "full preparation validation runs only after bind")
assert(index.fetch("design_review").fetch("approved").fetch("reviewer") == "subagent:5357-preparation-review", "design review is not approved")
claim = index.fetch("claim")
assert(claim.fetch("id") == "claim-5357-v0918-wp19-review-readiness", "wrong claim")
assert(claim.fetch("protected_paths") == PATHS, "claim is not exact preparation scope")
assert(claim.fetch("purpose").include?("exact-revision external review"), "claim omits review-readiness purpose")
assert(git("branch", "--show-current") == "codex/5357-v0918-preparation", "wrong branch")
assert(git("branch", "--show-current") != "main", "cannot prepare on main")

FILES.each { |name| assert(PREP.join(name).file?, "missing #{name}") }
FILES.each do |name|
  bytes = PREP.join(name).binread
  assert(bytes.end_with?("\n"), "#{name} lacks final newline")
  assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{name} has trailing whitespace")
end

registry = JSON.parse(ROOT.join("docs/templates/prompts/current.json").read)
native = registry.fetch("generations").fetch("csdlc_v2_native")
shape = JSON.parse(ROOT.join(native.fetch("shape_manifest_path")).read)
assert(registry.fetch("status") == "active" && registry.fetch("lifecycle").map(&:downcase) == CARDS, "registry mismatch")
CARDS.each do |name|
  card = ISSUE_DIR.join("cards/#{name}.md")
  values = JSON.parse(ISSUE_DIR.join("cards/#{name}.values.json").read)
  identity = values.fetch("identity")
  assert(card.file? && identity.fetch("issue") == ISSUE, "#{name} identity mismatch")
  assert(identity.fetch("slug") == "v0918-wp19-independent-external-review", "#{name} slug mismatch")
  assert(identity.fetch("template_version") == native.fetch("template_set"), "#{name} template mismatch")
  headings = card.read.lines.map { |line| line[/\A## (.+)\s*\z/, 1] }.compact
  assert(headings == shape.fetch("cards").fetch(name), "#{name} shape mismatch")
end

bootstrap = JSON.parse(PREP.join("bootstrap-request.json").read).fetch("initial")
assert(bootstrap.fetch("acceptance_criteria").length == 8, "acceptance criteria drift")
assert(bootstrap.fetch("non_goals").any? { |v| v.include?("Runtime v2") }, "Runtime v2 boundary missing")
stp = JSON.parse(ISSUE_DIR.join("cards/stp.values.json").read).dig("content", "values")
assert(stp.fetch("dependencies").any? { |value| value.include?("#5791") }, "current final-review dependency missing")

handoff = "docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md"
assert(ROOT.join(handoff).file?, "canonical handoff missing")
handoff_text = ROOT.join(handoff).read
assert(handoff_text.include?("Packet status: `ready_to_freeze_not_sent`"), "canonical handoff is not ready to freeze")
assert(handoff_text.include?("Review performed: false"), "canonical handoff overclaims review")
assert(handoff_text.include?("`#5791`"), "canonical handoff omits the final internal review gate")
%w[
  docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md
  docs/milestones/v0.91.8/baseline_and_ownership_v0.91.8.json
  docs/milestones/v0.91.8/RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md
  docs/milestones/v0.91.8/runtime_v3_functional_parity_plan_v0.91.8.json
  docs/milestones/v0.91.8/features/AI_AGENT_PODCAST_STUDIO_v0.91.8.md
  docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_PLAN_5356.md
  docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_5356.md
].each { |path| assert(handoff_text.include?(File.basename(path)), "handoff manifest omits #{path}") }

corpus_paths = git("ls-files", "docs/milestones/v0.91.8").lines.map(&:strip).select { |path| path.match?(/\.(?:md|ya?ml|json)\z/) }
assert(corpus_paths.length >= 40, "canonical v0.91.8 document corpus is unexpectedly small")
readiness = JSON.parse(ROOT.join(".csdlc/evidence/5357/documentation-readiness.v1.json").read)
assert(readiness.dig("current_corpus", "tracked_markdown_yaml_json") == corpus_paths.length, "documentation-readiness corpus count drift")
assert(readiness.fetch("freeze_ready") == true && readiness.fetch("external_review_dispatched") == false && readiness.fetch("release_approved") == false, "documentation-readiness evidence overclaims")
assert(readiness.dig("pre_freeze_review", "result") == "pass" && readiness.dig("pre_freeze_review", "blockers") == 0, "pre-freeze documentation review is not clean")
assert(ROOT.join(readiness.dig("pre_freeze_review", "evidence")).file?, "pre-freeze documentation review evidence missing")
wp17 = JSON.parse(ROOT.join(readiness.fetch("source_wp17_evidence")).read)
wp17_paths = (wp17.fetch("updated_paths") + wp17.fetch("verified_no_edit") + wp17.fetch("delegated_collisions").map { |entry| entry.fetch("path") }).uniq
assert(wp17.fetch("summary") == readiness.fetch("source_wp17_summary"), "WP-17 summary drift")
classified = (readiness.fetch("updated_for_wp19") + readiness.fetch("current_main_verified_without_edit") + readiness.fetch("integrate_from_final_internal_review")).uniq
assert((wp17_paths - classified).empty?, "WP-17 listed path lacks current disposition: #{(wp17_paths - classified).join(', ')}")
wp17_paths.each do |relative|
  path = ROOT.join(relative)
  assert(path.file?, "WP-17 listed path is missing: #{relative}")
  bytes = path.binread
  assert(bytes.end_with?("\n"), "#{relative} lacks final newline")
  assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{relative} has trailing whitespace")
  JSON.parse(bytes) if relative.end_with?(".json")
  YAML.safe_load(bytes, aliases: true) if relative.match?(/\.ya?ml\z/)
  validate_local_links(path) if relative.end_with?(".md")
end
assert(Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", "1b1ba9990bee81cf74ea449f09c52373aeb7e16c", "HEAD").last.success?, "merged #5791 source is not ancestral")
corpus_paths.each do |relative|
  path = ROOT.join(relative)
  assert(path.file?, "tracked canonical document is missing: #{relative}")
  bytes = path.binread
  unless relative.include?("/evidence/")
    assert(bytes.end_with?("\n"), "#{relative} lacks final newline")
    assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{relative} has trailing whitespace")
  end
  JSON.parse(bytes) if relative.end_with?(".json")
  YAML.safe_load(bytes, aliases: true) if relative.match?(/\.ya?ml\z/)
  validate_local_links(path) if relative.end_with?(".md")
end
corpus = JSON.parse(PREP.join("corpus-manifest.template.json").read)
assert(corpus.fetch("status") == "not_generated" && corpus.fetch("canonical_handoff") == handoff, "corpus template is not fail-closed")
receipt = JSON.parse(PREP.join("dispatch-receipt.template.json").read)
%w[base_branch head_branch base_sha head_sha dispatch_started_at dispatch_completed_at provider_outcome degradation_state attempts_sha256].each { |key| assert(receipt.key?(key), "receipt missing #{key}") }
%w[corpus_selector_identity prompt_author_identity prompt_selector_identity reviewer_identity provider_selector_identity process_owner_identity funder_identity retry_controller_identity].each { |key| assert(receipt.key?(key), "receipt missing identity #{key}") }
assert(receipt.fetch("status") == "not_dispatched" && receipt.fetch("review_performed") == false, "receipt overclaims dispatch")
receipt_schema = JSON.parse(PREP.join("dispatch-receipt.schema.json").read)
assert(receipt_schema.fetch("$id") == "adl.v0918.external_review_dispatch.v1", "receipt schema identity mismatch")
schema = JSON.parse(PREP.join("review-output.schema.json").read)
required = schema.fetch("properties").fetch("findings").fetch("items").fetch("required")
%w[observed_evidence inference open_author_decision].each { |key| assert(required.include?(key), "review schema missing #{key}") }
evidence_required = schema.dig("properties", "findings", "items", "properties", "observed_evidence", "items", "required")
assert(evidence_required == %w[path line_start line_end excerpt_sha256 statement], "review schema lacks exact file/line evidence")

lanes = JSON.parse(ISSUE_DIR.join("cards/vpp.values.json").read).dig("content", "values", "lanes")
expected = %w[preparation-contract wp18-terminal-gate corpus-dispatch-preflight review-output-contract complete post-merge-exact]
assert(lanes.map { |lane| lane.fetch("lane") } == expected, "PVF lane inventory mismatch")
expected_seconds = [120, 120, 300, 300, 900, 900]
assert(lanes.map { |lane| lane.fetch("budget_seconds") } == expected_seconds, "PVF budgets mismatch")
assert(lanes.first["defer_reason"].nil? && lanes.drop(1).all? { |lane| !lane.fetch("defer_reason").empty? }, "PVF deferral truth mismatch")
request = JSON.parse(PREP.join("execution-validation-request.json").read)
request_lanes = request.dig("manifest", "lanes")
assert(request_lanes.map { |lane| lane.fetch("id") } == expected, "typed PVF manifest does not declare all VPP lanes")
lanes.zip(request_lanes).each do |vpp, typed|
  assert(typed.fetch("timeout_seconds") == vpp.fetch("budget_seconds") && typed.dig("resources", "tokens") == vpp.fetch("budget_tokens"), "#{vpp['lane']} time/token budget drift")
  assert([typed.fetch("executable"), *typed.fetch("argv")] == vpp.fetch("argv"), "#{vpp['lane']} argv drift")
  assert(typed.fetch("parallel_group") == vpp.fetch("parallel_group"), "#{vpp['lane']} parallel group drift")
  assert(typed.fetch("proof_role") == vpp.fetch("proof_role"), "#{vpp['lane']} proof role drift")
  assert(typed.fetch("determinism") == (vpp.fetch("deterministic") ? "deterministic" : "nondeterministic"), "#{vpp['lane']} determinism drift")
  assert(typed.fetch("release_gate") == "required" && typed.fetch("network") == "denied" && typed.fetch("credentials") == [], "#{vpp['lane']} violates required offline policy")
end
assert(request.dig("selection", "requested_lanes") == ["preparation-contract"], "execution manifest must retain preparation entrypoint")
preparation_request = JSON.parse(PREP.join("validation-request.json").read)
assert(preparation_request.dig("manifest", "lanes").map { |lane| lane.fetch("id") } == ["preparation-contract"], "preparation request is not bounded")
assert(preparation_request.dig("selection", "requested_lanes") == ["preparation-contract"], "preparation selection drift")

review = PREP.join("preparation-review.md").read
findings = PREP.join("preparation-review-findings.md").read
assert(review.include?("Result: PASS") && review.include?("Actionable findings remaining: 0"), "preparation review not clean")
assert(findings.include?("Open actionable findings: 0"), "finding dispositions incomplete")

integrity = JSON.parse(ROOT.join(".csdlc/evidence/5357/card-integrity/current-registry-card-integrity.log").read)
assert(integrity.fetch("status") == "pass" && integrity.fetch("phase") == "initialized" && integrity.fetch("generation") == 1, "card integrity evidence mismatch")

assert(Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", BASE_REVISION, "HEAD").last.success?, "base is not ancestral")
changed = git("diff", "--name-only", "origin/main...HEAD").lines.map(&:strip)
status_out, status = Open3.capture2("git", "-C", ROOT.to_s, "status", "--porcelain")
assert(status.success?, "cannot inspect status")
changed = (changed + status_out.lines.map { |line| line[3..]&.strip }).compact.uniq
allowed_paths = PATHS + MERGED_TRANSFER_PATHS
assert(changed.all? { |path| allowed_paths.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") } }, "out-of-scope readiness change")

counts = FILES.to_h { |name| [name, PREP.join(name).read.lines.count { |line| !line.strip.empty? }] }
assert(counts.values.all? { |count| count < 500 } && counts.values.sum <= 1800, "preparation LoC budget exceeded")
assert(Dir.glob(PREP.join("*.rb")).sum { |path| File.read(path).scan(/\bassert\s*\(/).length } < 220, "assertion budget exceeded")

doctor_out, doctor_status = Open3.capture2e(binary("csdlc-doctor"), "--repo", ".", "--issue", ISSUE.to_s, chdir: ROOT.to_s)
doctor = JSON.parse(doctor_out)
assert(doctor_status.success? && doctor.fetch("status") == "pass" && doctor.fetch("phase") == "bound" && doctor.fetch("findings").empty?, "typed doctor not clean")
puts JSON.generate(status: "pass", issue: ISSUE, phase: "bound", generation: index.fetch("generation"), cards: 6, product_changes: 0, review: "pass", lines: counts.values.sum)
