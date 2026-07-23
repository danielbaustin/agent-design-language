#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

DEPENDENCIES = {
  5498 => "bounded Codex task and context-handoff adapter",
  5349 => "final WP-09 provider and governed-tool interface freeze"
}.freeze

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort("command failed: #{argv.join(' ')}\n#{stderr}") unless status.success?
  stdout.strip
end

root = Pathname.new(capture!("git", "rev-parse", "--show-toplevel"))
head = capture!("git", "rev-parse", "HEAD")

def merge_commits_for_issue(root, issue)
  stdout = capture!(
    "git", "log", "--format=%H%x00%s", "--merges", "origin/main",
    chdir: root.to_s
  )
  stdout.lines.map do |line|
    sha, subject = line.chomp.split("\0", 2)
    next unless subject&.include?("Merge pull request")
    next unless subject.include?("##{issue}") || subject.include?(issue.to_s)
    [sha, subject]
  end.compact
end

results = DEPENDENCIES.map do |issue, label|
  merges = merge_commits_for_issue(root, issue)
  if merges.empty?
    next {
      issue: issue,
      label: label,
      status: "waiting",
      blocker: "##{issue}: no live merge commit found on origin/main"
    }
  end

  merge_sha, subject = merges.first
  _out, _err, status = Open3.capture3(
    "git", "merge-base", "--is-ancestor", merge_sha, "HEAD",
    chdir: root.to_s
  )

  if status.success?
    {
      issue: issue,
      label: label,
      status: "ready",
      merge_sha: merge_sha,
      subject: subject
    }
  else
    {
      issue: issue,
      label: label,
      status: "waiting",
      merge_sha: merge_sha,
      subject: subject,
      blocker: "##{issue}: live merge #{merge_sha} is not ancestral to HEAD #{head}"
    }
  end
end

blockers = results.map { |result| result[:blocker] }.compact
payload = {
  status: blockers.empty? ? "ready" : "waiting",
  issues: DEPENDENCIES.keys,
  final_wp09_gate: 5349,
  predicate: "live merge on origin/main plus ancestry to HEAD",
  audit_only: ["typed closeout receipts", "retained lifecycle records"],
  results: results
}

if blockers.empty?
  puts JSON.generate(payload)
  exit 0
end

payload[:blockers] = blockers
puts JSON.generate(payload)
exit 3
