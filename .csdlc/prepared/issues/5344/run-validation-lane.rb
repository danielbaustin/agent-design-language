#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }

unless %w[rollback-fault-matrix representative-soak soak-budgets-and-evidence post-merge-exact].include?(lane)
  abort("unsupported WP-12 validation lane: #{lane}")
end

gate = ROOT.join(".csdlc/prepared/issues/5344/check-dependencies.rb")
execution_revision, revision_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
abort("cannot pin exact WP-12 execution revision") unless revision_status.success? && execution_revision.strip.match?(/\A[0-9a-f]{40}\z/)
system("ruby", gate.to_s, execution_revision.strip, chdir: ROOT.to_s) || abort("WP-12 dependencies are not live-merged and ancestral")

soak = ROOT.join("adl-v2/tools/run-soak.sh")
rollback = ROOT.join("adl-v2/tools/prove-rollback.sh")
abort("WP-12 product harnesses do not exist; implementation has not started") unless soak.file? && rollback.file?

command = case lane
          when "rollback-fault-matrix"
            ["bash", rollback.to_s, "--manifest", "docs/milestones/v0.91.8/evidence/wp12/manifest.json"]
          when "representative-soak"
            ["bash", soak.to_s, "--manifest", "docs/milestones/v0.91.8/evidence/wp12/manifest.json"]
          when "soak-budgets-and-evidence"
            ["bash", soak.to_s, "--verify-report", "docs/milestones/v0.91.8/evidence/wp12/report.json"]
          when "post-merge-exact"
            ["bash", soak.to_s, "--post-merge", "docs/milestones/v0.91.8/evidence/wp12/manifest.json"]
          end

exec(*command, chdir: ROOT.to_s)
