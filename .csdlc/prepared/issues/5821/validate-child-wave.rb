#!/usr/bin/env ruby
# frozen_string_literal: true

design_path = File.expand_path("design.md", __dir__)
text = File.read(design_path)
rows = text.lines.grep(/^\| WP-04\.\d{2} \|/)
abort "expected 16 child rows, found #{rows.length}" unless rows.length == 16

children = rows.map do |line|
  cells = line.split("|").map(&:strip).reject(&:empty?)
  abort "malformed child row: #{line}" unless cells.length == 4
  id, outcome, dependencies, protected = cells
  paths = protected.scan(/`([^`]+)`/).flatten
  abort "#{id} has no outcome" if outcome.empty?
  abort "#{id} has no dependency declaration" if dependencies.empty?
  abort "#{id} has no protected paths" if paths.empty?
  [id, dependencies, paths]
end

expected_ids = (1..16).map { |number| format("WP-04.%02d", number) }
ids = children.map(&:first)
abort "child identities differ from #{expected_ids.join(', ')}" unless ids == expected_ids

all_paths = children.flat_map { |id, _, paths| paths.map { |path| [path, id] } }
duplicates = all_paths.group_by(&:first).select { |_, entries| entries.length > 1 }
abort "duplicate protected paths: #{duplicates.keys.join(', ')}" unless duplicates.empty?

overlaps = all_paths.combination(2).select do |(left, left_id), (right, right_id)|
  left_id != right_id && (left.start_with?("#{right}/") || right.start_with?("#{left}/"))
end
abort "overlapping protected paths: #{overlaps.inspect}" unless overlaps.empty?

dependency_graph = {}
children.each do |id, dependencies, _|
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

required = [
  "`WP-04-IMP`",
  "implementation umbrella",
  "docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md",
  "docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md"
]
missing = required.reject { |value| text.include?(value) }
abort "missing gate contract: #{missing.join(', ')}" unless missing.empty?

puts "PASS: 16 unique children, #{all_paths.length} unique protected paths, resolvable dependencies"
