#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCY = 5351
MERGE_SHA = "2e9d2dd7c4260dcf6ec6af954b0eea97554212df"
QUALITY_GATE = ROOT.join("docs/milestones/v0.91.8/evidence/wp16/QUALITY_GATE.md")

def fail_gate(message)
  warn("#5360 WP-16 gate: #{message}")
  exit 1
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

begin
  head = git("rev-parse", "HEAD")
  _out, ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", MERGE_SHA, head)
  fail_gate("##{DEPENDENCY} merge #{MERGE_SHA} is not ancestral to #{head}") unless ancestry.success?
  fail_gate("missing retained WP-16 quality gate") unless QUALITY_GATE.file?
  fail_gate("WP-16 quality gate does not report pass") unless QUALITY_GATE.read.match?(/^Status: `pass`$/)

  puts JSON.generate(
    status: "pass",
    issue: 5360,
    dependency: DEPENDENCY,
    dependency_sha: MERGE_SHA,
    quality_gate: QUALITY_GATE.relative_path_from(ROOT).to_s,
    revision: head
  )
end
