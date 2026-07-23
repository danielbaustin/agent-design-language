#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "optparse"

options = {}
OptionParser.new do |parser|
  parser.on("--request PATH") { |path| options[:request] = path }
end.parse!
abort "--request is required" unless options[:request]

request = JSON.parse(File.read(options[:request]))
abort "unexpected issue" unless request.fetch("issue") == 5589

def run_git(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  abort "git #{args.join(' ')} failed: #{stderr}" unless status.success?
  stdout
end

def verify_revision!(label, revision)
  abort "#{label} is not a full revision" unless revision.match?(/\A[0-9a-f]{40}\z/)
  abort "#{label} does not resolve exactly" unless run_git("rev-parse", revision).strip == revision
end

def verify_ancestor!(older, newer)
  _stdout, stderr, status = Open3.capture3("git", "merge-base", "--is-ancestor", older, newer)
  abort "#{older} is not an ancestor of #{newer}: #{stderr}" unless status.success?
end

def changed_paths(from, to)
  run_git("diff", "--name-status", "#{from}..#{to}").lines.map do |line|
    status, path = line.chomp.split("\t", 2)
    abort "unparseable changed-path entry: #{line.inspect}" unless status && path
    { "status" => status, "path" => path }
  end
end

base = request.fetch("base_revision")
substantive = request.fetch("substantive_head_revision")
final = request.fetch("final_evidence_head_revision")
verify_revision!("base revision", base)
verify_revision!("substantive head revision", substantive)
verify_revision!("final evidence head revision", final)
verify_ancestor!(base, substantive)
verify_ancestor!(substantive, final)

full_actual = changed_paths(base, final)
delta_actual = changed_paths(substantive, final)
full_expected = request.fetch("full_range_changed_paths")
delta_expected = request.fetch("evidence_delta_changed_paths")
abort "full retained range is empty" if full_actual.empty?
abort "full changed-path inventory mismatch" unless full_actual == full_expected
abort "evidence delta must contain exactly 15 paths" unless delta_actual.length == 15
abort "evidence-delta changed-path inventory mismatch" unless delta_actual == delta_expected

allowed_prefixes = request.fetch("allowed_path_prefixes")
all_paths = (full_actual + delta_actual).map { |entry| entry.fetch("path") }
out_of_scope = all_paths.reject { |path| allowed_prefixes.any? { |prefix| path.start_with?(prefix) } }.uniq
abort "retained range contains out-of-scope paths: #{out_of_scope.join(', ')}" unless out_of_scope.empty?

run_git("diff", "--check", "#{base}..#{final}")
run_git("diff", "--check", "#{substantive}..#{final}")

puts "schema=#{request.fetch('schema')}"
puts "base_revision=#{base}"
puts "substantive_head_revision=#{substantive}"
puts "final_evidence_head_revision=#{final}"
puts "full_range_changed_path_count=#{full_actual.length}"
full_actual.each { |entry| puts "full_range_changed_path=#{entry.fetch('status')}\t#{entry.fetch('path')}" }
puts "evidence_delta_changed_path_count=#{delta_actual.length}"
delta_actual.each { |entry| puts "evidence_delta_changed_path=#{entry.fetch('status')}\t#{entry.fetch('path')}" }
puts "verification=pass"
