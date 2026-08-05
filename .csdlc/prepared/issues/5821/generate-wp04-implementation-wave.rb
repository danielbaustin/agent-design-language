#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "fileutils"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
OUT = File.join(__dir__, "wp04-implementation-wave")
REPOSITORY = "danielbaustin/agent-design-language"

CHILDREN = [
  {
    wp: "WP-04.01", key: "node-identity-enrollment",
    title: "Node identity and authenticated enrollment",
    outcome: "Implement stable node and Guardian identities plus explicit, fail-closed enrollment into one trust domain.",
    depends: ["WP-03", "#5821"],
    paths: ["adl-runtime/src/distributed/identity.rs", "adl-runtime/tests/distributed_identity.rs"],
    proof: "Exact nextest target distributed_identity proves identity creation, signed enrollment, wrong-domain rejection, replay rejection, and restart-stable identity.",
    rollback: "Disable enrollment and remove only issue-created node records; preserve the WP-03 single-node Guardian and all preexisting identity state."
  },
  {
    wp: "WP-04.02", key: "certificate-lifecycle",
    title: "Distributed certificate purposes and lifecycle",
    outcome: "Implement separate node, Guardian, transport, and signing certificate purposes with rotation, revocation, and expiry enforcement.",
    depends: ["WP-04.01"],
    paths: ["adl-runtime/src/distributed/certificates.rs", "adl-runtime/tests/distributed_certificates.rs"],
    proof: "Exact nextest target distributed_certificates proves purpose separation, chain validation, rotation overlap, revocation, expiry, and compromised-key denial.",
    rollback: "Restore the last valid certificate generation and trust set without disabling verification or deleting operator-owned key material."
  },
  {
    wp: "WP-04.03", key: "quic-tls-transport",
    title: "Maintained QUIC/TLS transport adapter",
    outcome: "Integrate a maintained QUIC/TLS stack with bounded authenticated channels and no custom cryptography or framing.",
    depends: ["WP-04.02"],
    paths: ["adl-runtime/src/distributed/transport.rs", "adl-runtime/tests/distributed_transport.rs", "adl-runtime/Cargo.toml", "adl-runtime/Cargo.lock"],
    proof: "Exact nextest target distributed_transport proves mutual authentication, channel bounds, cancellation, malformed-frame denial, peer mismatch, and dependency-lock parity.",
    rollback: "Remove the distributed transport feature and restore the prior manifest and lockfile while retaining the single-node Runtime API."
  },
  {
    wp: "WP-04.04", key: "seed-discovery-join",
    title: "Seed discovery and authenticated join",
    outcome: "Implement bounded seed discovery and authenticated join without making discovery an authority source.",
    depends: ["WP-04.03"],
    paths: ["adl-runtime/src/distributed/discovery.rs", "adl-runtime/tests/distributed_discovery.rs"],
    proof: "Exact nextest target distributed_discovery proves configured seed discovery, authenticated join, duplicate suppression, timeout, stale seed, and wrong-domain refusal.",
    rollback: "Disable distributed discovery and return to explicit single-node startup without persisting partial membership."
  },
  {
    wp: "WP-04.05", key: "membership-topology",
    title: "Membership state and topology convergence",
    outcome: "Implement deterministic membership epochs and bounded topology convergence from authenticated join events.",
    depends: ["WP-04.04"],
    paths: ["adl-runtime/src/distributed/membership.rs", "adl-runtime/tests/distributed_membership.rs"],
    proof: "Exact nextest target distributed_membership proves convergence, monotonic epochs, duplicate and out-of-order handling, restart recovery, and bounded membership size.",
    rollback: "Restore the last committed membership epoch and reject uncommitted topology updates."
  },
  {
    wp: "WP-04.06", key: "failure-partition",
    title: "Failure detection and partition classification",
    outcome: "Implement bounded failure detection that distinguishes suspect, unavailable, partitioned, and recovered nodes without granting authority.",
    depends: ["WP-04.05"],
    paths: ["adl-runtime/src/distributed/failure_detection.rs", "adl-runtime/tests/distributed_failure_detection.rs"],
    proof: "Exact nextest target distributed_failure_detection proves timeout bounds, false-positive recovery, partition classification, flapping limits, and deterministic event projection.",
    rollback: "Disable distributed failure decisions and retain the last committed membership state; never infer healthy ownership from silence."
  },
  {
    wp: "WP-04.07", key: "epoch-lease-authority",
    title: "Epoch and lease authority",
    outcome: "Implement monotonic epochs and bounded leases as prerequisites for distributed ownership decisions.",
    depends: ["WP-04.05"],
    paths: ["adl-runtime/src/distributed/lease.rs", "adl-runtime/tests/distributed_lease.rs"],
    proof: "Exact nextest target distributed_lease proves monotonic epochs, lease acquisition and renewal, expiry, stale-holder denial, clock-bound handling, and restart recovery.",
    rollback: "Expire issue-created leases, restore the last durable epoch, and leave no ambiguous owner."
  },
  {
    wp: "WP-04.08", key: "fencing-single-owner",
    title: "Fencing and single-owner enforcement",
    outcome: "Enforce one authoritative owner per lineage and reject stale, cloned, or partitioned actors.",
    depends: ["WP-04.06", "WP-04.07"],
    paths: ["adl-runtime/src/distributed/fencing.rs", "adl-runtime/tests/distributed_fencing.rs"],
    proof: "Exact nextest target distributed_fencing proves stale epoch, cloned state, split-brain, wrong owner, post-partition, and recovery fencing semantics.",
    rollback: "Fence all uncertain distributed owners and return authority to the last durable single-node owner."
  },
  {
    wp: "WP-04.09", key: "capability-advertisements",
    title: "Signed capability advertisements",
    outcome: "Implement bounded signed capability advertisements that are evidence inputs, never direct authority grants.",
    depends: ["WP-04.03"],
    paths: ["adl-runtime/src/distributed/capability_advertisement.rs", "adl-runtime/tests/distributed_capability_advertisement.rs"],
    proof: "Exact nextest target distributed_capability_advertisement proves signatures, expiry, replay, wrong signer, oversize, redaction, and deterministic projection.",
    rollback: "Withdraw issue-created advertisements and make placement treat missing capability data as unavailable."
  },
  {
    wp: "WP-04.10", key: "resource-weather-advertisements",
    title: "Signed resource-weather advertisements",
    outcome: "Implement bounded signed resource-weather observations for placement without converting telemetry into authority.",
    depends: ["WP-04.03"],
    paths: ["adl-runtime/src/distributed/resource_weather.rs", "adl-runtime/tests/distributed_resource_weather.rs"],
    proof: "Exact nextest target distributed_resource_weather proves signatures, freshness, bounds, replay denial, unavailable metrics, redaction, and deterministic projection.",
    rollback: "Withdraw issue-created observations and force placement to the declared no-data policy."
  },
  {
    wp: "WP-04.11", key: "bounded-placement",
    title: "Bounded placement decisions",
    outcome: "Implement deterministic bounded placement from membership, fencing, capability, and resource-weather inputs.",
    depends: ["WP-04.05", "WP-04.08", "WP-04.09", "WP-04.10"],
    paths: ["adl-runtime/src/distributed/placement.rs", "adl-runtime/tests/distributed_placement.rs"],
    proof: "Exact nextest target distributed_placement proves deterministic selection, capacity limits, stale input denial, no eligible target, policy bounds, and fenced-node exclusion.",
    rollback: "Disable remote placement and retain the current authoritative owner without automatic relocation."
  },
  {
    wp: "WP-04.12", key: "snapshot-catalog-transfer",
    title: "Snapshot catalog and transfer manifest",
    outcome: "Implement authenticated snapshot catalog entries and content-bound transfer manifests without exposing private state.",
    depends: ["WP-04.02", "WP-04.08"],
    paths: ["adl-runtime/src/distributed/snapshot_catalog.rs", "adl-runtime/tests/distributed_snapshot_catalog.rs"],
    proof: "Exact nextest target distributed_snapshot_catalog proves digest binding, schema version, authorization, redaction, replay denial, incomplete transfer, and corruption rejection.",
    rollback: "Delete only incomplete issue-created transfers and retain the last valid local snapshot catalog."
  },
  {
    wp: "WP-04.13", key: "migration-state-machine",
    title: "Migration state machine",
    outcome: "Implement prepare, quiesce, checkpoint, transfer, validate, fence, activate, and commit with source authority retained until validation and fencing succeed.",
    depends: ["WP-04.08", "WP-04.11", "WP-04.12"],
    paths: ["adl-runtime/src/distributed/migration.rs", "adl-runtime/tests/distributed_migration.rs"],
    proof: "Exact nextest target distributed_migration proves every transition, idempotence, source authority retention, target validation, fencing, interruption, and split-brain denial.",
    rollback: "Abort before commit, fence the target, resume the validated source owner, and preserve both transfer and audit evidence."
  },
  {
    wp: "WP-04.14", key: "rollback-recovery-relocation",
    title: "Rollback, recovery, and relocation failure",
    outcome: "Implement deterministic rollback and recovery for failed, interrupted, or ambiguous relocation.",
    depends: ["WP-04.13"],
    paths: ["adl-runtime/src/distributed/recovery.rs", "adl-runtime/tests/distributed_recovery.rs"],
    proof: "Exact nextest target distributed_recovery proves failures at each migration stage, restart recovery, target loss, source loss, audit continuity, and one-owner restoration.",
    rollback: "Fence both sides on ambiguity and require explicit recovery from the last validated durable owner."
  },
  {
    wp: "WP-04.15", key: "distributed-projection",
    title: "Versioned distributed topology and migration projection",
    outcome: "Expose redacted versioned topology, certificate, failure, lease, placement, and migration state through the Runtime API contract.",
    depends: ["WP-04.05", "WP-04.08", "WP-04.13", "WP-04.14"],
    paths: ["adl-runtime/src/distributed/projection.rs", "adl-runtime/tests/distributed_projection.rs", "docs/api/runtime-v3/v1/distributed.openapi.json"],
    proof: "Exact nextest target distributed_projection plus OpenAPI validation proves schema parity, redaction, stable identity, ordering, stale state, denied detail, and backward compatibility.",
    rollback: "Disable the new projection version and restore the prior API catalog without weakening authentication or exposing private state."
  },
  {
    wp: "WP-04.16", key: "integration-adversarial-native-proof",
    title: "Distributed integration, adversarial, and native-platform proof",
    outcome: "Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts.",
    depends: (1..15).map { |n| format("WP-04.%02d", n) },
    paths: ["adl-runtime/src/distributed/mod.rs", "adl-runtime/src/lib.rs", "adl-runtime/tests/distributed_guardian.rs", "adl/tools/validate_v092_distributed_guardian.sh", "adl/tools/validate_v092_distributed_native_receipts.rb"],
    proof: "Exact distributed_guardian test and live validator launch production Guardians and kernels, exercise authenticated API/WSS, partition, fencing, migration, recovery, shutdown, and digest-bound native receipts.",
    rollback: "Remove module registration and distributed launch configuration, fence remote ownership, and prove the WP-03 single-node Guardian remains healthy from unchanged durable state."
  }
].freeze

