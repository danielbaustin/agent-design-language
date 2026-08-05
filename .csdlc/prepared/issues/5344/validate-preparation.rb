#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = "5344"
BASE = ROOT.join(".csdlc/issues/#{ISSUE}")
PREP = ROOT.join(".csdlc/prepared/issues/#{ISSUE}")

def require_truth(condition, message)
  raise(message) unless condition
end

index = JSON.parse(BASE.join("index.json").read)
require_truth(%w[initialized bound reviewed].include?(index["phase"]), "unexpected preparation phase")
require_truth(index["terminal"].nil?, "preparation cannot be terminal")
require_truth(index["publication"].nil?, "preparation cannot be published")

cards = %w[sip stp spp vpp srp sor]
cards.each do |card|
  require_truth(BASE.join("cards/#{card}.md").file?, "missing #{card}.md")
  require_truth(BASE.join("cards/#{card}.values.json").file?, "missing #{card}.values.json")
end

text = cards.map { |card| BASE.join("cards/#{card}.md").read }.join("\n")
%w[5350 5361 live\ merged ancestry audit-only receipt rollback selector Runtime\ v2 no-deferral PVF].each do |needle|
  require_truth(text.include?(needle.gsub("\\ ", " ")), "cards omit #{needle}")
end

design = PREP.join("design.md").read
diagram = PREP.join("diagram.mmd").read
%w[5350 5361 live\ merge ancestry audit-only receipt exact prior bytes compare-and-swap #5343 Runtime\ v2].each do |needle|
  require_truth(design.downcase.include?(needle.gsub("\\ ", " ").downcase), "design omits #{needle}")
end
require_truth(diagram.include?("flowchart TD"), "diagram is not Mermaid flowchart")
require_truth(diagram.include?("#5350") && diagram.include?("#5361") && diagram.include?("#5343"), "diagram omits gates")

claim = index.fetch("claim")
paths = claim.fetch("protected_paths")
required_paths = [
  ".csdlc/issues/5344",
  ".csdlc/locks/5344.lock",
  ".csdlc/prepared/issues/5344",
  ".csdlc/evidence/5344",
  "adl-v2/tools/run-soak.sh",
  "adl-v2/tools/prove-rollback.sh",
  "docs/milestones/v0.91.8/evidence/wp12"
]
require_truth((required_paths - paths).empty?, "protected paths incomplete")
forbidden = paths.grep(/runtime_v2|Runtime v2|adl-runtime|generation-selector\.json/)
require_truth(forbidden.empty?, "protected paths trespass Runtime or selector authority: #{forbidden.join(", ")}")

require_truth(text.include?("800") && text.include?("1,200"), "cards omit LoC budgets")
require_truth((text.include?("1,800") || text.include?("1800")) && (text.include?("3,600") || text.include?("3600")), "cards omit time budgets")
require_truth(text.include?("COTS"), "cards omit COTS decision")
require_truth(text.include?("no deferred") || text.include?("No deferred") || text.include?("no-deferral"), "cards omit no-deferral contract")
require_truth(text.include?("audit-only"), "cards do not mark receipts and typed closeout audit-only")

puts(JSON.pretty_generate(status: "pass", issue: 5344, phase: index["phase"], generation: index["generation"], cards: cards.length, protected_paths: paths.length))
