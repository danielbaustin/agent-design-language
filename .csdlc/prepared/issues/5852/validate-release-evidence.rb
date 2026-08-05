#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "yaml"

DEFAULT_MANIFEST = ".csdlc/evidence/5852/release-evidence-manifest.json"
WAVE = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"

def git(*argv)
  out, err, status = Open3.capture3("git", *argv)
  abort "git #{argv.join(' ')} failed: #{err}" unless status.success?
  out.strip
end

def run!(*argv)
  out, err, status = Open3.capture3(*argv)
  abort "#{argv.join(' ')} failed: #{out}\n#{err}" unless status.success?
  [out, err]
end

def verify_ref!(ref, label)
  abort "#{label} reference malformed" unless ref.is_a?(Hash)
  path = ref["path"]
  abort "#{label} file missing" unless path.is_a?(String) && File.file?(path)
  abort "#{label} digest mismatch" unless Digest::SHA256.file(path).hexdigest == ref["sha256"]
  path
end

def milestone_issues
  wave = YAML.load_file(WAVE)
  rows = wave.fetch("work_packages") + wave.fetch("supporting_issues") + wave.fetch("execution_sprints")
  (rows.map { |row| row.fetch("issue") } + [wave.fetch("owner_issue"), wave.fetch("planning_review_issue"), 5860]).uniq.sort
end

mode = ARGV.fetch(0, "manifest")
manifest = JSON.parse(File.read(ARGV.fetch(1, DEFAULT_MANIFEST)))
target = manifest["target_sha"]
abort "release target SHA missing" unless target.to_s.match?(/\A[0-9a-f]{40}\z/)

case mode
when "manifest"
  abort "release target is not HEAD" unless target == git("rev-parse", "HEAD")
  rows = manifest["rows"]
  abort "release evidence rows missing" unless rows.is_a?(Array) && !rows.empty?
  rows.each do |row|
    %w[claim issue pr reviewed_head merge_sha].each { |field| abort "#{field} missing" if row[field].to_s.strip.empty? }
    abort "reviewed head malformed" unless row["reviewed_head"].match?(/\A[0-9a-f]{40}\z/)
    abort "merge SHA malformed" unless row["merge_sha"].match?(/\A[0-9a-f]{40}\z/)
    abort "merge is not ancestral to release target" unless system("git", "merge-base", "--is-ancestor", row["merge_sha"], target)
    %w[implementation validation review terminal residual_risk non_claim artifact].each do |kind|
      verify_ref!(row.fetch("#{kind}_ref"), "#{row['claim']} #{kind}")
    end
    review = JSON.parse(File.read(row.dig("review_ref", "path")))
    abort "review identity mismatch" unless review["reviewed_sha"] == row["reviewed_head"] && review["result"] == "passed"
    terminal = JSON.parse(File.read(row.dig("terminal_ref", "path")))
    abort "terminal identity mismatch" unless terminal["issue"] == Integer(row["issue"]) && terminal["phase"] == "closed_out" && terminal["claim_released"] == true
  end
  %w[release_notes checklist handoff residual_risk_summary non_claim_summary].each do |field|
    verify_ref!(manifest.fetch("#{field}_ref"), field)
  end
when "ceremony"
  abort "ceremony target is not HEAD" unless target == git("rev-parse", "HEAD")
  milestone_issues.each do |issue|
    index = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json"))
    abort "##{issue} is not terminal" unless index["phase"] == "closed_out" && index["terminal"].is_a?(Hash)
    abort "##{issue} has an active claim" unless index["claim"].nil?
  end
  run!("bash", "adl/tools/test_release_ceremony.sh")
  run!("bash", "adl/tools/release_ceremony.sh", "--version", "v0.92")
when "negative"
  stdout, stderr = run!("bash", "adl/tools/test_release_ceremony.sh")
  observed = stdout + stderr
  required = %w[dirty wrong-branch existing-tag missing-tag duplicate partial]
  missing = required.reject { |token| observed.downcase.include?(token) }
  abort "ceremony negative coverage missing: #{missing.join(', ')}" unless missing.empty?
  abort "ceremony test output digest mismatch" unless manifest["ceremony_test_output_sha256"] == Digest::SHA256.hexdigest(observed)
when "post-publication"
  tag = manifest.fetch("tag", "v0.92")
  abort "tag is not annotated" unless git("cat-file", "-t", "refs/tags/#{tag}") == "tag"
  abort "tag target mismatch" unless git("rev-list", "-n", "1", tag) == target
  repo = git("remote", "get-url", "origin").sub(%r{^.*github\.com[:/]}, "").sub(/\.git\z/, "")
  out, err, status = Open3.capture3("gh", "api", "repos/#{repo}/releases/tags/#{tag}")
  abort "GitHub release read failed: #{err}" unless status.success?
  release = JSON.parse(out)
  abort "release tag mismatch" unless release["tag_name"] == tag
  abort "release remains draft or prerelease" if release["draft"] || release["prerelease"]
  notes_path = verify_ref!(manifest.fetch("release_notes_ref"), "release notes")
  abort "release notes mismatch" unless Digest::SHA256.hexdigest(release["body"].to_s) == Digest::SHA256.file(notes_path).hexdigest
  expected_assets = manifest.fetch("assets")
  abort "release assets empty" unless expected_assets.is_a?(Array) && !expected_assets.empty?
  live_assets = release.fetch("assets").to_h { |asset| [asset["name"], asset] }
  expected_assets.each do |asset|
    live = live_assets[asset["name"]]
    abort "release asset missing: #{asset['name']}" unless live
    abort "release asset size mismatch" unless live["size"] == asset["size"]
    verify_ref!(asset.fetch("artifact_ref"), "asset #{asset['name']}")
  end
  milestone_issues.each do |issue|
    index = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json"))
    abort "##{issue} terminal/claim truth regressed" unless index["phase"] == "closed_out" && index["terminal"].is_a?(Hash) && index["claim"].nil?
  end
else
  abort "usage: #{$PROGRAM_NAME} manifest|ceremony|negative|post-publication [manifest.json]"
end

puts "PASS: exact-head release #{mode} proof"