def issue_body(item, umbrella_issue)
  dependencies = item[:depends].map { |dep| "- #{dep}" }.join("\n")
  paths = item[:paths].map { |path| "- `#{path}`" }.join("\n")
  <<~MARKDOWN
    ## Summary

    #{item[:outcome]}

    This is #{item[:wp]} in the v0.92 distributed Guardian implementation wave coordinated by WP-04-IMP issue ##{umbrella_issue}. It may start only after the architecture/security gate in #5821 is terminal and this issue's typed design packet is current.

    ## Dependencies

    #{dependencies}

    ## Exclusive Owned Paths

    #{paths}

    These product paths are exclusive to this child. Scope expansion or overlap requires architecture-gate reconciliation before binding.

    ## Required Proof

    #{item[:proof]}

    Proof must bind the exact source revision, executable argv, nonzero test count, output/artifact digests, runner identity, and platform identity where applicable. Fixtures and hand-authored success fields cannot replace live behavior.

    ## Rollback Responsibility

    #{item[:rollback]}

    ## Acceptance Criteria

    - The required outcome is implemented only in the exclusive paths above.
    - Positive, failure, replay/stale, bounded-resource, redaction, and authorization cases applicable to this slice are proven.
    - The exact named test target runs with `--no-tests=fail` or equivalent nonzero-test enforcement.
    - Review has no unresolved actionable findings and the child closes through its own typed lifecycle.

    ## Non-Goals

    - No work owned by another WP-04 child.
    - No Runtime v2 fallback, custom cryptography, plaintext transport, or v0.93 governance work.
    - No completion credit from architecture prose, issue creation, fixtures, or retained evidence alone.
  MARKDOWN
end

def umbrella_body
  <<~MARKDOWN
    ## Summary

    Execute and integrate the exact same-milestone WP-04.01 through WP-04.16 distributed Guardian child wave after #5821's architecture/security gate is terminal. This umbrella coordinates dependency order and final reconciliation; it does not preclaim child product paths or substitute umbrella work for child lifecycle authority.

    ## Dependencies

    - #5821 terminal with approved architecture, threat model, and exact child ledger.
    - WP-03 issue #5820 terminal with stable single-node Guardian, API/WSS, state, readiness, restart, and shutdown contracts.
    - All sixteen children have approved designs, ready SIP/STP/SPP/VPP, truthful pre-phase SRP/SOR, and null claims before implementation starts.

    ## Required Outcome

    - Exactly WP-04.01 through WP-04.16 execute in the approved dependency graph.
    - Every child retains exclusive paths, proof, review, PR, merge, closeout, and rollback authority.
    - Final integration proves production Guardian/kernel multi-node behavior, authenticated API/WSS continuity, partition and fencing, migration and recovery, clean shutdown, and native macOS/Linux/Windows receipts.
    - WP-14 issue #5832 remains blocked until this umbrella has terminal integrated output.

    ## Owned Paths

    This umbrella owns only its typed lifecycle records and `.csdlc/evidence/<issue>/` orchestration and reconciliation evidence. It owns no `adl-runtime/`, `adl-runtime-kernel/`, `adl/tools/`, or API-schema product path.

    ## Acceptance Criteria

    - The live denominator contains one umbrella and exactly sixteen open prepared child issues mapped to WP-04.01 through WP-04.16.
    - The mapping matches the canonical v0.92 wave and #5821 ledger exactly.
    - Dependency and protected-path graphs are acyclic, exclusive, and mechanically validated.
    - No child starts before #5821 is terminal and no child starts with an unapproved or claim-active preparation packet.
    - Final reconciliation derives terminal child and exact-head evidence rather than trusting self-attested status.

    ## Non-Goals

    - No product implementation directly in the umbrella.
    - No child merging, closeout, or proof substitution by umbrella authority.
    - No Runtime v2, custom cryptography, or v0.93 governance work.
  MARKDOWN
end

def exact_umbrella_body(numbers)
  mapping = numbers.map { |wp, issue| "- #{wp}: ##{issue}" }.join("\n")
  umbrella_body + <<~MARKDOWN

    ## Exact Live Child Mapping

    #{mapping}

    WP-04.16 issue ##{numbers.fetch("WP-04.16")} exclusively owns final module registration and integrated product proof.
  MARKDOWN
end

