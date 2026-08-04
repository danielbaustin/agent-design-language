#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "fileutils"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13-external-bands")
MANIFEST = File.join(EVIDENCE, "external-band-deletion-manifest.json")
COORDINATION = File.join(EVIDENCE, "wp13-deletion-coordination.json")
ACCOUNTING = File.join(EVIDENCE, "deletion-accounting.json")

def fail!(message)
  warn("#5347 validation failed: #{message}")
  exit(1)
end

def rel(path)
  path.sub(ROOT + "/", "")
end

def load_json(path)
  fail!("missing #{rel(path)}") unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  fail!("invalid JSON #{rel(path)}: #{error.message}")
end

def git!(*argv)
  out, err, status = Open3.capture3("git", "-C", ROOT, *argv)
  fail!("git #{argv.join(' ')} failed: #{err.lines.first}") unless status.success?
  out
end

def command!(*argv)
  target_tmp = File.join(ROOT, "adl/target/tmp")
  FileUtils.mkdir_p(target_tmp)
  env = { "TMPDIR" => target_tmp }
  out, err, status = Open3.capture3(env, *argv, chdir: ROOT)
  fail!("#{argv.join(' ')} failed:\n#{out}\n#{err}") unless status.success?
  out
end

def preserve_cargo_lock
  lock_path = File.join(ROOT, "adl/Cargo.lock")
  original = File.binread(lock_path)
  yield
ensure
  File.binwrite(lock_path, original) if original
end

def validate_relative_path!(path)
  fail!("absolute path #{path}") if path.start_with?("/")
  fail!("escaping path #{path}") if path.split("/").include?("..")
  fail!("non-canonical path #{path}") unless path == File.expand_path(path, "/").sub(%r{\A/}, "")
  fail!("build/cache path #{path}") if (path.split("/") & %w[target build dist node_modules .git]).any?
end

def baseline_revision
  manifest.fetch("baseline_revision")
end

def tracked_blob_lines(path, object)
  actual_object = git!("rev-parse", "#{baseline_revision}:#{path}").strip
  fail!("baseline object mismatch for #{path}") unless actual_object == object
  git!("show", "#{baseline_revision}:#{path}").lines.count
end

def manifest
  @manifest ||= load_json(MANIFEST)
end

def coordination
  @coordination ||= load_json(COORDINATION)
end

def accounting
  @accounting ||= load_json(ACCOUNTING)
end

def deleted_rows
  rows = manifest.fetch("deleted_files")
  fail!("deleted_files must be sorted") unless rows.map { |row| row.fetch("path") } == rows.map { |row| row.fetch("path") }.sort
  rows
end

def retained_paths
  manifest.fetch("retained_current_binaries")
end

def validate_manifest!
  fail!("manifest schema mismatch") unless manifest["schema"] == "adl.wp13.external_band_deletion_manifest.v1"
  fail!("issue mismatch") unless manifest["issue"] == 5347
  fail!("repository mismatch") unless manifest["repository"] == "danielbaustin/agent-design-language"
  fail!("baseline revision malformed") unless baseline_revision.match?(/\A[0-9a-f]{40}\z/)
  _out, _err, status = Open3.capture3("git", "-C", ROOT, "merge-base", "--is-ancestor", baseline_revision, "HEAD")
  fail!("baseline revision is not ancestral to HEAD") unless status.success?
  fail!("merge order must keep #5347 before #5346") unless manifest["merge_order"] == [5347, 5346]
  deleted_rows.each do |row|
    path = row.fetch("path")
    validate_relative_path!(path)
    fail!("deleted row must be regular file") unless row["file_kind"] == "regular_file"
    fail!("deleted row must be non-generated") unless row["generated"] == false
    fail!("unexpected disposition for #{path}") unless row["disposition"] == "delete_external"
    fail!("unexpected owner for #{path}") unless row["replacement_owner"].to_s.match?(/Runtime v3|ADL v2|retained evidence|historical evidence|C-SDLC v2/)
    fail!("missing replacement proof for #{path}") if row["replacement_proof"].to_s.empty?
    expected = tracked_blob_lines(path, row.fetch("baseline_object"))
    fail!("line count mismatch for #{path}") unless row["measured_lines"] == expected
  end
  retained_paths.each do |path|
    validate_relative_path!(path)
    fail!("retained current binary missing: #{path}") unless File.file?(File.join(ROOT, path))
  end
