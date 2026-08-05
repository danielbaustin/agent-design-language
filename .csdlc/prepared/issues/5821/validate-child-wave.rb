#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

design_path = File.expand_path("design.md", __dir__)
text = File.read(design_path)
rows = text.lines.grep(/^\| WP-04\.\d{2} \|/)
abort "expected 16 child rows, found #{rows.length}" unless rows.length == 16

children = rows.map do |line|
  cells = line.split("|").map(&:strip).reject(&:empty?)
  abort "malformed seven-field child row: #{line}" unless cells.length == 7
  id, issue_cell, owner, dependencies, protected, proof, rollback = cells
  issue = issue_cell[/#(\d+)/, 1]&.to_i
  paths = protected.scan(/`([^`]+)`/).flatten
  abort "#{id} missing live issue" unless issue
  abort "#{id} missing issue owner" unless owner.include?("Issue ##{issue}")
  abort "#{id} missing dependency declaration" if dependencies.empty?
  abort "#{id} missing protected paths" if paths.empty?
  abort "#{id} missing proving boundary" unless proof.include?("Exact nonzero") || id == "WP-04.16"
  abort "#{id} missing rollback responsibility" if rollback.length < 20
  [id, issue, dependencies, paths]
end

expected_ids = (1..16).map { |number| format("WP-04.%02d", number) }
expected_issues = (5863..5878).to_a
abort "child identities drifted" unless children.map(&:first) == expected_ids
abort "live issue mapping drifted" unless children.map { |row| row[1] } == expected_issues

all_paths = children.flat_map { |id, _, _, paths| paths.map { |path| [path, id] } }
duplicates = all_paths.group_by(&:first).select { |_, entries| entries.length > 1 }
abort "duplicate protected paths: #{duplicates.keys.join(', ')}" unless duplicates.empty?
overlaps = all_paths.combination(2).select do |(left, left_id), (right, right_id)|
  left_id != right_id && (left.start_with?("#{right}/") || right.start_with?("#{left}/"))
end
abort "overlapping protected paths: #{overlaps.inspect}" unless overlaps.empty?

dependency_graph = {}
children.each do |id, _, dependencies, _|
  child_dependencies = dependencies.scan(/WP-04\.\d{2}/)
  dependency_graph[id] = child_dependencies
  child_dependencies.each do |dependency|
    abort "#{id} references unknown dependency #{dependency}" unless expected_ids.include?(dependency)
    abort "#{id} depends on itself" if dependency == id
  end
end
visiting = {}
visited = {}
visit = lambda do |id|
  abort "dependency cycle reaches #{id}" if visiting[id]
  return if visited[id]
  visiting[id] = true
  dependency_graph.fetch(id).each { |dependency| visit.call(dependency) }
  visiting.delete(id)
  visited[id] = true
end
expected_ids.each { |id| visit.call(id) }

children.each do |id, issue, _, _|
  index_path = File.expand_path("../../../issues/#{issue}/index.json", __dir__)
  abort "missing typed record for #{id} ##{issue}" unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  abort "#{id} design not approved" unless index.dig("design_review", "approved", "revision").to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "#{id} preparation claim active" unless index["claim"].nil?
end

umbrella = JSON.parse(File.read(File.expand_path("../../../issues/5862/index.json", __dir__)))
abort "WP-04-IMP claim active" unless umbrella["claim"].nil?
abort "missing final integration registration owner" unless text.include?("`adl-runtime/src/distributed/mod.rs`") && text.include?("`adl-runtime/src/lib.rs`")

git_common, git_status = Open3.capture2("git", "rev-parse", "--git-common-dir")
abort "cannot resolve Git common directory" unless git_status.success?
github_binary = ENV.fetch("CSDLC_GITHUB_ISSUE_BIN", File.join(File.expand_path("..", git_common.strip), ".adl/bin/csdlc-v2/csdlc-github-issue"))
abort "missing typed GitHub issue binary" unless File.executable?(github_binary)
expected_titles = {5862 => "[v0.92][WP-04-IMP][umbrella] Execute distributed Guardian child wave"}
children.each do |id, issue, _, _|
  local_title = JSON.parse(File.read(File.expand_path("../../../issues/#{issue}/cards/stp.values.json", __dir__))).dig("identity", "title")
  expected_titles[issue] = local_title
end
expected_titles.each do |issue, title|
  request_path = File.join(__dir__, "wp04-implementation-wave", "read", "#{issue}.json")
  abort "missing live read request for ##{issue}" unless File.file?(request_path)
  stdout, stderr, status = Open3.capture3(github_binary, "run", "--request", request_path)
  abort "live read failed for ##{issue}: #{stderr} #{stdout}" unless status.success?
  packet = JSON.parse(stdout).fetch("issue")
  abort "live issue ##{issue} is not open" unless packet["state"] == "open"
  abort "live title drift for ##{issue}" unless packet["title"] == title
  if issue != 5862
    abort "live body lost WP-04-IMP dependency for ##{issue}" unless packet["body"].include?("#5862")
  end
end
umbrella_request = File.join(__dir__, "wp04-implementation-wave", "read", "5862.json")
stdout, = Open3.capture2(github_binary, "run", "--request", umbrella_request)
live_umbrella = JSON.parse(stdout).fetch("issue").fetch("body")
children.each { |id, issue, _, _| abort "umbrella missing #{id} ##{issue}" unless live_umbrella.include?("#{id}: ##{issue}") }

puts "PASS: live #5862 plus 16 mapped approved claim-null children, #{all_paths.length} exclusive paths, complete owner/proof/rollback fields"
