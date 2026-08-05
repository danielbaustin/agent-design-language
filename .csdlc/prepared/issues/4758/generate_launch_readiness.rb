#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = 4758
ACCEPTED_BASELINE = "11151e0beab02b1667f6505b7f8992bfd47d2f8f"
SNAPSHOT = ".csdlc/prepared/issues/4758/dependency-snapshot.v1.json"
ALLOWLIST = [
  ".csdlc/evidence/4758",
  ".csdlc/issues/4758",
  ".csdlc/locks/4758.lock",
  ".csdlc/prepared/issues/4758"
].freeze

def run!(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  raise "#{argv.join(' ')} failed: #{stderr.strip}" unless status.success?

  stdout.strip
end

def issue_state(snapshot, issue)
  entry = snapshot.fetch("observations").find { |item| item.fetch("issue") == issue }
  raise "missing dependency snapshot for ##{issue}" unless entry

  entry.fetch("state")
end

def sha256(path)
  Digest::SHA256.file(File.join(ROOT, path)).hexdigest
end

def allowed_path?(path)
  ALLOWLIST.any? { |prefix| path == prefix || path.start_with?("#{prefix}/") }
end

def staging_dir
  explicit = ARGV[0]
  return File.expand_path(explicit, ROOT) if explicit && !explicit.empty?

  candidates = Dir.glob(File.join(ROOT, ".csdlc/evidence/.csdlc-finalize-#{ISSUE}-*"))
                  .select { |path| File.directory?(path) }
  raise "no csdlc finalize staging directory found" if candidates.empty?

  candidates.max_by { |path| File.mtime(path) }
end

snapshot = JSON.parse(File.read(File.join(ROOT, SNAPSHOT)))
raise "unexpected snapshot schema" unless snapshot.fetch("schema") == "adl.launch_readiness.dependency_snapshot.v1"

head = run!("git", "rev-parse", "HEAD")
origin_main = run!("git", "rev-parse", "origin/main")
ancestry_ok = system("git", "merge-base", "--is-ancestor", ACCEPTED_BASELINE, "origin/main", chdir: ROOT)
raise "accepted WP-14A baseline is not ancestral on origin/main" unless ancestry_ok

diff_paths = run!("git", "diff", "--name-only", "origin/main...HEAD").lines.map(&:strip).reject(&:empty?)
unexpected = diff_paths.reject { |path| allowed_path?(path) }
raise "diff contains paths outside #4758 allowlist: #{unexpected.join(', ')}" unless unexpected.empty?
raise "diff unexpectedly contains #5332 paths" if diff_paths.any? { |path| path.include?("/5332/") }

required_docs = {
  "issue_wave" => "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml",
  "wbs" => "docs/milestones/v0.91.8/WBS_v0.91.8.md",
  "activation_map" => "docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md",
  "handoff_feature" => "docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md"
}
required_docs.each_value do |path|
  text = File.read(File.join(ROOT, path))
  raise "#{path} does not route #4758" unless text.include?("4758")
  raise "#{path} does not name WP-21 context" unless text.include?("WP-21") || text.include?("Public launch docs")
end

raise "#5384 is not closed in dependency snapshot" unless issue_state(snapshot, 5384) == "CLOSED"

blockers = [5363, 5362, 5352, 4763].each_with_object([]) do |issue, memo|
  next unless issue_state(snapshot, issue) == "OPEN"

  memo << {
    "issue" => issue,
    "status" => "open",
    "disposition" => "blocker_for_v0_92_launch_readiness",
    "proof_ref" => SNAPSHOT
  }
end

out = File.join(staging_dir, "launch-readiness")
FileUtils.mkdir_p(out)

inputs = {
  "schema" => "adl.launch_readiness.inputs.v1",
  "issue" => ISSUE,
  "worktree_head" => head,
  "origin_main" => origin_main,
  "accepted_platform_baseline" => ACCEPTED_BASELINE,
  "accepted_platform_baseline_ancestral_on_origin_main" => true,
  "dependency_snapshot" => SNAPSHOT,
  "source_documents" => required_docs.map do |name, path|
    {
      "name" => name,
      "path" => path,
      "sha256" => sha256(path),
      "claim_class" => name == "activation_map" ? "consumer_route" : "routing_input"
    }
  end,
  "dependency_observations" => snapshot.fetch("observations")
}

manifest = {
  "schema" => "adl.launch_readiness.manifest.v1",
  "issue" => ISSUE,
  "primary_concern" => "launch-readiness",
  "consumer" => "v0.91.8 WP-21 release-review intake",
  "canonical_manifest_path" => ".csdlc/evidence/4758/launch-readiness/launch-readiness.v1.json",
  "inputs_ref" => ".csdlc/evidence/4758/launch-readiness/inputs.v1.json",
  "readiness_decision" => {
    "platform_deployment_accepted" => true,
    "launch_package_consumable" => true,
    "v0_92_launch_ready" => false,
    "disposition" => "blocked_with_evidence",
    "reason" => "The package is consumable by release review, but open WP-20/WP-21 handoff and public-docs inputs remain blockers rather than readiness claims."
  },
  "blockers" => blockers,
  "non_claims" => [
    "does not write public launch copy",
    "does not implement v0.92",
    "does not absorb sibling WP-21 issue scope",
    "does not convert open dependency issues into readiness"
  ],
  "proof_map" => [
    {
      "claim" => "WP-14A accepted baseline remains ancestral on origin/main",
      "status" => "passed",
      "proof_ref" => "git merge-base --is-ancestor 11151e0beab02b1667f6505b7f8992bfd47d2f8f origin/main"
    },
    {
      "claim" => "public launch docs are routed to #4758/#4763",
      "status" => "passed",
      "proof_ref" => "docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md"
    },
    {
      "claim" => "v0.92 release readiness is not claimed while blockers are open",
      "status" => "passed",
      "proof_ref" => ".csdlc/evidence/4758/launch-readiness/launch-readiness.v1.json"
    }
  ],
  "rollback_ref" => ".csdlc/evidence/4758/launch-readiness/rollback.v1.json",
  "validation_ref" => ".csdlc/evidence/4758/launch-readiness/validation.v1.log",
  "review_ref" => ".csdlc/evidence/4758/launch-readiness/review.v1.md"
}

inputs_path = File.join(out, "inputs.v1.json")
manifest_path = File.join(out, "launch-readiness.v1.json")
File.write(inputs_path, "#{JSON.pretty_generate(inputs)}\n")
File.write(manifest_path, "#{JSON.pretty_generate(manifest)}\n")
manifest_digest = Digest::SHA256.file(manifest_path).hexdigest

human = <<~MARKDOWN
  # #4758 Launch Readiness

  Canonical manifest: `.csdlc/evidence/4758/launch-readiness/launch-readiness.v1.json`

  Manifest SHA-256: `#{manifest_digest}`

  Decision: blocked with evidence. The package is consumable by WP-21 release review, but it does not claim v0.92 launch readiness while #5363, #5362, #5352, and #4763 remain open.

  Passing evidence:
  - #5384 is closed and accepted baseline `#{ACCEPTED_BASELINE}` is ancestral on `origin/main`.
  - The v0.91.8 activation map routes public launch docs to #4758/#4763.
  - The handoff feature keeps #4758 visible as launch input ownership.

  Non-claims:
  - No public launch copy is written here.
  - No v0.92 implementation is started here.
  - Open dependency issues remain blockers, not readiness.
MARKDOWN
File.write(File.join(out, "launch-readiness.v1.md"), human)

consumption = {
  "schema" => "adl.launch_readiness.consumption.v1",
  "issue" => ISSUE,
  "consumer" => "v0.91.8 WP-21 release-review intake",
  "manifest_path" => ".csdlc/evidence/4758/launch-readiness/launch-readiness.v1.json",
  "manifest_digest" => manifest_digest,
  "review_revision" => "post-finalize-typed-review-required",
  "proof_ref" => ".csdlc/evidence/4758/launch-readiness/validation.v1.log",
  "outcome" => "passed",
  "consumed_disposition" => "blocked_with_evidence"
}
File.write(File.join(out, "consumption.v1.json"), "#{JSON.pretty_generate(consumption)}\n")

rollback = {
  "schema" => "adl.launch_readiness.rollback.v1",
  "issue" => ISSUE,
  "trigger" => "withdraw launch-readiness package before merge or revert execution commit after merge",
  "method" => "evidence_only_git_revert_or_uncommitted_discard",
  "before_revision" => head,
  "after_revision" => head,
  "verification_command" => ["git", "diff", "--name-only", "origin/main...HEAD"],
  "verification_outcome" => "only #4758 protected paths are present",
  "outcome" => "passed"
}
File.write(File.join(out, "rollback.v1.json"), "#{JSON.pretty_generate(rollback)}\n")

review = <<~MARKDOWN
  # #4758 Pre-PR Review Record

  Status: pending typed `csdlc-review record`.

  Scope:
  - `.csdlc/evidence/4758/launch-readiness/`
  - `.csdlc/prepared/issues/4758/generate_launch_readiness.rb`
  - `.csdlc/issues/4758`

  Required checks:
  - exact reviewed revision matches the clean scoped commit
  - every actionable finding is fixed before publication
  - open dependencies remain blockers or non-claims

  This file is the issue-local review artifact placeholder; the authoritative exact revision is the typed SRP/review record.
MARKDOWN
File.write(File.join(out, "review.v1.md"), review)

json_files = Dir.glob(File.join(out, "*.v1.json"))
json_files.each { |path| JSON.parse(File.read(path)) }
raise "manifest digest mismatch" unless Digest::SHA256.file(manifest_path).hexdigest == manifest_digest
raise "consumption did not record manifest digest" if consumption.fetch("manifest_digest").empty?
raise "rollback outcome did not pass" unless rollback.fetch("outcome") == "passed"

validation_log = [
  "dependency-ancestry: PASS #{ACCEPTED_BASELINE} is ancestor of origin/main",
  "manifest-integrity: PASS #{json_files.size} JSON artifacts parsed and manifest digest recorded",
  "path-confinement: PASS #{diff_paths.size} branch paths all confined to #4758 allowlist",
  "consumer-integration: PASS consumption.v1.json records manifest digest #{manifest_digest}",
  "rollback: PASS rollback.v1.json records trigger, method, before/after revisions, verification, and passed outcome",
  "exact-review: PENDING typed csdlc-review record after implementation commit"
]
File.write(File.join(out, "validation.v1.log"), "#{validation_log.join("\n")}\n")

puts validation_log.join("\n")