end

def validate_deletions!
  deleted_rows.each do |row|
    path = row.fetch("path")
    fail!("deleted file still exists: #{path}") if File.exist?(File.join(ROOT, path))
    staged = git!("status", "--short", "--", path).strip
    historical = git!("diff", "--name-status", "#{baseline_revision}..HEAD", "--", path).strip
    deleted_in_history = historical.start_with?("D\t")
    deleted_in_worktree = staged.start_with?("D ") || staged.start_with?(" D")
    fail!("#{path} is not deleted relative to baseline") unless deleted_in_history || deleted_in_worktree
  end
  retained_paths.each do |path|
    fail!("retained current binary changed unexpectedly: #{path}") unless git!("diff", "--", path).strip.empty?
  end
end

def validate_coordination!
  fail!("coordination schema mismatch") unless coordination["schema"] == "adl.wp13.deletion_coordination.v1"
  fail!("safe merge order drift") unless coordination["safe_serialized_merge_order"] == [5347, 5346]
  reserved = coordination.fetch("reserved_for_5346")
  deleted_rows.each do |row|
    path = row.fetch("path")
    reserved.each do |entry|
      prefix = entry.fetch("path_prefix")
      fail!("#5347 path overlaps #5346 reserved prefix #{prefix}: #{path}") if path == prefix || path.start_with?("#{prefix}/")
    end
  end
end

def validate_accounting!
  fail!("accounting schema mismatch") unless accounting["schema"] == "adl.wp13.external_band_deletion_accounting.v1"
  removed = deleted_rows.sum { |row| row.fetch("measured_lines") }
  fail!("removed line accounting mismatch") unless accounting["removed_lines"] == removed
  cargo_removed = accounting.fetch("cargo_toml_removed_lines")
  fail!("Cargo removal must be positive") unless cargo_removed.positive?
  fail!("net line accounting mismatch") unless accounting["net_removed_lines"] == removed + cargo_removed
  fail!("deleted file count mismatch") unless accounting["deleted_file_count"] == deleted_rows.length
  fail!("WP-16 must not be a dependency") if accounting.fetch("execution_dependencies").include?(5351)
  fail!("#5346 must be coordination, not prerequisite") if accounting.fetch("execution_dependencies").include?(5346)
  validate_review_finding_cleanup_accounting!
end

def diff_numstat(base, paths)
  return [] if paths.empty?

  lines = git!("diff", "--numstat", base, "--", *paths).lines
  lines.map do |line|
    added, removed, path = line.chomp.split("\t", 3)
    [Integer(added), Integer(removed), path]
  end
end

