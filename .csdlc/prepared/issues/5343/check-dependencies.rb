#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = %w[5344 5345].freeze

def fail_closed(message)
  warn(message)
  exit 2
end

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  [out.strip, err.strip, status]
end

head, _, head_status = git("rev-parse", "HEAD")
fail_closed("cannot resolve exact #5343 execution revision") unless head_status.success? && head.match?(/\A[0-9a-f]{40}\z/)

origin_main, _, origin_status = git("rev-parse", "origin/main")
fail_closed("origin/main is not available; fetch before evaluating live merge") unless origin_status.success?

common_dir, common_err, common_status = git("rev-parse", "--git-common-dir")
fail_closed("cannot resolve shared Git directory: #{common_err}") unless common_status.success?
common = Pathname.new(common_dir)
common = ROOT.join(common).cleanpath unless common.absolute?

def dependency_observation(issue, common, head, origin_main)
  record_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  fail_closed("##{issue} tracked lifecycle record is absent") unless record_path.file?
  record = JSON.parse(record_path.read)
  pull_request = record.dig("publication", "pull_request")
  fail_closed("##{issue} tracked publication identity is absent") unless pull_request.is_a?(Integer) && pull_request.positive?

  landing, _, landing_status = git("log", "-1", "--format=%H", "--grep=##{pull_request}", "origin/main")
  fail_closed("##{issue} has no live merged landing commit on origin/main") unless landing_status.success? && landing.match?(/\A[0-9a-f]{40}\z/)

  _, _, origin_ancestor = git("merge-base", "--is-ancestor", landing, origin_main)
  fail_closed("##{issue} landing #{landing} is not ancestral to current origin/main #{origin_main}") unless origin_ancestor.success?

  _, _, execution_ancestor = git("merge-base", "--is-ancestor", landing, head)
  fail_closed("##{issue} landing #{landing} is not ancestral to #5343 execution revision #{head}") unless execution_ancestor.success?

  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  receipt_audit = if receipt_path.file?
    receipt = JSON.parse(receipt_path.read)
    record = receipt["record"] || receipt
    terminal = record["terminal"] || {}
    { present: true, phase: record["phase"], disposition: terminal["disposition"] }
  else
    { present: false }
  end

  { issue: issue.to_i, pull_request: pull_request, landing: landing, receipt_audit: receipt_audit }
rescue JSON::ParserError => e
  fail_closed("##{issue} tracked JSON is malformed: #{e.message}")
end

dependencies = DEPENDENCIES.map { |issue| dependency_observation(issue, common, head, origin_main) }

handoff_candidates = [
  ROOT.join("docs/milestones/v0.91.8/evidence/wp12/cutover-handoff-5344.v1.json"),
  ROOT.join("docs/milestones/v0.91.8/evidence/wp12/soak-rollback-5344.v1.json")
]
handoff_path = handoff_candidates.find(&:file?)
fail_closed("#5344 accepted soak/rollback handoff is absent") unless handoff_path
handoff = JSON.parse(handoff_path.read)

required = %w[status reviewed_revision manifest_digest prior_selector_digest restored_selector_digest fresh_install_receipt rollback_window]
missing = required.reject { |key| handoff.key?(key) }
fail_closed("#5344 handoff omits #{missing.join(', ')}") unless missing.empty?
fail_closed("#5344 handoff is not accepted") unless handoff["status"] == "accepted"
fail_closed("#5344 selector restoration is not exact") unless handoff["prior_selector_digest"] == handoff["restored_selector_digest"]
fail_closed("#5344 handoff contains unresolved rows") unless Array(handoff["unresolved_rows"]).empty?

puts JSON.pretty_generate(
  status: "pass",
  gate: "live_merge_plus_ancestry",
  dependencies: dependencies.map do |dependency|
    {
      issue: dependency.fetch(:issue),
      pull_request: dependency.fetch(:pull_request),
      landing: dependency.fetch(:landing),
      receipt_audit: dependency.fetch(:receipt_audit)
    }
  end,
  handoff: handoff_path.relative_path_from(ROOT).to_s
)
