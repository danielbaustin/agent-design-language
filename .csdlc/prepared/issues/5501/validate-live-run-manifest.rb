#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
SCHEMA = "adl.wp10a.live-workcell-manifest.v1"
HEX40 = /\A[0-9a-f]{40}\z/
HEX64 = /\A(?:sha256:)?[0-9a-f]{64}\z/
SHARD_KEYS = %w[
  issue claim_id claim_generation claim_owner branch worktree source_revision
  protected_paths write_paths task_id context_envelope_digest output_ref
  output_digest review_ref
].freeze
FORBIDDEN_KEYS = /credential|secret|token|password|private_transcript/i

def fail_closed(message)
  warn(message)
  exit 2
end

def object(value, label)
  fail_closed("#{label} must be an object") unless value.is_a?(Hash)
  value
end

def text(value, label)
  fail_closed("#{label} must be non-empty text") unless value.is_a?(String) && !value.empty?
  value
end

def digest(value, label)
  text(value, label)
  fail_closed("#{label} must be a SHA-256 digest") unless value.match?(HEX64)
  value.delete_prefix("sha256:")
end

def revision(value, label)
  text(value, label)
  fail_closed("#{label} must be an exact Git revision") unless value.match?(HEX40)
end

def repo_paths(value, label)
  fail_closed("#{label} must be a non-empty array") unless value.is_a?(Array) && !value.empty?
  normalized = value.map.with_index do |path, index|
    text(path, "#{label}[#{index}]")
    candidate = Pathname.new(path)
    fail_closed("#{label}[#{index}] must be repository-relative") if candidate.absolute?
    clean = candidate.cleanpath.to_s
    fail_closed("#{label}[#{index}] escapes the repository") if clean == ".." || clean.start_with?("../")
    fail_closed("#{label}[#{index}] is not normalized") unless clean == path
    clean
  end
  fail_closed("#{label} contains duplicates") unless normalized.uniq.length == normalized.length
  normalized
end

def evidence_ref(value, expected_digest, label)
  text(value, "#{label}.ref")
  candidate = Pathname.new(value)
  fail_closed("#{label}.ref must be repository-relative") if candidate.absolute?
  clean = candidate.cleanpath.to_s
  fail_closed("#{label}.ref escapes the repository") if clean == ".." || clean.start_with?("../")
  fail_closed("#{label}.ref is not normalized") unless clean == value

  path = ROOT.join(clean).cleanpath
  fail_closed("#{label}.ref escapes the repository") unless path.to_s.start_with?("#{ROOT}/")
  fail_closed("#{label}.ref is absent") unless path.file?

  expected = digest(expected_digest, "#{label}.digest")
  actual = Digest::SHA256.file(path).hexdigest
  fail_closed("#{label}.digest does not match #{clean}") unless actual == expected
  clean
end

def overlap?(left, right)
  left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
end

def reject_forbidden_keys(value, label = "manifest")
  case value
  when Hash
    value.each do |key, child|
      fail_closed("forbidden manifest key at #{label}: #{key}") if key.match?(FORBIDDEN_KEYS)
      reject_forbidden_keys(child, "#{label}.#{key}")
    end
  when Array
    value.each_with_index { |child, index| reject_forbidden_keys(child, "#{label}[#{index}]") }
  end
end

begin
  path = ARGV.fetch(0) { abort("usage: validate-live-run-manifest.rb <manifest.json>") }
  manifest_path = Pathname.new(path)
  fail_closed("live-run manifest is absent") unless manifest_path.file?

manifest = JSON.parse(manifest_path.read)
object(manifest, "manifest")
fail_closed("unexpected manifest schema") unless manifest["schema"] == SCHEMA
fail_closed("template is not admissible live evidence") unless manifest["template"] == false
text(manifest["run_id"], "run_id")

reject_forbidden_keys(manifest)

