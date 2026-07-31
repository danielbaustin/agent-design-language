#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = {
  "5499" => "origin/codex/5499-v0918-wp10a-conductor",
  "5498" => "origin/codex/5498-v0918-wp10a-task-context-adapter"
}.freeze

def fail_closed(message)
  warn(message)
  exit 2
end

def git(*args)
  out, err, status = Open3.capture3("git", "-C", ROOT.to_s, *args)
  [out, err, status]
end

def rev(ref)
  out, _err, status = git("rev-parse", "--verify", "#{ref}^{commit}")
  status.success? ? out.strip : nil
end

def ancestor?(sha, target)
  _out, _err, status = git("merge-base", "--is-ancestor", sha, target)
  status.success?
end

def matching_main_commit(issue)
  out, _err, status = git(
    "log",
    "--format=%H",
    "--max-count=1",
    "--regexp-ignore-case",
    "--grep=(##{issue}|#{issue})",
    "origin/main"
  )
  status.success? && !out.strip.empty? ? out.lines.first.strip : nil
end

common_dir, status = Open3.capture2("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
fail_closed("cannot resolve shared Git directory") unless status.success?
common = Pathname.new(common_dir.strip)
common = ROOT.join(common).cleanpath unless common.absolute?

main = rev("origin/main")
fail_closed("origin/main is unavailable; fetch before dependency validation") unless main

results = []

DEPENDENCIES.each do |issue, branch|
  branch_sha = rev(branch)
  fail_closed("##{issue} dependency branch is unavailable: #{branch}") unless branch_sha

  live_sha = if ancestor?(branch_sha, "origin/main")
               branch_sha
             else
               matching_main_commit(issue)
             end
  fail_closed("##{issue} has no live merged revision on origin/main") unless live_sha
  fail_closed("##{issue} live merged revision is not ancestral to #5502") unless ancestor?(live_sha, "HEAD")

  audit = {
    issue: issue.to_i,
    branch: branch,
    branch_sha: branch_sha,
    live_merge_sha: live_sha,
    branch_tip_ancestral_to_origin_main: ancestor?(branch_sha, "origin/main")
  }

  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  if receipt_path.file?
    receipt = JSON.parse(receipt_path.read)
    record = receipt["record"] || receipt
    terminal = record["terminal"] || {}
    audit[:typed_closeout_phase] = record["phase"]
    audit[:typed_closeout_disposition] = terminal["disposition"]
    audit[:typed_closeout_observed_sha] =
      terminal["observed_sha"] || record["observed_sha"] || record["merge_sha"]
  else
    audit[:typed_closeout_phase] = "receipt_absent"
  end

  index_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  if index_path.file?
    index = JSON.parse(index_path.read)
    audit[:typed_projection_phase] = index["phase"]
    audit[:typed_projection_claim_active] = !index["claim"].nil?
  else
    audit[:typed_projection_phase] = "projection_absent"
  end

  results << audit
rescue JSON::ParserError => e
  fail_closed("##{issue} audit evidence is malformed: #{e.message}")
end

puts JSON.pretty_generate(status: "ready", origin_main: main, dependencies: results)
