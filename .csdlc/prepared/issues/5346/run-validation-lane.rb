#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
PREP = ROOT.join(".csdlc/prepared/issues/5346")
MANIFEST = ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json")
POST = ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json")
LANES = %w[eligibility-before-deletion complete-post-deletion post-merge-exact].freeze

def run!(*argv)
  out, status = Open3.capture2e(*argv, chdir: ROOT.to_s)
  abort("#5346 command failed: #{argv.join(' ')}\n#{out}") unless status.success?
  out
end

def compile_without_absorbing_lock_drift
  lock = ROOT.join("adl/Cargo.lock")
  before = lock.file? ? lock.read : nil
  out, status = Open3.capture2e(
    "cargo",
    "check",
    "--offline",
    "--manifest-path",
    "adl/Cargo.toml",
    "--target-dir",
    "/Volumes/FastWork/adl-wp-5346/target",
    chdir: ROOT.to_s
  )
  if before
    lock.write(before)
  elsif lock.exist?
    lock.delete
  end
  abort("#5346 offline compile failed:\n#{out}") unless status.success?
  out
end

def load_json(path, label)
  abort("#5346 missing #{label}: #{path}") unless path.file?
  JSON.parse(path.read)
rescue JSON::ParserError => e
  abort("#5346 invalid #{label}: #{e.message}")
end

def git_diff_accounting(commit)
  out = run!("git", "show", "--numstat", "--format=", commit)
  additions = 0
  deletions = 0
  out.lines.each do |line|
    added, deleted, = line.split("\t", 3)
    additions += added.to_i if added&.match?(/\A\d+\z/)
    deletions += deleted.to_i if deleted&.match?(/\A\d+\z/)
  end
  { "commit" => commit, "additions" => additions, "deletions" => deletions, "net" => additions - deletions }
end

lane = ARGV.fetch(0, "")
abort("unsupported #5346 validation lane: #{lane}") unless LANES.include?(lane)
run!("ruby", PREP.join("check-dependencies.rb").to_s)
manifest = load_json(MANIFEST, "eligibility manifest")
decision = load_json(ROOT.join(manifest.fetch("eligibility_decision")), "eligibility decision")
abort("#5346 eligibility rejected") unless decision["eligible"] == true
abort("#5346 decision must identify the #5346 manifest") unless decision["manifest"] == MANIFEST.relative_path_from(ROOT).to_s
abort("#5346 reviewed revision must remain null until fresh rereview") unless manifest["reviewed_revision"].nil?

if lane != "eligibility-before-deletion"
  packet = load_json(POST, "post-deletion validation packet")
  abort("#5346 post-deletion packet schema mismatch") unless packet["schema"] == "adl.wp13.post_deletion_validation.v1"
  abort("#5346 post-deletion packet is not green") unless packet["status"] == "pass" && packet["deferred"] == []
  accounting = packet.fetch("loc_accounting")
  total = accounting.fetch("deleted") + accounting.fetch("retained")
  abort("#5346 invalid LoC denominator") unless total == accounting.fetch("pinned_denominator") && total.positive?
  basis_points = accounting.fetch("deleted") * 10_000 / total
  abort("#5346 deletion is below 80 percent") if basis_points < 8_000
  abort("#5346 80-89 percent deletion lacks reviewed exception") if basis_points < 9_000 && packet["reviewed_80_to_89_exception"] != true
  diff_accounting = packet.fetch("git_diff_accounting")
  recomputed = git_diff_accounting(diff_accounting.fetch("commit"))
  abort("#5346 git diff accounting mismatch: #{diff_accounting} != #{recomputed}") unless diff_accounting == recomputed
  abort("#5346 expected exact reviewed deletion commit accounting is wrong") unless diff_accounting["additions"] == 1880 && diff_accounting["deletions"] == 46502 && diff_accounting["net"] == -44622
  abort("#5346 manifest execution revision mismatch") unless manifest["execution_revision"] == diff_accounting["commit"] && decision["execution_revision"] == diff_accounting["commit"]
  compile_without_absorbing_lock_drift
end

if lane == "post-merge-exact"
  packet = load_json(POST, "post-deletion validation packet")
  head = run!("git", "rev-parse", "HEAD").strip
  abort("#5346 post-merge packet revision mismatch") unless packet["post_merge_revision"] == head && packet["serialized_after_5347"] == true
end

puts JSON.generate(status: "pass", issue: 5346, lane: lane, revision: run!("git", "rev-parse", "HEAD").strip)
