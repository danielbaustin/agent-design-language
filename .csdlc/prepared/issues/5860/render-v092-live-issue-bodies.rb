#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "yaml"

WAVE_PATH = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"
OUTPUT_ROOT = ".csdlc/evidence/5860/live-issue-bodies"

def markdown_section(markdown, heading)
  lines = markdown.lines
  start = lines.index { |line| line.strip == "## #{heading}" }
  return nil unless start

  lines[(start + 1)..].take_while { |line| !line.match?(/^##\s+/) }.join
end

def owned_paths(issue)
  design = File.read(".csdlc/prepared/issues/#{issue}/design.md")
  section = markdown_section(design, "Owned Paths")
  abort "##{issue}: missing exact Owned Paths section" unless section

  paths = section.scan(/`([^`]+)`/).flatten.uniq
  abort "##{issue}: empty Owned Paths section" if paths.empty?
  paths
end

def values(issue, card)
  JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{card}.values.json"))
    .fetch("content").fetch("values")
end

def bullets(values)
  Array(values).map { |value| "- #{value}" }.join("\n")
end

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
rows = Array(wave["work_packages"]) + Array(wave["supporting_issues"])
rows = rows.select { |row| row["issue"].is_a?(Integer) && row["issue"] != 5817 }
selected = if (index = ARGV.index("--issues"))
             ARGV.fetch(index + 1).split(",").map(&:to_i)
           else
             rows.map { |row| row.fetch("issue") }
           end
write = ARGV.include?("--write")

unknown = selected - rows.map { |row| row.fetch("issue") }
abort "unknown v0.92 execution issues: #{unknown.join(', ')}" unless unknown.empty?

rendered = {}
rows.select { |row| selected.include?(row.fetch("issue")) }.sort_by { |row| row.fetch("issue") }.each do |row|
  issue = row.fetch("issue")
  stp = values(issue, "stp")
  vpp = values(issue, "vpp")
  wp = row["wp"] || row["owner_wp"] || "supporting"
  lanes = Array(vpp["lanes"]).map { |lane| lane.fetch("proof_role") }
  path_lines = owned_paths(issue).map { |path| "- `#{path}`" }.join("\n")
  outcome = row["primary_deliverable"] || stp.fetch("task_boundary")

  body = <<~MARKDOWN
    ## Summary

    Execute **#{wp}** for v0.92 within this issue's approved design and ownership boundary.

    #{stp.fetch("task_boundary")}

    ## Required Outcome

    #{outcome}

    ## Deliverables

    #{bullets(stp.fetch("deliverables"))}

    ## Dependencies

    #{bullets(stp.fetch("dependencies"))}

    ## Owned Paths

    #{path_lines}

    ## Validation And Proof

    #{bullets(lanes)}

    ## Acceptance Criteria

    #{bullets(stp.fetch("acceptance_criteria"))}

    ## Non-Goals

    #{bullets(stp.fetch("non_goals"))}

    ## Execution Boundary

    This issue is design-ready, not implemented. Execution must reverify terminal dependencies, reacquire a just-in-time claim for the exact owned paths, retain the declared positive and negative proof, complete exact-head review, and use `Closes ##{issue}` in its implementation PR.

    <!-- csdlc-github-operation:v092-execution-ready-body-#{issue}-20260805 -->
  MARKDOWN

  rendered[issue] = body
end

if write
  FileUtils.mkdir_p(OUTPUT_ROOT)
  rendered.each { |issue, body| File.write("#{OUTPUT_ROOT}/#{issue}.md", body) }
else
  rendered.each do |issue, body|
    path = "#{OUTPUT_ROOT}/#{issue}.md"
    abort "missing #{path}; run with --write explicitly" unless File.file?(path)
    abort "##{issue}: rendered live body drift" unless File.read(path) == body
  end
end

puts "v0.92 live issue body render: PASS (#{rendered.length} issues#{write ? ', written' : ''})"
