#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
expected_head="${2:-}"
root="$(git rev-parse --show-toplevel)"
manifest="$root/adl-v2/crates/adl-records/Cargo.toml"
target="/Volumes/FastWork/adl-5342/target"
export CARGO_TARGET_DIR="$target"
export CARGO_HOME="/Volumes/FastWork/adl-5342/cargo-home"
doctor="$root/.adl/bin/csdlc-v2/csdlc-doctor"
if [[ ! -x "$doctor" ]]; then
  doctor="/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor"
fi

run_bounded() {
  local seconds="$1"
  shift
  ruby -ropen3 -rtimeout -e '
    seconds = Integer(ARGV.shift)
    command = ARGV
    status = nil
    Open3.popen2e(*command, pgroup: true) do |stdin, output, wait|
      stdin.close
      reader = Thread.new { output.each_line { |line| STDOUT.write(line) } }
      begin
        Timeout.timeout(seconds) { status = wait.value }
      rescue Timeout::Error
        Process.kill("TERM", -wait.pid) rescue nil
        sleep 1
        Process.kill("KILL", -wait.pid) rescue nil
        wait.value rescue nil
        abort "validation timeout after #{seconds}s: #{command.join(" ")}" 
      ensure
        reader.join
      end
    end
    exit(status.success? ? 0 : status.exitstatus || 1)
  ' "$seconds" "$@"
}

verify_dependencies() {
  local common
  common="$(git rev-parse --git-common-dir)"
  ruby -rjson -rshellwords -e '
    root = ARGV.fetch(0)
    common = File.expand_path(ARGV.fetch(1), root)
    expected = {
      5339 => { reviewed: "ba604e5f0ee16af901a4d8d7cb801c323500828d", landing: "860aa9f18946a2cd9407b610d5c00d44ddc89053" },
      5340 => { reviewed: "f38f662acb3988ef0704a54da055b14812c898d9", landing: "19601faec54a53e8bab90af484f745bc4972f116" },
    }
    expected.each do |issue, identity|
      receipt = JSON.parse(File.read(File.join(common, "csdlc-v2/closeout/#{issue}.json")))
      record = receipt.fetch("record")
      terminal = record.fetch("terminal")
      publication = record.fetch("publication")
      abort "##{issue} receipt issue mismatch" unless receipt.fetch("issue") == issue
      abort "##{issue} is not closed_out" unless record.fetch("phase") == "closed_out"
      abort "##{issue} claim remains active" unless record["claim"].nil?
      abort "##{issue} is not merged" unless terminal.fetch("disposition") == "merged" && terminal.fetch("observed_state") == "merged"
      abort "##{issue} reviewed head drift" unless terminal.fetch("observed_sha") == identity.fetch(:reviewed)
      abort "##{issue} publication head drift" unless publication.fetch("revision").include?(identity.fetch(:reviewed))
      system("git", "-C", root, "merge-base", "--is-ancestor", identity.fetch(:landing), "origin/main") or abort "##{issue} landing is not ancestral"
      subject = `git -C #{Shellwords.escape(root)} show -s --format=%s #{identity.fetch(:landing)}`
      pr = publication.fetch("pull_request")
      abort "##{issue} landing identity mismatch" unless subject.include?("##{issue}") || subject.include?("##{pr}")
    end
  ' "$root" "$common"
}

verify_scope_and_claims() {
  ruby -rjson -rshellwords -e '
    root = ARGV.fetch(0)
    allowed = %r{\A(?:\.csdlc/(?:issues|prepared/issues|evidence)/5342(?:/|\z)|\.csdlc/locks/5342\.lock\z|adl-v2/crates/adl-records(?:/|\z))}
    base = `git -C #{Shellwords.escape(root)} merge-base HEAD origin/main`.strip
    abort "missing merge base" unless $?.success?
    paths = `git -C #{Shellwords.escape(root)} diff --name-only #{base}...HEAD`.lines.map(&:strip)
    paths += `git -C #{Shellwords.escape(root)} diff --name-only`.lines.map(&:strip)
    paths += `git -C #{Shellwords.escape(root)} diff --cached --name-only`.lines.map(&:strip)
    paths += `git -C #{Shellwords.escape(root)} ls-files --others --exclude-standard`.lines.map(&:strip)
    bad = paths.uniq.reject { |path| path.match?(allowed) }
    abort "out-of-scope changed paths: #{bad.join(", ")}" unless bad.empty?
    owned = [
      ".csdlc/issues/5342", ".csdlc/locks/5342.lock",
      ".csdlc/prepared/issues/5342", ".csdlc/evidence/5342",
      "adl-v2/crates/adl-records"
    ]
    overlap = lambda do |left, right|
      left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
    end
    Dir.glob(File.join(root, ".csdlc/issues/*/index.json")).each do |path|
      record = JSON.parse(File.read(path)) rescue next
      claim = record["claim"]
      next if claim.nil? || record["issue"] == 5342
      Array(claim["protected_paths"]).each do |protected|
        collision = owned.any? { |candidate| overlap.call(protected, candidate) }
        abort "protected-path collision with ##{record["issue"]}: #{protected}" if collision
      end
    end
  ' "$root"
}

