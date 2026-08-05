#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUES = [
  5786, 5795, 5800, 5801, 5812, 5818, 5819, 5820, 5821, 5822, 5823,
  5824, 5825, 5826, 5827, 5828, 5829, 5830, 5831, 5832, 5833, 5834,
  5835, 5836, 5837, 5838, 5839, 5840, 5841, 5842, 5843, 5844, 5845,
  5846, 5847, 5848, 5849, 5850, 5851, 5852, 5853, 5862, 5863, 5864,
  5865, 5866, 5867, 5868, 5869, 5870, 5871, 5872, 5873, 5874, 5875,
  5876, 5877, 5878
].freeze

BROAD_PATHS = %w[
  .github
  .github/workflows
  .csdlc
  .csdlc/evidence
  .csdlc/issues
  .csdlc/prepared
  adl
  adl/src
  adl/tools
  adl-runtime
  adl-runtime/src
  adl-runtime/tests
  adl-runtime-kernel
  adl-runtime-kernel/src
  adl-runtime-kernel/tests
  adl-v2
  adl-v2/crates
  csdlc-v2
  csdlc-v2/src
  csdlc-v2/tests
  demos
  docs
  docs/milestones
  docs/milestones/v0.92
  docs/reviews
  schemas
  tools
].freeze

BROAD_DIRECTORY_NAMES = %w[
  articles
  audio
  distributed
  docs
  episodes
  features
  publication
  review
  src
  tests
  tools
  workflows
].freeze

WP04 = {
  5863 => %w[adl-runtime/src/distributed/identity.rs adl-runtime/tests/distributed_identity.rs],
  5864 => %w[adl-runtime/src/distributed/certificates.rs adl-runtime/tests/distributed_certificates.rs],
  5865 => %w[adl-runtime/src/distributed/transport.rs adl-runtime/tests/distributed_transport.rs adl-runtime/Cargo.toml adl-runtime/Cargo.lock],
  5866 => %w[adl-runtime/src/distributed/discovery.rs adl-runtime/tests/distributed_discovery.rs],
  5867 => %w[adl-runtime/src/distributed/membership.rs adl-runtime/tests/distributed_membership.rs],
  5868 => %w[adl-runtime/src/distributed/failure_detection.rs adl-runtime/tests/distributed_failure_detection.rs],
  5869 => %w[adl-runtime/src/distributed/lease.rs adl-runtime/tests/distributed_lease.rs],
  5870 => %w[adl-runtime/src/distributed/fencing.rs adl-runtime/tests/distributed_fencing.rs],
  5871 => %w[adl-runtime/src/distributed/capability_advertisement.rs adl-runtime/tests/distributed_capability_advertisement.rs],
  5872 => %w[adl-runtime/src/distributed/resource_weather.rs adl-runtime/tests/distributed_resource_weather.rs],
  5873 => %w[adl-runtime/src/distributed/placement.rs adl-runtime/tests/distributed_placement.rs],
  5874 => %w[adl-runtime/src/distributed/snapshot_catalog.rs adl-runtime/tests/distributed_snapshot_catalog.rs],
  5875 => %w[adl-runtime/src/distributed/migration.rs adl-runtime/tests/distributed_migration.rs],
  5876 => %w[adl-runtime/src/distributed/recovery.rs adl-runtime/tests/distributed_recovery.rs],
  5877 => %w[adl-runtime/src/distributed/projection.rs adl-runtime/tests/distributed_projection.rs docs/api/runtime-v3/v1/distributed.openapi.json],
  5878 => %w[adl-runtime/src/distributed/mod.rs adl-runtime/src/lib.rs adl-runtime/tests/distributed_guardian.rs adl/tools/validate_v092_distributed_guardian.sh adl/tools/validate_v092_distributed_native_receipts.rb]
}.freeze

