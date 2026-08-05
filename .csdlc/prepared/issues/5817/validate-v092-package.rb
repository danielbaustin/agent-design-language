# frozen_string_literal: true

require "json"
require "pathname"
require "set"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
WAVE_PATH = File.join(ROOT, "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml")
WBS_PATH = File.join(ROOT, "docs/milestones/v0.92/WBS_v0.92.md")
CARD_NAMES = %w[sip stp spp vpp srp sor].freeze

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
raise "issue wave is not card-initialized" unless wave.fetch("status") == "cards_initialized"
raise "issue wave milestone is not v0.92" unless wave.fetch("milestone") == "v0.92"

rows = wave.fetch("work_packages")
supporting_rows = wave.fetch("supporting_issues")
wps = rows.map { |row| row.fetch("wp") }
issues = rows.map { |row| Integer(row.fetch("issue")) }
raise "expected 39 work packages, found #{rows.length}" unless rows.length == 39
raise "expected three supporting issues" unless supporting_rows.length == 3
raise "supporting issue set differs" unless supporting_rows.map { |row| Integer(row.fetch("issue")) }.sort == [5795, 5800, 5812]
raise "duplicate WP identifier" unless wps.uniq.length == wps.length
all_issues = issues + supporting_rows.map { |row| Integer(row.fetch("issue")) }
raise "duplicate issue mapping" unless all_issues.uniq.length == all_issues.length
raise "WP-02 must be repository migration issue 5819" unless rows.any? { |row| row["wp"] == "WP-02" && row["issue"] == 5819 }
raise "WP-02A must be CI issue 5801" unless rows.any? { |row| row["wp"] == "WP-02A" && row["issue"] == 5801 }
raise "WP-02B must be build acceleration issue 5853" unless rows.any? { |row| row["wp"] == "WP-02B" && row["issue"] == 5853 }

wbs_rows = File.read(WBS_PATH).scan(/^\| (WP-[^ |]+) \| ([^|]+?) \|/).map do |wp, title|
  [wp, title.strip]
end
wave_rows = rows.map { |row| [row.fetch("wp"), row.fetch("title")] }
raise "WBS and issue-wave rows differ" unless wbs_rows == wave_rows

known = wps.to_set
edges = Hash.new { |hash, key| hash[key] = Set.new }
rows.each do |row|
  Array(row["depends_on"]).flat_map { |value| value.scan(/WP-\d+[A-Z]?/) }.each do |dependency|
    raise "unknown dependency #{dependency} for #{row.fetch("wp")}" unless known.include?(dependency)
    edges[row.fetch("wp")] << dependency
  end
end

visiting = Set.new
visited = Set.new
visit = lambda do |wp|
  raise "dependency cycle at #{wp}" if visiting.include?(wp)
  return if visited.include?(wp)

  visiting << wp
  edges[wp].each { |dependency| visit.call(dependency) }
  visiting.delete(wp)
  visited << wp
end
wps.each { |wp| visit.call(wp) }

