#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE_ROOT = ROOT.join(".csdlc/prepared/issues/5107")

FILES = [
  ISSUE_ROOT.join("PREPARATION_PACKET.md"),
  ISSUE_ROOT.join("design.md"),
  ISSUE_ROOT.join("diagram.mmd"),
  ISSUE_ROOT.join("bootstrap-request.json"),
  ROOT.join("docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md"),
  ROOT.join("docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"),
  ROOT.join("docs/milestones/v0.92/WBS_v0.92.md"),
  ROOT.join("docs/milestones/v0.92/SPRINT_v0.92.md"),
  ROOT.join("docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"),
  ROOT.join("docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md")
].freeze

required_terms = [
  "48e0081bb1c576d4c9bf351e659390eeeef62e9c",
  "11151e0beab02b1667f6505b7f8992bfd47d2f8f",
  "f7258b07e9da414bfee518f0c89a76071bc03ee8",
  "fc75f4fc697262f89f99461679a406be0b4b3775",
  "fa39a8856dd5a23544831f8d2cdced1ffad492d8",
  "#5104 is historical input only",
  "Learning-driven graph mutation is not implemented",
  "Prompt",
  "Loop",
  "Adaptive Loop",
  "Reasoning Graph",
  "Adaptive Learning DAG",
  "No child implementation issues"
].freeze

errors = []

FILES.each do |path|
  errors << "missing #{path.relative_path_from(ROOT)}" unless path.file?
end

combined = FILES.select(&:file?).map(&:read).join("\n")
required_terms.each do |term|
  errors << "missing required term: #{term}" unless combined.include?(term)
end

bootstrap = JSON.parse(ISSUE_ROOT.join("bootstrap-request.json").read)
errors << "wrong issue" unless bootstrap["issue"] == 5107
errors << "wrong branch" unless bootstrap.dig("claim", "branch") == "codex/5107-v092-adaptive-learning-dag-queue"
errors << "validator lane missing" unless bootstrap.dig("initial", "validation_lanes").to_a.any? { |lane| lane["lane"] == "preparation-doc-contract" }
errors << "child issue non-goal missing" unless bootstrap.dig("initial", "non_goals").to_a.join("\n").include?("Child issue creation")
errors << "product/runtime non-goal missing" unless bootstrap.dig("initial", "non_goals").to_a.join("\n").include?("Runtime code")

forbidden_claims = [
  "adaptive learning is implemented",
  "learning-driven graph mutation is implemented",
  "production autonomous learning is implemented"
]
downcased = combined.downcase
forbidden_claims.each do |claim|
  errors << "forbidden implementation claim: #{claim}" if downcased.include?(claim)
end

if errors.any?
  warn(errors.join("\n"))
  exit 1
end

puts JSON.pretty_generate(
  schema: "adl.csdlc.preparation_validation.v1",
  issue: 5107,
  lane: "preparation-doc-contract",
  outcome: "passed",
  files: FILES.map { |path| path.relative_path_from(ROOT).to_s }
)
