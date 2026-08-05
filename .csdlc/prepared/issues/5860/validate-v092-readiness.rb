#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "yaml"

WAVE_PATH = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"
HASH_MANIFEST_PATH = ".csdlc/evidence/5860/V092_READINESS_ARTIFACT_SHA256.json"
LIVE_MANIFEST_PATH = ".csdlc/evidence/5860/V092_LIVE_ISSUE_CONTRACTS.json"
OWNERSHIP_VALIDATOR = ".csdlc/prepared/issues/5860/validate-v092-ownership.rb"
DOCTOR_VALIDATOR = ".csdlc/prepared/issues/5860/validate-v092-doctors.rb"
LIVE_BODY_PUBLISHER = ".csdlc/prepared/issues/5860/publish-v092-live-issue-bodies.rb"
LIVE_PREPARATION_VALIDATORS = [
  ".csdlc/prepared/issues/5821/validate-child-wave.rb",
  ".csdlc/prepared/issues/5862/validate-implementation-wave.rb"
].freeze
ALLOWED_DIFF_PREFIXES = [".adl/docs/TBD/", ".csdlc/", "docs/"].freeze

FORBIDDEN = {
  "placeholder design" => /Status: design required before Ready\./,
  "generic scope" => /implementation paths to be narrowed during preparation/i,
  "generic plan" => /Prepare the exact issue scope, implement the required outcome/i,
  "generic first step" => /Prepare exact scope, design, paths, and validation plan/i,
  "deferred path selection" => /(?:exact|specific|final|new) (?:implementation |module )?(?:path|file|module) names? (?:must be |will be )?(?:narrowed|selected|chosen|decided)/i,
  "unnamed implementation surface" => /unnamed (?:implementation |packet |schema |module |file |path)/i
}.freeze

def option_value(name)
  index = ARGV.index(name)
  index && ARGV[index + 1]
end

def repo_path?(value)
  value = value.to_s.strip.sub(/[.,;:]\z/, "")
  return false if value.empty? || value.match?(/\s/) || value.start_with?("/")
  return false if value.start_with?("http:", "https:")

  value.match?(%r{\A(?:\.?[A-Za-z0-9_-]+/|Cargo\.(?:toml|lock)\z)})
end

