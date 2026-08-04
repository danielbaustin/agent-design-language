#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
PACKET = ROOT.join("docs/reviews/v0.91.8/internal-review-5356")
LANES = %w[
  freeze-corpus code security tests docs architecture evidence synthesis
  review-quality complete post-merge-exact
].freeze

def run(*argv)
  out, status = Open3.capture2e(*argv, chdir: ROOT.to_s)
  { argv: argv, status: status.exitstatus, success: status.success?, output: out.strip }
end

def run_in(directory, *argv)
  out, status = Open3.capture2e(*argv, chdir: ROOT.join(directory).to_s)
  { argv: argv, cwd: directory, status: status.exitstatus, success: status.success?, output: out.strip }
end

def git(*args)
  result = run("git", *args)
  raise "git #{args.join(' ')} failed: #{result[:output]}" unless result[:success]

  result[:output]
end

def require_success(results, command)
  result = run(*command)
  results << result
  result[:success]
end

def require_success_in(results, directory, command)
  result = run_in(directory, *command)
  results << result
  result[:success]
end

def stale_release_tail_matches
  patterns = [
    "Active release-tail issue: WP-17",
    'WP-17 `#5360` owns the present',
    "WP-17 #5360 now",
    "WP-17 is aligning",
    "Active documentation and release-truth",
    "Pending internal milestone review"
  ]
  matches = []
  patterns.each do |pattern|
    result = run("rg", "-n", pattern, "docs/milestones/v0.91.8")
    next if result[:status] == 1

    matches << { pattern: pattern, output: result[:output] }
  end
  matches
end

def packet_files
  %w[
    README.md PACKET_MANIFEST.md LIVE_STATE.md SPECIALIST_LANE_RESULTS.md
    FINDINGS_REGISTER.md SYNTHESIS.md VALIDATION.md
  ].map { |name| PACKET.join(name) }
end

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
abort("unknown validation lane: #{lane}") unless LANES.include?(lane)

head = git("rev-parse", "HEAD")
commands = []
findings = []
warnings = []

case lane
when "freeze-corpus"
  require_success(commands, ["ruby", ".csdlc/prepared/issues/5356/check-dependencies.rb"])
  require_success(commands, ["git", "diff", "--check"])
when "code"
  source = ROOT.join("adl-runtime/src/runtime_api.rs").read
  unless source.include?('CSM_RUNTIME_API_ENDPOINTS: [&str; 3]')
    findings << "runtime API endpoint inventory is not narrowed to served routes"
  end
  unless source.include?('["/v1/health", "/v1/metrics", "/v1/acip/ws"]')
    findings << "runtime API advertised endpoints do not match mounted routes"
  end
  require_success_in(
    commands,
    "adl-runtime",
    [
      "cargo", "test", "--locked", "-p", "adl-runtime",
      "runtime_api_contract_advertises_only_served_routes"
    ]
  )
when "security"
  warnings << "historical retained evidence logs contain host paths; current packet does not require them as executable instructions"
  require_success(commands, ["git", "diff", "--check"])
when "tests"
  require_success(
    commands,
    ["python3", "docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/validate_v0918_demo_matrix.py"]
  )
  require_success(
    commands,
    [
      "ruby", "-ryaml", "-e",
      'YAML.safe_load(File.read("docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml"), aliases: true)'
    ]
  )
when "docs"
  stale = stale_release_tail_matches
  findings.concat(stale.map { |match| "stale release-tail wording: #{match[:pattern]}" })
  require_success(commands, ["git", "diff", "--check"])
when "architecture"
  require_success(
    commands,
    [
      "ruby", "-ryaml", "-e",
      'wave=YAML.safe_load(File.read("docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml"), aliases: true); raise "WP-18 dependency drift" unless wave["work_packages"].any?{|wp| wp["wp"]=="WP-18" && wp["depends_on"]==["WP-17"] }'
    ]
  )
when "evidence"
  missing = packet_files.reject(&:file?)
  findings.concat(missing.map { |path| "missing packet file #{path.relative_path_from(ROOT)}" })
  require_success(commands, ["ruby", ".csdlc/prepared/issues/5356/validate-preparation.rb"])
when "synthesis", "review-quality", "complete"
  missing = packet_files.reject(&:file?)
  findings.concat(missing.map { |path| "missing packet file #{path.relative_path_from(ROOT)}" })
  if ROOT.join("docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_5356.md").file? == false
    findings << "missing milestone review summary"
  end
  require_success(commands, ["git", "diff", "--check"])
when "post-merge-exact"
  warnings << "post-merge lane is only meaningful after PR merge; running pre-merge exact-head checks"
  require_success(commands, ["ruby", ".csdlc/prepared/issues/5356/check-dependencies.rb"])
  require_success(commands, ["git", "diff", "--check"])
end

commands.each do |command|
  next if command[:success]

  findings << "command failed: #{command[:argv].join(' ')}"
end

report = {
  schema: "adl.wp18.validation_lane_result.v1",
  issue: 5356,
  lane: lane,
  revision: head,
  packet_digest: PACKET.directory? ? Digest::SHA256.hexdigest(packet_files.select(&:file?).map { |p| "#{p.basename}:#{Digest::SHA256.file(p).hexdigest}" }.join("\n")) : nil,
  commands: commands,
  warnings: warnings,
  findings: findings,
  status: findings.empty? ? "pass" : "fail"
}

puts JSON.pretty_generate(report)
exit(findings.empty? ? 0 : 1)
