#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = 5360
CARDS = %w[sip stp spp vpp srp sor].freeze

def assert(condition, message)
  raise message unless condition
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  assert(status.success?, "git #{args.join(' ')} failed: #{out.strip}")
  out.strip
end

common = Pathname.new(git("rev-parse", "--git-common-dir"))
common = ROOT.join(common) unless common.absolute?
doctor_binary = common.parent.join(".adl/bin/csdlc-v2/csdlc-doctor")
assert(doctor_binary.file? && doctor_binary.executable?, "missing installed typed doctor")

index = JSON.parse(ROOT.join(".csdlc/issues/#{ISSUE}/index.json").read)
assert(index.fetch("phase") == "initialized", "card-integrity check must run before bind")
assert(index.fetch("generation") == 1, "card-integrity check must run immediately after typed design approval")
design_review = index.fetch("design_review").fetch("approved")
assert(design_review.fetch("reviewer") == "subagent:5360-preparation-review", "design is not typed approved by the bounded reviewer")

registry = JSON.parse(ROOT.join("docs/templates/prompts/current.json").read)
assert(registry.fetch("status") == "active", "prompt registry is not active")
assert(registry.fetch("lifecycle").map(&:downcase) == CARDS, "registry lifecycle mismatch")
native = registry.fetch("generations").fetch("csdlc_v2_native")
shape = JSON.parse(ROOT.join(native.fetch("shape_manifest_path")).read)
assert(shape.fetch("template_set") == native.fetch("template_set"), "native template authority mismatch")

CARDS.each do |name|
  card_path = ROOT.join(".csdlc/issues/#{ISSUE}/cards/#{name}.md")
  values_path = ROOT.join(".csdlc/issues/#{ISSUE}/cards/#{name}.values.json")
  assert(card_path.file? && values_path.file?, "missing #{name} projection pair")
  values = JSON.parse(values_path.read)
  identity = values.fetch("identity")
  assert(identity.fetch("issue") == ISSUE, "#{name} issue mismatch")
  assert(identity.fetch("repository") == "danielbaustin/agent-design-language", "#{name} repository mismatch")
  assert(identity.fetch("template_version") == native.fetch("template_set"), "#{name} template provenance mismatch")
  assert(identity.fetch("slug") == "v0918-wp17-documentation-release-truth-alignment", "#{name} slug mismatch")
  headings = card_path.read.lines.map { |line| line[/\A## (.+)\s*\z/, 1] }.compact
  assert(headings == shape.fetch("cards").fetch(name), "#{name} native shape mismatch")
end

doctor_out, doctor_status = Open3.capture2e(
  doctor_binary.to_s, "--repo", ".", "--issue", ISSUE.to_s, chdir: ROOT.to_s
)
doctor = JSON.parse(doctor_out)
assert(doctor_status.success?, "post-approval typed doctor failed: #{doctor_out.strip}")
assert(doctor.fetch("status") == "pass", "post-approval doctor did not pass")
assert(doctor.fetch("phase") == "initialized" && doctor.fetch("generation") == 1, "post-approval doctor identity mismatch")
codes = doctor.fetch("findings").map { |finding| finding.fetch("code") }
assert(codes.empty?, "typed doctor reported an integrity finding: #{codes.join(',')}")

puts JSON.generate(status: "pass", issue: ISSUE, cards: 6, template_set: native.fetch("template_set"), typed_integrity_findings: 0, phase: doctor.fetch("phase"), generation: doctor.fetch("generation"))