def section(text, heading)
  matches = text.scan(/^## #{Regexp.escape(heading)}\s*$\n(.*?)(?=^## |\z)/m)
  return [nil, "missing"] if matches.empty?
  return [nil, "duplicate"] unless matches.length == 1

  [matches.first.first, nil]
end

def parse_owned(issue, text, errors)
  body, problem = section(text, "Owned Paths")
  if problem
    errors << "##{issue}: #{problem} ## Owned Paths section"
    return []
  end

  lines = body.lines.map(&:strip).reject(&:empty?)
  if lines.empty?
    errors << "##{issue}: empty ## Owned Paths section"
    return []
  end

  paths = []
  lines.each do |line|
    match = line.match(/\A- `([^`]+)`\z/)
    unless match
      errors << "##{issue}: Owned Paths accepts only '- `repo/relative/path`' entries: #{line.inspect}"
      next
    end
    paths << match[1]
  end
  paths
end

def validate_path(issue, path, errors)
  errors << "##{issue}: empty owned path" if path.empty?
  errors << "##{issue}: absolute or home-relative path #{path.inspect}" if path.start_with?("/", "~", "\\") || path.match?(/\A[A-Za-z]:[\\\/]/)
  errors << "##{issue}: URI is not a repository path #{path.inspect}" if path.match?(/\A[a-z][a-z0-9+.-]*:\/\//i)
  errors << "##{issue}: wildcard or range syntax in #{path.inspect}" if path.match?(/[\*\?\[\]\{\}]/) || path.include?(" through ") || path.include?("...")
  errors << "##{issue}: trailing slash is not canonical #{path.inspect}" if path.end_with?("/")
  errors << "##{issue}: broad ambiguous path #{path.inspect}" if BROAD_PATHS.include?(path)
  errors << "##{issue}: broad ambiguous directory #{path.inspect}" if BROAD_DIRECTORY_NAMES.include?(Pathname.new(path).basename.to_s)

  clean = Pathname.new(path).cleanpath.to_s
  errors << "##{issue}: noncanonical path #{path.inspect}" unless clean == path
  errors << "##{issue}: parent traversal is forbidden #{path.inspect}" if Pathname.new(path).each_filename.include?("..")
end

def parse_gates(issue, text, errors)
  body, problem = section(text, "Serialization Gates")
  return [] if problem == "missing"
  if problem
    errors << "##{issue}: duplicate ## Serialization Gates section"
    return []
  end

  fence = body.match(/\A\s*```json\s*\n(.*?)\n```\s*\z/m)
  unless fence
    errors << "##{issue}: Serialization Gates must contain exactly one JSON fence"
    return []
  end

  begin
    value = JSON.parse(fence[1])
  rescue JSON::ParserError => e
    errors << "##{issue}: invalid serialization JSON: #{e.message}"
    return []
  end

  gates = value.is_a?(Array) ? value : [value]
  gates.each do |gate|
    unless gate.is_a?(Hash) && gate["schema"] == "csdlc.serialization_gate.v1"
      errors << "##{issue}: invalid serialization gate schema"
      next
    end
    errors << "##{issue}: serialization gate id missing" unless gate["id"].is_a?(String) && !gate["id"].empty?
    %w[paths issues order].each do |key|
      errors << "##{issue}: serialization gate #{gate['id']} has invalid #{key}" unless gate[key].is_a?(Array) && !gate[key].empty?
    end
    next unless gate["issues"].is_a?(Array) && gate["order"].is_a?(Array)

    errors << "##{issue}: gate #{gate['id']} does not name its own issue" unless gate["issues"].include?(issue)
    errors << "##{issue}: gate #{gate['id']} issues/order mismatch" unless gate["issues"].sort == gate["order"].sort && gate["order"].uniq == gate["order"]
    unknown = gate["issues"] - ISSUES
    errors << "##{issue}: gate #{gate['id']} names unknown issues #{unknown.inspect}" unless unknown.empty?
  end
  gates
end

errors = []
owned = {}
gates_by_issue = {}

ISSUES.each do |issue|
  design = ROOT.join(".csdlc/prepared/issues/#{issue}/design.md")
  unless design.file?
    errors << "##{issue}: missing design #{design.relative_path_from(ROOT)}"
    next
  end

  text = design.read
  paths = parse_owned(issue, text, errors)
  paths.each { |path| validate_path(issue, path, errors) }
  errors << "##{issue}: duplicate owned path" unless paths.uniq.length == paths.length
  owned[issue] = paths

  readonly, readonly_problem = section(text, "Read-Only Inputs")
  errors << "##{issue}: #{readonly_problem} ## Read-Only Inputs section" if readonly_problem
  errors << "##{issue}: empty ## Read-Only Inputs section" if readonly && readonly.strip.empty?

  gates_by_issue[issue] = parse_gates(issue, text, errors)

  spp_path = ROOT.join(".csdlc/issues/#{issue}/cards/spp.values.json")
  begin
    spp = JSON.parse(spp_path.read)
    actual_areas = spp.dig("content", "values", "affected_areas")
    gate_areas = gates_by_issue[issue].map { |gate| "SERIALIZATION_GATE #{JSON.generate(gate)}" }
    expected_areas = paths + gate_areas
    errors << "##{issue}: SPP affected_areas do not exactly mirror Owned Paths and Serialization Gates" unless actual_areas == expected_areas
  rescue Errno::ENOENT, JSON::ParserError => e
    errors << "##{issue}: cannot validate SPP ownership projection: #{e.message}"
  end
end

errors << "#5861 must not be in the ownership denominator" if ISSUES.include?(5861)
owned.each do |issue, paths|
  errors << "##{issue}: forbidden #5861 lifecycle ownership" if paths.any? { |path| path.match?(%r{\A\.csdlc/(?:issues|prepared/issues|evidence)/5861(?:/|\z)}) }
end

WP04.each do |issue, expected|
  actual = owned.fetch(issue, [])
  errors << "##{issue}: WP-04 exclusive denominator changed" unless actual == expected
end

canonical_gates = {}
gates_by_issue.each do |issue, gates|
  gates.each do |gate|
    id = gate["id"]
    next unless id
    normalized = JSON.generate(gate)
    if canonical_gates.key?(id) && canonical_gates[id][:json] != normalized
      errors << "##{issue}: gate #{id} differs from ##{canonical_gates[id][:issue]}"
    else
      canonical_gates[id] ||= { json: normalized, issue: issue, gate: gate }
    end
  end
end

canonical_gates.each_value do |entry|
  gate = entry[:gate]
  gate.fetch("issues", []).each do |issue|
    matching = gates_by_issue.fetch(issue, []).find { |candidate| candidate["id"] == gate["id"] }
    errors << "##{issue}: missing participant copy of gate #{gate['id']}" unless matching
    gate.fetch("paths", []).each do |path|
      errors << "##{issue}: gate #{gate['id']} path #{path} is not owned" unless owned.fetch(issue, []).include?(path)
    end
  end
end

entries = owned.flat_map { |issue, paths| paths.map { |path| [issue, path] } }
overlaps = []
entries.combination(2) do |left, right|
  left_issue, left_path = left
  right_issue, right_path = right
  next if left_issue == right_issue

  relation = if left_path == right_path
               "exact"
             elsif right_path.start_with?("#{left_path}/")
               "prefix"
             elsif left_path.start_with?("#{right_path}/")
               "prefix"
             end
  next unless relation

  shared_path = left_path.length <= right_path.length ? left_path : right_path
  gate = canonical_gates.values.map { |entry| entry[:gate] }.find do |candidate|
    candidate.fetch("paths", []).include?(shared_path) &&
      candidate.fetch("issues", []).include?(left_issue) &&
      candidate.fetch("issues", []).include?(right_issue)
  end
  if gate
    overlaps << [left_issue, right_issue, shared_path, gate["id"], relation]
  else
    errors << "unserialized #{relation} overlap: ##{left_issue} #{left_path} <-> ##{right_issue} #{right_path}"
  end
end

if errors.any?
  warn "v0.92 ownership validation FAILED (#{errors.length} errors)"
  errors.each { |error| warn "- #{error}" }
  exit 1
end

puts "v0.92 ownership validation PASS"
puts "issues=#{ISSUES.length} owned_paths=#{entries.length} serialization_gates=#{canonical_gates.length} serialized_overlaps=#{overlaps.length}"
canonical_gates.sort.each do |id, entry|
  gate = entry[:gate]
  puts "gate=#{id} issues=#{gate['issues'].join(',')} paths=#{gate['paths'].join(',')}"
end
overlaps.sort.each do |left, right, path, gate, relation|
  puts "overlap=#{relation} issues=#{left},#{right} path=#{path} disposition=serialized gate=#{gate}"
end
