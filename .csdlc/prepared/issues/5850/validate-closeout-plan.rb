#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "shellwords"
require "yaml"

WAVE = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"
UNIVERSE = ".csdlc/evidence/5850/issue-universe.json"
DAG = ".csdlc/evidence/5850/closeout-dag.json"
NEGATIVE = ".csdlc/evidence/5850/negative-cases.json"

def expected_issues
  wave = YAML.load_file(WAVE)
  ids = wave.fetch("work_packages").map { |row| row.fetch("issue") }
  ids.concat(wave.fetch("supporting_issues").map { |row| row.fetch("issue") })
  ids.concat(wave.fetch("execution_sprints").map { |row| row.fetch("issue") })
  ids.concat([wave.fetch("owner_issue"), wave.fetch("planning_review_issue"), 5860])
  ids.uniq.sort
end

def github_issue(issue)
  out, err, status = Open3.capture3("gh", "issue", "view", issue.to_s, "--json", "number,state,url")
  abort "GitHub read failed for ##{issue}: #{err}" unless status.success?
  JSON.parse(out)
end

def github_pr(number)
  fields = "number,state,baseRefName,headRefOid,mergeCommit,reviewDecision,statusCheckRollup"
  out, err, status = Open3.capture3("gh", "pr", "view", number.to_s, "--json", fields)
  abort "GitHub PR read failed for ##{number}: #{err}" unless status.success?
  JSON.parse(out)
end

def blocked_reasons(row, expected)
  reasons = []
  reasons << "unknown_issue" unless expected.include?(row["issue"])
  reasons << "github_open" unless row["github_state"] == "CLOSED"
  reasons << "stale_head" unless row["pr_head"] == row["target_sha"]
  reasons << "red_checks" unless row["checks_state"] == "success"
  reasons << "missing_review" unless row["review_state"] == "approved"
  reasons << "typed_nonterminal" unless row["typed_phase"] == "closed_out"
  reasons << "missing_receipt" unless row["receipt_state"] == "present"
  reasons << "active_claim" unless row["claim_state"] == "released"
  reasons << "dirty_worktree" if row["worktree_state"] == "dirty"
  reasons << "partial_release" if row.key?("release_state") && row["release_state"] != "complete"
  reasons << "duplicate_retry" if row["retry_state"] == "duplicate_mutation"
  reasons << "unowned_action" if row["owner"].to_s.empty?
  reasons
end

