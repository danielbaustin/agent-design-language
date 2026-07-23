#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ISSUE = 5340
BRANCH = "codex/5340-v0918-wp06-portable-engine"
CLAIM_ID = "claim-5340-v0918-wp06-owner-watch"
PROTECTED = [
  ".csdlc/issues/5340",
  ".csdlc/locks/5340.lock",
  ".csdlc/prepared/issues/5340",
  ".csdlc/evidence/5340",
  "adl-v2/crates/adl-engine"
].freeze

def capture!(*argv, chdir: nil)
  stdout, stderr, status = Open3.capture3(*argv, chdir: chdir)
  abort("command failed: #{argv.join(' ')}\n#{stderr}#{stdout}") unless status.success?
  stdout
end

def within?(path, allowed)
  path == allowed || path.start_with?(allowed + "/")
end

def overlap?(left, right)
  within?(left, right) || within?(right, left)
end

root = File.realpath(File.expand_path("../../../..", __dir__))
record_path = File.join(root, ".csdlc/issues/#{ISSUE}/index.json")
record = JSON.parse(File.read(record_path))
claim = record.fetch("claim")
abort("#5340 typed claim id drift") unless claim.fetch("id") == CLAIM_ID
abort("#5340 typed claim owner drift") unless claim.fetch("owner") == "codex:5340-owner-watch"
abort("#5340 branch claim drift") unless claim.fetch("branch") == BRANCH
abort("#5340 worktree claim drift") unless claim.fetch("worktree") == "."
abort("#5340 protected paths drift") unless claim.fetch("protected_paths").sort == PROTECTED.sort
now = Time.now.to_i
abort("#5340 typed claim generation drift") unless claim.fetch("generation") == record.fetch("generation")
abort("#5340 typed claim is expired or temporally invalid") unless claim.fetch("acquired_unix_seconds") <= claim.fetch("heartbeat_unix_seconds") && claim.fetch("heartbeat_unix_seconds") <= now && claim.fetch("expires_unix_seconds") > now
branch = capture!("git", "branch", "--show-current", chdir: root).strip
abort("not on the dedicated #5340 branch") unless branch == BRANCH

changed = []
[
  ["diff", "--name-only", "origin/main...HEAD"],
  ["diff", "--name-only", "--cached"],
  ["diff", "--name-only"],
  ["ls-files", "--others", "--exclude-standard"]
].each do |args|
  changed.concat(capture!("git", *args, chdir: root).lines.map(&:strip).reject(&:empty?))
end
changed.uniq!
outside = changed.reject { |path| PROTECTED.any? { |allowed| within?(path, allowed) } }
abort("changed paths escape the exact #5340 allowlist: #{outside.join(', ')}") unless outside.empty?

worktrees = capture!("git", "worktree", "list", "--porcelain", chdir: root)
  .lines.select { |line| line.start_with?("worktree ") }
  .map { |line| line.delete_prefix("worktree ").strip }
conflicts = []
worktrees.each do |worktree|
  Dir.glob(File.join(worktree, ".csdlc/issues/*/index.json")).each do |candidate|
    next if File.expand_path(candidate) == File.expand_path(record_path)
    other = JSON.parse(File.read(candidate))
    other_claim = other["claim"]
    next unless other_claim.is_a?(Hash)
    other_claim.fetch("protected_paths", []).each do |other_path|
      PROTECTED.each do |local_path|
        next unless overlap?(local_path, other_path)
        conflicts << {
          issue: other["issue"], claim: other_claim["id"], worktree: worktree,
          local_path: local_path, other_path: other_path
        }
      end
    end
  end
end
abort("active or stale typed claim overlap: #{JSON.generate(conflicts)}") unless conflicts.empty?

puts JSON.generate(
  schema: "adl.csdlc.scope-proof.v1",
  issue: ISSUE,
  branch: branch,
  claim: CLAIM_ID,
  protected_paths: PROTECTED,
  changed_paths: changed.sort,
  overlapping_claims: conflicts,
  outcome: "passed"
)
