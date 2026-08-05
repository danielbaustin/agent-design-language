#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
TERMINAL_DEPENDENCIES = {
  5344 => "WP-12 soak and rollback",
  5343 => "WP-12 reviewed selector switch",
  5358 => "current C-SDLC v2 acceptance",
  5361 => "current Runtime v3 acceptance",
  5384 => "WP-14A integrated platform acceptance"
}.freeze
MERGED_CLOSED_DEPENDENCIES = {
  5354 => {
    "label" => "WP-15 convergence",
    "merge_commit" => "97427f324c87d97cb1b36c7804c50bf80c9389d8"
  },
  5352 => {
    "label" => "WP-21 v0.92 consumption handoff",
    "merge_commit" => "64632f8812dcf4a861902b97b981a72291d81beb"
  }
}.freeze
MANIFESTS = {
  5346 => ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json")
}.freeze
EXTERNAL_5347_MANIFEST = ROOT.join(".csdlc/evidence/5346/5347-external-band-deletion-manifest.json")
EXTERNAL_5347_SOURCE = Pathname.new("/Volumes/FastWork/adl-wp-5347/docs/milestones/v0.91.8/evidence/wp13-external-bands/external-band-deletion-manifest.json")
HEX40 = /\A[0-9a-f]{40}\z/
HEX64 = /\A[0-9a-f]{64}\z/

def fail_gate(message)
  warn("#5346 dependency gate: #{message}")
  exit 1
end

def capture_git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

def relative_path(value, label)
  fail_gate("#{label} must be a non-empty repository-relative path") unless value.is_a?(String) && !value.empty?
  path = Pathname.new(value)
  fail_gate("#{label} must be repository-relative") if path.absolute?
  clean = path.cleanpath.to_s
  fail_gate("#{label} contains traversal or is not normalized: #{value}") if clean == "." || clean.start_with?("../") || clean != value
  clean
end

def load_json(path, label)
  display = path.absolute? && !path.to_s.start_with?(ROOT.to_s) ? path.to_s : path.relative_path_from(ROOT).to_s
  fail_gate("missing #{label}: #{display}") unless path.file?
  JSON.parse(path.read)
rescue JSON::ParserError => e
  fail_gate("invalid #{label}: #{e.message}")
end

def git_tree_identity(revision, path_value, expected_oid, label)
  tree = capture_git("ls-tree", revision, "--", path_value)
  fail_gate("#{label} missing from baseline tree: #{path_value}") if tree.empty?
  _mode, kind, oid_and_path = tree.split(" ", 3)
  oid, tree_path = oid_and_path.split("\t", 2)
  fail_gate("#{label} tree identity path mismatch: #{path_value}") unless tree_path == path_value
  fail_gate("#{label} tree object mismatch for #{path_value}") unless oid == expected_oid
  { "kind" => kind, "object" => oid }
end

def merged_ancestor_sha(terminal, label)
  observed = terminal["observed_sha"]
  _out, status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", observed, "HEAD")
  return [observed, "observed_sha"] if status.success?

  pr = terminal["pull_request"]
  fail_gate("#{label} observed head is not ancestral and no PR is recorded") unless pr
  pr_json, pr_status = Open3.capture2e("gh", "pr", "view", pr.to_s, "--json", "state,mergeCommit")
  fail_gate("cannot verify merge commit for #{label}: #{pr_json.strip}") unless pr_status.success?
  data = JSON.parse(pr_json)
  fail_gate("#{label} PR is not merged") unless data["state"] == "MERGED"
  merge_sha = data.dig("mergeCommit", "oid")
  fail_gate("#{label} merge commit is invalid") unless merge_sha&.match?(HEX40)
  _out, merge_status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, "HEAD")
  fail_gate("#{label} merge commit is not ancestral to the execution revision") unless merge_status.success?
  [merge_sha, "merge_commit"]
end