verify_budget_and_cots() {
  local implementation_lines test_lines
  implementation_lines="$(find "$root/adl-v2/crates/adl-records/src" -type f -name '*.rs' -print0 | xargs -0 awk 'END { print NR + 0 }')"
  test_lines="$(find "$root/adl-v2/crates/adl-records/tests" -type f \( -name '*.rs' -o -name '*.json' \) -print0 | xargs -0 awk 'END { print NR + 0 }')"
  [[ "$implementation_lines" -le 3000 ]] || { printf 'implementation LoC exceeded: %s\n' "$implementation_lines" >&2; exit 73; }
  [[ "$test_lines" -le 3000 ]] || { printf 'test/fixture LoC exceeded: %s\n' "$test_lines" >&2; exit 73; }
  cargo metadata --manifest-path "$manifest" --locked --offline --format-version 1 >"$target/metadata.json"
  ruby -rjson -e '
    metadata = JSON.parse(File.read(ARGV.fetch(0)))
    root = metadata.fetch("packages").find { |package| package.fetch("name") == "adl-records" } or abort "adl-records package absent"
    exact = {
      "serde" => "1.0.229", "serde_json" => "1.0.151", "schemars" => "1.2.1",
      "ed25519-dalek" => "2.2.0", "sha2" => "0.10.9", "hex" => "0.4.3",
      "jsonschema" => "0.48.2"
    }
    actual = root.fetch("dependencies").to_h { |dep| [dep.fetch("name"), dep.fetch("req").delete_prefix("=")] }
    abort "direct COTS drift: #{actual.inspect}" unless actual == exact
    forbidden = %w[tokio reqwest hyper axum aws-config aws-sdk-s3 sqlx diesel tracing opentelemetry]
    bad = metadata.fetch("packages").map { |package| package.fetch("name") } & forbidden
    abort "forbidden dependency graph: #{bad.join(", ")}" unless bad.empty?
  ' "$target/metadata.json"
  printf 'implementation_lines=%s test_fixture_lines=%s\n' "$implementation_lines" "$test_lines"
}

case "$mode" in
  preparation)
    set +e
    doctor_json="$("$doctor" --repo "$root" --issue 5342)"
    doctor_status=$?
    set -e
    printf '%s\n' "$doctor_json"
    ruby -rjson -e '
      report = JSON.parse(STDIN.read)
      findings = Array(report["findings"])
      allowed = findings.all? do |finding|
        finding.fetch("code", "") == "design_review_missing_or_stale" ||
          finding.fetch("message", "") == "design/diagram references are stale"
      end
      abort "typed doctor has non-design findings" unless report.fetch("status") == "pass" || allowed
    ' <<<"$doctor_json"
    [[ "$doctor_status" -eq 0 || "$doctor_status" -eq 3 ]] || exit "$doctor_status"
    verify_dependencies
    verify_scope_and_claims
    exit 0
    ;;
esac

if [[ ! -f "$manifest" ]]; then
  printf 'adl-records implementation is not present yet\n' >&2
  exit 73
fi

mkdir -p "$target"
case "$mode" in
  focused)
    run_bounded 120 cargo test --manifest-path "$manifest" --all-targets --locked
    ;;
  quality)
    run_bounded 120 cargo fmt --manifest-path "$manifest" -- --check
    run_bounded 120 cargo clippy --manifest-path "$manifest" --all-targets --locked -- -D warnings
    ;;
  tamper)
    run_bounded 300 cargo test --manifest-path "$manifest" --test tamper_channel --locked
    ;;
  all|post-merge)
    [[ -n "$expected_head" ]] || { printf 'expected exact review head is required\n' >&2; exit 64; }
    [[ "$(git rev-parse HEAD)" == "$expected_head" ]] || { printf 'exact review head mismatch\n' >&2; exit 73; }
    started=$SECONDS
    verify_dependencies
    verify_scope_and_claims
    verify_budget_and_cots
    run_bounded 120 bash -c 'cargo run --quiet --manifest-path "$1" --example generate_schema --locked | diff -u "$2" -' _ "$manifest" "$root/adl-v2/crates/adl-records/schema/adl-records.schema.json"
    run_bounded 600 cargo test --manifest-path "$manifest" --all-targets --locked
    run_bounded 300 cargo test --manifest-path "$manifest" --test tamper_channel --locked
    run_bounded 120 cargo fmt --manifest-path "$manifest" -- --check
    run_bounded 120 cargo clippy --manifest-path "$manifest" --all-targets --locked -- -D warnings
    elapsed=$((SECONDS - started))
    [[ "$elapsed" -le 600 ]] || { printf 'aggregate validation exceeded 600s: %s\n' "$elapsed" >&2; exit 73; }
    printf 'loc_method=physical_lines_in_rs_and_json elapsed_seconds=%s exact_head=%s\n' "$elapsed" "$expected_head"
    if [[ "$mode" == "post-merge" ]]; then
      : "${ADL_EXPECTED_PR_HEAD:?ADL_EXPECTED_PR_HEAD is required for post-merge proof}"
      : "${ADL_EXPECTED_MERGE_SHA:?ADL_EXPECTED_MERGE_SHA is required for post-merge proof}"
      git merge-base --is-ancestor "$ADL_EXPECTED_PR_HEAD" "$ADL_EXPECTED_MERGE_SHA"
      git merge-base --is-ancestor "$ADL_EXPECTED_MERGE_SHA" origin/main
      [[ "$(git rev-parse HEAD)" == "$ADL_EXPECTED_MERGE_SHA" ]] || { printf 'post-merge checkout identity mismatch\n' >&2; exit 73; }
    fi
    ;;
  *)
    printf 'unknown validation mode: %s\n' "$mode" >&2
    exit 64
    ;;
esac