def exact_child_body(item, umbrella_issue, numbers)
  body = issue_body(item, umbrella_issue)
  dependencies = dependency_text(item, numbers).map { |value| "- #{value}" }.join("\n")
  body.sub(/## Dependencies\n\n.*?\n\n## Exclusive Owned Paths/m, "## Dependencies\n\n#{dependencies}\n- WP-04-IMP issue ##{umbrella_issue}\n- Architecture/security gate issue #5821 terminal\n\n## Exclusive Owned Paths")
end

def gate_live_body(numbers)
  rows = numbers.map { |wp, issue| "- #{wp}: ##{issue}" }.join("\n")
  <<~MARKDOWN
    ## Summary

    Complete the v0.92 WP-04 distributed Guardian architecture, security, and exact child-wave gate. This issue freezes the contract and validates the live implementation denominator; it performs no distributed product implementation or integration.

    ## Required Outcome

    - Review and approve the distributed Guardian architecture and threat model.
    - Validate exactly WP-04.01 through WP-04.16 with stable live issue identities, owners, dependencies, exclusive paths, proof boundaries, and rollback responsibilities.
    - Require all sixteen children to be execution-ready and claim-null before WP-04-IMP #5862 schedules implementation.
    - Keep WP-14 #5832 blocked until #5862 has terminal integrated output.

    ## Exact Live Denominator

    - WP-04-IMP: #5862
    #{rows}

    ## Acceptance Criteria

    - Architecture covers identity, enrollment, certificate purposes, discovery, membership, maintained QUIC/TLS, failure detection, epochs, leases, fencing, placement, snapshot transfer, migration, rollback, observability, and safe failure semantics.
    - Threat modeling covers partition, replay, stale lease, cloned state, wrong trust domain, certificate compromise/expiry, relocation/rollback failure, and split-brain activation.
    - The seven-field ledger has exactly sixteen unique mapped children with acyclic dependencies and no duplicate or overlapping paths.
    - Typed child records have approved designs, ready SIP/STP/SPP/VPP, truthful pre-phase SRP/SOR, and null preparation claims.
    - Independent architecture/security review binds exact artifact digests and has no unresolved actionable findings.

    ## Owned Paths

    #5821 owns only its typed records/evidence and the architecture/threat-model documents. #5862 owns orchestration records only. Child product paths remain exclusively child-owned; #5878 owns module registration and integration proof.

    ## Non-Goals

    - No distributed product implementation, child integration, multi-node proof, or terminal child credit in #5821.
    - No Runtime v2 fallback, custom cryptography, plaintext mode, or v0.93 governance work.
  MARKDOWN
end

def wp14_live_body
  <<~MARKDOWN
    ## Summary

    Reconcile ACIP and A2A into one versioned semantic contract, canonical protobuf family, deterministic JSON projection, public catalog, and real authenticated full-duplex Rustls WSS carrier.

    ## Dependencies

    - WP-04 architecture/security gate #5821 terminal.
    - WP-04-IMP #5862 terminal after all sixteen children #5863 through #5878 integrate.
    - Current ACIP stream and trace/replay baselines requalified at the implementation revision.

    ## Exclusive Owned Paths

    - `adl-runtime/src/acip.rs`
    - `adl-runtime/src/runtime_api_auth.rs`
    - `adl-runtime-kernel/src/acip.rs`
    - `adl-runtime-kernel/src/protocol_adapters.rs`
    - `adl-runtime/tests/runtime_api_wss.rs`
    - `schemas/acip/v1/acip.proto`
    - `schemas/acip/v1/catalog.json`
    - `docs/api/runtime-v3/v1/acip.openapi.json`
    - `adl/tools/validate_v092_acip_wss.sh`
    - `adl/tools/validate_v092_acip_native_receipts.rb`

    ## Required Proof

    Exact nonzero schema/round-trip/negative tests; production Guardian/kernel authenticated Rustls WSS exchange with binary and JSON parity, reconnect and backpressure; and digest-bound macOS, Linux, and native Windows receipts. Fixtures, generic owner lanes, hand-authored booleans, and #5821 gate completion cannot substitute for #5862 terminal integrated output.

    ## Non-Goals

    Distributed child implementation, consumer UI changes, Shepherd model behavior, cloud bridges, or custom cryptography/transport.
  MARKDOWN
end

def create_request(title:, body:, operation_key:)
  {
    action: "issue_create",
    repository: REPOSITORY,
    title: title,
    body: body,
    labels: ["track:roadmap", "type:feature", "area:runtime", "version:v0.92"],
    assignees: [],
    require_review: false,
    required_checks: [],
    operation_key: operation_key
  }
end

def write_json(path, value)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, JSON.pretty_generate(value) + "\n")
end

def child_numbers
  CHILDREN.to_h do |item|
    stem = item[:wp].downcase.tr(".", "-")
    result = JSON.parse(File.read(File.join(OUT, "results", "#{stem}.json")))
    [item[:wp], result.fetch("issue").fetch("number")]
  end
end

def dependency_text(item, numbers)
  item[:depends].map do |dependency|
    if numbers.key?(dependency)
      "#{dependency} issue ##{numbers.fetch(dependency)}"
    elsif dependency == "WP-03"
      "WP-03 issue #5820 terminal"
    else
      dependency
    end
  end
end

def child_design(item, issue, umbrella_issue, numbers)
  paths = item[:paths].map { |path| "- `#{path}`" }.join("\n")
  dependencies = dependency_text(item, numbers).map { |value| "- #{value}" }.join("\n")
  <<~MARKDOWN
    # Issue #{issue} Design: #{item[:wp]} #{item[:title]}

    ## Outcome And Boundary

    #{item[:outcome]} This child is one exclusive implementation slice under
    WP-04-IMP issue ##{umbrella_issue}; it does not absorb sibling work or
    receive completion credit from the #5821 architecture gate.

    ## Source Baseline

    - `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md` defines the milestone feature and claim boundary.
    - `.csdlc/prepared/issues/5821/design.md` freezes the Guardian-owned architecture, threat model, dependency graph, and sixteen-child denominator.
    - `adl-runtime/src/guardian.rs`, `adl-runtime/src/networking.rs`, `adl-runtime/src/topology.rs`, and `adl-runtime/src/runtime_api.rs` are current Runtime v3 integration authorities.
    - `adl-runtime/tests/guardian_cli.rs` and `adl-runtime/tests/runtime_api_wss.rs` are retained launch and authenticated carrier proof inputs, not substitutes for this child's named proof.

    ## Exclusive Owned Paths

    #{paths}

    No other WP-04 child may edit these paths. This child may read sibling and
    upstream contracts but may not widen its claim. WP-04.16 alone owns final
    module registration and integrated proof paths.

    ## Design And Failure Semantics

    #{item[:outcome]} The implementation must preserve Guardian as process 0,
    bounded queues and timeouts, authenticated transport, deterministic
    projections, durable state authority, redaction, and fail-closed behavior.
    Missing, stale, replayed, malformed, unauthorized, wrong-domain, or
    resource-exhausted inputs remain explicit failures and never trigger an
    insecure fallback.

    ## Dependencies

    #{dependencies}
    - WP-04-IMP issue ##{umbrella_issue} coordinates ordering but owns no child product path.
    - #5821 must be terminal before implementation binding.

    ## Proof Boundary

    #{item[:proof]}

    The execution receipt must bind the exact source revision, exact argv,
    nonzero selected test count, output and artifact SHA-256 digests, runner
    identity, negative cases, and native platform identity where claimed.
    Hand-authored status booleans, retained fixtures, and prose do not prove
    working behavior.

    ## Rollback Responsibility

    #{item[:rollback]}

    ## Estimate

    Budget this bounded child at 8 elapsed hours, 90,000 reasoning tokens, and
    90 minutes of focused validation and review. Replan before widening paths,
    dependencies, proof surface, or rollback authority.

    ## Non-Goals

    - Sibling WP-04 paths, WP-14 protocol reconciliation, consumer UI work, or v0.93 governance.
    - Runtime v2 fallback, custom cryptography, plaintext transport, or unbounded queues.
    - Completion credit from issue creation, architecture approval, fixtures, or self-attested receipts.
  MARKDOWN
end

