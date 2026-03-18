#!/usr/bin/env ruby
# frozen_string_literal: true

require "optparse"
require "pathname"
require "psych"
require "time"
require "uri"

class ValidationError < StandardError; end

ROOT = Pathname(__dir__).join("..", "..").realpath
TOOLS_ROOT = Pathname(__dir__).join("..").realpath

TYPE_TO_CONTRACT = {
  "stp" => TOOLS_ROOT.join("schemas", "structured_task_prompt.contract.yaml"),
  "sip" => TOOLS_ROOT.join("schemas", "structured_implementation_prompt.contract.yaml"),
  "sor" => TOOLS_ROOT.join("schemas", "structured_output_record.contract.yaml")
}.freeze

options = {
  phase: nil
}

OptionParser.new do |opts|
  opts.banner = "Usage: validate_structured_prompt.rb --type <stp|sip|sor> --input <path> [--phase <phase>]"
  opts.on("--type TYPE", "Artifact type: stp, sip, or sor") { |v| options[:type] = v }
  opts.on("--input PATH", "Input artifact path") { |v| options[:input] = v }
  opts.on("--phase PHASE", "Validation phase (bootstrap, authored, completed)") { |v| options[:phase] = v }
end.parse!

def die!(message)
  warn "ERROR: #{message}"
  exit 1
end

type = options[:type]
input_path = options[:input]
die!("missing --type") if type.to_s.empty?
die!("missing --input") if input_path.to_s.empty?
contract_path = TYPE_TO_CONTRACT[type]
die!("unsupported --type: #{type}") unless contract_path
die!("contract not found: #{contract_path}") unless contract_path.file?

input = Pathname(input_path)
die!("input not found: #{input}") unless input.file?

contract = Psych.safe_load(contract_path.read, aliases: false)

def parse_front_matter(text)
  lines = text.lines
  raise ValidationError, "missing YAML front matter opener" unless lines.first&.strip == "---"

  closing = lines[1..].find_index { |line| line.strip == "---" }
  raise ValidationError, "missing YAML front matter closer" unless closing

  front_matter = lines[1..closing].join
  body = lines[(closing + 2)..]&.join.to_s
  parsed = Psych.safe_load(front_matter, aliases: false)
  raise ValidationError, "front matter must be a YAML mapping" unless parsed.is_a?(Hash)

  [parsed, body]
end

