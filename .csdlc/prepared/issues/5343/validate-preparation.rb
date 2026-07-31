#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = "5343"
BASE = ROOT.join(".csdlc/issues/#{ISSUE}")
PREP = ROOT.join(".csdlc/prepared/issues/#{ISSUE}")

def require_truth(condition, message)
  raise(message) unless condition
end

index = JSON.parse(BASE.join("index.json").read)
require_truth(%w[initialized bound implemented reviewed].include?(index["phase"]), "unexpected pre-publication phase")
require_truth(index["terminal"].nil?, "pre-publication issue cannot be terminal")
require_truth(index["publication"].nil?, "pre-publication issue cannot be published")

cards = %w[sip stp spp vpp srp sor]
cards.each do |card|
  require_truth(BASE.join("cards/#{card}.md").file?, "missing #{card}.md")
  require_truth(BASE.join("cards/#{card}.values.json").file?, "missing #{card}.values.json")
end

text = cards.map { |card| BASE.join("cards/#{card}.md").read }.join("\n")
required_terms = [
  "#5344", "live merged", "audit-only", "receipt", "ancestr", "fresh-install",
  "explicit v1", "rollback window", "compare-and-swap", "Runtime v2", "COTS",
  "PVF", "no deferred", "500", "800", "1200"
]
required_terms.each do |term|
  require_truth(text.downcase.include?(term.downcase), "cards omit #{term}")
end

design = PREP.join("design.md").read
diagram = PREP.join("diagram.mmd").read
%w[#5344 live\ merge audit-only receipt ancestry fresh-install explicit\ v1 rollback\ window compare-and-swap WP-13 Runtime\ v2].each do |term|
  require_truth(design.downcase.include?(term.gsub("\\ ", " ").downcase), "design omits #{term}")
end
require_truth(diagram.include?("flowchart TD"), "diagram is not a Mermaid flowchart")
require_truth(diagram.include?("#5344") && diagram.include?("#5345") && diagram.include?("WP-13"), "diagram omits key gates")

spp = JSON.parse(BASE.join("cards/spp.values.json").read).dig("content", "values")
vpp = JSON.parse(BASE.join("cards/vpp.values.json").read).dig("content", "values")
design_review = index.fetch("design_review")
approved_revision = design_review.dig("approved", "revision")
require_truth(approved_revision.is_a?(String) && !approved_revision.empty?, "design is not approved at an exact digest")
require_truth(spp["design_digest"] == approved_revision, "SPP design digest differs from approved design")
require_truth(vpp["design_digest"] == approved_revision, "VPP design digest differs from approved design")
require_truth(spp["diagram_digest"].is_a?(String) && !spp["diagram_digest"].empty?, "SPP diagram digest is absent")
require_truth(vpp["diagram_digest"] == spp["diagram_digest"], "SPP and VPP diagram digests differ")

claim = index.fetch("claim")
paths = claim.fetch("protected_paths")
required_paths = [
  ".csdlc/issues/5343",
  ".csdlc/locks/5343.lock",
  ".csdlc/prepared/issues/5343",
  ".csdlc/evidence/5343",
  "docs/milestones/v0.91.8/evidence/wp12/cutover-5343"
]
require_truth((required_paths - paths).empty?, "protected paths incomplete")
forbidden = paths.grep(/runtime_v2|adl-runtime|adl-v2\/crates|install-adl-v2|generation-selector/)
require_truth(forbidden.empty?, "protected paths trespass product authority: #{forbidden.join(', ')}")

request = JSON.parse(PREP.join("bootstrap-request.json").read)
constraints = request.dig("initial", "operator_constraints").join("\n")
require_truth(constraints.include?("Preparation only"), "preparation-only boundary absent")
require_truth(constraints.include?("no selector transaction"), "selector execution prohibition absent")
if index["phase"] == "implemented"
  require_truth(claim.fetch("purpose").include?("reversible ADL default switch"), "execution claim purpose is stale")
end

puts JSON.pretty_generate(
  status: "pass",
  issue: 5343,
  phase: index["phase"],
  generation: index["generation"],
  cards: cards.length,
  protected_paths: paths.length,
  dependency_gate: "accepted_live_merge_ancestry_and_handoff"
)