mode = ARGV.fetch(0, "universe")
case mode
when "universe"
  packet = JSON.parse(File.read(ARGV.fetch(1, UNIVERSE)))
  head = `git rev-parse HEAD`.strip
  abort "target is not HEAD" unless packet["target_sha"] == head
  rows = packet["rows"]
  abort "issue universe mismatch" unless rows.is_a?(Array) && rows.map { |r| r["issue"] }.sort == expected_issues
  worktrees = `git worktree list --porcelain`
  rows.each do |row|
    issue = Integer(row["issue"])
    live = github_issue(issue)
    abort "##{issue} GitHub state mismatch" unless row["github_state"] == live["state"]
    index_path = ".csdlc/issues/#{issue}/index.json"
    abort "##{issue} typed index missing" unless File.file?(index_path)
    index = JSON.parse(File.read(index_path))
    abort "##{issue} typed phase mismatch" unless row["typed_phase"] == index["phase"]
    sor = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/sor.values.json"))
    abort "##{issue} SOR state mismatch" unless row["sor_state"] == sor["status"]
    expected_claim = index["claim"].nil? ? "released" : "active"
    abort "##{issue} claim mismatch" unless row["claim_state"] == expected_claim
    expected_receipt = index["terminal"].is_a?(Hash) ? "present" : "absent"
    abort "##{issue} receipt mismatch" unless row["receipt_state"] == expected_receipt
    %w[owner classification next_action checks_state review_state worktree_state].each do |field|
      abort "##{issue} #{field} missing" if row[field].to_s.strip.empty?
    end
    pr = github_pr(Integer(row.fetch("pr")))
    abort "##{issue} PR state mismatch" unless row["pr_state"] == pr["state"]
    abort "##{issue} PR base mismatch" unless row["pr_base"] == pr["baseRefName"]
    abort "##{issue} PR head mismatch" unless row["pr_head"] == pr["headRefOid"]
    abort "##{issue} PR merge mismatch" unless row["pr_merge"] == pr.dig("mergeCommit", "oid")
    checks = pr.fetch("statusCheckRollup")
    abort "##{issue} required check set is empty" unless checks.is_a?(Array) && !checks.empty?
    live_checks = checks.all? { |check| %w[SUCCESS SKIPPED NEUTRAL].include?(check["conclusion"]) } ? "success" : "not_green"
    abort "##{issue} checks mismatch" unless row["checks_state"] == live_checks
    abort "##{issue} review mismatch" unless row["review_state"] == pr["reviewDecision"].to_s.downcase
    if row["worktree_path"]
      abort "##{issue} unknown worktree" unless worktrees.include?("worktree #{row['worktree_path']}")
      status_out, status_err, status = Open3.capture3("git", "-C", row["worktree_path"], "status", "--porcelain")
      abort "##{issue} worktree status failed: #{status_err}" unless status.success?
      dirty = !status_out.strip.empty?
      abort "##{issue} worktree state mismatch" unless row["worktree_state"] == (dirty ? "dirty" : "clean")
    end
    evidence = row.fetch("evidence")
    abort "##{issue} evidence set is empty" unless evidence.is_a?(Array) && !evidence.empty?
    evidence.each do |ref|
      abort "##{issue} evidence missing" unless File.file?(ref["path"])
      abort "##{issue} evidence digest mismatch" unless Digest::SHA256.file(ref["path"]).hexdigest == ref["sha256"]
    end
  end
when "dag"
  dag = JSON.parse(File.read(ARGV.fetch(1, DAG)))
  nodes = dag.fetch("nodes")
  edges = dag.fetch("edges")
  expected_nodes = (expected_issues.map(&:to_s) + %w[WP-29 WP-30 umbrella-closeout v0.93-acceptance]).sort
  abort "DAG universe mismatch" unless nodes.sort == expected_nodes
  incoming = nodes.to_h { |node| [node, 0] }
  edges.each { |from, to| abort "unknown DAG node" unless incoming.key?(from) && incoming.key?(to); incoming[to] += 1 }
  queue = incoming.select { |_node, count| count.zero? }.keys
  visited = []
  until queue.empty?
    node = queue.shift
    visited << node
    edges.select { |from, _to| from == node }.each { |_from, to| incoming[to] -= 1; queue << to if incoming[to].zero? }
  end
  abort "closeout DAG contains a cycle" unless visited.length == nodes.length
when "negative"
  packet = JSON.parse(File.read(ARGV.fetch(1, NEGATIVE)))
  classes = %w[stale_head red_checks missing_review missing_receipt active_claim dirty_worktree partial_release duplicate_retry unknown_issue unowned_action].sort
  abort "negative class mismatch" unless packet.fetch("cases").map { |row| row["class"] }.sort == classes
  packet.fetch("cases").each do |row|
    baseline = row.fetch("baseline_row")
    mutated = row.fetch("mutated_row")
    klass = row["class"]
    abort "negative baseline is not accepted" unless blocked_reasons(baseline, expected_issues).empty?
    changed = (baseline.keys | mutated.keys).select { |key| baseline[key] != mutated[key] }
    abort "negative fixture must mutate exactly one declared field" unless changed == [row.fetch("mutated_field")]
    reasons = blocked_reasons(mutated, expected_issues)
    abort "negative fixture did not reconstruct #{klass}: #{reasons.inspect}" unless reasons.include?(klass)
    abort "negative fixture produced undeclared blockers: #{reasons.inspect}" unless reasons == [klass]
  end
else
  abort "usage: #{$PROGRAM_NAME} universe|dag|negative"
end

puts "PASS: derived closeout #{mode} truth"
