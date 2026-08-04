# frozen_string_literal: true

module FeatureDecisions5594
  GROUPS = {
    "K" => {
      classification: "kernel_continuity_ingress",
      owner_issues: [5591],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Runtime execution, lifecycle, continuity, replay, or canonical-ingress behavior must be proved on the Runtime v3 kernel path."
    },
    "R" => {
      classification: "reasoning_adaptive_cognition",
      owner_issues: [5592],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Reasoning, memory, cognition, affect, or adaptive behavior must be proved or explicitly dispositioned by Runtime v3 Parity-B."
    },
    "O" => {
      classification: "governed_operations",
      owner_issues: [5589],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Governance, identity, provider, state, time, tool, or operational-service behavior belongs to Runtime v3 Parity-C."
    },
    "A" => {
      classification: "secure_access_observatory",
      owner_issues: [5590],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Secure local or remote access, communications, telemetry, guardian, or Observatory behavior belongs to Runtime v3 Parity-D."
    },
    "C" => {
      classification: "csdlc_v2_acceptance",
      owner_issues: [5358],
      disposition: "external_owner_acceptance_required",
      basis: "This is a C-SDLC authoring, review, validation, quality, or control-plane capability governed by C-SDLC v2 acceptance rather than Runtime v3 parity."
    },
    "S" => {
      classification: "adl_v2_signing",
      owner_issues: [5342],
      disposition: "external_owner_acceptance_required",
      basis: "Signing, verification, and trust-policy replacement is explicitly owned by ADL v2 WP-07 rather than Runtime v3 governed-operations parity."
    },
    "P" => {
      classification: "provider_and_secure_transport",
      owner_issues: [5589, 5590],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Provider operations belong to Runtime v3 Parity-C while secure transport and remote access belong to Parity-D; both proofs are required."
    },
    "D" => {
      classification: "retained_or_later_milestone",
      owner_issues: [5347],
      disposition: "deferred_to_canonical_next_target",
      basis: "This feature is retained evidence, a product or demonstration surface, or explicitly owned by its canonical later milestone; WP-17 prevents deletion without disposition."
    }
  }.freeze

  # Explicit, source-line-pinned decisions for every feature row in the canonical
  # matrix. The source digest in the generated artifact makes line drift fail
  # closed; comments keep each decision human-reviewable.
  BY_SOURCE_LINE = {
    210 => "K", # Deterministic workflow execution
    211 => "K", # ExecutionPlan runtime
    212 => "K", # Sequential + fork/join coordination
    213 => "K", # Bounded concurrency and retry/failure controls
    214 => "K", # Run artifacts and replay-oriented inspection
    215 => "S", # Signing, verification, and trust policy
    216 => "P", # Provider and transport substrate
    217 => "A", # Remote execution baseline
    218 => "O", # Human-in-the-loop pause/resume
    219 => "C", # Structured authoring model
    220 => "C", # Structured planning and Structured Review Prompt workflow
    221 => "C", # Control-plane lifecycle
    222 => "C", # Editor and command-adapter surfaces
    223 => "C", # Review and validation surfaces
    224 => "C", # Task-bundle workflow
    225 => "R", # Agency, cognitive loop, and cognitive stack
    226 => "R", # Fast/slow thinking and cognitive arbitration
    227 => "R", # Bounded Godel loop
    228 => "R", # Godel agents and Godel-Hadamard-Bayes algorithm
    229 => "R", # ObsMem indexing, retrieval, and evidence-aware ranking
    230 => "R", # Shared ObsMem foundation
    231 => "O", # Trace validation, trace review, and trace-to-memory ingestion
    232 => "R", # Bounded cognitive path
    233 => "O", # Freedom Gate baseline
    234 => "O", # Freedom Gate v2
    235 => "K", # Trace substrate
    236 => "D", # Operational skills substrate
    237 => "K", # Runtime environment and lifecycle completion
    238 => "K", # Execution boundaries and capability-aware local execution
    239 => "K", # Local runtime resilience and Shepherd preservation
    240 => "O", # Chronosense / temporal substrate
    241 => "O", # Temporal query, retrieval, identity semantics, and continuity hooks
    242 => "O", # Commitments, deadlines, and bounded temporal causality
    243 => "O", # Cost model, accounting primitives, and bounded economics hooks
    244 => "D", # PHI-style integration metrics
    245 => "R", # Instinct and bounded agency
    246 => "D", # Paper Sonata public-facing proof surface
    247 => "D", # Deep-agents comparative proof
    248 => "O", # AEE 1.0 convergence
    249 => "O", # Decision, action, and skill-governance surfaces
    250 => "O", # Delegation, refusal, and coordination contracts
    251 => "A", # Provider-extension packaging and safe extension boundaries
    252 => "A", # Security, posture, and trust-under-adversary package
    253 => "A", # Adversarial runtime, exploit/replay, and self-attack band
    254 => "C", # Demo proof entry points and quality gate
    255 => "D", # Five-agent Hey Jude MIDI demo
    256 => "D", # arXiv paper writer and three-paper program
    257 => "K", # Long-lived supervisor, heartbeat, and cycle artifacts
    258 => "D", # Stock-league long-lived demo family
    259 => "A", # Minimal status/inspection boundary
    260 => "D", # CodeFriend review showcase and architecture-document generation
    261 => "C", # Coverage ratchet, test tracker, and quality tracking
    262 => "C", # Rust refactoring tracker and evidence-driven maintenance
    263 => "D", # Milestone compression and repo visibility prototypes
    264 => "D", # HTML milestone dashboard and compression reporting
    265 => "K", # Runtime v2 foundation prototype
    266 => "A", # CSM Observatory visibility and operator-report surfaces
    267 => "K", # Runtime v2 hardening, recovery, quarantine, and expanded invariants
    268 => "K", # First bounded CSM run
    269 => "C", # Third-party review and review-quality gates
    270 => "D", # ANRM / shepherd-model experiments
    271 => "D", # CSM Shepherd model and Gemma training path
    272 => "D", # Capability-testing evidence and Aptitude Atlas boundary
    273 => "O", # Governed tool calls and capability contracts
    274 => "D", # Cognitive Compression Cost instrumentation
    275 => "D", # Web-based code editor integration
    276 => "R", # Reasoning graph baseline
    277 => "S", # Signed trace and trace query
    278 => "R", # Wellbeing, affect, kindness, moral cognition, humor
    279 => "A", # Secure Agent Communication and Invocation Protocol
    280 => "K", # Inhabited runtime readiness
    281 => "K", # Runtime/polis architecture alignment
    282 => "O", # Agent lifecycle state model
    283 => "A", # CSM Observatory active agent runtime
    284 => "O", # Citizen standing and citizen state follow-on
    285 => "R", # Memory, Theory of Mind, capability testing, intelligence metrics, governed learning, and ANRM/Gemma
    286 => "A", # ACIP hardening and local encryption boundary
    287 => "A", # A2A adapter boundary
    288 => "K", # Runtime inhabitant proof
    289 => "D", # UTS + ACC multi-model benchmark and provider-native tool-call comparison
    290 => "C", # Runtime/test-cycle recovery and coverage ergonomics
    291 => "D", # CodeFriend repo-review product layer
    292 => "C", # Review heuristics and reviewer demo lane
    293 => "D", # Google Workspace CMS bridge and Rust-native adapter boundary
    294 => "D", # Automated repository modernization and external refactoring integration
    295 => "D", # Generic speculative decoding runtime acceleration
    296 => "D", # Repo visibility follow-on
    297 => "D", # Publication packet program and GHB paper lane
    298 => "D", # General-intelligence paper packet
    299 => "C", # Rustdoc/doc cleanup
    300 => "C", # Workflow guardrails
    301 => "C", # Cognitive SDLC first slice and transition manifest
    302 => "C", # Cognitive SDLC default operation and five-minute-sprint repeatability
    303 => "A", # Logging, observability, and OTel-compatible proof-loop readiness
    304 => "K", # Resilience, citizen persistence, and operational sleep/wake
    305 => "C", # Public prompt records export, redaction, validation, and indexing
    306 => "O", # Provider/model reliability and multi-agent readiness
    307 => "A", # Security readiness and Continuous Adversarial Verification
    308 => "R", # Curiosity Engine and Discovery Substrate
    309 => "R", # Constructability Gate for shared ADL reality
    310 => "R", # Reasoning graph, loop runtime, and adl.skill.v1
    311 => "R", # ACP / cognitive profiles runtime surface
    312 => "A", # ACIP binary schema and WebSocket carrier
    313 => "O", # Identity, stable name, and continuity substrate
    314 => "O", # Memory grounding, capability envelope, and birth witnesses/receipt
    315 => "R", # Memory Palace navigable context topology
    316 => "R", # First true Godel-agent birthday
    317 => "O", # Constitutional citizenship, rights/duties, and governance review
    318 => "R", # Bounded Theory of Mind, relationship, reputation, and shared social memory boundary
    319 => "O", # Delegation, upstream delegation, IAM, standing transition, and challenge/appeal governance
    320 => "O", # Guilds and collective organization
    321 => "A", # Enterprise security for the ADL polis
    322 => "A", # Secure execution, policy, identity/auth, isolation, and provider-trust convergence
    323 => "R", # Mental time travel / temporal self-projection
    324 => "D", # Payments, settlement, economic agency, and x402 / Lightning adapters
    325 => "O", # Bounded contract-market and resource-stewardship bridge
    326 => "K", # Distributed execution integration
    327 => "D", # CodeFriend v1 and portable adapter v2
    328 => "D", # Capability-testing evidence consumption / Aptitude Atlas boundary
    329 => "D", # Demo catalog and polished MVP walkthrough
    330 => "C", # Control-plane Rust migration / tooling hardening
    331 => "D"  # Zed integration
  }.freeze
end