child_rows = rows.reject { |row| row.fetch("wp") == "WP-01" } + supporting_rows
child_rows.each do |row|
  issue = Integer(row.fetch("issue"))
  issue_root = File.join(ROOT, ".csdlc/issues", issue.to_s)
  record = JSON.parse(File.read(File.join(issue_root, "index.json")))
  raise "issue #{issue} is not initialized" unless record.fetch("phase") == "initialized"
  raise "issue #{issue} record identity mismatch" unless record.fetch("issue") == issue

  CARD_NAMES.each do |card|
    %W[#{card}.md #{card}.values.json].each do |name|
      path = File.join(issue_root, "cards", name)
      raise "issue #{issue} missing #{name}" unless File.file?(path) && !File.zero?(path)
    end

    values = JSON.parse(File.read(File.join(issue_root, "cards", "#{card}.values.json")))
    identity = values.fetch("identity")
    content = values.fetch("content")
    raise "issue #{issue} #{card} identity mismatch" unless identity.fetch("issue") == issue
    raise "issue #{issue} #{card} version mismatch" unless identity.fetch("version") == "v0.92"
    raise "issue #{issue} #{card} kind mismatch" unless content.fetch("card_kind") == card
    raise "issue #{issue} #{card} values missing" unless content.fetch("values").is_a?(Hash)
  end

  stp = JSON.parse(File.read(File.join(issue_root, "cards/stp.values.json")))
    .fetch("content").fetch("values")
  expected_deliverables = row.fetch("deliverables", [row.fetch("primary_deliverable"), row.fetch("proof_surface")])
  expected_task = row.fetch("task_boundary", "Deliver #{row.fetch("primary_deliverable")}.")
  raise "issue #{issue} task does not match wave" unless stp.fetch("task_boundary") == expected_task
  raise "issue #{issue} deliverables do not match wave" unless stp.fetch("deliverables") == expected_deliverables
  raise "issue #{issue} dependencies do not match wave" unless stp.fetch("dependencies") == Array(row["depends_on"]).map(&:to_s)
end

claim_paths = child_rows.flat_map do |row|
  record = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues", row.fetch("issue").to_s, "index.json")))
  record.fetch("claim").fetch("protected_paths")
end
path_counts = claim_paths.each_with_object(Hash.new(0)) { |path, counts| counts[path] += 1 }
duplicates = path_counts.select { |_path, count| count > 1 }
raise "duplicate child claim paths: #{duplicates.keys.join(", ")}" unless duplicates.empty?

sprints = wave.fetch("execution_sprints")
raise "expected five execution sprints" unless sprints.length == 5
sprint_names = sprints.map { |sprint| sprint.fetch("sprint") }
raise "duplicate execution sprint" unless sprint_names.uniq.length == sprint_names.length
sprint_issues = sprints.map { |sprint| Integer(sprint.fetch("issue")) }
raise "sprint umbrella issue set differs" unless sprint_issues.sort == [5854, 5855, 5856, 5857, 5858]
expected_members = rows.reject { |row| row.fetch("wp") == "WP-01" }.map { |row| row.fetch("wp") } +
  supporting_rows.map { |row| "issue-#{row.fetch("issue")}" }
actual_members = sprints.flat_map { |sprint| sprint.fetch("members").map(&:to_s) }
raise "duplicate sprint member" unless actual_members.uniq.length == actual_members.length
raise "sprint membership differs from child issue wave" unless actual_members.sort == expected_members.sort
sprints.each do |sprint|
  raise "sprint lacks execution mode" unless %w[sequential parallel hybrid].include?(sprint.fetch("execution_mode"))
  raise "sprint lacks serial gates" unless sprint.fetch("serial_gates").is_a?(Array)
  raise "sprint lacks parallel-lane declaration" unless sprint.fetch("parallel_lanes").is_a?(Array)
  raise "sprint lacks write-surface boundary" if sprint.fetch("write_surface").strip.empty?

  prompt_path = File.join(ROOT, sprint.fetch("session_prompt_path"))
  raise "sprint #{sprint.fetch("sprint")} session prompt missing" unless File.file?(prompt_path) && !File.zero?(prompt_path)
  prompt = File.read(prompt_path)
  raise "sprint #{sprint.fetch("sprint")} prompt omits AGENTS.md" unless prompt.include?("AGENTS.md")
  unless prompt.match?(/issue-bound goal|own goal|child issue goal|session goal/)
    raise "sprint #{sprint.fetch("sprint")} prompt omits child-goal rule"
  end
  raise "sprint #{sprint.fetch("sprint")} prompt omits main protection" unless prompt.include?("main")
  unless prompt.include?("/private/tmp")
    raise "sprint #{sprint.fetch("sprint")} prompt omits /private/tmp prohibition"
  end

  issue = Integer(sprint.fetch("issue"))
  issue_root = File.join(ROOT, ".csdlc/issues", issue.to_s)
  record = JSON.parse(File.read(File.join(issue_root, "index.json")))
  raise "sprint issue #{issue} is not initialized" unless record.fetch("phase") == "initialized"
  raise "sprint issue #{issue} record identity mismatch" unless record.fetch("issue") == issue
  CARD_NAMES.each do |card|
    %W[#{card}.md #{card}.values.json].each do |name|
      path = File.join(issue_root, "cards", name)
      raise "sprint issue #{issue} missing #{name}" unless File.file?(path) && !File.zero?(path)
    end
    values = JSON.parse(File.read(File.join(issue_root, "cards", "#{card}.values.json")))
    identity = values.fetch("identity")
    content = values.fetch("content")
    raise "sprint issue #{issue} #{card} identity mismatch" unless identity.fetch("issue") == issue
    raise "sprint issue #{issue} #{card} version mismatch" unless identity.fetch("version") == "v0.92"
    raise "sprint issue #{issue} #{card} kind mismatch" unless content.fetch("card_kind") == card
  end

  packet_path = File.join(ROOT, ".csdlc/prepared/issues", issue.to_s, "sprint-execution-packet.yaml")
  packet = YAML.safe_load(File.read(packet_path), aliases: true)
  raise "sprint packet #{issue} identity mismatch" unless Integer(packet.fetch("sprint_issue")) == issue
  member_issues = sprint.fetch("members").map do |member|
    member = member.to_s
    if member.start_with?("issue-")
      Integer(member.delete_prefix("issue-"))
    else
      Integer(rows.find { |row| row.fetch("wp") == member }.fetch("issue"))
    end
  end
  raise "sprint packet #{issue} membership differs" unless packet.fetch("ordered_issue_numbers") == member_issues
  raise "sprint packet #{issue} lacks review path" if packet.fetch("review_path").strip.empty?
  raise "sprint packet #{issue} lacks activity log" if packet.fetch("activity_log_path").strip.empty?
  raise "sprint packet #{issue} lacks child goal rule" if packet.fetch("child_goal_rule").strip.empty?

  human_packet_path = File.join(ROOT, ".csdlc/prepared/issues", issue.to_s, "sprint-execution-packet.md")
  human_packet = File.read(human_packet_path)
  required_sections = [
    "## Child Issue Wave",
    "## Recommended Execution Order",
    "## Watcher Policy",
    "## Budget And Goal Accounting",
    "## Watcher Plan",
    "## Safe Parallel Lanes",
    "## Candidate Parallel Lanes",
    "## Serial Gates",
    "## Parallelism Outcome Plan",
    "## Sprint Closeout Rollup Expectations"
  ]
  missing_sections = required_sections.reject { |section| human_packet.include?(section) }
  raise "sprint packet #{issue} missing sections: #{missing_sections.join(", ")}" unless missing_sections.empty?
end

expected_sources = %w[
  .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
  .adl/docs/TBD/POST_GITHUB_MIGRATION_BUILD_ACCELERATION_EXPERIMENT_PLAN.md
  .adl/docs/TBD/publication/ADL_MEDIUM_ARTICLE_LIST.md
  .adl/docs/TBD/publication/MEDIUM_ARTICLE_SERIES_PLAN.md
  .adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md
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
missing_sources = expected_sources - disposition_sources
raise "missing source dispositions: #{missing_sources.join(", ")}" unless missing_sources.empty?

required_docs = %w[
  QUALITY_GATE_v0.92.md
  FEATURE_PROOF_COVERAGE_v0.92.md
  WP_EXECUTION_READINESS_v0.92.md
  NEXT_MILESTONE_HANDOFF_v0.92.md
]
required_docs.each do |name|
  raise "missing #{name}" unless File.file?(File.join(ROOT, "docs/milestones/v0.92", name))
end

coverage_audit = File.join(ROOT, ".csdlc/evidence/5817/feature-and-issue-coverage-audit.md")
raise "missing feature and live-issue coverage audit" unless File.file?(coverage_audit) && !File.zero?(coverage_audit)

feature_index = File.read(File.join(ROOT, "docs/milestones/v0.92/features/README.md"))
required_features = %w[
  ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
  ACP_COGNITIVE_PROFILES_v0.92.md
  ADAPTIVE_LEARNING_DAG_v0.92.md
  CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md
  DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
  FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md
  FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
  IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
  MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
  MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md
  OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md
  PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md
  RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md
]
required_features.each do |name|
  path = File.join(ROOT, "docs/milestones/v0.92/features", name)
  raise "missing feature contract #{name}" unless File.file?(path) && !File.zero?(path)
  raise "feature index omits #{name}" unless feature_index.include?(name)
end
raise "feature completion gate missing" unless feature_index.include?("## v0.92 Completion Gate")

wp01b = rows.find { |row| row.fetch("wp") == "WP-01B" }
raise "WP-01B omits canonical feature list" unless wp01b.fetch("primary_deliverable").include?("docs/planning/ADL_FEATURE_LIST.md")
wp04 = rows.find { |row| row.fetch("wp") == "WP-04" }
raise "WP-04 does not require all 16 child issues" unless wp04.fetch("proof_surface").include?("all 16 child issues landed")
wp22 = rows.find { |row| row.fetch("wp") == "WP-22" }
raise "WP-22 does not gate all v0.92 features" unless wp22.fetch("primary_deliverable").include?("every indexed v0.92 feature")
delivery_standard = wave.fetch("delivery_standard")
raise "delivery standard is incomplete" unless delivery_standard.length >= 7
raise "delivery standard permits synthetic proof" unless delivery_standard.any? { |rule| rule.include?("synthetic success") }
raise "delivery standard permits intent-only docs" unless delivery_standard.any? { |rule| rule.include?("restates intent") }

canonical = %w[README.md WBS_v0.92.md SPRINT_v0.92.md WP_ISSUE_WAVE_v0.92.yaml].map do |name|
  File.read(File.join(ROOT, "docs/milestones/v0.92", name))
end.join("\n")
raise "stale unopened-wave language remains" if canonical.include?("not an opened GitHub issue wave")
raise "stale WP-01A numbering remains" if canonical.include?("WP-01A")
raise "stale candidate-wave language remains" if canonical.match?(/Candidate (WP Sequence|issue wave)/)
raise "stale pre-open sprint language remains" if canonical.include?("should be generated after WP-01")

milestone_root = File.join(ROOT, "docs/milestones/v0.92")
milestone_text = Dir.glob(File.join(milestone_root, "**/*.{md,yaml}")).sort.map { |path| File.read(path) }.join("\n")
forbidden_proof_credit = [
  "mock WebSocket",
  "mock/loopback WebSocket",
  "synthetic but structurally complete",
  "mock or loopback WebSocket proof",
  "mock WebSocket session"
]
forbidden_proof_credit.each do |phrase|
  raise "stale mock or synthetic release credit remains: #{phrase}" if milestone_text.include?(phrase)
end

Dir.glob(File.join(milestone_root, "**/*.md")).sort.each do |markdown_path|
  text = File.read(markdown_path)
  text.scan(/\[[^\]]+\]\(([^)]+)\)/).flatten.each do |raw_target|
    target = raw_target.strip.delete_prefix("<").delete_suffix(">")
    next if target.empty? || target.start_with?("#", "http://", "https://", "mailto:")

    relative = target.split("#", 2).first.split("?", 2).first
    next if relative.empty?

    resolved = File.expand_path(relative, File.dirname(markdown_path))
    raise "broken Markdown link #{target} in #{Pathname.new(markdown_path).relative_path_from(Pathname.new(ROOT))}" unless File.exist?(resolved)
  end
end

card_artifact_count = (child_rows.length + sprints.length) * CARD_NAMES.length * 2
puts "v0.92 WP-01 validation passed: #{rows.length} WPs, #{child_rows.length} child issues, #{sprints.length} sprint umbrellas, #{card_artifact_count} card artifacts"
