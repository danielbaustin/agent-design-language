# frozen_string_literal: true

require "yaml"

wave_path = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"
wbs_path = "docs/milestones/v0.92/WBS_v0.92.md"
review_path = ".csdlc/evidence/5359/V092_PLANNING_REVIEW.md"

wave = YAML.safe_load(File.read(wave_path), aliases: true)
raise "issue wave must remain draft_pre_open" unless wave.fetch("status") == "draft_pre_open"

yaml_wps = wave.fetch("work_packages").map { |row| [row.fetch("wp"), row.fetch("title")] }
raise "duplicate WP identifier" unless yaml_wps.map(&:first).uniq.length == yaml_wps.length

wbs_wps = File.read(wbs_path).scan(/^\| (WP-[^ |]+) \| ([^|]+?) \|/).map do |wp, title|
  [wp, title.strip]
end
raise "WBS and issue-wave WP rows differ" unless wbs_wps == yaml_wps

expected_sources = %w[
  .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
  .adl/docs/TBD/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md
  .adl/docs/TBD/resilience/RUNTIME_V3_LONG_LIVED_AGENT_OS_PLAN.md
  .adl/docs/TBD/CSDLC_V2_SESSION_ESTIMATION_RECONNECTION_PLAN.md
  .adl/docs/TBD/workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md
  .adl/docs/TBD/workflow_tooling/planning/REMOTE_BUILD_RUNNER_PILOT_PLAN.md
  .adl/docs/TBD/acip/AGENT_COMMUNICATION_AND_INVOCATION_PROTOCOL.md
  .adl/docs/TBD/a2a/ADL_A2A_ADAPTER.md
  .adl/docs/TBD/capability_testing/ADL_CAPABILITY_TAXONOMY.md
  .adl/docs/TBD/MLX_APPLE_METAL_PROVIDER_PLAN.md
  .adl/docs/TBD/OCI_MODEL_PACKAGING_METHOD_PLAN.md
  .adl/docs/TBD/OBSERVATORY_UNITY_DESIGN.md
  .adl/docs/TBD/multiagent_demos/
  .adl/docs/TBD/workflow_tooling/planning/V0917_PROMPT_CARD_ENUM_TYPING_PLAN.md
  .adl/docs/TBD/CSM_RUNTIME_DISTRIBUTED_DESIGN.md
  .adl/docs/TBD/CSM_RUNTIME_DISTRIBUTED_EXECUTION_PLAN.md
  .adl/docs/TBD/ADL_REPOSITORY_CODE_REDUCTION_PLAN_v0.91.8.md
  .adl/docs/TBD/AGENT_LOGIC_WEBSITE_DESIGN_v2.1.md
  .adl/docs/TBD/AGENT_LOGIC_INVESTOR_MATERIAL_TRACKING_AND_PUBLICATION_PLAN.md
  .adl/docs/TBD/general-intelligence-paper/
  .adl/docs/TBD/publication/medium_launch_articles/1-WHY-ADL.md
]
disposition_sources = %w[scheduled_tbd_inputs deferred_planning later_backlog].flat_map do |key|
  wave.fetch(key).flat_map { |row| row.fetch("sources") }
end
missing = expected_sources - disposition_sources
raise "missing source dispositions: #{missing.join(", ")}" unless missing.empty?

review = File.read(review_path)
%w[Blockers Stale\ Assumptions Overclaims Non-Claims WP-23].each do |heading|
  raise "review packet missing #{heading}" unless review.include?(heading)
end

tracked = [wbs_path, "docs/milestones/v0.92/SPRINT_v0.92.md", wave_path, review_path]
raise "stale five-card policy" if tracked.any? { |path| File.read(path).include?("all five") }

puts "v0.92 package validation passed: #{yaml_wps.length} WPs, #{expected_sources.length} source dispositions"
