#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = "5502"
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
%w[#5499 #5498 live merged ancestry audit-only stale forged overlap replan COTS PVF 2,500 600].each do |term|
  require_truth(text.downcase.include?(term.downcase), "cards omit #{term}")
end

design = PREP.join("design.md").read
diagram = PREP.join("diagram.mmd").read
%w[#5499 #5498 live merged audit-only convergence replan serde serde_json blake3 thiserror serialized].each do |term|
  require_truth(design.downcase.include?(term.downcase), "design omits #{term}")
end
require_truth(diagram.include?("flowchart TD"), "diagram is not a Mermaid flowchart")
require_truth(diagram.include?("#5499") && diagram.include?("#5498") && diagram.include?("#5501"), "diagram omits gates")

claim = index.fetch("claim")
expected = [
  ".csdlc/issues/5502",
  ".csdlc/locks/5502.lock",
  ".csdlc/prepared/issues/5502",
  ".csdlc/evidence/5502"
]
require_truth(claim.fetch("protected_paths").sort == expected.sort, "protected paths are not exact")
forbidden = claim.fetch("protected_paths").grep(/adl-v2\/crates|runtime_v2|adl-runtime/)
require_truth(forbidden.empty?, "claim grants product authority")

_out, _err, status = Open3.capture3("ruby", PREP.join("check-dependencies.rb").to_s, chdir: ROOT.to_s)
require_truth([0, 2].include?(status.exitstatus), "dependency gate returned an unexpected status")

base = PREP.join("preparation-base.txt").read.strip
require_truth(base.match?(/\A[0-9a-f]{40}\z/), "preparation base is not an exact commit")
_out, _err, base_status = Open3.capture3("git", "cat-file", "-e", "#{base}^{commit}", chdir: ROOT.to_s)
require_truth(base_status.success?, "preparation base commit is unavailable")
_out, _err, ancestor_status = Open3.capture3("git", "merge-base", "--is-ancestor", base, "HEAD", chdir: ROOT.to_s)
require_truth(ancestor_status.success?, "preparation base is not an ancestor of HEAD")
changed = Open3.capture3("git", "diff", "--name-only", "#{base}..HEAD", chdir: ROOT.to_s).first.lines.map(&:strip)
changed += Open3.capture3("git", "ls-files", "--others", "--exclude-standard", chdir: ROOT.to_s).first.lines.map(&:strip)
allowed = [".csdlc/issues/5502/", ".csdlc/prepared/issues/5502/", ".csdlc/evidence/5502/"]
bad = changed.uniq.reject { |path| allowed.any? { |prefix| path.start_with?(prefix) } || path == ".csdlc/locks/5502.lock" }
require_truth(bad.empty?, "out-of-scope changes present: #{bad.join(', ')}")

puts JSON.pretty_generate(status: "pass", issue: 5502, phase: index["phase"], cards: cards.length,
                          protected_paths: expected.length, product_changes: 0)
