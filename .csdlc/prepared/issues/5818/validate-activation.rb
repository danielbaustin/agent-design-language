#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
INVENTORY = ROOT.join(".csdlc/evidence/5818/canonical-surface-inventory.json")
ALLOWED = %w[update already_current historical_preserve not_authoritative].freeze
REQUIRED_PATHS = [
  "README.md",
  "docs/README.md",
  "docs/milestones/v0.92/README.md",
  "docs/planning/ADL_FEATURE_LIST.md",
  "adl/README.md",
  "csdlc-v2/README.md",
  "adl/Cargo.toml",
  "adl/Cargo.lock",
  "adl-v2/Cargo.toml",
  "adl-v2/Cargo.lock",
  *%w[
    adl-adapters adl-cli adl-compiler adl-engine adl-language adl-records
    adl-runtime-v3-adapter adl-workcell-conductor adl-workcell-convergence
    adl-workcell-task-adapter
  ].map { |name| "adl-v2/crates/#{name}/Cargo.toml" },
  "adl-runtime/Cargo.toml",
  "adl-runtime/Cargo.lock",
  "adl-runtime-kernel/Cargo.toml",
  "adl-runtime-kernel/Cargo.lock",
  "adl-resilience/Cargo.toml",
  "adl-resilience/Cargo.lock",
  "adl-characterization/Cargo.toml",
  "adl-characterization/Cargo.lock",
  "csdlc-v2/Cargo.toml",
  "csdlc-v2/Cargo.lock",
  "AGENTS.md",
  "REVIEW.md",
  "docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md",
  "docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md",
  *%w[init github finish review shepherd doctor validate bind clean card-editor publish].map do |name|
    "csdlc-v2/operator/skills/csdlc-v2-#{name}/SKILL.md"
  end
].freeze

DOC_VERSION_PATTERNS = {
  "README.md" => /Current ADL package and workspace version: `0\.92\.0`/,
  "docs/README.md" => /`v0\.92` is the active development\s+milestone/,
  "docs/milestones/v0.92/README.md" => /`0\.92\.0` package\/workspace activation/,
  "docs/planning/ADL_FEATURE_LIST.md" => /`v0\.92` \| \*\*Active development milestone\.\*\*/,
  "adl/README.md" => /Current crate version: \*\*0\.92\.0\*\*/,
  "csdlc-v2/README.md" => /Current package version: `0\.92\.0`/
}.freeze

CARGO_EXPECTATIONS = {
  "adl/Cargo.toml" => %w[adl],
  "adl-v2/Cargo.toml" => %w[
    adl-adapters adl-cli adl-compiler adl-engine adl-language adl-records
    adl-runtime-v3-adapter adl-workcell-conductor adl-workcell-convergence
    adl-workcell-task-adapter
  ],
  "adl-runtime/Cargo.toml" => %w[adl-runtime],
  "adl-runtime-kernel/Cargo.toml" => %w[adl-runtime-kernel],
  "adl-resilience/Cargo.toml" => %w[adl-resilience],
  "adl-characterization/Cargo.toml" => %w[adl-characterization],
  "csdlc-v2/Cargo.toml" => %w[csdlc-v2]
}.freeze

abort "missing canonical surface inventory" unless INVENTORY.file? && !INVENTORY.zero?
rows = JSON.parse(INVENTORY.read)
abort "inventory must be a nonempty array" unless rows.is_a?(Array) && !rows.empty?
abort "duplicate inventory path" unless rows.map { |row| row["path"] }.uniq.length == rows.length
inventory_paths = rows.map { |row| row["path"] }
missing_required = REQUIRED_PATHS - inventory_paths
abort "canonical inventory omits required paths: #{missing_required.join(', ')}" unless missing_required.empty?

rows.each do |row|
  path = row["path"].to_s
  abort "invalid inventory row" if path.empty? || row["owner"].to_s.empty? || !ALLOWED.include?(row["disposition"])
  next if row["disposition"] == "not_authoritative"

  target = ROOT.join(path).cleanpath
  abort "inventory path escapes repository: #{path}" unless target.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  abort "missing inventoried path: #{path}" unless target.exist?

  if target.file? && target.extname == ".json"
    JSON.parse(target.read)
  elsif target.file? && %w[.yaml .yml].include?(target.extname)
    YAML.safe_load(target.read, permitted_classes: [], aliases: false)
  end

  expected = row["expected_version"]
  observed = row["observed_version"]
  abort "version declaration lacks observed version: #{path}" if expected && observed.to_s.empty?
  abort "version mismatch #{path}: #{observed.inspect} != #{expected.inspect}" if expected && observed != expected
end

DOC_VERSION_PATTERNS.each do |path, pattern|
  content = ROOT.join(path).read
  abort "current documentation version mismatch: #{path}" unless content.match?(pattern)
end

CARGO_EXPECTATIONS.each do |manifest, expected_packages|
  stdout, stderr, status = Open3.capture3(
    "cargo", "metadata", "--locked", "--offline", "--no-deps",
    "--format-version", "1", "--manifest-path", manifest,
    chdir: ROOT.to_s
  )
  abort "Cargo metadata failed for #{manifest}: #{stderr.strip}" unless status.success?

  packages = JSON.parse(stdout).fetch("packages").to_h { |package| [package.fetch("name"), package] }
  expected_packages.each do |name|
    package = packages[name] or abort "Cargo metadata omits #{name} from #{manifest}"
    version = package.fetch("version")
    abort "Cargo version mismatch #{name}: #{version} != 0.92.0" unless version == "0.92.0"
  end
end

markdown_paths = rows.each_with_object([]) do |row, paths|
  next unless %w[update already_current].include?(row["disposition"])
  path = ROOT.join(row["path"].to_s)
  paths << path if path.file? && path.extname.downcase == ".md"
end
markdown_paths.each do |path|
  path.read.scan(/\[[^\]]*\]\(([^)]+)\)/).flatten.each do |raw|
    link = raw.strip.split(/\s+/, 2).first.to_s.delete_prefix("<").delete_suffix(">")
    next if link.empty? || link.start_with?("#", "http://", "https://", "mailto:")
    relative = link.split("#", 2).first
    next if relative.empty?
    target = path.dirname.join(relative).cleanpath
    abort "broken Markdown link #{path.relative_path_from(ROOT)} -> #{link}" unless target.exist?
  end
end

historical = %w[
  docs/milestones/v0.91.8
  docs/releases
  docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md
  .csdlc/evidence
]
changed = `git diff --name-only origin/main -- #{historical.join(' ')}`.lines.map(&:strip).reject(&:empty?)
unauthorized = changed.reject { |path| path.start_with?(".csdlc/evidence/5818/") }
abort "historical surface changed: #{unauthorized.join(', ')}" unless unauthorized.empty?

puts "WP-01B activation evidence valid: #{rows.length} surfaces, #{markdown_paths.length} Markdown files"
