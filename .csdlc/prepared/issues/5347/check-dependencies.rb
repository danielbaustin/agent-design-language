#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"

ROOT = File.expand_path("../../../..", __dir__)
DEPENDENCIES = [5346, 5344, 5343, 5358, 5361].freeze
ORDER_RESOLUTION = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13/dependency-order-resolution.json")
CORE_MANIFEST = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13-core/final-core-deletion-manifest.json")
REPOSITORY = "danielbaustin/agent-design-language"
HEX40 = /\A[0-9a-f]{40}\z/

def fail!(message)
  warn("#5347 dependency gate blocked: #{message}")
  exit(1)
end

def load_json(path, label)
  fail!("missing #{label}: #{path}") unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => e
  fail!("invalid #{label}: #{e.message}")
end

audit_receipts = {}
DEPENDENCIES.each do |issue|
  record = load_json(File.join(ROOT, ".csdlc/issues/#{issue}/index.json"), "typed projection for ##{issue}")
  fail!("##{issue} is not typed closed_out") unless record["phase"] == "closed_out"
  fail!("##{issue} still has an active claim") unless record["claim"].nil?
  terminal = record.fetch("terminal") { fail!("##{issue} projection has no terminal evidence") }
  fail!("##{issue} terminal state is not merged") unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
  sha = terminal["observed_sha"]
  fail!("##{issue} has invalid observed SHA") unless sha.to_s.match?(HEX40)
  ok = Open3.capture2("git", "-C", ROOT, "merge-base", "--is-ancestor", sha, "origin/main").last.success?
  fail!("##{issue} observed SHA is not ancestral to current origin/main") unless ok
  common = Open3.capture2("git", "-C", ROOT, "rev-parse", "--git-common-dir").first.strip
  receipt = File.expand_path("csdlc-v2/closeout/#{issue}.json", File.expand_path(common, ROOT))
  audit_receipts[issue.to_s] = Digest::SHA256.file(receipt).hexdigest if File.file?(receipt)
end

fail!("missing authoritative #5346/#5347 dependency-order resolution") unless File.file?(ORDER_RESOLUTION)
resolution = JSON.parse(File.read(ORDER_RESOLUTION))
fail!("dependency-order schema mismatch") unless resolution["schema"] == "adl.wp13.dependency_order.v1"
fail!("dependency order does not require terminal #5346 before #5347") unless resolution["order"] == [5346, 5347]
fail!("dependency-order resolution is not reviewed") unless resolution["review_status"] == "accepted"
fail!("dependency-order reviewer missing") if resolution["reviewer"].to_s.empty?
fail!("dependency-order revision malformed") unless resolution["revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
fail!("dependency-order revision is not current ancestry") unless Open3.capture2("git", "-C", ROOT, "merge-base", "--is-ancestor", resolution["revision"], "HEAD").last.success?
fail!("missing exact #5346 manifest") unless File.file?(CORE_MANIFEST)
core_sha256 = Digest::SHA256.file(CORE_MANIFEST).hexdigest
fail!("dependency-order #5346 manifest digest mismatch") unless resolution["core_manifest_sha256"] == core_sha256

puts(JSON.generate({schema: "adl.wp13.external_band_dependency_gate.v1", issue: 5347, status: "pass", dependencies: DEPENDENCIES, audit_receipts: audit_receipts}))