plan = object(manifest["admitted_plan"], "admitted_plan")
fail_closed("admitted plan must come from #5499") unless plan["issue"] == 5499
revision(plan["revision"], "admitted_plan.revision")
evidence_ref(plan["review_ref"], plan["digest"], "admitted_plan")

negative = object(manifest["negative_case"], "negative_case")
text(negative["kind"], "negative_case.kind")
fail_closed("negative case must retain a real refusal") unless negative["refused"] == true
evidence_ref(negative["evidence_ref"], negative["evidence_digest"], "negative_case")

shards = manifest["shards"]
fail_closed("live proof requires two to four real shards") unless shards.is_a?(Array) && (2..4).cover?(shards.length)
identities = []
all_paths = []
shards.each_with_index do |raw, index|
  shard = object(raw, "shards[#{index}]")
  missing = SHARD_KEYS.reject { |key| shard.key?(key) }
  fail_closed("shards[#{index}] omits #{missing.join(', ')}") unless missing.empty?
  fail_closed("shards[#{index}].issue must be positive") unless shard["issue"].is_a?(Integer) && shard["issue"].positive?
  text(shard["claim_id"], "shards[#{index}].claim_id")
  fail_closed("shards[#{index}].claim_generation must be positive") unless shard["claim_generation"].is_a?(Integer) && shard["claim_generation"].positive?
  %w[claim_owner branch worktree task_id review_ref].each do |key|
    text(shard[key], "shards[#{index}].#{key}")
  end
  revision(shard["source_revision"], "shards[#{index}].source_revision")
  digest(shard["context_envelope_digest"], "shards[#{index}].context_envelope_digest")
  evidence_ref(shard["output_ref"], shard["output_digest"], "shards[#{index}].output")
  protected_paths = repo_paths(shard["protected_paths"], "shards[#{index}].protected_paths")
  write_paths = repo_paths(shard["write_paths"], "shards[#{index}].write_paths")
  fail_closed("shards[#{index}] writes outside its protected paths") unless write_paths.all? do |write|
    protected_paths.any? { |protected| write == protected || write.start_with?("#{protected}/") }
  end
  identity = [shard["issue"], shard["claim_id"], shard["branch"], shard["worktree"], shard["task_id"]]
  fail_closed("duplicate shard identity") if identities.include?(identity)
  identities << identity
  all_paths << [index, protected_paths + write_paths]
end

all_paths.combination(2) do |(left_index, left_paths), (right_index, right_paths)|
  collision = left_paths.product(right_paths).find { |left, right| overlap?(left, right) }
  fail_closed("shards #{left_index} and #{right_index} overlap at #{collision.join(' / ')}") if collision
end

dashboard = object(manifest["dashboard"], "dashboard")
fail_closed("dashboard must come from #5500") unless dashboard["source_issue"] == 5500
fail_closed("dashboard cannot contain manual green assertions") unless dashboard["manual_assertions"] == false
evidence_ref(dashboard["observation_ref"], dashboard["observation_digest"], "dashboard.observation")

convergence = object(manifest["convergence"], "convergence")
fail_closed("convergence must come from #5502") unless convergence["source_issue"] == 5502
evidence_ref(convergence["decision_ref"], convergence["decision_digest"], "convergence.decision")

baseline = object(manifest["baseline"], "baseline")
evidence_ref(
  baseline["equivalence_review_ref"],
  baseline["declared_work_digest"],
  "baseline.equivalence_review"
)
fail_closed("baseline budget must be 1..1800 seconds") unless baseline["budget_seconds"].is_a?(Integer) && (1..1800).cover?(baseline["budget_seconds"])

review = object(manifest["manifest_review"], "manifest_review")
text(review["reviewer"], "manifest_review.reviewer")
evidence_ref(review["review_ref"], review["reviewed_digest"], "manifest_review")
fail_closed("manifest review must pass") unless review["result"] == "pass"

  puts JSON.pretty_generate(status: "pass", shards: shards.length, run_id: manifest["run_id"])
rescue JSON::ParserError => e
  fail_closed("live-run manifest is malformed: #{e.message}")
end
