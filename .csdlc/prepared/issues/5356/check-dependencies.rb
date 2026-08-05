#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCY = 5360
RECEIPT_REF = "csdlc-v2/closeout/5360.json"
HEX40 = /\A[0-9a-f]{40}\z/

def fail_gate(message)
  warn("#5356 WP-17 gate: #{message}")
  exit 1
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git command failed") unless status.success?
  out.strip
end

def git_success?(*args)
  _out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  status.success?
end

def tree_for(commit)
  git("show", "--no-patch", "--format=%T", commit)
end

def ancestral_equivalent_for(pr_number, observed_sha, head)
  return observed_sha if git_success?("merge-base", "--is-ancestor", observed_sha, head)

  observed_tree = tree_for(observed_sha)
  candidates = git(
    "log",
    "--first-parent",
    "--format=%H",
    "--grep=(##{pr_number})",
    "#{observed_sha}..#{head}"
  ).lines.map(&:strip).reject(&:empty?)
  candidates.find do |candidate|
    git_success?("merge-base", "--is-ancestor", candidate, head) &&
      tree_for(candidate) == observed_tree
  end
end

def installed_binary(name)
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  binary = common.parent.join(".adl/bin/csdlc-v2", name)
  fail_gate("missing installed typed binary #{name}") unless binary.file? && binary.executable?
  binary.to_s
end

begin
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  receipt_path = common.join(RECEIPT_REF)
  fail_gate("missing retained terminal receipt #{RECEIPT_REF}") unless receipt_path.file?
  receipt = JSON.parse(receipt_path.read)
  record = receipt.fetch("record")
  current_path = ROOT.join(".csdlc/issues/#{DEPENDENCY}/index.json")
  fail_gate("missing current typed WP-17 record") unless current_path.file?
  current = JSON.parse(current_path.read)
  fail_gate("current WP-17 record differs from retained receipt") unless current == record
  fail_gate("WP-17 is not typed closed_out") unless record["phase"] == "closed_out"
  fail_gate("WP-17 still has an active claim") unless record["claim"].nil?

  out, status = Open3.capture2e(installed_binary("csdlc-doctor"), "--repo", ROOT.to_s, "--issue", DEPENDENCY.to_s)
  fail_gate("typed doctor rejected WP-17") unless status.success?
  doctor = JSON.parse(out)
  fail_gate("WP-17 doctor is not clean closed_out") unless doctor["status"] == "pass" && doctor["phase"] == "closed_out" && Array(doctor["findings"]).empty?

  terminal = record.fetch("terminal")
  fail_gate("WP-17 terminal disposition is not merged") unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
  pr_number = terminal["pull_request"]
  fail_gate("WP-17 terminal PR is missing") unless pr_number.is_a?(Integer)
  sha = terminal["observed_sha"]
  fail_gate("WP-17 merged SHA is invalid") unless sha&.match?(HEX40)
  head = git("rev-parse", "HEAD")
  landed_sha = ancestral_equivalent_for(pr_number, sha, head)
  fail_gate("WP-17 merged SHA is not ancestral or tree-equivalent to an ancestral PR merge commit") unless landed_sha

  puts JSON.generate(status: "pass", issue: 5356, dependency: DEPENDENCY, dependency_sha: sha,
                     landed_sha: landed_sha, receipt_sha256: Digest::SHA256.file(receipt_path).hexdigest, revision: head)
rescue JSON::ParserError, KeyError => e
  fail_gate("invalid retained receipt: #{e.message}")
end