def markdown_section(markdown, heading)
  lines = markdown.lines
  start = lines.index { |line| line.strip == "## #{heading}" }
  return nil unless start

  lines[(start + 1)..].take_while { |line| !line.match?(/^##\s+/) }.join
end

def owned_paths(design)
  section = markdown_section(design, "Owned Paths")
  return [] unless section

  section.scan(/`([^`]+)`/).flatten.map { |value| value.sub(/[.,;:]\z/, "") }
    .select { |value| repo_path?(value) }.uniq.sort
end


def canonical_graph(wave)
  wp_rows = Array(wave["work_packages"])
  supporting_rows = Array(wave["supporting_issues"])
  rows = wp_rows + supporting_rows
  issues = rows.map { |row| row["issue"] if row["issue"].is_a?(Integer) && row["issue"] != 5817 }.compact.uniq.sort
  wp_to_issue = wp_rows.to_h { |row| [row["wp"], row["issue"]] }
  issue_to_sprint = {}

  Array(wave["execution_sprints"]).each do |sprint|
    sprint_issue = sprint.fetch("issue")
    Array(sprint["members"]).each do |member|
      token = member.to_s
      issue = token.match?(/\Aissue-\d+\z/) ? token[/\d+/].to_i : wp_to_issue[token]
      raise "unknown sprint member #{member.inspect}" unless issue
      raise "issue ##{issue} appears in multiple sprints" if issue_to_sprint.key?(issue)

      issue_to_sprint[issue] = sprint_issue
    end
  end

  missing = issues - issue_to_sprint.keys
  raise "canonical issues missing sprint membership: #{missing.join(', ')}" unless missing.empty?

  sprints = issue_to_sprint.group_by { |_issue, sprint| sprint }
    .transform_values { |pairs| pairs.map(&:first).sort }
  [wp_rows, supporting_rows, issues, sprints]
end

def dependency_tokens(values, canonical: false)
  Array(values).flat_map do |value|
    text = value.to_s
    text.to_enum(:scan, /WP-\d+(?:[A-Z]|-IMP|\.\d+)?|issue(?:-|\s+)#?\d+|#\d+/i).each_with_object([]) do |_matched, tokens|
      match = Regexp.last_match
      next if canonical && text[0...match.begin(0)].match?(/before\s+\z/i)

      token = match[0]
      if token.start_with?("#") || token.match?(/\Aissue/i)
        tokens << "issue-#{token[/\d+/]}"
      else
        tokens << token.upcase
      end
    end
  end.uniq
end

def artifact_paths(issue)
  root = ".csdlc/issues/#{issue}"
  prepared = ".csdlc/prepared/issues/#{issue}"
  paths = ["#{root}/index.json", "#{prepared}/design.md", "#{prepared}/diagram.mmd"]
  %w[sip stp spp vpp srp sor].each do |card|
    paths << "#{root}/cards/#{card}.md"
    paths << "#{root}/cards/#{card}.values.json"
  end
  paths
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

def preparation_validator_error(path)
  return "missing preparation validator #{path}" unless File.file?(path) && !File.zero?(path)

  command = case File.extname(path)
            when ".rb" then ["ruby", "-c", path]
            when ".sh" then ["bash", "-n", path]
            else return nil
            end
  _stdout, stderr, status = Open3.capture3(*command)
  status.success? ? nil : "invalid preparation validator #{path}: #{stderr.strip}"
end

def live_issue_inventory
  stdout, stderr, status = Open3.capture3(
    "gh", "issue", "list", "--state", "all", "--limit", "200",
    "--json", "number,title,body,state,url"
  )
  raise "gh issue list failed: #{stderr.strip}" unless status.success?

  JSON.parse(stdout).to_h do |parsed|
    issue = parsed.fetch("number")
    [issue, {
      "number" => issue,
      "title" => parsed.fetch("title"),
      "body_sha256" => Digest::SHA256.hexdigest(parsed.fetch("body")),
      "state" => parsed.fetch("state"),
      "url" => parsed.fetch("url")
    }]
  end
end

if ARGV.include?("--self-test")
  raise "dependency token normalization failed" unless dependency_tokens(["WP-02 and #5815", "Issue 5800"]) == ["WP-02", "issue-5815", "issue-5800"]
  raise "dependency direction failed" unless dependency_tokens(["coordination before WP-03"], canonical: true).empty?
  raise "repo path acceptance failed" unless repo_path?("docs/milestones/v0.92/README.md")
  raise "absolute path rejection failed" if repo_path?("/Users/example/repo/file.md")
  raise "command rejection failed" if repo_path?("cargo test --locked")
  sample = "## Read-Only Inputs\n\n- `docs/input.md`\n\n## Owned Paths\n\n- `docs/a.md`\n- `adl/src/lib.rs`\n"
  paths = owned_paths(sample)
  raise "owned path extraction failed" unless paths == ["adl/src/lib.rs", "docs/a.md"]
  puts "v0.92 readiness validator self-test: PASS"
  exit 0
end

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
wp_rows, supporting_rows, issues, sprints = canonical_graph(wave)
rows_by_issue = (wp_rows + supporting_rows).each_with_object({}) do |row, memo|
  issue = row["issue"]
  memo[issue] = row if issues.include?(issue)
end
wp_to_issue = wp_rows.to_h { |row| [row["wp"], row["issue"]] }
wp_dependencies = wp_rows.to_h do |row|
  [row["wp"], dependency_tokens(row["depends_on"], canonical: true).select { |token| token.start_with?("WP-") }]
end
dependency_closure = lambda do |wp, seen = []|
  return [] if wp.nil? || seen.include?(wp)

  direct = Array(wp_dependencies[wp])
  direct | direct.flat_map { |dependency| dependency_closure.call(dependency, seen + [wp]) }
end

errors = []
rows = []
matrix_path = option_value("--write-matrix")
write_hash_path = option_value("--write-hash-manifest")
write_live_path = option_value("--write-live-manifest")
verify_live = ARGV.include?("--verify-live")
live_inventory = nil

diff_stdout, diff_stderr, diff_status = Open3.capture3("git", "diff", "--name-only", "origin/main...HEAD")
errors << "cannot inspect documentation-only diff boundary: #{diff_stderr.strip}" unless diff_status.success?
if diff_status.success?
  changed_paths = diff_stdout.lines.map(&:strip).reject(&:empty?)
  changed_paths.each do |path|
    errors << "product or undeclared path changed: #{path}" unless ALLOWED_DIFF_PREFIXES.any? { |prefix| path.start_with?(prefix) }
    errors << "externally owned #5861 path changed: #{path}" if path.match?(%r{(?:^|/)5861(?:/|\.|$)})
  end
end

missing_wave = issues - rows_by_issue.keys
errors.concat(missing_wave.map { |issue| "##{issue}: missing canonical wave row" })

if write_hash_path
  files = issues.flat_map { |issue| artifact_paths(issue) }.sort
  missing = files.reject { |path| File.file?(path) && !File.zero?(path) }
  abort "cannot write hash manifest; missing: #{missing.join(', ')}" unless missing.empty?
  payload = {
    "schema" => "adl.v092.readiness-artifact-sha256.v1",
    "authority" => "exact-head independent review input; normal validation never rewrites this file",
    "files" => files.to_h { |path| [path, sha256(path)] }
  }
  File.write(write_hash_path, JSON.pretty_generate(payload) + "\n")
end

if write_live_path
  live_inventory = live_issue_inventory
  payload = {
    "schema" => "adl.v092.live-issue-contracts.v1",
    "repository" => "danielbaustin/agent-design-language",
    "issues" => issues.sort.to_h { |issue| [issue.to_s, live_inventory.fetch(issue)] }
  }
  File.write(write_live_path, JSON.pretty_generate(payload) + "\n")
end

if File.file?(LIVE_MANIFEST_PATH)
  live_manifest = JSON.parse(File.read(LIVE_MANIFEST_PATH)).fetch("issues", {})
  errors << "live issue manifest does not cover the exact #{issues.length}-issue set" unless live_manifest.keys.sort == issues.map(&:to_s).sort
  issues.sort.each do |issue|
    contract = live_manifest[issue.to_s] || {}
    errors << "##{issue}: live manifest number mismatch" unless contract["number"] == issue
    errors << "##{issue}: live issue is not open" unless contract["state"] == "OPEN"
    errors << "##{issue}: live issue title is empty" if contract["title"].to_s.strip.empty?
    errors << "##{issue}: live issue body digest is invalid" unless contract["body_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
  end
  if verify_live
    live_inventory ||= live_issue_inventory
    issues.sort.each do |issue|
      errors << "##{issue}: live GitHub issue drift" unless live_manifest[issue.to_s] == live_inventory[issue]
    end
  end
else
  errors << "missing #{LIVE_MANIFEST_PATH}; pin live issue contracts after all issue repairs"
end

if File.file?(HASH_MANIFEST_PATH)
  manifest = JSON.parse(File.read(HASH_MANIFEST_PATH))
  expected_files = issues.flat_map { |issue| artifact_paths(issue) }.sort
  actual_files = manifest.fetch("files", {}).keys.sort
  errors << "artifact manifest path set differs from the #{issues.length}-issue contract" unless actual_files == expected_files
  manifest.fetch("files", {}).each do |path, expected|
    if !File.file?(path)
      errors << "artifact manifest references missing #{path}"
    elsif sha256(path) != expected
      errors << "artifact SHA-256 drift: #{path}"
    end
  end
else
  errors << "missing #{HASH_MANIFEST_PATH}; write it explicitly after all issue repairs"
end

sprints.each do |sprint, sprint_issues|
  sprint_issues.each do |issue|
    root = ".csdlc/issues/#{issue}"
    prepared = ".csdlc/prepared/issues/#{issue}"
    index_path = "#{root}/index.json"
    design_path = "#{prepared}/design.md"
    diagram_path = "#{prepared}/diagram.mmd"

    [index_path, design_path, diagram_path].each do |path|
      errors << "##{issue}: missing #{path}" unless File.file?(path) && !File.zero?(path)
    end
    next unless File.file?(index_path) && File.file?(design_path) && File.file?(diagram_path)

    index = JSON.parse(File.read(index_path))
    design = File.read(design_path)
    corpus = [design]
    statuses = {}
    card_values = {}

    %w[sip stp spp vpp srp sor].each do |card|
      rendered = "#{root}/cards/#{card}.md"
      values = "#{root}/cards/#{card}.values.json"
      unless File.file?(rendered) && File.file?(values)
        errors << "##{issue}: missing #{card} rendered or values card"
        next
      end
      parsed = JSON.parse(File.read(values))
      card_values[card] = parsed
      text = File.read(rendered)
      corpus << text if %w[sip stp spp vpp].include?(card)
      statuses[card] = text[/^Status:\s*(.+)$/, 1]
      errors << "##{issue}: #{card} values issue mismatch" unless parsed.dig("identity", "issue") == issue
      errors << "##{issue}: #{card} values generation mismatch" unless parsed.dig("identity", "generation") == index["generation"]
      errors << "##{issue}: #{card} rendered/value status mismatch" unless parsed["status"] == statuses[card]
    rescue JSON::ParserError => e
      errors << "##{issue}: invalid #{card} values JSON: #{e.message}"
    end

    FORBIDDEN.each do |label, pattern|
      errors << "##{issue}: #{label}" if corpus.join("\n").match?(pattern)
    end

    approval = index["design_review"]
    approved = approval.is_a?(Hash) && approval.key?("approved")
    errors << "##{issue}: design review is not approved" unless approved
    errors << "##{issue}: active preparation claim remains" if index["claim"]

    if approved && card_values.key?("spp") && card_values.key?("vpp")
      revision = approval.dig("approved", "revision")
      spp = card_values.dig("spp", "content", "values") || {}
      vpp = card_values.dig("vpp", "content", "values") || {}
      errors << "##{issue}: SPP design digest does not match approval" unless spp["design_digest"] == revision
      errors << "##{issue}: VPP design digest does not match approval" unless vpp["design_digest"] == revision
      errors << "##{issue}: SPP/VPP diagram digest mismatch" unless !spp["diagram_digest"].to_s.empty? && spp["diagram_digest"] == vpp["diagram_digest"]
      errors << "##{issue}: SPP design reference mismatch" unless spp["design_ref"] == design_path
      errors << "##{issue}: VPP design reference mismatch" unless vpp["design_ref"] == design_path
      errors << "##{issue}: SPP diagram reference mismatch" unless spp["diagram_ref"] == diagram_path
      errors << "##{issue}: VPP diagram reference mismatch" unless vpp["diagram_ref"] == diagram_path
    end

    stp = card_values.dig("stp", "content", "values") || {}
    %w[deliverables acceptance_criteria dependencies repo_inputs non_goals].each do |field|
      errors << "##{issue}: STP #{field} is empty" if !stp[field].is_a?(Array) || stp[field].empty?
    end

    canonical = rows_by_issue.fetch(issue, {})
    expected_dependencies = Array(canonical["depends_on"])
    expected_tokens = dependency_tokens(expected_dependencies, canonical: true)
    declared_tokens = dependency_tokens(stp["dependencies"])
    expected_tokens.each do |token|
      alias_token = token.start_with?("WP-") && wp_to_issue[token] ? "issue-#{wp_to_issue[token]}" : nil
      unless declared_tokens.include?(token) || (alias_token && declared_tokens.include?(alias_token))
        errors << "##{issue}: canonical dependency #{token} is absent from STP"
      end
    end
    declared_tokens.each do |token|
      issue_number = token.start_with?("issue-") ? token[/\d+/].to_i : nil
      wp_alias = issue_number && wp_to_issue.key(issue_number)
      dependency_wp = token.start_with?("WP-") ? token : wp_alias
      next if expected_tokens.include?(token) || (wp_alias && expected_tokens.include?(wp_alias))
      next if dependency_wp && dependency_closure.call(canonical["wp"] || canonical["owner_wp"]).include?(dependency_wp)
      next if issue == 5862 && token.match?(/\AWP-04\.\d+\z/)
      next if issue.between?(5863, 5878) && ["WP-04-IMP", "issue-5862"].include?(token)
      next if Array(stp["dependencies"]).any? do |entry|
        entry_tokens = dependency_tokens([entry])
        entry_tokens.include?(token) && entry.to_s.match?(/(?:\bPR\b|retained|reviewed .*plan|serializ|coordination|mapped to|blocked until|created only after|waits? for|before final)/i)
      end
      next if sprints[sprint].include?(issue_number) && Array(stp["dependencies"]).any? { |entry| entry.to_s.match?(/coordination/i) }

      errors << "##{issue}: declared dependency #{token} is absent from the canonical wave"
    end

    paths = owned_paths(design)
    errors << "##{issue}: design has no exact repo-relative Owned Paths" if paths.empty?
    rollback = markdown_section(design, "Rollback")
    errors << "##{issue}: design lacks an explicit nonempty Rollback section" if rollback.to_s.strip.empty?

    spp = card_values.dig("spp", "content", "values") || {}
    %w[steps invariants risks stop_conditions].each do |field|
      errors << "##{issue}: SPP #{field} is empty" if !spp[field].is_a?(Array) || spp[field].empty?
    end
    vpp = card_values.dig("vpp", "content", "values") || {}
    lanes = vpp["lanes"]
    if !lanes.is_a?(Array) || lanes.empty?
      errors << "##{issue}: VPP lanes are empty"
    else
      lanes.each_with_index do |lane, lane_index|
        errors << "##{issue}: VPP lane #{lane_index + 1} lacks proof role" if lane["proof_role"].to_s.strip.empty?
        errors << "##{issue}: VPP lane #{lane_index + 1} lacks acceptance mapping" if !lane["acceptance_ids"].is_a?(Array) || lane["acceptance_ids"].empty?
        argv = lane["argv"]
        errors << "##{issue}: VPP lane #{lane_index + 1} lacks argv" if !argv.is_a?(Array) || argv.empty?
        errors << "##{issue}: VPP lane #{lane_index + 1} contains an empty argv token" if Array(argv).any? { |arg| arg.to_s.strip.empty? }
        Array(argv).select { |arg| arg.to_s.start_with?(".csdlc/prepared/issues/") }.each do |path|
          error = preparation_validator_error(path)
          errors << "##{issue}: #{error}" if error
        end
      end
    end

    %w[sip stp spp vpp].each do |card|
      errors << "##{issue}: #{card} status is #{statuses[card].inspect}, expected ready" unless statuses[card] == "ready"
    end
    %w[srp sor].each do |card|
      allowed = %w[pre_phase draft]
      errors << "##{issue}: #{card} status is #{statuses[card].inspect}, expected truthful pre-execution state" unless allowed.include?(statuses[card])
    end

    rows << {
      issue: issue,
      wp: canonical["wp"] || canonical["owner_wp"] || "supporting",
      sprint: sprint,
      phase: index["phase"],
      approved: approved,
      design_digest: approval&.dig("approved", "revision"),
      dependencies: expected_dependencies,
      declared_dependencies: Array(stp["dependencies"]),
      paths: paths,
      lanes: Array(lanes).map { |lane| lane["id"] || lane["name"] || lane["proof_role"] },
      statuses: statuses
    }
  end
end

unless errors.empty?
  warn "v0.92 readiness: FAIL (#{errors.length} findings)"
  errors.each { |error| warn "- #{error}" }
  exit 1
end

if File.file?(OWNERSHIP_VALIDATOR)
  _stdout, stderr, status = Open3.capture3("ruby", OWNERSHIP_VALIDATOR)
  abort "ownership validation failed: #{stderr.strip}" unless status.success?
else
  abort "missing #{OWNERSHIP_VALIDATOR}"
end

if File.file?(DOCTOR_VALIDATOR)
  _stdout, stderr, status = Open3.capture3("ruby", DOCTOR_VALIDATOR)
  abort "typed doctor validation failed: #{stderr.strip}" unless status.success?
else
  abort "missing #{DOCTOR_VALIDATOR}"
end

if verify_live
  LIVE_PREPARATION_VALIDATORS.each do |validator|
    abort "missing #{validator}" unless File.file?(validator)

    _stdout, stderr, status = Open3.capture3("ruby", validator)
    abort "preparation control failed for #{validator}: #{stderr.strip}" unless status.success?
  end
  _stdout, stderr, status = Open3.capture3("ruby", LIVE_BODY_PUBLISHER)
  abort "live issue body parity failed: #{stderr.strip}" unless status.success?
end

puts "v0.92 readiness: PASS (#{rows.length} design-ready issues across #{sprints.length} sprints)"

if matrix_path
  lines = [
    "# v0.92 Execution-Issue Readiness Matrix",
    "",
    "Generated by `.csdlc/prepared/issues/5860/validate-v092-readiness.rb` after full structural, dependency, path, digest-projection, and artifact-integrity validation.",
    "",
    "`design_ready` means the issue-specific design and six-card execution packet are complete. It does not claim that dependency gates are terminal or that an execution claim is currently active.",
    "",
    "| Issue | WP | Sprint | Design digest | Owned paths | VPP lanes | Packet disposition | Execution gate |",
    "|---:|---|---:|---|---:|---:|---|---|"
  ]
  rows.sort_by { |row| row[:issue] }.each do |row|
    lines << "| ##{row[:issue]} | `#{row[:wp]}` | ##{row[:sprint]} | `#{row[:design_digest][0, 12]}` | #{row[:paths].length} | #{row[:lanes].length} | `design_ready` | dependency proof + just-in-time claim reacquisition |"
  end
  lines.concat(["", "## Complete Contracts", ""])
  rows.sort_by { |row| row[:issue] }.each do |row|
    lines << "### ##{row[:issue]} - #{row[:wp]}"
    lines << ""
    lines << "- Design digest: `#{row[:design_digest]}`"
    lines << "- Canonical dependencies: #{row[:dependencies].map { |value| "`#{value}`" }.join(', ')}"
    lines << "- Declared dependency gates: #{row[:declared_dependencies].map { |value| "`#{value}`" }.join(', ')}"
    lines << "- Exact owned paths: #{row[:paths].map { |value| "`#{value}`" }.join(', ')}"
    lines << "- Validation lanes: #{row[:lanes].map { |value| "`#{value}`" }.join(', ')}"
    lines << "- Readiness disposition: `design_ready`; execution remains gated on terminal dependency proof and just-in-time claim reacquisition."
    lines << ""
  end
  lines.concat([
    "Result: **PASS** - #{rows.length} design-ready issues across #{sprints.length} sprint umbrellas.",
    "",
    "Artifact integrity is pinned by `#{HASH_MANIFEST_PATH}`. Normal validation never rewrites that manifest.",
    "Live issue title/body/state identity is pinned by `#{LIVE_MANIFEST_PATH}` and checked against GitHub with `--verify-live`."
  ])
  File.write(matrix_path, lines.join("\n") + "\n")
end