def child_diagram(item, umbrella_issue)
  deps = item[:depends].join(", ")
  <<~MERMAID
    flowchart LR
      G["#5821 architecture/security gate"] --> U["##{umbrella_issue} WP-04-IMP"]
      D["Dependencies: #{deps}"] --> C["#{item[:wp]} #{item[:title]}"]
      U --> C
      C --> P["Exact named proof and negative cases"]
      P --> R["Child review, merge, closeout"]
      R --> I["WP-04.16 integrated proof"]
  MERMAID
end

def proof_validator(item, issue)
  expected_paths = JSON.generate(item[:paths])
  test_name = item[:paths].find { |path| path.include?("tests/") }&.then { |path| File.basename(path, ".rs") } || item[:key].tr("-", "_")
  platforms = item[:wp] == "WP-04.16" ? %w[macos linux windows] : []
  <<~RUBY
    #!/usr/bin/env ruby
    # frozen_string_literal: true
    require "json"
    require "digest"
    require "open3"

    expected_issue = #{issue}
    expected_wp = #{item[:wp].inspect}
    expected_paths = #{expected_paths}
    expected_test = #{test_name.inspect}
    required_platforms = #{platforms.inspect}
    evidence_path = ARGV.fetch(0, ".csdlc/evidence/#{issue}/execution-proof.json")
    abort "missing execution proof: \#{evidence_path}" unless File.file?(evidence_path)
    proof = JSON.parse(File.read(evidence_path))
    abort "wrong schema" unless proof["schema"] == "adl.wp04.execution_proof.v1"
    abort "wrong issue" unless proof["issue"] == expected_issue
    abort "wrong WP" unless proof["wp"] == expected_wp
    head, status = Open3.capture2("git", "rev-parse", "HEAD")
    abort "cannot resolve HEAD" unless status.success?
    abort "stale source revision" unless proof["source_revision"] == head.strip
    abort "proof did not pass" unless proof["status"] == "passed"
    abort "protected path drift" unless proof["protected_paths"] == expected_paths
    commands = Array(proof["commands"])
    matching = commands.select do |command|
      argv = Array(command["argv"])
      argv.include?(expected_test) && argv.include?("--no-tests=fail") && command["exit_code"] == 0 && command["selected_tests"].to_i.positive?
    end
    abort "missing nonzero exact test command \#{expected_test}" unless matching.length == 1
    abort "negative cases missing" if Array(proof["negative_cases"]).empty?
    artifacts = Array(proof["artifacts"])
    abort "artifacts missing" if artifacts.empty?
    artifacts.each do |artifact|
      path = artifact.fetch("path")
      abort "artifact missing: \#{path}" unless File.file?(path)
      digest = Digest::SHA256.file(path).hexdigest
      abort "artifact digest mismatch: \#{path}" unless digest == artifact.fetch("sha256")
    end
    receipts = Array(proof["native_receipts"])
    required_platforms.each do |platform|
      receipt = receipts.find { |entry| entry["platform"] == platform }
      abort "missing native receipt for \#{platform}" unless receipt
      abort "stale native receipt for \#{platform}" unless receipt["source_revision"] == head.strip
      abort "missing native argv for \#{platform}" if Array(receipt["argv"]).empty?
      abort "missing runner identity for \#{platform}" if receipt["runner_identity"].to_s.empty?
      abort "invalid output digest for \#{platform}" unless receipt["output_sha256"].to_s.match?(/\\A[0-9a-f]{64}\\z/)
    end
    puts "PASS: \#{expected_wp} exact-revision execution proof"
  RUBY
end

def child_initial(item, issue, umbrella_issue, numbers)
  dependency_values = dependency_text(item, numbers) + ["WP-04-IMP issue #{umbrella_issue}", "Architecture/security gate issue 5821 terminal"]
  test_name = item[:paths].find { |path| path.include?("tests/") }&.then { |path| File.basename(path, ".rs") } || item[:key].tr("-", "_")
  {
    title: "[v0.92][#{item[:wp]}] #{item[:title]}",
    slug: "v0-92-#{item[:wp].downcase.tr('.', '-')}-#{item[:key]}", version: "v0.92",
    goal: item[:outcome], required_outcome: item[:outcome],
    declared_scope: item[:paths],
    authority_boundary: ["Issue #{issue} exclusively owns the declared paths", "WP-04-IMP issue #{umbrella_issue} coordinates only", "WP-04.16 alone owns final module registration", "No sibling, Runtime v2, or v0.93 authority"],
    operator_constraints: ["Do not start before #5821 is terminal", "Bind only the exact exclusive paths", "Use nonzero exact test selection", "Fix all actionable pre-PR findings"],
    task_boundary: item[:outcome],
    deliverables: [item[:outcome], "Focused positive and negative tests", "Digest-bound execution proof", "Reviewed rollback evidence"],
    acceptance_criteria: ["Implement only the declared exclusive paths", "Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants", "Run the exact named test with nonzero test enforcement", "Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases", "Bind all evidence to the exact source revision and artifact digests", "Complete independent review and child-owned typed closeout"],
    dependencies: dependency_values,
    repo_inputs: ["docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md", ".csdlc/prepared/issues/5821/design.md", "adl-runtime/src/guardian.rs", "adl-runtime/src/networking.rs", "adl-runtime/src/runtime_api.rs"],
    non_goals: ["Sibling WP-04 paths", "Runtime v2 fallback", "Custom cryptography or plaintext", "WP-14, consumer UI, or v0.93 work", "Self-attested completion"],
    plan_summary: "Verify gates, implement the exclusive slice, run exact proving tests and negatives, validate rollback, resolve review, and close through child authority.",
    steps: [
      {id: "S1", action: "Verify #5821 terminal ancestry, dependency receipts, exact paths, and source contracts.", acceptance_ids: ["AC-1", "AC-2"], status: "pending"},
      {id: "S2", action: "Implement the bounded #{item[:wp]} outcome in the exclusive paths.", acceptance_ids: ["AC-1", "AC-2"], status: "pending"},
      {id: "S3", action: "Run exact positive, negative, failure, recovery, and receipt validation.", acceptance_ids: ["AC-3", "AC-4", "AC-5"], status: "pending"},
      {id: "S4", action: "Resolve independent review and complete child-owned publication and closeout.", acceptance_ids: ["AC-6"], status: "pending"}
    ],
    invariants: ["Exclusive paths remain disjoint", "Guardian stays process 0", "No insecure or Runtime v2 fallback", "Queues and waits remain bounded", "Evidence is exact-revision and digest bound"],
    risks: ["Dependency contract drift", "Cross-child path overlap", "False-green zero-test selection", "Self-attested platform or recovery evidence"],
    planning_profile: "medium",
    stop_conditions: ["#5821 is not terminal", "A dependency is not terminal", "Any declared path overlaps an active claim", "The exact test target is absent or selects zero tests", "Scope or rollback authority must widen"],
    validation_lanes: [
      {lane: "exact-child-tests", proof_role: item[:proof], deterministic: true, resource_profile: "medium", parallel_group: "child", budget_seconds: 1800, budget_tokens: 8000, argv: ["cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", test_name, "--no-tests=fail"], acceptance_ids: ["AC-1", "AC-2", "AC-3", "AC-4"]},
      {lane: "exact-revision-proof-receipt", proof_role: "Recompute source, command, nonzero test, artifact, negative-case, and native receipt bindings.", deterministic: true, resource_profile: "small", parallel_group: "receipt", budget_seconds: 300, budget_tokens: 3000, argv: ["ruby", ".csdlc/prepared/issues/#{issue}/validate-proof-receipt.rb"], acceptance_ids: ["AC-3", "AC-4", "AC-5", "AC-6"]}
    ],
    failure_policy: "Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.",
    review_prompts: ["Is the implementation confined to exclusive paths?", "Do exact tests prove the named behavior and negatives?", "Are receipts exact-revision and digest bound?", "Does rollback restore one authoritative owner without weakening security?"],
    review_scope: "#{item[:wp]} exclusive implementation, tests, proof receipts, and rollback evidence."
  }
end

