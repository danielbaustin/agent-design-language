#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = "5501"
BASE = ROOT.join(".csdlc/issues/#{ISSUE}")
PREP = ROOT.join(".csdlc/prepared/issues/#{ISSUE}")

def require_truth(condition, message)
  raise(message) unless condition
end

index = JSON.parse(BASE.join("index.json").read)
require_truth(%w[initialized bound reviewed].include?(index["phase"]), "unexpected preparation phase")
require_truth(index["terminal"].nil?, "preparation cannot be terminal")
require_truth(index["publication"].nil?, "preparation cannot be published")
design_review = index["design_review"]
design_approved = design_review == "approved" || (design_review.is_a?(Hash) && design_review.key?("approved"))
if index["phase"] == "initialized"
  require_truth(design_review == "pending" || design_approved, "initialized preparation has invalid design-review state")
else
  require_truth(design_approved, "typed design approval is required before bound preparation proof")
end

cards = %w[sip stp spp vpp srp sor]
cards.each do |card|
  require_truth(BASE.join("cards/#{card}.md").file?, "missing #{card}.md")
  require_truth(BASE.join("cards/#{card}.values.json").file?, "missing #{card}.values.json")
end

text = cards.map { |card| BASE.join("cards/#{card}.md").read }.join("\n")
required = %w[#5349 #5499 #5498 #5500 #5502 live merged ancestry audit-only real writable shard context dashboard converge baseline COTS PVF 2,500 3,600]
required.each { |term| require_truth(text.downcase.include?(term.downcase), "cards omit #{term}") }
%w[fixture-only prose-only screenshot-only library-only].each do |term|
  require_truth(text.downcase.include?(term), "cards omit non-proof boundary #{term}")
end

design = PREP.join("design.md").read
diagram = PREP.join("diagram.mmd").read
%w[#5349 #5499 #5498 #5500 #5502 live merged ancestry real codex claim worktree live-run-manifest context dashboard convergence baseline serialized].each do |term|
  require_truth(design.downcase.include?(term.downcase), "design omits #{term}")
end
require_truth(diagram.include?("flowchart TD"), "diagram is not a Mermaid flowchart")
%w[#5349 #5499 #5498 #5500 #5502 #5497 #5361].each do |term|
  require_truth(diagram.include?(term), "diagram omits #{term}")
end

template = PREP.join("live-run-manifest.template.json")
manifest_validator = PREP.join("validate-live-run-manifest.rb")
require_truth(template.file?, "live-run manifest template is absent")
require_truth(manifest_validator.file?, "live-run manifest validator is absent")
_out, _err, template_status = Open3.capture3("ruby", manifest_validator.to_s, template.to_s, chdir: ROOT.to_s)
require_truth(template_status.exitstatus == 2, "empty live-run template must fail closed")

claim = index.fetch("claim")
expected = [
  ".csdlc/issues/5501",
  ".csdlc/locks/5501.lock",
  ".csdlc/prepared/issues/5501",
  ".csdlc/evidence/5501"
]
require_truth(claim.fetch("protected_paths").sort == expected.sort, "protected paths are not exact")
forbidden = claim.fetch("protected_paths").grep(/adl-v2\/crates|runtime_v2|adl-runtime|csdlc-v2\/src/)
require_truth(forbidden.empty?, "claim grants product authority")

_out, _err, dependency_status = Open3.capture3("ruby", PREP.join("check-dependencies.rb").to_s, chdir: ROOT.to_s)
require_truth([0, 2].include?(dependency_status.exitstatus), "dependency gate returned an unexpected status")

base = PREP.join("preparation-base.txt").read.strip
require_truth(base.match?(/\A[0-9a-f]{40}\z/), "preparation base is not an exact commit")
_out, _err, base_status = Open3.capture3("git", "cat-file", "-e", "#{base}^{commit}", chdir: ROOT.to_s)
require_truth(base_status.success?, "preparation base commit is unavailable")
_out, _err, ancestor_status = Open3.capture3("git", "merge-base", "--is-ancestor", base, "HEAD", chdir: ROOT.to_s)
require_truth(ancestor_status.success?, "preparation base is not an ancestor of HEAD")
changed = Open3.capture3("git", "diff", "--name-only", "#{base}..HEAD", chdir: ROOT.to_s).first.lines.map(&:strip)
changed += Open3.capture3("git", "ls-files", "--others", "--exclude-standard", chdir: ROOT.to_s).first.lines.map(&:strip)
allowed = [".csdlc/issues/5501/", ".csdlc/prepared/issues/5501/", ".csdlc/evidence/5501/"]
bad = changed.uniq.reject { |path| allowed.any? { |prefix| path.start_with?(prefix) } || path == ".csdlc/locks/5501.lock" }
require_truth(bad.empty?, "out-of-scope changes present: #{bad.join(', ')}")

puts JSON.pretty_generate(
  status: "pass",
  issue: 5501,
  phase: index["phase"],
  cards: cards.length,
  dependencies: 5,
  minimum_real_writable_shards: 2,
  protected_paths: expected.length,
  product_changes: 0
)
