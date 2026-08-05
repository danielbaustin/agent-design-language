#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
CARDS = %w[sip stp spp vpp srp sor].freeze

def assert(condition, message)
  raise message unless condition
end

common_out, common_status = Open3.capture2e("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
assert(common_status.success?, "cannot resolve shared Git directory")
common = Pathname.new(common_out.strip)
common = ROOT.join(common) unless common.absolute?
doctor = common.parent.join(".adl/bin/csdlc-v2/csdlc-doctor")
assert(doctor.file? && doctor.executable?, "missing installed csdlc-doctor")

registry = JSON.parse(ROOT.join("docs/templates/prompts/current.json").read)
assert(registry.fetch("status") == "active", "registry is not active")
assert(registry.fetch("lifecycle").map(&:downcase) == CARDS, "registry lifecycle mismatch")
native = registry.fetch("generations").fetch("csdlc_v2_native")
assert(native.fetch("projection_family") == "compact_native", "native projection mismatch")
assert(ROOT.join(native.fetch("shape_manifest_path")).file?, "native shape manifest missing")
native_template = native.fetch("template_set")

index_path = ROOT.join(".csdlc/issues/5356/index.json")
assert(index_path.file?, "missing typed index")
index = JSON.parse(index_path.read)
assert(index.fetch("cards").keys.sort == CARDS.sort, "typed card inventory mismatch")
CARDS.each do |name|
  card = ROOT.join(".csdlc/issues/5356/cards/#{name}.md")
  values = ROOT.join(".csdlc/issues/5356/cards/#{name}.values.json")
  assert(card.file? && values.file?, "missing #{name} card/value pair")
  parsed = JSON.parse(values.read)
  assert(parsed.dig("identity", "issue") == 5356, "wrong #{name} issue")
  assert(parsed.dig("content", "card_kind") == name, "wrong #{name} kind")
  assert(parsed.dig("identity", "template_version") == native_template,
         "#{name} does not use the registry-selected native template")
end

out, status = Open3.capture2e(doctor.to_s, "--repo", ".", "--issue", "5356", chdir: ROOT.to_s)
assert(status.success?, "typed doctor rejected cards: #{out.strip}")
report = JSON.parse(out)
assert(report["status"] == "pass" && Array(report["findings"]).empty?, "typed doctor not clean")
puts JSON.generate(status: "pass", issue: 5356,
                   registry: registry.fetch("csdlc_prompt_template_set"),
                   native_template: native_template, cards: CARDS.length)