def umbrella_design(issue, numbers)
  mapping = numbers.map { |wp, number| "| #{wp} | ##{number} |" }.join("\n")
  <<~MARKDOWN
    # Issue #{issue} Design: WP-04-IMP Distributed Guardian Implementation Umbrella

    ## Outcome And Boundary

    Coordinate and reconcile exactly WP-04.01 through WP-04.16 after #5821 is
    terminal. This umbrella owns scheduling and final evidence reconciliation
    only. It owns no product path and cannot implement, review, merge, or close
    a child on the child's behalf.

    ## Exact Live Denominator

    | Child | GitHub issue |
    | --- | --- |
    #{mapping}

    ## Owned Paths

    - `.csdlc/issues/#{issue}/`
    - `.csdlc/prepared/issues/#{issue}/`
    - `.csdlc/evidence/#{issue}/`

    No `adl-runtime/`, `adl-runtime-kernel/`, `adl/tools/`, or API schema path is
    owned by the umbrella. WP-04.16 owns final module registration and product
    integration paths.

    ## Scheduling And Integration

    #5821 and #5820 are hard terminal gates. Children execute only when their
    own listed dependencies are terminal and their exclusive paths can be
    claimed. WP-04.16 runs only after WP-04.01 through WP-04.15 are terminal,
    integrates module registration, and produces real multi-node and native
    proof. WP-14 #5832 waits for this umbrella's terminal integrated output.

    ## Validation And Reconciliation

    The issue-local validator compares the exact mapping against the canonical
    v0.92 wave, local typed records, approved design digests, null preparation
    claims, child dependencies, and exclusive paths. Final reconciliation must
    derive live issue/PR/merge/terminal receipts and exact-head evidence; status
    booleans and umbrella prose are not authority.

    ## Rollback

    Stop scheduling, preserve child branches and evidence, fence uncertain
    distributed owners, and return to the terminal WP-03 single-node Guardian.
    The umbrella never rewrites child history or claims product paths.

    ## Non-Goals

    - Direct product implementation or child lifecycle authority.
    - Denominator changes after #5821 approval without a new gate review.
    - Runtime v2 fallback, custom cryptography, or v0.93 governance.
  MARKDOWN
end

def bootstrap_request(issue:, design_path:, diagram_path:, initial:, paths:)
  now = Time.now.to_i
  claim = {
    id: "claim-#{issue}-v092-preparation", owner: "codex:5860-runtime-wp04-wave", generation: 0,
    acquired_unix_seconds: now, expires_unix_seconds: now + 86_400, heartbeat_unix_seconds: now,
    branch: "codex/5860-v092-execution-readiness", worktree: ".",
    protected_paths: [".csdlc/issues/#{issue}", ".csdlc/prepared/issues/#{issue}", ".csdlc/evidence/#{issue}"],
    purpose: "Prepare issue #{issue} design, cards, and proof contract without product implementation."
  }
  [{issue: issue, repository: REPOSITORY, design_path: design_path, diagram_path: diagram_path,
    design_reviewer: "codex:5860-runtime-wp04-design-review", design_approved: true, claim: claim, initial: initial}, claim]
end

def umbrella_initial(issue, numbers)
  {
    title: "[v0.92][WP-04-IMP][umbrella] Execute distributed Guardian child wave",
    slug: "v0-92-wp-04-imp-distributed-guardian-child-wave", version: "v0.92",
    goal: "Coordinate and reconcile the exact sixteen-child distributed Guardian implementation wave.",
    required_outcome: "All sixteen child issues execute under exclusive authority and converge in WP-04.16 real multi-node and native-platform proof.",
    declared_scope: [".csdlc/evidence/#{issue}", ".csdlc/prepared/issues/#{issue}"],
    authority_boundary: ["Umbrella owns scheduling and reconciliation only", "Children own implementation, proof, review, PR, closeout, and rollback", "WP-04.16 owns module registration and integration", "WP-14 #5832 waits for terminal integrated output"],
    operator_constraints: ["Start no child before #5821 is terminal", "Preserve exact sixteen-child denominator", "Never preclaim product paths", "Use derived live terminal evidence"],
    task_boundary: "Schedule and reconcile exactly WP-04.01 through WP-04.16 without direct product implementation.",
    deliverables: ["Validated live sixteen-child mapping", "Dependency-aware scheduling record", "Child terminal evidence matrix", "WP-04.16 integrated proof handoff to #5832"],
    acceptance_criteria: ["Exactly sixteen prepared mapped children exist", "Mapping and dependencies match #5821 and canonical wave", "All preparation claims are null before scheduling", "Each child executes only after dependencies and path claims pass", "WP-04.16 proves real multi-node API/WSS, partition, fencing, migration, recovery, shutdown, and native platforms", "Final reconciliation derives exact-head terminal evidence"],
    dependencies: ["#5821 architecture/security gate terminal", "#5820 WP-03 terminal", "WP-04.01 through WP-04.16 prepared"],
    repo_inputs: [".csdlc/prepared/issues/5821/design.md", "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml", "docs/milestones/v0.92/WBS_v0.92.md"],
    non_goals: ["Direct product implementation", "Child lifecycle substitution", "Denominator drift", "Runtime v2 or v0.93 work"],
    plan_summary: "Validate gates and denominator, schedule dependency-ready children, require child-owned proof and closeout, then reconcile WP-04.16 integration.",
    steps: [{id: "S1", action: "Validate #5821, #5820, exact mapping, cards, null claims, dependencies, and exclusive paths.", acceptance_ids: ["AC-1", "AC-2", "AC-3"], status: "pending"}, {id: "S2", action: "Schedule only dependency-ready children under their own claims and lifecycles.", acceptance_ids: ["AC-4"], status: "pending"}, {id: "S3", action: "Require WP-04.16 real integration and native proof after children 01 through 15 are terminal.", acceptance_ids: ["AC-5"], status: "pending"}, {id: "S4", action: "Derive exact-head child terminal evidence and hand stable contracts to #5832.", acceptance_ids: ["AC-6"], status: "pending"}],
    invariants: ["Exactly sixteen children", "No umbrella product paths", "Exclusive child ownership", "One authoritative Runtime owner", "Exact-head derived evidence"],
    risks: ["Denominator drift", "Dependency bypass", "Path collision", "Self-attested terminal state", "False platform proof"], planning_profile: "large",
    stop_conditions: ["#5821 or #5820 is not terminal", "Any child is missing or not prepared", "Any claim is active before scheduling", "Any dependency or path collision exists", "WP-04.16 proof is not real and native"],
    validation_lanes: [{lane: "live-wave-contract", proof_role: "Verify exact mapping, approved records, null claims, dependency graph, exclusive paths, and the WP-04.16 integration-proof gate.", deterministic: true, resource_profile: "small", parallel_group: "planning", budget_seconds: 600, budget_tokens: 5000, argv: ["ruby", ".csdlc/prepared/issues/#{issue}/validate-implementation-wave.rb"], acceptance_ids: ["AC-1", "AC-2", "AC-3", "AC-4", "AC-5", "AC-6"]}],
    failure_policy: "Fail closed on missing children, denominator drift, active preparation claims, dependency bypass, path overlap, or self-attested integration.",
    review_prompts: ["Does the live wave match #5821 exactly?", "Are all child paths exclusive?", "Can any child or #5832 bypass a terminal gate?", "Is integration proof real, exact-head, and native?"],
    review_scope: "WP-04-IMP mapping, scheduling, reconciliation, and child handoff contracts."
  }
end