def validate_external_5347_manifest(path)
  manifest = load_json(path, "#5347 external deletion manifest")
  fail_gate("#5347 external manifest schema mismatch") unless manifest["schema"] == "adl.wp13.external_band_deletion_manifest.v1"
  fail_gate("#5347 external manifest issue mismatch") unless manifest["issue"] == 5347
  fail_gate("#5347 external manifest repository mismatch") unless manifest["repository"] == "danielbaustin/agent-design-language"
  baseline = manifest["baseline_revision"]
  fail_gate("#5347 external manifest baseline is invalid") unless baseline&.match?(HEX40)
  fail_gate("#5347 merge order must serialize #5347 before #5346") unless manifest["merge_order"] == [5347, 5346]
  rows = manifest.fetch("deleted_files") { fail_gate("#5347 external manifest has no deleted_files") }
  fail_gate("#5347 external manifest deleted_files must be non-empty") unless rows.is_a?(Array) && !rows.empty?
  seen = {}
  rows.map do |row|
    path_value = relative_path(row["path"], "#5347 external path")
    fail_gate("#5347 duplicate external path #{path_value}") if seen[path_value]
    seen[path_value] = true
    fail_gate("#5347 external path must stay in adl/src/bin: #{path_value}") unless path_value.start_with?("adl/src/bin/")
    fail_gate("#5347 external disposition mismatch for #{path_value}") unless row["disposition"] == "delete_external"
    fail_gate("#5347 external path must be a regular file, not symlink/generated: #{path_value}") unless row["file_kind"] == "regular_file" && row["generated"] == false
    oid = row["baseline_object"]
    fail_gate("#5347 external baseline object is invalid for #{path_value}") unless oid&.match?(HEX40)
    identity = git_tree_identity(baseline, path_value, oid, "#5347 external manifest")
    fail_gate("#5347 external path is not a blob: #{path_value}") unless identity["kind"] == "blob"
    fail_gate("#5347 measured_lines invalid for #{path_value}") unless row["measured_lines"].is_a?(Integer) && row["measured_lines"].positive?
    fail_gate("#5347 replacement proof missing for #{path_value}") if row["replacement_owner"].to_s.empty? || row["replacement_proof"].to_s.empty?
    {
      "path" => path_value,
      "symlink_target" => nil,
      "cargo_memberships" => path_value.start_with?("adl/src/bin/") ? ["adl/Cargo.toml:auto-bin:#{File.basename(path_value, ".rs")}"] : [],
      "prefix" => "adl/src/bin"
    }
  end
rescue KeyError => e
  fail_gate("invalid #5347 external manifest: #{e.message}")
end

def prove_pairwise_disjoint(primary_rows, external_rows)
  primary_paths = primary_rows.map { |row| row.fetch("path") }
  external_paths = external_rows.map { |row| row.fetch("path") }
  path_overlap = primary_paths & external_paths
  fail_gate("#5346/#5347 exact path overlap: #{path_overlap.join(', ')}") unless path_overlap.empty?

  prefix_overlap = primary_paths.product(external_paths).select do |left, right|
    left.start_with?("#{right}/") || right.start_with?("#{left}/")
  end
  fail_gate("#5346/#5347 prefix overlap: #{prefix_overlap.inspect}") unless prefix_overlap.empty?

  primary_symlinks = primary_rows.map { |row| row["symlink_target"] }.compact
  external_symlinks = external_rows.map { |row| row["symlink_target"] }.compact
  symlink_overlap = primary_symlinks.product(external_paths).select { |target, path| target == path || target.start_with?("#{path}/") } +
                    external_symlinks.product(primary_paths).select { |target, path| target == path || target.start_with?("#{path}/") }
  fail_gate("#5346/#5347 symlink target overlap: #{symlink_overlap.inspect}") unless symlink_overlap.empty?

  primary_cargo = primary_rows.flat_map { |row| Array(row["cargo_memberships"]) }
  external_cargo = external_rows.flat_map { |row| Array(row["cargo_memberships"]) }
  cargo_overlap = primary_cargo & external_cargo
  fail_gate("#5346/#5347 Cargo membership overlap: #{cargo_overlap.join(', ')}") unless cargo_overlap.empty?

  {
    "exact_path_overlap" => 0,
    "prefix_overlap" => 0,
    "symlink_target_overlap" => 0,
    "cargo_membership_overlap" => 0,
    "primary_paths" => primary_paths.length,
    "external_paths" => external_paths.length,
    "external_manifest" => EXTERNAL_5347_MANIFEST.relative_path_from(ROOT).to_s
  }
end

common_dir = Pathname.new(capture_git("rev-parse", "--git-common-dir"))
common_dir = ROOT.join(common_dir) unless common_dir.absolute?
head = capture_git("rev-parse", "HEAD")

dependency_evidence = {}
TERMINAL_DEPENDENCIES.each do |issue, label|
  record_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  record = load_json(record_path, "typed projection for ##{issue} #{label}")
  fail_gate("##{issue} #{label} is not typed closed_out") unless record["phase"] == "closed_out"
  fail_gate("##{issue} #{label} still has an active claim") unless record["claim"].nil?
  terminal = record.fetch("terminal") { fail_gate("##{issue} #{label} projection has no terminal evidence") }
  fail_gate("##{issue} #{label} is not merged") unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
  sha = terminal["observed_sha"]
  fail_gate("##{issue} #{label} projection has invalid merged SHA") unless sha&.match?(HEX40)
  ancestral_sha, ancestry_source = merged_ancestor_sha(terminal, "##{issue} #{label}")

  path = common_dir.join("csdlc-v2/closeout/#{issue}.json")
  audit_receipt = path.file? ? { "path" => path.relative_path_from(common_dir).to_s, "sha256" => Digest::SHA256.file(path).hexdigest } : nil
  dependency_evidence[issue.to_s] = { "label" => label, "sha" => ancestral_sha, "ancestry_source" => ancestry_source, "audit_receipt" => audit_receipt }
