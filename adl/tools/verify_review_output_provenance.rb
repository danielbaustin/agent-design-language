#!/usr/bin/env ruby
# frozen_string_literal: true

require "optparse"
require "pathname"
require "psych"
require "time"

POINTER_PREFIX_ORDER = %w[path: command: ci: artifact:].freeze
DECISIONS = %w[PASS MINOR_FIXES MAJOR_ISSUES].freeze
EVIDENCE_STATES = %w[contradicted not_evidenced not_applicable].freeze
VALIDATION_RESULTS = %w[PASS FAIL PARTIAL].freeze
ABSOLUTE_HOST_PATH_RE = %r{(?:^|[\s`'"])/(Users|home|tmp|var/folders)/}.freeze

def parse_args
  options = {}
  OptionParser.new do |opts|
    opts.banner = "Usage: verify_review_output_provenance.rb --review <yaml>"
    opts.on("--review PATH", "Review artifact path") { |value| options[:review] = value }
  end.parse!

  abort("missing --review") unless options[:review]
  options
end

def repo_root
  root = `git rev-parse --show-toplevel 2>/dev/null`.strip
  abort("not in a git repo") if root.empty?
  Pathname(root)
end

def repo_relative?(value)
  return false if value.nil? || value.empty?
  return false if value.start_with?("/")
  return false if value.match?(/\A[A-Za-z]:\\/)
  return false if value.include?("..")

  true
end

def classify_pointer(pointer)
  prefix = POINTER_PREFIX_ORDER.find { |candidate| pointer.start_with?(candidate) }
  abort("unsupported evidence pointer prefix: #{pointer}") unless prefix

  [POINTER_PREFIX_ORDER.index(prefix), pointer]
end

def ensure_sorted!(items, label)
  sorted = items.sort_by { |item| classify_pointer(item) }
  abort("#{label} must use canonical evidence-pointer ordering") unless items == sorted
end

def assert!(condition, message)
  abort(message) unless condition
end

options = parse_args
root = repo_root
review_path = Pathname(File.expand_path(options[:review], Dir.pwd))
assert!(review_path.file?, "review artifact not found: #{review_path}")

review = Psych.safe_load(review_path.read, permitted_classes: [Time], aliases: false)
assert!(review.is_a?(Hash), "review artifact must be a YAML mapping")
assert!(review["review_format_version"] == "card_review_output.v1", "unexpected review_format_version")
assert!(DECISIONS.include?(review["decision"]), "invalid decision enum")

target = review.fetch("review_target")
input_path = target.fetch("input_card_path")
output_path = target.fetch("output_card_path")
assert!(repo_relative?(input_path), "input_card_path must be repo-relative")
assert!(repo_relative?(output_path), "output_card_path must be repo-relative")
assert!(root.join(input_path).file?, "input card path does not exist: #{input_path}")
assert!(root.join(output_path).file?, "output card path does not exist: #{output_path}")

findings = review.fetch("findings")
assert!(findings.is_a?(Array), "findings must be an array")
findings.each do |finding|
  assert!(EVIDENCE_STATES.include?(finding.fetch("evidence_state")), "invalid evidence_state")
  evidence = finding.fetch("evidence")
  assert!(evidence.is_a?(Array) && !evidence.empty?, "finding evidence must be a non-empty array")
  ensure_sorted!(evidence, "finding evidence")
  evidence.each do |pointer|
    if pointer.start_with?("path:") || pointer.start_with?("artifact:")
      path_value = pointer.split(":", 2).last
      assert!(repo_relative?(path_value), "evidence pointer path must be repo-relative: #{pointer}")
    end
  end
end

validation_checks = review.fetch("validation_checks")
assert!(VALIDATION_RESULTS.include?(validation_checks.fetch("validation_result")), "invalid validation_result enum")
commands = validation_checks.fetch("commands")
assert!(commands.is_a?(Array), "validation commands must be an array")
commands.each do |command|
  assert!(!command.match?(ABSOLUTE_HOST_PATH_RE), "validation command leaks absolute host path")
end

security_checks = review.fetch("security_privacy_checks")
assert!([true, false].include?(security_checks.fetch("absolute_host_paths_present")), "absolute_host_paths_present must be boolean")
if security_checks["absolute_host_paths_present"] == false
  serialized = review_path.read
  assert!(!serialized.match?(ABSOLUTE_HOST_PATH_RE), "review artifact contradicts absolute_host_paths_present=false")
end

puts "PASS: review output provenance valid for #{review_path}"
