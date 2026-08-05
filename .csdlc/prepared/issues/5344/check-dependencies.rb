#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = %w[5350 5361].freeze
BRANCHES = {
  "5350" => "origin/codex/5350-v0918-wp11-shadow-parity",
  "5361" => "origin/codex/5361-v0918-runtime-v3-acceptance-readiness"
}.freeze

def fail_gate(message)
  warn("dependency gate failed: #{message}")
  exit(1)
end

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  [out.strip, err.strip, status]
end

head, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_gate("cannot resolve exact execution revision") unless head_status.success?
execution_revision = (ARGV[0] || head.strip).strip
fail_gate("execution revision is not an exact SHA") unless execution_revision.match?(/\A[0-9a-f]{40}\z/)
fail_gate("checkout moved from exact execution revision #{execution_revision}") unless head.strip == execution_revision

origin_main, _, origin_status = git("rev-parse", "origin/main")
fail_gate("origin/main is not available; fetch before evaluating live merge") unless origin_status.success?

common_dir, common_err, common_status = git("rev-parse", "--git-common-dir")
fail_gate("cannot resolve shared Git directory: #{common_err}") unless common_status.success?
common = Pathname.new(common_dir)
common = ROOT.join(common) unless common.absolute?
primary_root = common.parent
doctor = primary_root.join(".adl/bin/csdlc-v2/csdlc-doctor")

observations = DEPENDENCIES.map do |issue|
  branch = BRANCHES.fetch(issue)
  branch_head, branch_err, branch_status = git("rev-parse", branch)
  fail_gate("##{issue} branch #{branch} is unavailable: #{branch_err}") unless branch_status.success?

  landing, _, landing_status = git("log", "-1", "--format=%H", "--grep=##{issue}", "origin/main")
  fail_gate("##{issue} has no live merged landing commit on origin/main") unless landing_status.success? && landing.match?(/\A[0-9a-f]{40}\z/)

  _, _, origin_ancestor = git("merge-base", "--is-ancestor", landing, origin_main)
  fail_gate("##{issue} landing #{landing} is not ancestral to current origin/main #{origin_main}") unless origin_ancestor.success?

  _, _, execution_ancestor = git("merge-base", "--is-ancestor", landing, execution_revision)
  fail_gate("##{issue} landing #{landing} is not ancestral to execution revision #{execution_revision}") unless execution_ancestor.success?

  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  receipt_audit = if receipt_path.file?
    receipt = JSON.parse(receipt_path.read)
    terminal = receipt["terminal"] || receipt.dig("record", "terminal") || {}
    { present: true, disposition: terminal["disposition"] || terminal["state"] }
  else
    { present: false }
  end

  doctor_audit = if doctor.executable?
    doctor_out, doctor_err, doctor_status = Open3.capture3(
      doctor.to_s,
      "--repo",
      ROOT.to_s,
      "--issue",
      issue,
      chdir: ROOT.to_s
    )
    if doctor_status.success?
      report = JSON.parse(doctor_out)
      { available: true, status: report["status"], phase: report["phase"], findings: report.fetch("findings", []).length }
    else
      { available: true, error: doctor_err.strip }
    end
  else
    { available: false }
  end

  {
    issue: issue.to_i,
    branch: branch,
    branch_head: branch_head,
    landing: landing,
    receipt_audit: receipt_audit,
    typed_doctor_audit: doctor_audit
  }
end

puts(JSON.generate(status: "pass", gate: "live_merge_plus_ancestry", dependencies: observations, execution_revision: execution_revision))
