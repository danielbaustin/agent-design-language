#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "pathname"
require "tmpdir"

ISSUE = 5340
DEPENDENCY = 5338
REPOSITORY = "danielbaustin/agent-design-language"

def capture!(*argv, chdir: nil)
  stdout, stderr, status = if chdir
                             Open3.capture3(*argv, chdir: chdir)
                           else
                             Open3.capture3(*argv)
                           end
  abort("command failed: #{argv.join(' ')}\n#{stderr}#{stdout}") unless status.success?
  stdout
end

root = File.realpath(File.expand_path("../../../..", __dir__))
common = capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir", chdir: root).strip
primary = File.dirname(common)
scope_output = capture!("ruby", ".csdlc/prepared/issues/5340/verify-scope.rb", chdir: root)
scope = JSON.parse(scope_output)
abort("BLOCKED: #5340 sole-writer scope proof failed") unless scope.fetch("outcome") == "passed"

record = JSON.parse(File.read(File.join(root, ".csdlc/issues/5340/index.json")))
allowed_phases = %w[bound implemented reviewed published merge_ready]
abort("BLOCKED: #5340 is not typed-bound for implementation") unless allowed_phases.include?(record.fetch("phase"))
doctor = File.join(primary, ".adl/bin/csdlc-v2/csdlc-doctor")
doctor_report = JSON.parse(capture!(doctor, "--repo", root, "--issue", ISSUE.to_s))
abort("BLOCKED: #5340 typed claim/record doctor is not passing") unless doctor_report.fetch("status") == "pass" && doctor_report.fetch("findings").empty? && doctor_report.fetch("phase") == record.fetch("phase")

receipt_path = File.join(common, "csdlc-v2/closeout/#{DEPENDENCY}.json")
abort("BLOCKED: retained typed closeout receipt for ##{DEPENDENCY} is absent") unless File.file?(receipt_path)

expected_origin_main = ENV.fetch("ADL_WP5340_EXPECTED_ORIGIN_MAIN_SHA", "")
fetched_at = Integer(ENV.fetch("ADL_WP5340_FETCHED_UNIX_SECONDS", "0"), 10)
now = Time.now.to_i
abort("BLOCKED: immediately preceding fetch-only observation is absent or stale") unless expected_origin_main.match?(/\A[0-9a-f]{40}\z/) && fetched_at <= now && now - fetched_at <= 300
origin_main = capture!("git", "rev-parse", "origin/main^{commit}", chdir: root).strip
abort("BLOCKED: origin/main changed after the fetch observation") unless origin_main == expected_origin_main
head = capture!("git", "rev-parse", "HEAD^{commit}", chdir: root).strip
fastwork = File.realpath("/Volumes/FastWork")
validator = File.join(primary, ".adl/bin/csdlc-v2/csdlc-closeout")
abort("BLOCKED: installed typed closeout validator is absent") unless File.executable?(validator)

