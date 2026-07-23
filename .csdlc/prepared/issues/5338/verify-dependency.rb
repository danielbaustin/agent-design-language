#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "English"

root = `git rev-parse --show-toplevel`.strip
abort("not in a Git worktree") unless $CHILD_STATUS.success?
common = `git rev-parse --path-format=absolute --git-common-dir`.strip
abort("cannot resolve Git common directory") unless $CHILD_STATUS.success?
receipt_path = File.join(common, "csdlc-v2/closeout/5339.json")
abort("BLOCKED: retained typed closeout receipt for #5339 is absent") unless File.file?(receipt_path)

receipt = JSON.parse(File.read(receipt_path))
record = receipt.fetch("record")
terminal = record.fetch("terminal")
publication = record.fetch("publication")
abort("BLOCKED: #5339 retained typed phase is not closed_out") unless record.fetch("phase") == "closed_out"
abort("BLOCKED: #5339 terminal disposition is not merged") unless terminal.fetch("disposition") == "merged"
merged_sha = terminal.fetch("observed_sha")
abort("BLOCKED: #5339 merged SHA is absent") unless merged_sha.is_a?(String) && !merged_sha.empty?
abort("BLOCKED: #5339 receipt does not identify PR #5612") unless terminal.fetch("pull_request") == 5612 && publication.fetch("pull_request") == 5612
abort("BLOCKED: #5339 receipt head does not match reviewed head") unless merged_sha == "ba604e5f0ee16af901a4d8d7cb801c323500828d"

merge_commit = "860aa9f18946a2cd9407b610d5c00d44ddc89053"
system("git", "merge-base", "--is-ancestor", merge_commit, "HEAD", out: File::NULL, err: File::NULL)
abort("BLOCKED: #5338 branch does not contain #5339 squash-merge commit #{merge_commit}") unless $CHILD_STATUS.success?

puts JSON.generate(schema: "adl.csdlc.dependency-gate.v1", dependency_issue: 5339, phase: "closed_out", disposition: "merged", reviewed_head_sha: merged_sha, merge_commit: merge_commit, outcome: "passed")
