#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "yaml"

MAP = ".csdlc/evidence/5849/v093-prerequisite-map.json"
SCAN = ".csdlc/evidence/5849/claim-boundary-scan.json"
CANDIDATE_ROOT = "docs/milestones/v0.93"

def terminal_issue!(issue)
  out, err, status = Open3.capture3("gh", "issue", "view", issue.to_s, "--json", "number,state")
  abort "GitHub read failed for ##{issue}: #{err}" unless status.success?
  abort "##{issue} is not closed" unless JSON.parse(out)["state"] == "CLOSED"
  index = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json"))
  abort "##{issue} is not receipt-backed terminal" unless index["phase"] == "closed_out" && index["claim"].nil? && index["terminal"].is_a?(Hash)
end

terminal_issue!(5848)
packet = JSON.parse(File.read(ARGV.fetch(0, MAP)))
head = `git rev-parse HEAD`.strip
abort "handoff target is not HEAD" unless packet["target_sha"] == head
candidate_files = `git ls-files -- #{CANDIDATE_ROOT}`.lines.map(&:strip).reject(&:empty?).sort
abort "v0.93 candidate corpus empty" if candidate_files.empty?
rows = packet["rows"]
abort "candidate corpus mismatch" unless rows.is_a?(Array) && rows.map { |r| r["candidate_path"] }.sort == candidate_files
rows.each do |row|
  %w[candidate_path work_area disposition owner acceptance_hook evidence_path evidence_sha256].each do |field|
    abort "#{field} missing" if row[field].to_s.strip.empty?
  end
  abort "candidate file missing" unless File.file?(row["candidate_path"])
  abort "evidence missing" unless File.file?(row["evidence_path"])
  abort "evidence digest mismatch" unless Digest::SHA256.file(row["evidence_path"]).hexdigest == row["evidence_sha256"]
  abort "invalid disposition" unless %w[evidence blocker follow_on non_claim].include?(row["disposition"])
end

scan = JSON.parse(File.read(ARGV.fetch(1, SCAN)))
abort "scan target mismatch" unless scan["target_sha"] == head
forbidden = %w[activated implementation_started issues_opened release_scheduled legal_personhood production_authority certified]
abort "claim-boundary classes missing" unless scan["checks"].is_a?(Hash) && (forbidden - scan["checks"].keys).empty?
abort "claim-boundary violation" unless forbidden.all? { |key| scan["checks"][key] == false }
forbidden_patterns = {
  "activated" => /status:\s*(active|activated)/i,
  "implementation_started" => /implementation\s+(has\s+)?started/i,
  "issues_opened" => /issues?\s+(have\s+been\s+)?opened/i,
  "release_scheduled" => /release\s+(date|scheduled\s+for)/i,
  "legal_personhood" => /legal\s+personhood\s+(is|has\s+been)\s+(granted|established)/i,
  "production_authority" => /production\s+(constitutional\s+)?authority\s+(is|has\s+been)\s+(granted|active)/i,
  "certified" => /certified\s+(for\s+)?production/i
}
candidate_files.each do |path|
  text = File.read(path)
  forbidden_patterns.each do |klass, pattern|
    abort "#{klass} claim found in #{path}" if text.match?(pattern)
  end
end
scan.fetch("artifacts").each do |ref|
  abort "scan artifact missing" unless File.file?(ref["path"])
  abort "scan artifact digest mismatch" unless Digest::SHA256.file(ref["path"]).hexdigest == ref["sha256"]
end

puts "PASS: live WP-27 terminal truth, complete candidate corpus, digest-bound handoff, and non-claim boundary"