# The installed v2 closeout binary exposes receipt validation through its
# idempotent retain operation. Run that operation only in an isolated scratch
# clone: the real worktree, root checkout, shared receipt, and refs remain
# read-only. Hydrating the scratch projection from the receipt lets the binary
# validate retained receipts even when post-merge closeout projections are not
# tracked on origin/main.
typed_receipt = nil
Dir.mktmpdir("adl-wp-5340-receipt-", fastwork) do |scratch|
  scratch = File.realpath(scratch)
  abort("scratch receipt validator escaped FastWork") unless scratch.start_with?(fastwork + File::SEPARATOR)
  clone = File.join(scratch, "repo")
  capture!("git", "clone", "--shared", "--no-checkout", primary, clone)
  capture!(
    "git", "fetch", "--no-tags", primary,
    "+refs/remotes/origin/main:refs/remotes/source/origin-main",
    chdir: clone
  )
  capture!("git", "checkout", "--detach", origin_main, chdir: clone)

  untrusted = JSON.parse(File.read(receipt_path))
  card_kinds = untrusted.fetch("cards").keys.sort
  expected_card_kinds = %w[sip sor spp srp stp vpp]
  abort("BLOCKED: receipt card kinds are not the canonical six") unless card_kinds == expected_card_kinds
  untrusted_record = untrusted.fetch("record")
  expected_design = ".csdlc/prepared/issues/#{DEPENDENCY}/design.md"
  expected_diagram = ".csdlc/prepared/issues/#{DEPENDENCY}/diagram.mmd"
  expected_artifacts = [expected_design, expected_diagram].sort
  actual_artifacts = untrusted.fetch("authored_artifacts").keys.sort
  abort("BLOCKED: receipt artifact paths are not the exact dependency design and diagram") unless actual_artifacts == expected_artifacts
  abort("BLOCKED: receipt design path drift") unless untrusted_record.fetch("design_path") == expected_design
  abort("BLOCKED: receipt diagram path drift") unless untrusted_record.fetch("diagram_path") == expected_diagram

  safe_write = lambda do |relative, contents|
    path = Pathname.new(relative)
    abort("BLOCKED: unsafe scratch hydration path") if path.absolute? || path.each_filename.any? { |part| part == ".." }
    target = File.expand_path(relative, clone)
    abort("BLOCKED: scratch hydration escaped clone") unless target.start_with?(File.realpath(clone) + File::SEPARATOR)
    parent = File.dirname(target)
    FileUtils.mkdir_p(parent)
    abort("BLOCKED: scratch hydration parent escaped clone") unless File.realpath(parent).start_with?(File.realpath(clone) + File::SEPARATOR)
    abort("BLOCKED: scratch hydration target is a symlink") if File.symlink?(target)
    File.write(target, contents)
  end
  issue_root = File.join(clone, ".csdlc/issues/#{DEPENDENCY}")
  FileUtils.mkdir_p(File.join(issue_root, "cards"))
  safe_write.call(".csdlc/issues/#{DEPENDENCY}/index.json", JSON.pretty_generate(untrusted_record) + "\n")
  untrusted.fetch("cards").each do |kind, values|
    safe_write.call(".csdlc/issues/#{DEPENDENCY}/cards/#{kind}.values.json", JSON.pretty_generate(values) + "\n")
  end
  untrusted.fetch("authored_artifacts").each do |relative, contents|
    safe_write.call(relative, contents)
  end
  clone_common = capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir", chdir: clone).strip
  clone_receipt = File.join(clone_common, "csdlc-v2/closeout/#{DEPENDENCY}.json")
  FileUtils.mkdir_p(File.dirname(clone_receipt))
  FileUtils.cp(receipt_path, clone_receipt)

  stdout, stderr, status = Open3.capture3(
    validator, "--root", clone, "retain-receipt", "--issue", DEPENDENCY.to_s
  )
  abort("BLOCKED: typed retained-receipt validation failed\n#{stderr}#{stdout}") unless status.success?
  typed_receipt = JSON.parse(stdout)
end

abort("BLOCKED: typed receipt issue mismatch") unless typed_receipt.fetch("issue") == DEPENDENCY
abort("BLOCKED: typed receipt repository mismatch") unless typed_receipt.fetch("repository") == REPOSITORY
dependency_record = typed_receipt.fetch("record")
terminal = dependency_record.fetch("terminal")
abort("BLOCKED: ##{DEPENDENCY} retained typed phase is not closed_out") unless dependency_record.fetch("phase") == "closed_out"
abort("BLOCKED: ##{DEPENDENCY} retained claim was not released") unless dependency_record["claim"].nil?
abort("BLOCKED: ##{DEPENDENCY} terminal disposition is not merged") unless terminal.fetch("disposition") == "merged"
abort("BLOCKED: ##{DEPENDENCY} observed GitHub state is not merged") unless terminal.fetch("observed_state") == "merged"
pull_request = terminal.fetch("pull_request")
abort("BLOCKED: ##{DEPENDENCY} pull request identity is absent") unless pull_request.is_a?(Integer) && pull_request.positive?
merged_sha = terminal.fetch("observed_sha")
abort("BLOCKED: ##{DEPENDENCY} merged SHA is absent") unless merged_sha.is_a?(String) && merged_sha.match?(/\A[0-9a-f]{40}\z/)

[
  [merged_sha, origin_main, "dependency merge is not on refreshed origin/main"],
  [merged_sha, head, "#5340 HEAD does not contain the dependency merge"],
  [origin_main, head, "#5340 HEAD does not contain current origin/main"]
].each do |ancestor, descendant, message|
  _out, _err, status = Open3.capture3("git", "merge-base", "--is-ancestor", ancestor, descendant, chdir: root)
  abort("BLOCKED: #{message}") unless status.success?
end

puts JSON.generate(
  schema: "adl.csdlc.dependency-gate.v2",
  issue: ISSUE,
  dependency_issue: DEPENDENCY,
  repository: REPOSITORY,
  phase: dependency_record.fetch("phase"),
  disposition: terminal.fetch("disposition"),
  observed_state: terminal.fetch("observed_state"),
  pull_request: pull_request,
  merged_sha: merged_sha,
  origin_main_sha: origin_main,
  fetched_unix_seconds: fetched_at,
  head_sha: head,
  typed_receipt_digest: typed_receipt.fetch("digest"),
  typed_receipt_validation: "scratch_retain_receipt_passed",
  ancestry: %w[merge_to_origin_main merge_to_head origin_main_to_head],
  sole_writer: true,
  outcome: "passed"
)
