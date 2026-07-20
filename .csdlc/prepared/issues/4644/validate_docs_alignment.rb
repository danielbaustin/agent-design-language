#!/usr/bin/env ruby

require "date"
require "digest"
require "json"
require "yaml"

ROOT = File.expand_path(ARGV.fetch(0, "."))
VALIDATION_ISSUE = Integer(ARGV.fetch(1, "4644"))

def fail_check(message)
  warn JSON.generate({ schema: "adl.v0917.wp17.validation_error.v1", error: message })
  exit 1
end

def digest_paths(paths)
  Digest::SHA256.hexdigest(paths.sort.join("\n") + "\n")
end

Dir.chdir(ROOT) do
  files = IO.popen(["git", "ls-tree", "-r", "--name-only", "HEAD"], &:read)
            .lines.map(&:strip).reject(&:empty?)
  audit_path = "docs/milestones/v0.91.7/review/wp17_docs_alignment_4644/audit.json"
  audit = JSON.parse(File.read(audit_path))

  readmes = files.select do |path|
    File.basename(path).match?(/\Areadme.*\.md\z/i) && File.file?(path)
  end.sort
  milestone_files = files.select { |path| path.start_with?("docs/milestones/v0.91.7/") }.sort
  milestone_markdown = milestone_files.select { |path| path.end_with?(".md") && File.file?(path) }
  milestone_json = milestone_files.select { |path| path.end_with?(".json") && File.file?(path) }
  milestone_yaml = milestone_files.select { |path| path.match?(/\.ya?ml\z/) && File.file?(path) }

  inventory = audit.fetch("inventory")
  checks = {
    "readme_count" => readmes.length == inventory.fetch("tracked_readme_markdown_files_case_insensitive"),
    "readme_digest" => digest_paths(readmes) == inventory.fetch("readme_path_list_sha256"),
    "milestone_file_count" => milestone_files.length == inventory.fetch("v0917_tracked_files"),
    "milestone_file_digest" => digest_paths(milestone_files) == inventory.fetch("v0917_path_list_sha256"),
    "milestone_markdown_count" => milestone_markdown.length == inventory.fetch("v0917_markdown_files"),
    "milestone_markdown_digest" => digest_paths(milestone_markdown) == inventory.fetch("v0917_markdown_path_list_sha256"),
    "milestone_json_count" => milestone_json.length == inventory.fetch("v0917_json_files"),
    "milestone_yaml_count" => milestone_yaml.length == inventory.fetch("v0917_yaml_files")
  }

  targets = (readmes + milestone_markdown + ["REVIEW.md"]).uniq
  local_links_checked = 0
  broken_links = []
  targets.each do |file|
    File.read(file).scan(/\[[^\]]*\]\(([^)]+)\)/).flatten.each do |raw|
      target = raw.strip
      next if target.empty? || target.start_with?("#", "http://", "https://", "mailto:", "tel:", "data:")

      target = target.split("#", 2).first
      next if target.empty?

      local_links_checked += 1
      resolved = File.expand_path(target, File.dirname(file))
      broken_links << { "source" => file, "target" => raw } unless File.exist?(resolved)
    end
  end
  validation = audit.fetch("validation")
  checks["entrypoint_count"] = targets.length == validation.fetch("unique_markdown_entrypoints_scanned")
  checks["local_link_count"] = local_links_checked == validation.fetch("local_links_checked")
  checks["local_links_resolve"] = broken_links.empty?

  invalid_json = []
  milestone_json.each do |file|
    JSON.parse(File.read(file))
  rescue JSON::ParserError
    invalid_json << file
  end
  expected_invalid = audit.fetch("expected_non_parsing_json").map { |row| row.fetch("path") }.sort
  checks["json_expected_exceptions_only"] = invalid_json.sort == expected_invalid

  invalid_yaml = []
  milestone_yaml.each do |file|
    YAML.safe_load(File.read(file), permitted_classes: [Date, Time], aliases: true)
  rescue StandardError
    invalid_yaml << file
  end
  checks["yaml_parses"] = invalid_yaml.empty?

  issue_wave = YAML.safe_load(
    File.read("docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml"),
    permitted_classes: [Date, Time], aliases: true
  )
  closeout_truth = issue_wave.fetch("closeout_tail_truth")
  checks["wp17_closed"] = closeout_truth.fetch("closed_wps").include?("WP-17")
  checks["open_wps_current"] = closeout_truth.fetch("open_wps") ==
                                %w[WP-18 WP-19 WP-20 WP-21A WP-23]

  bridge_requirements = {
    "README.md" => ["docs/milestones/v0.91.8/README.md",
                    "docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md"],
    "docs/milestones/v0.91.7/README.md" => ["../v0.91.8/README.md",
                                                   "../v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md"],
    "docs/planning/ADL_FEATURE_LIST.md" => ["../milestones/v0.91.8/README.md",
                                             "../milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md"]
  }
  required_precedence = {
    "README.md" => "v0.92 consumes the reviewed v0.91.8 exact-revision handoff",
    "docs/milestones/v0.91.7/README.md" =>
      "v0.92 may consume only the reviewed v0.91.8 exact-revision handoff",
    "docs/planning/ADL_FEATURE_LIST.md" =>
      "v0.92 consumes its exact-revision handoff rather than v0.91.7 prose directly"
  }
  stale_precedence = [
    "v0.91.7 is the active final pre-v0.92",
    "v0.91.7 is the final implementation/readiness tranche before v0.92",
    "finish v0.91.7 as the final direct tranche before v0.92"
  ]
  checks["v0918_bridge_precedence"] = bridge_requirements.all? do |file, required|
    content = File.read(file)
    normalized = content.gsub(/[\x60*]/, "").gsub(/\s+/, " ")
    required.all? { |link| content.include?(link) } &&
      normalized.include?(required_precedence.fetch(file)) &&
      stale_precedence.none? { |phrase| normalized.include?(phrase) }
  end

  live_metadata_docs = %w[
    README.md
    WBS_v0.91.7.md
    DESIGN_v0.91.7.md
    PLANNING_SOURCE_CAPTURE_v0.91.7.md
    MILESTONE_CHECKLIST_v0.91.7.md
    VISION_v0.91.7.md
    DECISIONS_v0.91.7.md
    FEATURE_DOCS_v0.91.7.md
    REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md
    V092_HANDOFF_v0.91.7.md
  ].map { |file| "docs/milestones/v0.91.7/#{file}" }
  checks["metadata_dates_unambiguous"] = live_metadata_docs.all? do |file|
    content = File.read(file)
    !content.include?("- Date: `2026-06-21`") &&
      content.include?("- Created: `2026-06-21`") &&
      content.include?("- Last verified: `2026-07-18`")
  end

  accepted = File.read("docs/adr/README.md")
                 .scan(/^- \x60(\d{4}-[^\x60]+\.md)\x60$/).flatten
  v0917_adrs = File.read("docs/milestones/v0.91.7/review/V0917_ADR_INDEX_4989.md")
                   .scan(/\x60(docs\/adr\/[0-9]{4}-[^\x60]+\.md)\x60/).flatten.uniq
  adr_ids = Dir["docs/adr/[0-9][0-9][0-9][0-9]-*.md"].map { |file| File.basename(file)[0, 4] }
  duplicate_ids = adr_ids.group_by(&:itself).select { |_id, paths| paths.length > 1 }
  checks["accepted_adr_count"] = accepted.length == inventory.fetch("accepted_adr_index_entries")
  checks["v0917_adr_count"] = v0917_adrs.length == inventory.fetch("v0917_adr_index_entries")
  checks["adr_paths_exist"] = accepted.all? { |file| File.exist?("docs/adr/#{file}") } &&
                              v0917_adrs.all? { |file| File.exist?(file) }
  checks["adr_ids_unique"] = duplicate_ids.empty?

  cargo_manifests = audit.fetch("package_versions")
  cargo_metadata = cargo_manifests.keys.to_h do |manifest|
    passed = system("cargo", "metadata", "--manifest-path", manifest, "--no-deps", "--locked",
                    "--format-version", "1", out: File::NULL, err: File::NULL)
    [manifest, passed]
  end
  checks["cargo_metadata_locked"] = cargo_metadata.values.all?

  untracked = IO.popen(["git", "ls-files", "--others", "--exclude-standard"], &:read)
                .lines.map(&:strip).reject(&:empty?)
  checks["working_tree_matches_head"] = untracked.empty? &&
                                        system("git", "diff", "--quiet", "HEAD",
                                               out: File::NULL, err: File::NULL) &&
                                        system("git", "diff", "--cached", "--quiet", "HEAD",
                                               out: File::NULL, err: File::NULL)
  checks["git_diff_check"] = [
    ["git", "diff", "--check", "origin/main...HEAD"],
    ["git", "diff", "--check"],
    ["git", "diff", "--cached", "--check"]
  ].all? { |command| system(*command, out: File::NULL, err: File::NULL) }

  failed = checks.select { |_name, passed| !passed }.keys
  report = {
    "schema" => "adl.v0917.wp17.validation_receipt.v1",
    "issue" => VALIDATION_ISSUE,
    "source_issue" => 4644,
    "command" => ["ruby", ".csdlc/prepared/issues/4644/validate_docs_alignment.rb", ".",
                  VALIDATION_ISSUE.to_s],
    "completed_at" => Time.now.utc.strftime("%Y-%m-%dT%H:%M:%SZ"),
    "exit_status" => failed.empty? ? 0 : 1,
    "status" => failed.empty? ? "passed" : "failed",
    "checks" => checks,
    "observed" => {
      "readmes" => readmes.length,
      "markdown_entrypoints" => targets.length,
      "local_links_checked" => local_links_checked,
      "broken_links" => broken_links,
      "invalid_json" => invalid_json.sort,
      "invalid_yaml" => invalid_yaml.sort,
      "untracked_files" => untracked,
      "cargo_metadata" => cargo_metadata
    },
    "aws_used" => false
  }
  puts JSON.generate(report)
  fail_check("failed checks: #{failed.join(', ')}") unless failed.empty?
end