end

MERGED_CLOSED_DEPENDENCIES.each do |issue, expected|
  merge_commit = expected.fetch("merge_commit")
  _out, status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_commit, "HEAD")
  fail_gate("##{issue} #{expected.fetch('label')} merge commit is not ancestral to the execution revision") unless status.success?

  issue_json, issue_status = Open3.capture2e(
    "gh",
    "issue",
    "view",
    issue.to_s,
    "--json",
    "state,stateReason"
  )
  fail_gate("cannot verify live GitHub state for ##{issue}: #{issue_json.strip}") unless issue_status.success?
  state = JSON.parse(issue_json)
  fail_gate("##{issue} #{expected.fetch('label')} is not live closed/completed") unless state["state"] == "CLOSED" && state["stateReason"] == "COMPLETED"
  dependency_evidence[issue.to_s] = {
    "label" => expected.fetch("label"),
    "sha" => merge_commit,
    "issue_state" => state["state"],
    "state_reason" => state["stateReason"],
    "typed_closeout_nonblocking" => true
  }
end

def validate_manifest(issue, path, _head, _dependency_evidence)
  manifest = load_json(path, "##{issue} deletion manifest")
  fail_gate("##{issue} manifest schema mismatch") unless manifest["schema"] == "adl.wp13.deletion_eligibility.v1"
  fail_gate("##{issue} manifest issue mismatch") unless manifest["issue"] == issue
  %w[baseline_revision execution_revision].each do |field|
    fail_gate("##{issue} #{field} is invalid") unless manifest[field]&.match?(HEX40)
  end
  _out, status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", manifest["baseline_revision"], "HEAD")
  fail_gate("##{issue} baseline revision is not ancestral to current HEAD") unless status.success?
  _out, execution_status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", manifest["execution_revision"], "HEAD")
  fail_gate("##{issue} execution revision is not ancestral to current HEAD") unless execution_status.success?
  fail_gate("##{issue} reviewed revision must remain null before fresh rereview") unless manifest["reviewed_revision"].nil? || manifest["reviewed_revision"].match?(HEX40)
  review = manifest["review"]
  if review
    fail_gate("##{issue} manifest review is not a pass") unless review["result"] == "pass" && !review["reviewer"].to_s.empty?
    fail_gate("##{issue} review revision mismatch") unless review["reviewed_revision"] == manifest["reviewed_revision"]
  end
  rollback = manifest.fetch("rollback") { fail_gate("##{issue} manifest has no rollback evidence") }
  rollback_refs = Array(rollback["evidence_refs"])
  fail_gate("##{issue} rollback window is not complete") unless rollback["window_complete"] == true && !rollback_refs.empty? && rollback_refs.all? { |ref| !ref.to_s.empty? }
  request = relative_path(manifest["eligibility_request"], "##{issue} eligibility request")
  decision = relative_path(manifest["eligibility_decision"], "##{issue} eligibility decision")
  [request, decision].each { |ref| fail_gate("##{issue} missing eligibility artifact #{ref}") unless ROOT.join(ref).file? }
  decision_json = load_json(ROOT.join(decision), "##{issue} eligibility decision")
  fail_gate("##{issue} is not eligible") unless decision_json["eligible"] == true
  fail_gate("##{issue} decision manifest mismatch") unless decision_json["manifest"] == path.relative_path_from(ROOT).to_s

  rows = manifest.fetch("paths") { fail_gate("##{issue} manifest has no paths") }
  fail_gate("##{issue} manifest paths must be a non-empty array") unless rows.is_a?(Array) && !rows.empty?
  seen = {}
  rows.map do |row|
    path_value = relative_path(row["path"], "##{issue} path")
    fail_gate("##{issue} duplicate path #{path_value}") if seen[path_value]
    seen[path_value] = true
    fail_gate("#5346 Runtime v2 is categorically outside scope: #{path_value}") if issue == 5346 && path_value.split("/").include?("runtime_v2")
    fail_gate("##{issue} invalid Git mode for #{path_value}") unless row["git_mode"].to_s.match?(/\A(?:100644|100755|120000|160000)\z/)
    fail_gate("##{issue} invalid Git object id for #{path_value}") unless row["git_object_id"]&.match?(HEX40)
    tree = capture_git("ls-tree", manifest["baseline_revision"], "--", path_value)
    object_type = row["git_mode"] == "160000" ? "commit" : "blob"
    expected = "#{row['git_mode']} #{object_type} #{row['git_object_id']}\t#{path_value}"
    fail_gate("##{issue} Git identity mismatch for #{path_value}") unless tree == expected
    fail_gate("##{issue} invalid baseline LoC for #{path_value}") unless row["baseline_physical_loc"].is_a?(Integer) && row["baseline_physical_loc"] >= 0
    fail_gate("##{issue} invalid disposition for #{path_value}") unless %w[remove retain].include?(row["disposition"])
    if row["disposition"] == "retain"
      fail_gate("##{issue} retained path lacks owner/justification: #{path_value}") if row["retained_owner"].to_s.empty? || row["retained_justification"].to_s.empty?
    else
      replacement = row.fetch("replacement") { fail_gate("##{issue} removed path lacks replacement proof: #{path_value}") }
      fail_gate("##{issue} removed path lacks replacement owner/path/proof: #{path_value}") if replacement["owner"].to_s.empty? || replacement["path"].to_s.empty? || Array(replacement["proof_refs"]).empty?
      fail_gate("##{issue} removed path still exists in working tree: #{path_value}") if ROOT.join(path_value).exist?
    end
    symlink_target = row["symlink_target"]
    relative_path(symlink_target, "##{issue} symlink target") unless symlink_target.nil?
    generated_owner = row["generated_owner"]
    fail_gate("##{issue} generated owner must be explicit or null: #{path_value}") unless generated_owner.nil? || !generated_owner.to_s.empty?
    cargo = Array(row["cargo_memberships"])
    fail_gate("##{issue} Cargo membership must be normalized strings: #{path_value}") unless cargo.all? { |member| relative_path(member, "Cargo membership") == member }
    { "path" => path_value, "symlink_target" => symlink_target, "generated_owner" => generated_owner, "cargo_memberships" => cargo, "retained_owner" => row["retained_owner"] }
  end
