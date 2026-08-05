#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
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
  default_branch = repo["default_branch"].to_s
  abort "live repository lacks default branch for #{repository}" if default_branch.empty?
  commit = gh_json("repos/#{repository}/commits/#{default_branch}")
  abort "live default HEAD mismatch for #{repository}" unless commit["sha"] == row["exact_head"]

  issue_items = pages("repos/#{repository}/issues?state=all")
  issues = issue_items.reject { |item| item.key?("pull_request") }
  pulls = pages("repos/#{repository}/pulls?state=all")
  packages = PACKAGE_TYPES.flat_map do |type|
    pages("orgs/agent-logic/packages?package_type=#{type}").select { |pkg| pkg.dig("repository", "full_name") == repository }
  end
  workflows = gh_json("repos/#{repository}/actions/workflows").fetch("workflows", [])
  actions_permissions = gh_json("repos/#{repository}/actions/permissions")
  secrets = gh_json("repos/#{repository}/actions/secrets").fetch("secrets", [])
  variables = gh_json("repos/#{repository}/actions/variables").fetch("variables", [])
  installations = gh_json("repos/#{repository}/installations").fetch("installations", [])

  live = {
    "visibility" => repo["visibility"],
    "history" => {"default_branch" => default_branch, "default_head" => commit["sha"]},
    "issues" => issues.map { |item| {"number" => item["number"], "state" => item["state"], "assignees" => Array(item["assignees"]).map { |a| a["login"] }.sort} },
    "pull_requests" => pulls.map { |item| {"number" => item["number"], "state" => item["state"], "assignees" => Array(item["assignees"]).map { |a| a["login"] }.sort} },
    "assignees" => (issues + pulls).flat_map { |item| Array(item["assignees"]).map { |a| a["login"] } }.uniq.sort,
    "collaborators" => pages("repos/#{repository}/collaborators?affiliation=all").map { |item| {"login" => item["login"], "permissions" => item["permissions"]} },
    "teams" => pages("repos/#{repository}/teams").map { |item| {"slug" => item["slug"], "permission" => item["permission"]} },
    "oidc" => gh_json("repos/#{repository}/actions/oidc/customization/sub", allow_missing: true),
    "webhooks" => pages("repos/#{repository}/hooks").map { |item| {"id" => item["id"], "active" => item["active"], "events" => item["events"], "type" => item["type"]} },
    "apps" => installations.map { |item| {"id" => item["id"], "app_id" => item["app_id"], "account" => item.dig("account", "login"), "permissions" => item["permissions"], "events" => item["events"]} },
    "rulesets" => gh_json("repos/#{repository}/rulesets"),
    "releases" => pages("repos/#{repository}/releases").map { |item| {"id" => item["id"], "tag_name" => item["tag_name"], "draft" => item["draft"], "prerelease" => item["prerelease"]} },
    "actions" => {"permissions" => actions_permissions, "workflows" => workflows.map { |item| {"id" => item["id"], "path" => item["path"], "state" => item["state"]} }},
    "pages" => gh_json("repos/#{repository}/pages", allow_missing: true),
    "packages" => packages.map { |item| {"id" => item["id"], "name" => item["name"], "package_type" => item["package_type"]} },
    "secrets" => secrets.map { |item| {"name" => item["name"], "created_at" => item["created_at"], "updated_at" => item["updated_at"]} },
    "variables" => variables.map { |item| {"name" => item["name"], "created_at" => item["created_at"], "updated_at" => item["updated_at"], "visibility" => item["visibility"]} }
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

controls = report.fetch("negative_controls")
{"danielbaustin/asksifu" => "asksifu", "danielbaustin/Horust" => "Horust"}.each do |repository, key|
  live = gh_json("repos/#{repository}")
  abort "negative-control identity mismatch" unless live["full_name"] == repository
  expected = controls.fetch(key)
  abort "negative-control repository id changed" unless live["id"].to_s == expected["repository_id"].to_s
  branch = live["default_branch"].to_s
  head = gh_json("repos/#{repository}/commits/#{branch}")["sha"]
  abort "negative-control HEAD changed" unless head == expected["exact_head"]
end

puts "WP-02 live verification valid: five repositories, full destination inventory, exact default HEADs, and two live negative controls"
