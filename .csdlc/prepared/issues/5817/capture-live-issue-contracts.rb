# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "time"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
WAVE_PATH = File.join(ROOT, "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml")
OUTPUT_PATH = File.join(ROOT, ".csdlc/evidence/5817/live-issue-contracts.json")

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
issue_numbers = [5817]
issue_numbers.concat(wave.fetch("work_packages").map { |row| Integer(row.fetch("issue")) })
issue_numbers.concat(wave.fetch("supporting_issues").map { |row| Integer(row.fetch("issue")) })
issue_numbers.concat(wave.fetch("execution_sprints").map { |row| Integer(row.fetch("issue")) })
issue_numbers = issue_numbers.uniq.sort

git_common_dir, status = Open3.capture2("git", "-C", ROOT, "rev-parse", "--git-common-dir")
raise "cannot resolve Git common directory" unless status.success?

git_common_dir = File.expand_path(git_common_dir.strip, ROOT)
repository_root = File.dirname(git_common_dir)
binary = ENV.fetch(
  "CSDLC_GITHUB_BIN",
  File.join(repository_root, ".adl/bin/csdlc-v2/csdlc-github")
)
raise "missing stable csdlc-github binary: #{binary}" unless File.executable?(binary)

request_dir = File.join(git_common_dir, "csdlc-v2/requests/5817-live-contracts")
FileUtils.mkdir_p(request_dir)
packets = issue_numbers.map do |issue|
  request = {
    repository: "danielbaustin/agent-design-language",
    action: "issue_read",
    issue: issue
  }
  request_path = File.join(request_dir, "read-#{issue}.json")
  File.write(request_path, JSON.pretty_generate(request) + "\n")
  stdout, stderr, command_status = Open3.capture3(binary, "run", "--request", request_path)
  raise "live issue read failed for ##{issue}: #{stderr}#{stdout}" unless command_status.success?

  result = JSON.parse(stdout)
  packet = result.fetch("issue")
  raise "live issue identity mismatch for ##{issue}" unless Integer(packet.fetch("number")) == issue

  packet
end

evidence = {
  schema: "adl.v092.live_issue_contracts.v1",
  repository: "danielbaustin/agent-design-language",
  observed_at: Time.now.utc.iso8601,
  producer: ".adl/bin/csdlc-v2/csdlc-github",
  wave_sha256: Digest::SHA256.file(WAVE_PATH).hexdigest,
  issues: packets
}
File.write(OUTPUT_PATH, JSON.pretty_generate(evidence) + "\n")
puts "captured #{packets.length} live v0.92 issue contracts in #{OUTPUT_PATH}"