rescue KeyError => e
  fail_gate("invalid ##{issue} manifest: #{e.message}")
end

surfaces = MANIFESTS.to_h { |issue, path| [issue, validate_manifest(issue, path, head, dependency_evidence)] }
if EXTERNAL_5347_SOURCE.file?
  copied = Digest::SHA256.file(EXTERNAL_5347_MANIFEST).hexdigest
  source = Digest::SHA256.file(EXTERNAL_5347_SOURCE).hexdigest
  fail_gate("#5347 external manifest copy differs from source") unless copied == source
end
external_5347 = validate_external_5347_manifest(EXTERNAL_5347_MANIFEST)
issue_5347_json, issue_5347_status = Open3.capture2e(
  "gh",
  "issue",
  "view",
  "5347",
  "--json",
  "state,stateReason"
)
fail_gate("cannot verify live GitHub state for #5347: #{issue_5347_json.strip}") unless issue_5347_status.success?
issue_5347_state = JSON.parse(issue_5347_json)
disjointness = prove_pairwise_disjoint(surfaces.fetch(5346), external_5347)

projection = load_json(ROOT.join(".csdlc/issues/5346/index.json"), "#5346 typed projection")
protected_paths = if projection["phase"] == "closed_out"
                    terminal = projection.fetch("terminal") { fail_gate("#5346 closed-out projection has no terminal evidence") }
                    terminal.fetch("released_protected_paths") { fail_gate("#5346 terminal evidence has no released protected paths") }
                  else
                    claim = projection.fetch("claim") { fail_gate("#5346 non-terminal projection has no active claim") }
                    claim.fetch("protected_paths") { fail_gate("#5346 active claim has no protected paths") }
                  end
delete_paths = surfaces.fetch(5346).map { |row| row.fetch("path") }
delete_paths.each do |path|
  covered = protected_paths.any? { |protected| path == protected || path.start_with?("#{protected}/") }
  fail_gate("#5346 deletion path is not protected by active claim: #{path}") unless covered
end

puts JSON.generate(
  status: "pass",
  issue: 5346,
  revision: head,
  typed_terminal_dependencies: TERMINAL_DEPENDENCIES.keys,
  merged_closed_dependencies: MERGED_CLOSED_DEPENDENCIES.keys,
  dependency_evidence: dependency_evidence,
  external_5347: {
    issue_state: issue_5347_state["state"],
    state_reason: issue_5347_state["stateReason"],
    merge_order: [5347, 5346],
    merge_required_before_5346_merge: issue_5347_state["state"] != "CLOSED"
  },
  peer_disjointness: disjointness,
  disjoint: disjointness.values_at("exact_path_overlap", "prefix_overlap", "symlink_target_overlap", "cargo_membership_overlap").all?(&:zero?)
)