def parse_markdown_sections(text)
  text.scan(/^## (.+)$/).flatten
end

def parse_prompt_card(text)
  fields = {}
  blocks = Hash.new { |h, k| h[k] = {} }
  current_block = nil

  text.each_line do |line|
    stripped = line.chomp
    case stripped
    when /^## /
      current_block = nil
    when "Context:"
      current_block = "Context"
    when "Execution:"
      current_block = "Execution"
    when "## Main Repo Integration (REQUIRED)"
      current_block = nil
    when /^- ([^:]+):\s*(.*)$/
      key = Regexp.last_match(1)
      value = Regexp.last_match(2).to_s
      if current_block
        blocks[current_block][key] = value
      elsif blocks.key?("Main Repo Integration")
        blocks["Main Repo Integration"][key] = value
      end
    when /^([A-Za-z][A-Za-z0-9 \/()_-]+):\s*(.*)$/
      key = Regexp.last_match(1)
      value = Regexp.last_match(2).to_s
      fields[key] = value
      current_block = "Main Repo Integration" if key == "Integration state"
    end

    if stripped == "## Main Repo Integration (REQUIRED)"
      current_block = "Main Repo Integration"
    end
  end

  { "fields" => fields, "blocks" => blocks, "sections" => parse_markdown_sections(text) }
end

def section_body(text, heading)
  match = text.match(/^## #{Regexp.escape(heading)}\n(.*?)(?=^## |\z)/m)
  match ? match[1].to_s : ""
end

def bullet_paths(section_text)
  section_text.lines.map do |line|
    match = line.match(/^- `([^`]+)`$/)
    match && match[1]
  end.compact
end

def repo_relative_path?(value)
  return false if blank?(value)
  return false if value.start_with?("/")
  return false if value.match?(/\A[A-Za-z]:\\/)
  return false if value.include?("..")

  true
end

def placeholder_value?(value)
  return true if blank?(value)

  placeholders = [
    "none | list explicitly",
    "worktree_only | pr_open | merged",
    "worktree | pr_branch | main_repo",
    "PASS | FAIL",
    "PASS | FAIL | PARTIAL | NOT_RUN",
    "true | false | unknown",
    "true | false | not_applicable | unknown"
  ]
  placeholders.include?(value)
end

def fetch_path(data, path)
  path.split(".").reduce(data) do |acc, key|
    return nil unless acc.is_a?(Hash)
    acc[key]
  end
end

def blank?(value)
  value.nil? || (value.is_a?(String) && value.strip.empty?)
end

def valid_slug?(value)
  value.match?(/\A[a-z0-9]+(?:-[a-z0-9]+)*\z/)
end

def valid_task_id?(value)
  value.match?(/\Aissue-\d{4}\z/)
end

def valid_version?(value)
  value.match?(/\Av\d+\.\d+\z/)
end

def valid_branch?(value)
  value.match?(/\Acodex\/[a-z0-9][a-z0-9-]*\z/)
end

def valid_github_url?(value, kind)
  return false if blank?(value)
  regex =
    case kind
    when :issue then %r{\Ahttps://github\.com/[^/]+/[^/]+/issues/\d+\z}
    when :pr then %r{\Ahttps://github\.com/[^/]+/[^/]+/pull/\d+\z}
    else raise "unsupported github url kind: #{kind}"
    end
  value.match?(regex)
end

def valid_reference?(value)
  return false if blank?(value)
  value.start_with?("http://", "https://") || value.match?(/\A[\w.\-\/]+\z/)
end

def valid_iso8601_datetime?(value)
  Time.iso8601(value)
  true
rescue ArgumentError
  false
end

def check_type(path, value, spec)
  type = spec.fetch("type")
  case type
  when "string"
    raise ValidationError, "#{path} must be a string" unless value.is_a?(String)
  when "integer"
    raise ValidationError, "#{path} must be an integer" unless value.is_a?(Integer)
  when "boolean"
    raise ValidationError, "#{path} must be true or false" unless value == true || value == false || %w[true false].include?(value)
  when "string_array"
    raise ValidationError, "#{path} must be an array of strings" unless value.is_a?(Array) && value.all? { |v| v.is_a?(String) }
    if spec["min_items"] && value.length < spec["min_items"]
      raise ValidationError, "#{path} must contain at least #{spec['min_items']} item(s)"
    end
  when "enum"
    values = spec.fetch("values")
    raise ValidationError, "#{path} must be one of: #{values.join(', ')}" unless values.include?(value)
  when "slug"
    raise ValidationError, "#{path} must be a normalized slug" unless value.is_a?(String) && valid_slug?(value)
  when "task_id"
    raise ValidationError, "#{path} must match issue-0000" unless value.is_a?(String) && valid_task_id?(value)
  when "version"
    raise ValidationError, "#{path} must match v0.85-style version format" unless value.is_a?(String) && valid_version?(value)
  when "branch"
    raise ValidationError, "#{path} must be a codex/ branch" unless value.is_a?(String) && valid_branch?(value)
  when "github_issue_url"
    raise ValidationError, "#{path} must be a GitHub issue URL" unless value.is_a?(String) && valid_github_url?(value, :issue)
  when "github_pr_url"
    raise ValidationError, "#{path} must be a GitHub PR URL" unless value.is_a?(String) && valid_github_url?(value, :pr)
  when "reference"
    raise ValidationError, "#{path} must be a repo-relative reference or URL" unless value.is_a?(String) && valid_reference?(value)
  when "iso8601_datetime"
    raise ValidationError, "#{path} must be ISO 8601 date-time" unless value.is_a?(String) && valid_iso8601_datetime?(value)
  else
    raise ValidationError, "unsupported contract type #{type.inspect} for #{path}"
  end
end

def normalized_hash_for(type, text)
  case type
  when "stp"
    front_matter, body = parse_front_matter(text)
    front_matter["sections"] = parse_markdown_sections(body)
    front_matter
  when "sip", "sor"
    parse_prompt_card(text)
  else
    raise ValidationError, "unsupported type: #{type}"
  end
end

def allow_blank_fields(contract, phase)
  return [] unless phase
  contract.fetch("phases", {}).fetch(phase, {}).fetch("allow_blank", [])
end

def validate_completed_sor!(text, normalized)
  fields = normalized.fetch("fields", {})
  blocks = normalized.fetch("blocks", {})
  execution = blocks.fetch("Execution", {})
  integration = blocks.fetch("Main Repo Integration", {})

  status = fields["Status"]
  raise ValidationError, "completed SOR must use terminal Status (DONE or FAILED)" unless %w[DONE FAILED].include?(status)

  %w[Actor Model Provider Start\ Time End\ Time].each do |field|
    raise ValidationError, "completed SOR missing Execution.#{field}" if blank?(execution[field])
  end

  %w[Integration\ state Verification\ scope Integration\ method\ used Result].each do |field|
    value = integration[field]
    raise ValidationError, "completed SOR missing Main Repo Integration.#{field}" if placeholder_value?(value)
  end

  worktree_remaining = integration["Worktree-only paths remaining"]
  raise ValidationError, "completed SOR still contains unresolved worktree placeholder" if placeholder_value?(worktree_remaining)

  artifacts = bullet_paths(section_body(text, "Artifacts produced"))
  raise ValidationError, "completed SOR must list at least one artifact path" if artifacts.empty?
  invalid_artifact = artifacts.find { |path| !repo_relative_path?(path) }
  raise ValidationError, "completed SOR artifact path must be repo-relative: #{invalid_artifact}" if invalid_artifact

  validation_section = section_body(text, "Validation")
  raise ValidationError, "completed SOR must record validation commands" unless validation_section.include?("- Tests / checks run:")
  raise ValidationError, "completed SOR must record validation results" unless validation_section.include?("- Results:")
end

begin
  text = input.read
  normalized = normalized_hash_for(type, text)
  allowed_blank = allow_blank_fields(contract, options[:phase])

  contract.fetch("required_sections", []).each do |section|
    sections = type == "stp" ? normalized.fetch("sections", []) : normalized.fetch("sections", [])
    raise ValidationError, "missing required section: #{section}" unless sections.include?(section)
  end

  contract.fetch("fields", {}).each do |path, spec|
    data =
      case type
      when "stp"
        normalized
      else
        if path.include?(".")
          root, *rest = path.split(".")
          root == "Context" || root == "Execution" || root == "Main Repo Integration" ? normalized.fetch("blocks", {}) : normalized.fetch("fields", {})
        else
          normalized.fetch("fields", {})
        end
      end

    value = fetch_path(data, path)
    if blank?(value)
      next if !spec["required"] || allowed_blank.include?(path)
      raise ValidationError, "missing required field: #{path}"
    end

    check_type(path, value, spec)
  end

  if type == "sip"
    prompt_lint = Pathname(__dir__).join("lint_prompt_spec.sh").realpath
    ok = system(prompt_lint.to_s, "--input", input.to_s, out: File::NULL, err: File::NULL)
    raise ValidationError, "Prompt Spec lint failed for #{input}" unless ok
  end

  validate_completed_sor!(text, normalized) if type == "sor" && options[:phase] == "completed"

  puts "PASS: #{type} contract valid for #{input}"
rescue ValidationError => e
  die!(e.message)
end