def validate_review_finding_cleanup_accounting!
  cleanup = accounting.fetch("review_finding_cleanup")
  fail!("review cleanup base malformed") unless cleanup.fetch("base_revision").match?(/\A[0-9a-f]{40}\z/)
  _out, _err, status = Open3.capture3("git", "-C", ROOT, "merge-base", "--is-ancestor", cleanup.fetch("base_revision"), "HEAD")
  fail!("review cleanup base is not ancestral to HEAD") unless status.success?

  deleted = cleanup.fetch("deleted_files")
  modified = cleanup.fetch("modified_files")
  validation_lane = cleanup.fetch("validation_lane")
  fail!("review cleanup deleted files must be sorted") unless deleted == deleted.sort
  fail!("review cleanup modified files must be sorted") unless modified == modified.sort
  (deleted + modified + [validation_lane]).each { |path| validate_relative_path!(path) }
  deleted.each { |path| fail!("review cleanup deleted file still exists: #{path}") if File.exist?(File.join(ROOT, path)) }
  modified.each { |path| fail!("review cleanup modified file missing: #{path}") unless File.file?(File.join(ROOT, path)) }
  fail!("review cleanup validation lane missing: #{validation_lane}") unless File.file?(File.join(ROOT, validation_lane))
  fail!("review cleanup deleted file count mismatch") unless cleanup.fetch("deleted_file_count") == deleted.length
  fail!("review cleanup modified file count mismatch") unless cleanup.fetch("modified_file_count") == modified.length

  cleanup_rows = diff_numstat(cleanup.fetch("base_revision"), deleted + modified)
  added = cleanup_rows.sum { |row| row[0] }
  removed = cleanup_rows.sum { |row| row[1] }
  fail!("review cleanup added line mismatch") unless cleanup.fetch("added_lines") == added
  fail!("review cleanup removed line mismatch") unless cleanup.fetch("removed_lines") == removed
  fail!("review cleanup net line mismatch") unless cleanup.fetch("net_removed_lines") == removed - added

  validation_rows = diff_numstat(cleanup.fetch("base_revision"), [validation_lane])
  validation_added = validation_rows.sum { |row| row[0] }
  validation_removed = validation_rows.sum { |row| row[1] }
  fail!("review cleanup validation added line mismatch") unless cleanup.fetch("validation_lane_added_lines") == validation_added
  fail!("review cleanup validation removed line mismatch") unless cleanup.fetch("validation_lane_removed_lines") == validation_removed
  validation_net_added = validation_added - validation_removed
  fail!("review cleanup validation net-added mismatch") unless cleanup.fetch("validation_lane_net_added_lines") == validation_net_added
  total_net_removed = cleanup.fetch("net_removed_lines") - validation_net_added
  fail!("review cleanup total net mismatch") unless cleanup.fetch("total_net_removed_lines_including_validation_lane") == total_net_removed
end

def deleted_file_tokens
  deleted_rows.map { |row| File.basename(row.fetch("path"), ".rs") }.sort
end

def reserved_for_5346?(path)
  coordination.fetch("reserved_for_5346").any? do |entry|
    prefix = entry.fetch("path_prefix")
    path == prefix || path.start_with?("#{prefix}/")
  end
end

def live_reference_scan_paths
  git!("ls-files", "adl/Cargo.toml", "adl/src", "adl/tests", "adl/tools").lines.map(&:strip).select do |path|
    next false if deleted_rows.any? { |row| row.fetch("path") == path }
    next false if reserved_for_5346?(path)

    File.file?(File.join(ROOT, path))
  end
end

def validate_no_deleted_binary_live_references!
  tokens = deleted_file_tokens
  findings = []
  live_reference_scan_paths.each do |path|
    text = File.read(File.join(ROOT, path), invalid: :replace)
    hits = tokens.select { |token| text.include?(token) }
    findings << [path, hits] unless hits.empty?
  end
  return if findings.empty?

  rendered = findings.map { |path, hits| "#{path}: #{hits.join(', ')}" }.join("\n")
  fail!("surviving live references to #5347-deleted binaries:\n#{rendered}")
end

def compile_relevant_integration_tests!
  target_dir = ENV.fetch("ADL_5347_VALIDATION_TARGET_DIR", File.join(ROOT, "adl/target/5347-validation"))
  preserve_cargo_lock do
    command!(
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--target-dir",
      target_dir,
      "--test",
      "demo_tests",
      "--no-run"
    )
    command!(
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--target-dir",
      target_dir,
      "--tests",
      "--no-run"
    )
  end
end

case ARGV.fetch(0, nil)
when "execution"
  validate_manifest!
  validate_deletions!
  validate_coordination!
  validate_accounting!
  validate_no_deleted_binary_live_references!
  compile_relevant_integration_tests!
when "validate-contracts", "manifest-disjointness", "owner-and-consumer-proof", "deletion-budgets-and-evidence", "post-deletion-exact"
  validate_manifest!
  validate_deletions!
  validate_coordination!
  validate_accounting!
  validate_no_deleted_binary_live_references!
else
  fail!("unknown lane #{ARGV.fetch(0, '<missing>')}; expected execution")
end

puts(JSON.generate({
  schema: "adl.wp13.external_band_validation.v1",
  issue: 5347,
  lane: ARGV.fetch(0),
  status: "pass",
  deleted_files: deleted_rows.length,
  removed_lines: accounting["net_removed_lines"],
  live_reference_scan: "pass",
  integration_compile: ARGV.fetch(0) == "execution" ? "pass" : "not_run_for_contract_lane"
}))
