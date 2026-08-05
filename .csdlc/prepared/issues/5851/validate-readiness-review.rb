#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "rbconfig"
require "yaml"

WAVE = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"
COMPARISON = ".csdlc/evidence/5851/universe-comparison.json"
HANDOFF = ".csdlc/evidence/5851/handoff-review.json"
NEGATIVE = ".csdlc/evidence/5851/negative-cases.json"

def expected_issues
  wave = YAML.load_file(WAVE)
  ids = wave.fetch("work_packages").map { |row| row.fetch("issue") }
  ids.concat(wave.fetch("supporting_issues").map { |row| row.fetch("issue") })
  ids.concat(wave.fetch("execution_sprints").map { |row| row.fetch("issue") })
  ids.concat([wave.fetch("owner_issue"), wave.fetch("planning_review_issue"), 5860])
  ids.uniq.sort
end

def gh(kind, number, fields)
  out, err, status = Open3.capture3("gh", kind, "view", number.to_s, "--json", fields)
  abort "GitHub #{kind} read failed for ##{number}: #{err}" unless status.success?
  JSON.parse(out)
end

def rebuild_row(issue)
  live_issue = gh("issue", issue, "number,state,url")
  index_path = ".csdlc/issues/#{issue}/index.json"
  abort "typed index missing for ##{issue}" unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  publication = index["publication"]
  abort "publication missing for ##{issue}" unless publication.is_a?(Hash) && publication["pull_request"].is_a?(Integer)
  pr = gh("pr", publication["pull_request"], "number,state,baseRefName,headRefOid,mergeCommit,reviewDecision,statusCheckRollup")
  checks = pr["statusCheckRollup"]
  abort "required checks empty for ##{issue}" unless checks.is_a?(Array) && !checks.empty?
  {
    "issue" => issue,
    "github_state" => live_issue["state"],
    "typed_phase" => index["phase"],
    "receipt_state" => index["terminal"].is_a?(Hash) ? "present" : "absent",
    "claim_state" => index["claim"].nil? ? "released" : "active",
    "pr" => pr["number"],
    "pr_state" => pr["state"],
    "pr_base" => pr["baseRefName"],
    "pr_head" => pr["headRefOid"],
    "pr_merge" => pr.dig("mergeCommit", "oid"),
    "review_state" => pr["reviewDecision"].to_s.downcase,
    "checks_state" => checks.all? { |check| %w[SUCCESS SKIPPED NEUTRAL].include?(check["conclusion"]) } ? "success" : "not_green"
  }
end

mode = ARGV.fetch(0, "comparison")
case mode
when "comparison"
  comparison = JSON.parse(File.read(ARGV.fetch(1, COMPARISON)))
  abort "review target is not HEAD" unless comparison["target_sha"] == `git rev-parse HEAD`.strip
  expected = expected_issues
  abort "comparison issue denominator mismatch" unless comparison.fetch("rebuilt_rows").map { |row| row["issue"] }.sort == expected
  rebuilt = expected.map { |issue| rebuild_row(issue) }
  abort "independently rebuilt live universe mismatch" unless comparison["rebuilt_rows"] == rebuilt
  abort "upstream row slice is forbidden" if comparison.key?("source_sha256") || comparison.key?("source_rows")
when "handoff"
  packet = JSON.parse(File.read(ARGV.fetch(1, HANDOFF)))
  abort "handoff review is not exact HEAD" unless packet["reviewed_head"] == `git rev-parse HEAD`.strip
  abort "reviewer identity missing" if packet["reviewer"].to_s.strip.empty?
  findings = packet["findings"]
  abort "findings missing" unless findings.is_a?(Array)
  abort "handoff review blockers remain" if findings.any? { |finding| %w[P0 P1 P2].include?(finding["severity"]) && finding["disposition"] != "resolved" }
  refs = packet["reviewed_artifacts"]
  abort "reviewed artifacts empty" unless refs.is_a?(Array) && !refs.empty?
  refs.each do |ref|
    abort "reviewed artifact missing" unless File.file?(ref["path"])
    abort "reviewed artifact digest mismatch" unless Digest::SHA256.file(ref["path"]).hexdigest == ref["sha256"]
  end
  corpus = `git grep -n -E 'status: (active|implementation|released)|issue creation authorized|legal personhood|certified production' -- docs/milestones/v0.93`
  abort "v0.93 activation/claim boundary violated: #{corpus}" unless corpus.strip.empty?
when "negative"
  packet = JSON.parse(File.read(ARGV.fetch(1, NEGATIVE)))
  classes = %w[missing_row stale_sha red_checks active_claim absent_receipt dirty_cleanup partial_release duplicate_retry premature_closeout v093_activation].sort
  cases = packet.fetch("cases")
  abort "negative class mismatch" unless cases.map { |row| row["class"] }.sort == classes
  cases.each do |row|
    fixture = row["fixture_path"]
    validator = row["validator"]
    abort "negative fixture missing" unless File.file?(fixture)
    allowed = {
      "comparison" => [RbConfig.ruby, __FILE__, "comparison", fixture],
      "handoff" => [RbConfig.ruby, __FILE__, "handoff", fixture]
    }
    argv = allowed[validator]
    abort "negative validator not allowlisted" unless argv
    out, err, status = Open3.capture3(*argv)
    abort "negative case escaped: #{row['class']}" if status.success?
    abort "negative output digest mismatch" unless Digest::SHA256.hexdigest(out + err) == row["observed_sha256"]
  end
else
  abort "usage: #{$PROGRAM_NAME} comparison|handoff|negative"
end

puts "PASS: independent live GitHub and typed-state readiness-review #{mode} proof"
