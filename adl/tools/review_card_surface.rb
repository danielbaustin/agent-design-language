#!/usr/bin/env ruby
# frozen_string_literal: true

require "optparse"
require "pathname"
require "yaml"

REQUIRED_OUTPUT_SECTIONS = [
  "Summary",
  "Artifacts produced",
  "Actions taken",
  "Main Repo Integration (REQUIRED)",
  "Validation",
  "Verification Summary",
  "Determinism Evidence",
  "Security / Privacy Checks",
  "Replay Artifacts",
  "Artifact Verification",
  "Decisions / Deviations",
  "Follow-ups / Deferred work"
].freeze

ALLOWED_STATUS = %w[NOT_STARTED IN_PROGRESS DONE FAILED].freeze
REQUIRED_REVIEW_SURFACES = [
  "card_review_checklist.v1",
  "card_review_output.v1",
  "card_reviewer_gpt.v1.1"
].freeze

def parse_args
  options = {}
  OptionParser.new do |opts|
    opts.banner = "Usage: review_card_surface.rb --input <input.md> --output <output.md>"

    opts.on("--input PATH", "Input card path") { |value| options[:input] = value }
    opts.on("--output PATH", "Output card path") { |value| options[:output] = value }
  end.parse!

  abort("missing --input") unless options[:input]
  abort("missing --output") unless options[:output]
  options
end

def repo_root
  root = `git rev-parse --show-toplevel 2>/dev/null`.strip
  abort("not in a git repo") if root.empty?
  root
end

def lines_for_section(text, heading)
  match = text.match(/^## #{Regexp.escape(heading)}\n(.*?)(?=^## |\z)/m)
  return [] unless match

  match[1].lines.map(&:rstrip)
end

def field_value(text, label)
  match = text.match(/^(?:- )?#{Regexp.escape(label)}:\s*(.*?)\s*$/)
  match && match[1]
end

def artifact_paths(lines)
  lines.map do |line|
    match = line.match(/^- `([^`]+)`$/)
    match && match[1]
  end.compact
end

def parse_review_surfaces(text)
  match = text.match(/review_surfaces:\n((?:\s+- .*\n)+)/m)
  return [] unless match

  match[1].lines.map do |line|
    surface = line.strip.sub(/^- /, "")
    surface unless surface.empty?
  end.compact
end

def repo_relative_path?(path)
  !path.empty? &&
    !path.start_with?("/") &&
    !path.match?(%r{\A[A-Za-z]:\\}) &&
    !path.include?("..")
end

def absolute_host_path_present?(text)
  text.match?(%r{(?:^|[\s`'"])(/Users/|/home/|/tmp/|/var/folders/)})
end

def add_check(checks, id:, domain:, severity:, passed:, title:, evidence:, notes:)
  checks << {
    "id" => id,
    "domain" => domain,
    "severity" => severity,
    "status" => passed ? "PASS" : "FAIL",
    "title" => title,
    "evidence" => evidence,
    "notes" => notes
  }
end

def decision_for(checks)
  severities = checks.reject { |check| check["status"] == "PASS" }.map { |check| check["severity"] }
  return "MAJOR_ISSUES" if severities.include?("high")
  return "MINOR_FIXES" if severities.include?("medium")

  "PASS"
end

options = parse_args
root = repo_root
input_path = File.expand_path(options[:input], Dir.pwd)
output_path = File.expand_path(options[:output], Dir.pwd)
input_rel = Pathname.new(input_path).relative_path_from(Pathname.new(root)).to_s
output_rel = Pathname.new(output_path).relative_path_from(Pathname.new(root)).to_s
input_text = File.read(input_path)
output_text = File.read(output_path)

checks = []
output_sections = REQUIRED_OUTPUT_SECTIONS.all? { |heading| output_text.match?(/^## #{Regexp.escape(heading)}$/) }
add_check(
  checks,
  id: "RVS-STR-001",
  domain: "structure",
  severity: "high",
  passed: output_sections,
  title: "Required output-card sections are present",
  evidence: ["path:#{output_rel}"],
  notes: output_sections ? "All required sections were found." : "One or more required sections are missing."
)

status = field_value(output_text, "Status")
status_valid = ALLOWED_STATUS.include?(status)
add_check(
  checks,
  id: "RVS-STR-002",
  domain: "structure",
  severity: "medium",
  passed: status_valid,
  title: "Output-card status is normalized",
  evidence: ["path:#{output_rel}"],
  notes: status_valid ? "Status=#{status}" : "Status must be one of #{ALLOWED_STATUS.join(', ')}."
)

execution_values = %w[Actor Model Provider Start\ Time End\ Time].map { |field| field_value(output_text, field) }
execution_complete = execution_values.all? { |value| value && !value.empty? }
add_check(
  checks,
  id: "RVS-STR-003",
  domain: "structure",
  severity: "medium",
  passed: execution_complete,
  title: "Execution metadata is present",
  evidence: ["path:#{output_rel}"],
  notes: execution_complete ? "Actor/model/provider/start/end fields are populated." : "Execution metadata contains blank fields."
)

artifacts = artifact_paths(lines_for_section(output_text, "Artifacts produced"))
artifact_paths_valid = !artifacts.empty? && artifacts.all? { |path| repo_relative_path?(path) }
add_check(
  checks,
  id: "RVS-ART-001",
  domain: "artifacts",
  severity: "high",
  passed: artifact_paths_valid,
  title: "Artifact paths are explicit and repo-relative",
  evidence: ["path:#{output_rel}"],
  notes: artifact_paths_valid ? "Artifact list is populated with repo-relative paths." : "Artifact list is blank or contains non-repo-relative paths."
)

absolute_path_free = !absolute_host_path_present?(output_text)
add_check(
  checks,
  id: "RVS-SEC-001",
  domain: "security_privacy",
  severity: "high",
  passed: absolute_path_free,
  title: "Output card contains no absolute host paths",
  evidence: ["path:#{output_rel}"],
  notes: absolute_path_free ? "No absolute host paths detected." : "Absolute host path pattern detected."
)

surfaces = parse_review_surfaces(input_text)
surfaces_valid = surfaces.empty? || surfaces == REQUIRED_REVIEW_SURFACES
add_check(
  checks,
  id: "RVS-PRM-001",
  domain: "validation",
  severity: "medium",
  passed: surfaces_valid,
  title: "Prompt-spec review surfaces are in canonical order when present",
  evidence: ["path:#{input_rel}"],
  notes: surfaces.empty? ? "No prompt-spec review surfaces were declared." : "Review surface ordering is #{surfaces_valid ? 'valid' : 'invalid'}."
)

summary = {
  "review_surface_version" => "adl.review_surface.v1",
  "review_target" => {
    "input_card_path" => input_rel,
    "output_card_path" => output_rel
  },
  "decision" => decision_for(checks),
  "checks" => checks
}

puts YAML.dump(summary)
