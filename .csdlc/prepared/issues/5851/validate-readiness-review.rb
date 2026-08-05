#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "rbconfig"

UNIVERSE = ".csdlc/evidence/5850/issue-universe.json"
COMPARISON = ".csdlc/evidence/5851/universe-comparison.json"
HANDOFF = ".csdlc/evidence/5851/handoff-review.json"
NEGATIVE = ".csdlc/evidence/5851/negative-cases.json"

mode = ARGV.fetch(0, "comparison")
case mode
when "comparison"
  out, err, status = Open3.capture3(RbConfig.ruby, ".csdlc/prepared/issues/5850/validate-closeout-plan.rb", "universe", UNIVERSE)
  abort "source universe is not live-valid: #{out}\n#{err}" unless status.success?
  source = JSON.parse(File.read(UNIVERSE))
  comparison = JSON.parse(File.read(ARGV.fetch(1, COMPARISON)))
  abort "review target mismatch" unless comparison["target_sha"] == source["target_sha"]
  rebuilt = source.fetch("rows").map { |row| row.slice("issue", "github_state", "typed_phase", "receipt_state", "claim_state", "worktree_state", "owner") }
  abort "independent universe mismatch" unless comparison["rebuilt_rows"] == rebuilt
  abort "comparison artifact digest mismatch" unless comparison["source_sha256"] == Digest::SHA256.file(UNIVERSE).hexdigest
when "handoff"
  packet = JSON.parse(File.read(ARGV.fetch(1, HANDOFF)))
  abort "handoff review is not exact HEAD" unless packet["reviewed_head"] == `git rev-parse HEAD`.strip
  abort "reviewer identity missing" if packet["reviewer"].to_s.strip.empty?
  abort "handoff review blockers remain" unless packet["findings"].is_a?(Array) && packet["findings"].none? { |f| %w[P0 P1 P2].include?(f["severity"]) && f["disposition"] != "resolved" }
  packet.fetch("reviewed_artifacts").each do |ref|
    abort "reviewed artifact missing" unless File.file?(ref["path"])
    abort "reviewed artifact digest mismatch" unless Digest::SHA256.file(ref["path"]).hexdigest == ref["sha256"]
  end
  corpus = `git grep -n -E 'status: (active|implementation|released)|issue creation authorized|legal personhood|certified production' -- docs/milestones/v0.93`
  abort "v0.93 activation/claim boundary violated: #{corpus}" unless corpus.strip.empty?
when "negative"
  packet = JSON.parse(File.read(ARGV.fetch(1, NEGATIVE)))
  classes = %w[missing_row stale_sha red_checks active_claim absent_receipt dirty_cleanup partial_release duplicate_retry premature_closeout v093_activation].sort
  abort "negative class mismatch" unless packet.fetch("cases").map { |row| row["class"] }.sort == classes
  packet.fetch("cases").each do |row|
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

puts "PASS: independent readiness-review #{mode} proof"
