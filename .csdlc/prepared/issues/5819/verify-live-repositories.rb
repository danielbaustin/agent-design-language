#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../..").cleanpath
REPORT = ROOT.join(".csdlc/evidence/5819/migration-report.json")
REPOS = %w[cognitive-sdlc-paper godel-hadamard-bayes-paper general-intelligence-paper-private universal-tool-schema agent-design-language].freeze
PACKAGE_TYPES = %w[container docker maven npm nuget rubygems].freeze

def gh_json(*args, allow_missing: false)
  stdout, stderr, status = Open3.capture3("gh", "api", *args)
  return {"state" => "absent"} if allow_missing && !status.success? && stderr.match?(/HTTP 404|Not Found/i)
  abort "gh api #{args.join(' ')} failed: #{stderr.strip}" unless status.success?
  JSON.parse(stdout)
end

def pages(path)
  output = []
  loop do
    batch = gh_json("#{path}#{path.include?('?') ? '&' : '?'}per_page=100&page=#{output.length / 100 + 1}")
    break unless batch.is_a?(Array) && !batch.empty?
    output.concat(batch)
    break if batch.length < 100
  end
  output
end

def canonical(value)
  case value
  when Hash then value.keys.sort.to_h { |key| [key, canonical(value.fetch(key))] }
  when Array then value.map { |item| canonical(item) }.sort_by { |item| JSON.generate(item) }
  else value
  end
end

abort "missing migration report" unless REPORT.file? && !REPORT.zero?
report = JSON.parse(REPORT.read)

REPOS.each do |name|
  repository = "agent-logic/#{name}"
  row = report.fetch("repositories").find { |candidate| candidate["name"] == name } || abort("missing report row #{name}")
  after_path = ROOT.join(row.fetch("after_manifest_path")).cleanpath
  after = JSON.parse(after_path.read)

  repo = gh_json("repos/#{repository}")
  abort "live repository identity mismatch for #{repository}" unless repo["full_name"] == repository
  abort "live repository lacks default branch for #{repository}" if repo["default_branch"].to_s.empty?

  issues = pages("repos/#{repository}/issues?state=all").reject { |item| item.key?("pull_request") }
  pulls = pages("repos/#{repository}/pulls?state=all")
  packages = PACKAGE_TYPES.flat_map do |type|
    pages("orgs/agent-logic/packages?package_type=#{type}").select { |pkg| pkg.dig("repository", "full_name") == repository }
  end
  live = {
    "issues" => issues.map { |item| {"number" => item["number"], "state" => item["state"], "assignees" => Array(item["assignees"]).map { |a| a["login"] }.sort} },
    "pull_requests" => pulls.map { |item| {"number" => item["number"], "state" => item["state"], "assignees" => Array(item["assignees"]).map { |a| a["login"] }.sort} },
    "assignees" => (issues + pulls).flat_map { |item| Array(item["assignees"]).map { |a| a["login"] } }.uniq.sort,
    "rulesets" => gh_json("repos/#{repository}/rulesets"),
    "releases" => pages("repos/#{repository}/releases").map { |item| {"id" => item["id"], "tag_name" => item["tag_name"], "draft" => item["draft"], "prerelease" => item["prerelease"]} },
    "actions" => gh_json("repos/#{repository}/actions/workflows").fetch("workflows", []).map { |item| {"id" => item["id"], "path" => item["path"], "state" => item["state"]} },
    "pages" => gh_json("repos/#{repository}/pages", allow_missing: true),
    "packages" => packages.map { |item| {"id" => item["id"], "name" => item["name"], "package_type" => item["package_type"]} },
    "integrations" => {"hooks" => pages("repos/#{repository}/hooks").map { |item| {"id" => item["id"], "active" => item["active"], "events" => item["events"]} }}
  }

  expected = after.fetch("live_snapshot")
  live.each do |surface, actual|
    abort "#{repository} live #{surface} differs from after manifest" unless canonical(actual) == canonical(expected.fetch(surface))
  end

  lfs = after.fetch("surfaces").fetch("lfs")
  receipt = ROOT.join(lfs.fetch("fsck_receipt_path")).cleanpath
  abort "#{repository} LFS receipt missing" unless receipt.file? && !receipt.zero?
  abort "#{repository} LFS receipt digest mismatch" unless Digest::SHA256.file(receipt).hexdigest == lfs.fetch("fsck_receipt_sha256")
end

%w[danielbaustin/asksifu danielbaustin/Horust].each do |repository|
  row = gh_json("repos/#{repository}")
  abort "negative-control identity mismatch" unless row["full_name"] == repository
end

puts "WP-02 live verification valid: five repositories across issues, PRs, assignees, rulesets, releases, Actions, Pages, packages, LFS, and integrations"