def implementation_wave_validator(umbrella_issue, numbers)
  expected = JSON.generate(numbers)
  <<~RUBY
    #!/usr/bin/env ruby
    # frozen_string_literal: true
    require "json"
    require "yaml"
    expected = #{expected}
    abort "expected sixteen children" unless expected.length == 16
    all_paths = {}
    expected.each do |wp, issue|
      index_path = ".csdlc/issues/\#{issue}/index.json"
      abort "missing index for \#{wp} ##\#{issue}" unless File.file?(index_path)
      index = JSON.parse(File.read(index_path))
      abort "issue mismatch for \#{wp}" unless index["issue"] == issue
      abort "\#{wp} design not approved" unless index.dig("design_review", "approved", "revision").to_s.match?(/\\A[0-9a-f]{64}\\z/)
      abort "\#{wp} preparation claim remains active" unless index["claim"].nil?
      %w[sip stp spp vpp].each do |card|
        values = JSON.parse(File.read(".csdlc/issues/\#{issue}/cards/\#{card}.values.json"))
        abort "\#{wp} \#{card} not ready" unless values["status"] == "ready"
      end
      %w[srp sor].each do |card|
        values = JSON.parse(File.read(".csdlc/issues/\#{issue}/cards/\#{card}.values.json"))
        abort "\#{wp} \#{card} not truthful pre-phase" unless %w[pre_phase draft].include?(values["status"])
      end
      design = File.read(".csdlc/prepared/issues/\#{issue}/design.md")
      section = design[/## Exclusive Owned Paths\\n\\n(.*?)\\n\\n## /m, 1]
      abort "\#{wp} missing exact owned paths" unless section
      paths = section.scan(/`([^`]+)`/).flatten
      abort "\#{wp} has no owned paths" if paths.empty?
      paths.each do |path|
        abort "path collision \#{path}: \#{all_paths[path]} and \#{wp}" if all_paths.key?(path)
        all_paths[path] = wp
      end
    end
    umbrella = JSON.parse(File.read(".csdlc/issues/#{umbrella_issue}/index.json"))
    abort "umbrella claim remains active" unless umbrella["claim"].nil?
    gate = File.read(".csdlc/prepared/issues/5821/design.md")
    expected.each do |wp, issue|
      abort "gate mapping missing \#{wp} ##\#{issue}" unless gate.include?("| \#{wp} | ##\#{issue} |")
    end
    puts "PASS: WP-04-IMP ##{umbrella_issue}, sixteen approved claim-null children, \#{all_paths.length} exclusive paths"
  RUBY
end

mode = ARGV.shift
case mode
when "umbrella-request"
  request = create_request(
    title: "[v0.92][WP-04-IMP][umbrella] Execute distributed Guardian child wave",
    body: umbrella_body,
    operation_key: "v092-wp04-imp-distributed-guardian-umbrella"
  )
  write_json(File.join(OUT, "create", "wp04-imp.json"), request)
when "child-requests"
  umbrella_issue = Integer(ARGV.fetch(0))
  CHILDREN.each do |item|
    request = create_request(
      title: "[v0.92][#{item[:wp]}] #{item[:title]}",
      body: issue_body(item, umbrella_issue),
      operation_key: "v092-#{item[:wp].downcase.tr('.', '-')}-#{item[:key]}"
    )
    write_json(File.join(OUT, "create", "#{item[:wp].downcase.tr('.', '-')}.json"), request)
  end
when "execute-child-creates"
  binary = ARGV.fetch(0)
  CHILDREN.each do |item|
    stem = item[:wp].downcase.tr(".", "-")
    request_path = File.join(OUT, "create", "#{stem}.json")
    stdout, stderr, status = Open3.capture3(binary, "run", "--request", request_path, chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "typed create failed for #{item[:wp]}" unless status.success?

    result = JSON.parse(stdout)
    issue = result.fetch("issue")
    abort "create did not reconcile #{item[:wp]}" unless result.fetch("reconciled") && issue.fetch("marker_present")
    write_json(File.join(OUT, "results", "#{stem}.json"), result)
    puts "#{item[:wp]} ##{issue.fetch('number')} #{issue.fetch('title')}"
  end
when "materialize"
  umbrella_issue = Integer(ARGV.fetch(0))
  numbers = child_numbers
  CHILDREN.each do |item|
    issue = numbers.fetch(item[:wp])
    prepared = File.join(ROOT, ".csdlc", "prepared", "issues", issue.to_s)
    FileUtils.mkdir_p(prepared)
    File.write(File.join(prepared, "design.md"), child_design(item, issue, umbrella_issue, numbers))
    File.write(File.join(prepared, "diagram.mmd"), child_diagram(item, umbrella_issue))
    File.write(File.join(prepared, "validate-proof-receipt.rb"), proof_validator(item, issue))
    request, claim = bootstrap_request(issue: issue, design_path: ".csdlc/prepared/issues/#{issue}/design.md", diagram_path: ".csdlc/prepared/issues/#{issue}/diagram.mmd", initial: child_initial(item, issue, umbrella_issue, numbers), paths: item[:paths])
    write_json(File.join(prepared, "bootstrap-request.json"), request)
    write_json(File.join(prepared, "bind-request.json"), {issue: issue, base_branch: "main", branch: claim[:branch], worktree: ".", claim: claim})
  end
  prepared = File.join(ROOT, ".csdlc", "prepared", "issues", umbrella_issue.to_s)
  FileUtils.mkdir_p(prepared)
  File.write(File.join(prepared, "design.md"), umbrella_design(umbrella_issue, numbers))
  File.write(File.join(prepared, "diagram.mmd"), "flowchart LR\n  G[\"#5821 gate\"] --> U[\"##{umbrella_issue} WP-04-IMP\"]\n  U --> C[\"WP-04.01 through WP-04.16\"]\n  C --> P[\"WP-04.16 integrated proof\"]\n  P --> W[\"#5832 WP-14\"]\n")
  File.write(File.join(prepared, "validate-implementation-wave.rb"), implementation_wave_validator(umbrella_issue, numbers))
  request, claim = bootstrap_request(issue: umbrella_issue, design_path: ".csdlc/prepared/issues/#{umbrella_issue}/design.md", diagram_path: ".csdlc/prepared/issues/#{umbrella_issue}/diagram.mmd", initial: umbrella_initial(umbrella_issue, numbers), paths: [])
  write_json(File.join(prepared, "bootstrap-request.json"), request)
  write_json(File.join(prepared, "bind-request.json"), {issue: umbrella_issue, base_branch: "main", branch: claim[:branch], worktree: ".", claim: claim})
  write_json(File.join(OUT, "mapping.json"), {umbrella: umbrella_issue, children: numbers})
  puts JSON.pretty_generate({umbrella: umbrella_issue, children: numbers})
when "bootstrap-local"
  init_binary, bind_binary, umbrella = ARGV
  issues = [Integer(umbrella)] + child_numbers.values
  issues.each do |issue|
    prepared = File.join(ROOT, ".csdlc", "prepared", "issues", issue.to_s)
    if File.file?(File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json"))
      puts "already prepared ##{issue}"
      next
    end
    stdout, stderr, status = Open3.capture3(init_binary, "--root", ROOT, "--request", File.join(prepared, "bootstrap-request.json"), chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "typed init failed for ##{issue}: #{stdout}" unless status.success?
    File.write(File.join(prepared, "init-result.json"), stdout)
    stdout, stderr, status = Open3.capture3(bind_binary, "--root", ROOT, "--request", File.join(prepared, "bind-request.json"), chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "typed bind failed for ##{issue}: #{stdout}" unless status.success?
    File.write(File.join(prepared, "bind-result.json"), stdout)
    puts "prepared and bound ##{issue}"
  end
when "revoke-local"
  bind_binary, umbrella = ARGV
  issues = [Integer(umbrella)] + child_numbers.values
  issues.each do |issue|
    prepared = File.join(ROOT, ".csdlc", "prepared", "issues", issue.to_s)
    index = JSON.parse(File.read(File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")))
    claim = index.fetch("claim")
    request = {issue: issue, repository: REPOSITORY, expected_claim_id: claim.fetch("id"), expected_generation: index.fetch("generation"), expected_digest: index.fetch("digest"), now_unix_seconds: Time.now.to_i, actor: "codex:5860-runtime-wp04-wave", operator_authority: "operator:daniel-issue-5860-preparation-only", reason: "Release completed preparation claim for just-in-time child implementation handoff."}
    request_path = File.join(prepared, "revoke-preparation-claim.json")
    write_json(request_path, request)
    stdout, stderr, status = Open3.capture3(bind_binary, "--root", ROOT, "--revoke-request", request_path, chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "typed revoke failed for ##{issue}: #{stdout}" unless status.success?
    File.write(File.join(prepared, "revoke-result.json"), stdout)
    puts "released preparation claim ##{issue}"
  end
when "reacquire-runtime"
  bind_binary = ARGV.fetch(0)
  [5800, 5820, 5795, 5821, 5832, 5837].each do |issue|
    prepared = File.join(ROOT, ".csdlc", "prepared", "issues", issue.to_s)
    index = JSON.parse(File.read(File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")))
    abort "##{issue} already has a claim" unless index["claim"].nil?
    now = Time.now.to_i
    claim = {id: "claim-#{issue}-v092-preparation", owner: "codex:5860-runtime-wp04-second-pass", generation: index.fetch("generation"), acquired_unix_seconds: now, expires_unix_seconds: now + 86_400, heartbeat_unix_seconds: now, branch: "codex/5860-v092-execution-readiness", worktree: ".", protected_paths: [".csdlc/issues/#{issue}", ".csdlc/prepared/issues/#{issue}", ".csdlc/evidence/#{issue}"], purpose: "Second-pass Runtime/WP-04 readiness repair without product implementation."}
    request = {issue: issue, expected_generation: index.fetch("generation"), expected_digest: index.fetch("digest"), now_unix_seconds: now, actor: "codex:5860-runtime-wp04-second-pass", reason: "Reacquire issue-local preparation authority for exact-path, dependency, and proof-contract repair.", replacement: claim}
    request_path = File.join(prepared, "second-pass-reacquire.json")
    write_json(request_path, request)
    stdout, stderr, status = Open3.capture3(bind_binary, "--root", ROOT, "--reacquire-request", request_path, chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "typed reacquire failed for ##{issue}: #{stdout}" unless status.success?
    File.write(File.join(prepared, "second-pass-reacquire-result.json"), stdout)
    puts "reacquired ##{issue}"
  end
when "repair-runtime-cards"
  edit_binary, bind_binary = ARGV
  exact_paths = {
    5800 => %w[adl-runtime/src/local_tls.rs adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs adl-runtime/tests/local_tls.rs demos/html-observatory/runtime-v3.config.json demos/html-observatory/README.md adl/tools/validate_v092_browser_trusted_observatory.mjs],
    5820 => %w[adl-runtime/src/bin/adl-runtime-guardian.rs adl-runtime/src/guardian.rs adl-runtime/src/shutdown.rs adl-runtime/src/supervision.rs adl-runtime/src/resident_agent.rs adl-runtime-kernel/src/bin/adl-runtime-kernel.rs adl-runtime-kernel/src/config.rs adl-runtime-kernel/src/durable_state.rs adl-runtime-kernel/src/supervisor.rs infra/runtime-v3/runtime-init.toml adl-runtime/tests/runtime_guardian_lifecycle.rs adl/tools/validate_v092_runtime_guardian_lifecycle.sh adl/tools/validate_v092_runtime_native_receipts.rb],
    5795 => %w[adl-runtime-kernel/src/shepherd.rs adl-runtime-kernel/tests/shepherd.rs demos/html-observatory/shepherd.js demos/html-observatory/index.html adl/tools/validate_v092_shepherd_browser_roundtrip.mjs],
    5821 => [".csdlc/issues/5821", ".csdlc/prepared/issues/5821", ".csdlc/evidence/5821", "docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md", "docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md"],
    5832 => %w[adl-runtime/src/acip.rs adl-runtime/src/runtime_api_auth.rs adl-runtime-kernel/src/acip.rs adl-runtime-kernel/src/protocol_adapters.rs adl-runtime/tests/runtime_api_wss.rs schemas/acip/v1/acip.proto schemas/acip/v1/catalog.json docs/api/runtime-v3/v1/acip.openapi.json adl/tools/validate_v092_acip_wss.sh adl/tools/validate_v092_acip_native_receipts.rb],
    5837 => %w[demos/html-observatory/app.js demos/html-observatory/styles.css demos/v0.91.6/unity-observatory/Assets/Scripts/RuntimeV3Client.cs demos/v0.91.6/unity-observatory/Assets/Resources/runtime-v3-contract.json demos/v0.91.6/unity-observatory/Assets/Tests/RuntimeV3ClientTests.cs adl/tools/validate_v092_html_observatory_live.mjs adl/tools/validate_v092_unity_observatory_live.sh adl/tools/validate_v092_observatory_restart_reconnect.sh]
  }
  lane = lambda do |name, role, argv, acceptance, group = "runtime", deterministic = true, defer_reason = nil|
    {lane: name, proof_role: role, acceptance_ids: acceptance, deterministic: deterministic, resource_profile: "medium", budget_seconds: 900, budget_tokens: 6000, argv: argv, parallel_group: group, defer_reason: defer_reason}
  end
  operations = Hash.new { |hash, key| hash[key] = [] }
  exact_paths.each do |issue, paths|
    operations[issue] << ["sip", {operation: "replace_planning_collection", field: "declared_scope", values: paths}]
    operations[issue] << ["spp", {operation: "replace_planning_collection", field: "affected_areas", values: paths}]
  end
  operations[5820] << ["vpp", {operation: "replace_validation_lanes", lanes: [
    lane.call("guardian-lifecycle-contract", "Run the exact nonzero production Guardian lifecycle target over init, supervision, restart, durable state, degradation, shutdown, and logs.", ["cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "runtime_guardian_lifecycle", "--no-tests=fail"], %w[AC-1 AC-2 AC-3 AC-4 AC-5]),
    lane.call("production-guardian-api-wss-restart", "Launch the production Guardian/kernel and prove authenticated HTTPS/WSS, child kill, bounded restart, durable state, readiness, clean shutdown, and clean logs.", ["bash", "adl/tools/validate_v092_runtime_guardian_lifecycle.sh"], %w[AC-1 AC-2 AC-3 AC-4 AC-5 AC-6]),
    lane.call("native-guardian-receipts", "Recompute digest-bound macOS, Linux, and native Windows production Guardian lifecycle receipts.", ["ruby", ".csdlc/prepared/issues/5820/validate-runtime-native-receipts.rb"], %w[AC-6 AC-7]),
    lane.call("exact-head-review-preflight", "Reject diff damage before exact-head review and issue-closing publication.", ["git", "diff", "--check"], %w[AC-8], "review")
  ]}]
  operations[5821] += [
    ["sip", {operation: "replan", field: "goal", value: "Freeze and independently approve the distributed Guardian architecture/security contract, validate the live #5862 plus #5863-#5878 denominator, and stop without product implementation."}],
    ["sip", {operation: "replan", field: "required_outcome", value: "One approved architecture and threat model plus an exact live sixteen-child ledger with owners, dependencies, exclusive paths, proof boundaries, rollback responsibilities, prepared cards, and null claims."}],
    ["sip", {operation: "replace_planning_collection", field: "authority_boundary", values: ["Issue 5821 owns only the architecture/security gate, live denominator, and gate review", "WP-04-IMP issue 5862 owns orchestration and reconciliation only", "Issues 5863 through 5878 own child implementation, proof, review, PR, closeout, and rollback", "Issue 5878 alone owns module registration and final integration", "Issue 5832 remains blocked until issue 5862 has terminal integrated output"]}],
    ["stp", {operation: "replace_planning_collection", field: "dependencies", values: ["WP-03 issue 5820 terminal with stable ingress, lifecycle, state, API/WSS, readiness, restart, and shutdown", "WP-04-IMP issue 5862 mapped to exactly issues 5863 through 5878", "All sixteen child designs approved with ready SIP/STP/SPP/VPP, truthful pre-phase SRP/SOR, and null claims", "WP-14 issue 5832 blocked until issue 5862 terminal integrated output"]}],
    ["vpp", {operation: "replace_validation_lanes", lanes: [
      lane.call("live-child-wave-ledger", "Validate #5862 plus exactly #5863-#5878, complete owner/dependency/path/proof/rollback fields, acyclic dependencies, exclusive paths, approved designs, and null claims.", ["ruby", ".csdlc/prepared/issues/5821/validate-child-wave.rb"], %w[AC-3 AC-4 AC-5 AC-6 AC-7], "planning-gate", false, "Requires live GitHub read access through the typed v2 issue owner binary."),
      lane.call("architecture-security-review", "Validate required architecture/threat coverage and an independent accepted exact-packet review with recomputed artifact digests.", ["ruby", ".csdlc/prepared/issues/5821/validate-architecture-security-review.rb"], %w[AC-1 AC-2 AC-8], "planning-gate")
    ]}]
  ]
  operations[5832] += [
    ["stp", {operation: "replace_planning_collection", field: "dependencies", values: ["WP-04 gate issue 5821 terminal", "WP-04-IMP issue 5862 terminal after issues 5863 through 5878 integrate", "Current ACIP stream and trace/replay baselines requalified at the implementation revision", "Stable Runtime API/auth ownership before issues 5795 and 5837 integrate"]}],
    ["spp", {operation: "replace_planning_collection", field: "stop_conditions", values: ["Issue 5821 architecture/security gate is not terminal", "Issue 5862 WP-04-IMP or any of issues 5863 through 5878 lacks terminal integrated evidence", "Semantic envelope, schema, catalog, JSON, or WSS authority remains ambiguous", "Any declared path overlaps an active owner", "Native macOS, Linux, or Windows proof cannot be produced"]}],
    ["vpp", {operation: "replace_validation_lanes", lanes: [
      lane.call("acip-schema-roundtrip-negatives", "Run exact nonzero ACIP schema, catalog, protobuf/JSON round-trip, negotiation, replay, malformed, oversized, and denied-dispatch tests.", ["cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "runtime_api_wss", "--no-tests=fail"], %w[AC-1 AC-2 AC-3 AC-4 AC-6]),
      lane.call("production-acip-wss", "Launch production Guardian/kernel and prove real authenticated Rustls WSS binary/JSON full-duplex exchange, correlation, backpressure, reconnect, and typed errors.", ["bash", "adl/tools/validate_v092_acip_wss.sh"], %w[AC-1 AC-2 AC-3 AC-4 AC-5 AC-6]),
      lane.call("native-acip-receipts", "Recompute exact-revision macOS, Linux, and native Windows ACIP/WSS receipts with binary/schema/transcript digests and nonzero exchanges/negatives.", ["ruby", ".csdlc/prepared/issues/5832/validate-acip-native-receipts.rb"], %w[AC-5 AC-6 AC-7]),
      lane.call("exact-head-review-preflight", "Reject diff damage before exact-head review.", ["git", "diff", "--check"], %w[AC-8], "review")
    ]}]
  ]

  exact_paths.each_key do |issue|
    prepared = File.join(ROOT, ".csdlc", "prepared", "issues", issue.to_s)
    index = JSON.parse(File.read(File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")))
    if index["claim"].nil?
      puts "already repaired and released ##{issue}"
      next
    end
    claim_id = index.fetch("claim").fetch("id")
    approval = {issue: issue, expected_generation: index.fetch("generation"), expected_digest: index.fetch("digest"), claim_id: claim_id, reviewer: "codex:5860-runtime-wp04-second-pass-design-review"}
    approval_path = File.join(prepared, "second-pass-approve-design.json")
    write_json(approval_path, approval)
    stdout, stderr, status = Open3.capture3(edit_binary, "--repo", ROOT, "approve-design", "--request", approval_path, chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "design approval failed for ##{issue}: #{stdout}" unless status.success?
    operations.fetch(issue).each_with_index do |(card, operation), position|
      index = JSON.parse(File.read(File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")))
      request = {issue: issue, card: card, expected_generation: index.fetch("generation"), expected_digest: index.fetch("digest"), claim_id: claim_id, actor: "codex:5860-runtime-wp04-second-pass", reason: "Reconcile exact Runtime/WP-04 ownership, dependencies, and proving validation.", operation: operation}
      request_path = File.join(prepared, format("second-pass-edit-%02d-%s.json", position + 1, card))
      write_json(request_path, request)
      stdout, stderr, status = Open3.capture3(edit_binary, "--repo", ROOT, "apply", "--request", request_path, chdir: ROOT)
      warn stderr unless stderr.empty?
      abort "card edit failed for ##{issue} #{card}: #{stdout}" unless status.success?
    end
    index = JSON.parse(File.read(File.join(ROOT, ".csdlc", "issues", issue.to_s, "index.json")))
    request = {issue: issue, repository: REPOSITORY, expected_claim_id: claim_id, expected_generation: index.fetch("generation"), expected_digest: index.fetch("digest"), now_unix_seconds: Time.now.to_i, actor: "codex:5860-runtime-wp04-second-pass", operator_authority: "operator:daniel-issue-5860-preparation-only", reason: "Release completed second-pass preparation claim for just-in-time implementation handoff."}
    request_path = File.join(prepared, "second-pass-revoke-claim.json")
    write_json(request_path, request)
    stdout, stderr, status = Open3.capture3(bind_binary, "--root", ROOT, "--revoke-request", request_path, chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "claim revoke failed for ##{issue}: #{stdout}" unless status.success?
    puts "repaired and released ##{issue}"
  end
when "update-live-contracts"
  github_binary = ARGV.fetch(0)
  umbrella_issue = 5862
  numbers = child_numbers
  updates = []
  updates << [umbrella_issue, "[v0.92][WP-04-IMP][umbrella] Execute distributed Guardian child wave", exact_umbrella_body(numbers), "v092-wp04-imp-exact-live-mapping"]
  CHILDREN.each do |item|
    issue = numbers.fetch(item[:wp])
    updates << [issue, "[v0.92][#{item[:wp]}] #{item[:title]}", exact_child_body(item, umbrella_issue, numbers), "v092-#{item[:wp].downcase.tr('.', '-')}-exact-live-dependencies"]
  end
  updates << [5821, "[v0.92][WP-04] Distributed Guardian/polis runtime", gate_live_body(numbers), "v092-wp04-live-gate-denominator-reconcile"]
  updates << [5832, "[v0.92][WP-14] ACIP and A2A contract reconciliation", wp14_live_body, "v092-wp14-wp04-imp-dependency-reconcile"]
  updates.each do |issue, title, body, operation_key|
    request = {action: "issue_update", repository: REPOSITORY, issue: issue, title: title, body: body, labels: [], assignees: [], require_review: false, required_checks: [], operation_key: operation_key}
    request_path = File.join(OUT, "update", "#{issue}.json")
    write_json(request_path, request)
    stdout, stderr, status = Open3.capture3(github_binary, "run", "--request", request_path, chdir: ROOT)
    warn stderr unless stderr.empty?
    abort "typed live update failed for ##{issue}: #{stdout}" unless status.success?
    result = JSON.parse(stdout)
    abort "live update did not reconcile ##{issue}" unless result["reconciled"] && result.dig("issue", "marker_present")
    write_json(File.join(OUT, "update-results", "#{issue}.json"), result)
    write_json(File.join(OUT, "read", "#{issue}.json"), {action: "issue_read", repository: REPOSITORY, issue: issue, labels: [], assignees: [], require_review: false, required_checks: [], operation_key: nil})
    puts "updated live contract ##{issue}"
  end
else
  abort "usage: #{$PROGRAM_NAME} umbrella-request | child-requests <umbrella-issue> | execute-child-creates <csdlc-github-issue> | materialize <umbrella-issue> | bootstrap-local <csdlc-init> <csdlc-bind> <umbrella-issue> | revoke-local <csdlc-bind> <umbrella-issue>"
end
