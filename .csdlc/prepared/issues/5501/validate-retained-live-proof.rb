#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
HEX40 = /\A[0-9a-f]{40}\z/
TASK_ID = /\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/
FORBIDDEN_KEYS = /credential|secret|token|password|private_transcript/i

def fail_closed(message)
  warn(message)
  exit 2
end

def read_json(path)
  JSON.parse(path.read)
rescue JSON::ParserError => e
  fail_closed("#{path} is malformed JSON: #{e.message}")
end

def object(value, label)
  fail_closed("#{label} must be an object") unless value.is_a?(Hash)
  value
end

def array(value, label)
  fail_closed("#{label} must be an array") unless value.is_a?(Array)
  value
end

def text(value, label)
  fail_closed("#{label} must be non-empty text") unless value.is_a?(String) && !value.empty?
  value
end

def revision(value, label)
  text(value, label)
  fail_closed("#{label} must be an exact Git revision") unless value.match?(HEX40)
  value
end

def repo_paths(value, label)
  paths = array(value, label)
  fail_closed("#{label} must not be empty") if paths.empty?
  paths.map.with_index do |path, index|
    text(path, "#{label}[#{index}]")
    candidate = Pathname.new(path)
    fail_closed("#{label}[#{index}] must be repository-relative") if candidate.absolute?
    clean = candidate.cleanpath.to_s
    fail_closed("#{label}[#{index}] escapes the repository") if clean == ".." || clean.start_with?("../")
    fail_closed("#{label}[#{index}] is not normalized") unless clean == path
    clean
  end
end

def overlap?(left, right)
  left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
end

def reject_forbidden_keys(value, label = "proof")
  case value
  when Hash
    value.each do |key, child|
      fail_closed("forbidden key at #{label}: #{key}") if key.match?(FORBIDDEN_KEYS)
      reject_forbidden_keys(child, "#{label}.#{key}")
    end
  when Array
    value.each_with_index { |child, index| reject_forbidden_keys(child, "#{label}[#{index}]") }
  end
end

proof_path = ROOT.join(".csdlc/evidence/5501/retained-live-proof.json")
baseline_path = ROOT.join(".csdlc/evidence/5501/single-agent-comparison.json")
negative_path = ROOT.join(".csdlc/evidence/5501/negative-case-refusal.json")
convergence_path = ROOT.join(".csdlc/evidence/5501/convergence-decision.json")
binding_path = ROOT.join(".csdlc/prepared/issues/5501/evidence-review-binding.json")

[proof_path, baseline_path, negative_path, convergence_path, binding_path].each do |path|
  fail_closed("#{path.relative_path_from(ROOT)} is absent") unless path.file?
end

proof = object(read_json(proof_path), "proof")
reject_forbidden_keys(proof)
fail_closed("unexpected proof schema") unless proof["schema"] == "adl.wp10a.retained-live-proof.v1"
fail_closed("proof issue must be 5501") unless proof["issue"] == 5501
revision(proof["execution_head"], "execution_head")
revision(proof["merged_dependency_head"], "merged_dependency_head")
unless proof["execution_head_role"] == "dependency_integration_head"
  fail_closed("execution_head role must distinguish dependency integration from evidence review")
end
unless proof["review_binding_ref"] == binding_path.relative_path_from(ROOT).to_s
  fail_closed("proof review binding ref is absent or stale")
end
fail_closed("proof must state transcript exclusion") unless text(proof["task_content_boundary"], "task_content_boundary").include?("private transcript")

binding = object(read_json(binding_path), "review_binding")
unless binding["schema"] == "adl.wp10a.evidence-review-binding.v1" && binding["issue"] == 5501
  fail_closed("review binding identity mismatch")
end
evidence_revision = revision(binding["evidence_revision"], "review_binding.evidence_revision")
scope = repo_paths(binding["scope"], "review_binding.scope")
unless binding["relation"] == "reviewed_head_must_descend_from_evidence_revision_with_zero_scoped_diff"
  fail_closed("review binding relation mismatch")
end
ancestor = system(
  "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", evidence_revision, "HEAD",
  out: File::NULL, err: File::NULL
)
fail_closed("evidence revision is not ancestral to reviewed head") unless ancestor
unchanged = system(
  "git", "-C", ROOT.to_s, "diff", "--quiet", "#{evidence_revision}..HEAD", "--", *scope,
  out: File::NULL, err: File::NULL
)
fail_closed("reviewed evidence differs from its bound evidence revision") unless unchanged
execution_ancestral = system(
  "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", proof["execution_head"], evidence_revision,
  out: File::NULL, err: File::NULL
)
fail_closed("dependency integration head is not ancestral to evidence revision") unless execution_ancestral

