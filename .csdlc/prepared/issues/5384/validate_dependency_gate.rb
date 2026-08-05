#!/usr/bin/env ruby

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
manifest = JSON.parse(File.read(File.join(__dir__, "dependency-gate.json")))
failures = []

def capture_json(*command)
  output, status = Open3.capture2e(*command)
  return [nil, output] unless status.success?

  [JSON.parse(output.lines.reject { |line| line.start_with?("adl_event ") }.join), nil]
rescue JSON::ParserError => error
  [nil, "invalid JSON from #{command.first}: #{error.message}"]
end

expected_inputs = [5358, 5361, 5344, 5343]
actual_inputs = manifest.fetch("direct_inputs").map { |entry| entry.fetch("issue") }
failures << "direct inputs must be exactly #{expected_inputs.inspect}" unless actual_inputs == expected_inputs

deferred = manifest.fetch("deferred_non_blocking").first
failures << "WP-13 deletion deferral is missing" unless deferred.fetch("issues") == [5346, 5347]
failures << "WP-13 must execute immediately before #5356" unless deferred.fetch("execute_before") == 5356

base_sha, base_status = Open3.capture2("git", "-C", root, "rev-parse", manifest.fetch("base_ref"))
failures << "cannot resolve #{manifest.fetch("base_ref")}" unless base_status.success?
accepted_baseline = manifest.fetch("accepted_baseline_sha")
if base_status.success?
  _, ancestry_status = Open3.capture2(
    "git", "-C", root, "merge-base", "--is-ancestor", accepted_baseline, base_sha.strip
  )
  failures << "accepted baseline #{accepted_baseline} is not in #{manifest.fetch("base_ref")}" unless ancestry_status.success?
end

issue_bin = ENV.fetch("ADL_ISSUE_BIN", "adl-issue")
pr_bin = ENV.fetch("ADL_PR_VALIDATION_BIN", "adl-pr-validation")

manifest.fetch("direct_inputs").each do |entry|
  issue = entry.fetch("issue")
  issue_state, issue_error = capture_json(
    issue_bin, "view", issue.to_s, "-R", "danielbaustin/agent-design-language", "--json"
  )
  if issue_error
    failures << "##{issue}: cannot read live issue state: #{issue_error.lines.last&.strip}"
  elsif issue_state.fetch("state").downcase != "closed"
    failures << "##{issue}: expected closed, observed #{issue_state.fetch("state")}"
  end

  pr = entry.fetch("pull_request")
  pr_state, pr_error = capture_json(
    pr_bin, pr.to_s, "-R", "danielbaustin/agent-design-language", "--json"
  )
  if pr_error
    failures << "PR ##{pr}: cannot read validation state: #{pr_error.lines.last&.strip}"
  else
    failures << "PR ##{pr}: expected MERGED" unless pr_state.fetch("pr_state") == "MERGED"
    failures << "PR ##{pr}: required checks are not green" unless pr_state.fetch("disposition") == "success"
    unless pr_state.fetch("commit_sha") == entry.fetch("reviewed_head")
      failures << "PR ##{pr}: reviewed head drift"
    end
  end

  merge_revision = entry.fetch("merge_revision")
  _, merge_status = Open3.capture2(
    "git", "-C", root, "merge-base", "--is-ancestor", merge_revision, accepted_baseline
  )
  failures << "##{issue}: merge revision is absent from accepted baseline" unless merge_status.success?
end

result = {
  schema: "adl.wp14a.direct_input_gate.result.v2",
  issue: manifest.fetch("issue"),
  accepted_baseline_sha: accepted_baseline,
  current_base_sha: base_sha.strip,
  ready: failures.empty?,
  failures: failures
}
puts JSON.pretty_generate(result)
exit(failures.empty? ? 0 : 3)
