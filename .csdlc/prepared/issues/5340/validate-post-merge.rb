#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "tmpdir"

ISSUE = 5340
REPOSITORY = "danielbaustin/agent-design-language"

def capture!(*argv, chdir: nil, env: {})
  stdout, stderr, status = if chdir
                             Open3.capture3(env, *argv, chdir: chdir)
                           else
                             Open3.capture3(env, *argv)
                           end
  abort("command failed: #{argv.join(' ')}\n#{stderr}#{stdout}") unless status.success?
  stdout
end

issue_root = File.realpath(File.expand_path("../../../..", __dir__))
common = capture!("git", "rev-parse", "--path-format=absolute", "--git-common-dir", chdir: issue_root).strip
primary = File.dirname(common)
doctor = File.join(primary, ".adl/bin/csdlc-v2/csdlc-doctor")
doctor_report = JSON.parse(capture!(doctor, "--repo", issue_root, "--issue", ISSUE.to_s))
abort("typed post-merge lifecycle record is not healthy") unless doctor_report.fetch("status") == "pass" && doctor_report.fetch("findings").empty? && %w[published merge_ready].include?(doctor_report.fetch("phase"))
record = JSON.parse(File.read(File.join(issue_root, ".csdlc/issues/#{ISSUE}/index.json")))
abort("post-merge record phase differs from typed doctor") unless record.fetch("phase") == doctor_report.fetch("phase")
publication = record.fetch("publication")
readiness = record.fetch("readiness")
abort("typed publication is not a merged non-draft observation") unless publication.fetch("repository") == REPOSITORY && publication.fetch("issue") == ISSUE && publication.fetch("observed_state") == "merged" && publication.fetch("draft") == false
abort("typed readiness is not current and merge-ready") unless readiness.fetch("ready") == true && readiness.fetch("pull_request") == publication.fetch("pull_request")
reviewed_head = readiness.fetch("head_sha")
abort("typed reviewed head is malformed") unless reviewed_head.match?(/\A[0-9a-f]{40}\z/)
expected_revision_prefix = "git-blake3:#{reviewed_head}:"
abort("typed publication revision is not the exact readiness head") unless publication.fetch("revision").start_with?(expected_revision_prefix)

capture!("git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main", chdir: issue_root)
integration_sha = capture!("git", "rev-parse", "origin/main^{commit}", chdir: issue_root).strip
_out, _err, ancestry = Open3.capture3("git", "merge-base", "--is-ancestor", reviewed_head, integration_sha, chdir: issue_root)
abort("governed reviewed head is not integrated into captured origin/main") unless ancestry.success?

validator = File.join(primary, ".adl/bin/csdlc-v2/csdlc-validate")
fastwork = File.realpath("/Volumes/FastWork")
durable = File.join(issue_root, ".csdlc/evidence/5340/post-merge-exact")
FileUtils.mkdir_p(durable)
report = nil
Dir.mktmpdir("adl-wp-5340-postmerge-", fastwork) do |scratch|
  clone = File.join(scratch, "repo")
  capture!("git", "clone", "--shared", "--no-checkout", primary, clone)
  capture!("git", "checkout", "--detach", integration_sha, chdir: clone)
  env = {
    "ADL_WP5340_REVIEWED_HEAD_SHA" => reviewed_head,
    "ADL_WP5340_INTEGRATION_SHA" => integration_sha,
    "ADL_WP5340_FAST_ROOT" => File.join(scratch, "build"),
    "ADL_WP5340_CARGO_HOME" => "/Volumes/FastWork/adl-wp-5340/cargo-home"
  }
  request = File.join(clone, ".csdlc/prepared/issues/5340/pvf/postmerge.json")
  stdout = capture!(validator, "--request", request, chdir: clone, env: env)
  report = JSON.parse(stdout)
  evidence = report.fetch("evidence")
  abort("typed post-merge PVF did not pass") unless report.fetch("schema") == "csdlc.pvf.report.v1" && report.fetch("disposition") == "local_pass" && evidence.length == 1 && evidence.first.fetch("lane") == "post-merge-exact" && evidence.first.fetch("status") == "passed"
  source_log = File.join(clone, ".csdlc/evidence/5340/post-merge/post-merge-exact.log")
  FileUtils.cp(source_log, File.join(durable, "post-merge-exact.log"))
end
durable_report = report.merge(
  "pull_request" => publication.fetch("pull_request"),
  "reviewed_head_sha" => reviewed_head,
  "integration_sha" => integration_sha
)
File.write(File.join(durable, "report.json"), JSON.pretty_generate(durable_report) + "\n")
puts JSON.generate(
  schema: "adl.wp06.post-merge-proof.v2",
  pull_request: publication.fetch("pull_request"),
  reviewed_head_sha: reviewed_head,
  integration_sha: integration_sha,
  typed_disposition: report.fetch("disposition"),
  outcome: "passed"
)