shards = array(proof["shards"], "shards")
fail_closed("proof must retain exactly two real shards") unless shards.length == 2
expected = {
  5500 => {
    task_id: "019f8c3b-ec87-7741-a212-812501f7fc4a",
    branch: "codex/5500-v0918-wp10a-dashboard",
    source_revision: "a24992cfaecfb6adaa2f82ea1b780dd7d1cc6803",
    merge_revision: "fa49c2d0f32147547f0aafdca8bfbc841c49258a"
  },
  5502 => {
    task_id: "019f8c3b-ed47-78e1-a671-d78b1fd3a063",
    branch: "codex/5502-v0918-preparation",
    source_revision: "3b900210cd30ee381c860a624499d1e4d8aea0d8",
    merge_revision: "1cbbf4eb5531814f7b4f0fdc9edeaa1df78410cd"
  }
}
seen = []
paths_by_issue = {}
shards.each_with_index do |raw, index|
  shard = object(raw, "shards[#{index}]")
  issue = shard["issue"]
  fail_closed("unexpected shard issue #{issue.inspect}") unless expected.key?(issue)
  exp = expected.fetch(issue)
  fail_closed("shards[#{index}].task_id mismatch") unless shard["task_id"] == exp[:task_id] && shard["task_id"].match?(TASK_ID)
  fail_closed("shards[#{index}].branch mismatch") unless shard["branch"] == exp[:branch]
  fail_closed("shards[#{index}].source_revision mismatch") unless shard["source_revision"] == exp[:source_revision]
  fail_closed("shards[#{index}].merge_revision mismatch") unless shard["merge_revision"] == exp[:merge_revision]
  fail_closed("shards[#{index}].outcome must be merged") unless shard["outcome"] == "merged"
  fail_closed("duplicate shard issue") if seen.include?(issue)
  seen << issue
  revision(shard["source_revision"], "shards[#{index}].source_revision")
  revision(shard["merge_revision"], "shards[#{index}].merge_revision")
  text(shard["claim_id"], "shards[#{index}].claim_id")
  text(shard["claim_owner"], "shards[#{index}].claim_owner")
  fail_closed("claim generation must be positive") unless shard["claim_generation"].is_a?(Integer) && shard["claim_generation"].positive?
  repo_paths(shard["protected_paths"], "shards[#{index}].protected_paths")
  repo_paths(shard["write_paths"], "shards[#{index}].write_paths")
  paths_by_issue[issue] = shard["protected_paths"] + shard["write_paths"]
end

paths_by_issue.to_a.combination(2) do |(left_issue, left_paths), (right_issue, right_paths)|
  collision = left_paths.product(right_paths).find { |left, right| overlap?(left, right) }
  fail_closed("shards #{left_issue} and #{right_issue} overlap at #{collision.join(' / ')}") if collision
end

negative = object(read_json(negative_path), "negative")
reject_forbidden_keys(negative, "negative")
fail_closed("negative case must be refused") unless negative["refused"] == true
fail_closed("negative case must prove path overlap") unless negative["refusal_code"] == "PathOverlap"

convergence = object(read_json(convergence_path), "convergence")
fail_closed("convergence decision must integrate") unless convergence["decision"] == "integrate"
fail_closed("convergence must cover both shards") unless convergence["integrated_issues"] == [5500, 5502]

baseline = object(read_json(baseline_path), "baseline")
reject_forbidden_keys(baseline, "baseline")
fail_closed("baseline schema mismatch") unless baseline["schema"] == "adl.wp10a.single-agent-comparison.v1"
unless baseline["baseline_status"] == "measured_lifecycle_window_comparison"
  fail_closed("baseline must retain the measured lifecycle-window comparison")
end
parallel_seconds = baseline.dig("parallel_workcell", "observed_window", "elapsed_seconds")
serialized_seconds = baseline.dig("single_agent_baseline", "serialized_observed_window_seconds")
unless parallel_seconds.is_a?(Integer) && parallel_seconds.positive?
  fail_closed("parallel observed window must be measured in positive seconds")
end
unless serialized_seconds.is_a?(Integer) && serialized_seconds >= parallel_seconds
  fail_closed("serialized observed window must be measured and no shorter than the parallel window")
end
children = array(baseline.dig("parallel_workcell", "child_windows"), "parallel_workcell.child_windows")
fail_closed("comparison must retain both child timing windows") unless children.map { |child| child["issue"] } == [5500, 5502]
children.each_with_index do |child, index|
  fail_closed("child window #{index} lacks measured seconds") unless child["elapsed_seconds"].is_a?(Integer) && child["elapsed_seconds"].positive?
  fail_closed("child window #{index} lacks retry counts") unless child["ci_reruns"].is_a?(Integer) && child["integration_retries"].is_a?(Integer)
  text(child["observed_failure"], "child_windows[#{index}].observed_failure")
end
unless baseline.dig("comparison", "fairness_result") == "measured_bounded_comparison_without_speedup_claim"
  fail_closed("baseline comparison fairness boundary is absent")
end
fail_closed("baseline must not claim numeric speedup") unless baseline.dig("comparison", "numeric_speedup_claim") == false

puts JSON.pretty_generate(
  status: "pass",
  retained_shards: shards.map { |shard| shard["issue"] },
  negative_case: negative["refusal_code"],
  baseline: baseline["baseline_status"]
)
