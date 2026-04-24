window.milestoneData = {
  milestone: "v0.90.4",
  version: "0.90.4",
  owner: "Daniel Austin / Agent Logic",
  updated: "2026-04-23",
  snapshotGeneratedAt: "2026-04-24T00:25:00Z",
  snapshotMaxAgeHours: 24,
  status: "active",
  statusLabel: "Active compression visibility; static read-only snapshot",
  summary:
    "v0.90.4 is the contract-market substrate milestone. This dashboard mirrors the live milestone package, v0.90 compression authority surfaces, and a bounded GitHub snapshot so operators can see issue-wave posture, release-tail gates, and the next safe action without treating the page as release authority.",
  boundary: [
    "Read-only visibility surface; it must not mutate issues, PRs, branches, cards, releases, or closeout state.",
    "Canonical truth remains in milestone docs, GitHub issues and PRs, task cards, validation output, and review records.",
    "Unknown or stale evidence is shown as unknown or stale rather than green."
  ],
  authority: [
    {
      label: "v0.90.4 README",
      path: "../../milestones/v0.90.4/README.md",
      note: "Tracked execution package and current milestone scope."
    },
    {
      label: "v0.90.4 WBS",
      path: "../../milestones/v0.90.4/WBS_v0.90.4.md",
      note: "Canonical 21-row WP map, including WP-14A."
    },
    {
      label: "v0.90.4 issue wave",
      path: "../../milestones/v0.90.4/WP_ISSUE_WAVE_v0.90.4.yaml",
      note: "Canonical WP-to-issue mapping for #2420 through #2440."
    },
    {
      label: "v0.90.4 checklist",
      path: "../../milestones/v0.90.4/MILESTONE_CHECKLIST_v0.90.4.md",
      note: "Live planning, review, and release truth markers."
    },
    {
      label: "v0.90.4 release plan",
      path: "../../milestones/v0.90.4/RELEASE_PLAN_v0.90.4.md",
      note: "Current release gates, non-claims, and handoff boundaries."
    },
    {
      label: "v0.90 compression model",
      path: "../../milestones/v0.90/milestone_compression/README.md",
      note: "Read-only compression pilot and authority boundary."
    },
    {
      label: "canonical milestone state",
      path: "../../milestones/v0.90/milestone_compression/CANONICAL_MILESTONE_STATE_v0.90.yaml",
      note: "Minimal canonical state surface for milestone-compression truth."
    },
    {
      label: "drift check report",
      path: "../../milestones/v0.90/milestone_compression/DRIFT_CHECK_REPORT_v0.90.md",
      note: "Known classifications and read-only drift-check boundary."
    },
    {
      label: "finish validation profiles",
      path: "../../milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md",
      note: "Focused-validation safety boundaries inherited by later milestones."
    }
  ],
  signals: [
    {
      label: "WP wave",
      value: "21",
      note: "The live v0.90.4 issue wave is #2420 through #2440, including WP-14A.",
      tone: "good"
    },
    {
      label: "Closed WPs",
      value: "2",
      note: "WP-01 and WP-02 are closed in the live issue wave.",
      tone: "good"
    },
    {
      label: "Current edge",
      value: "WP-03",
      note: "Contract schema is the next execution edge for the contract-market lane.",
      tone: "warn"
    },
    {
      label: "Open milestone PRs",
      value: "0",
      note: "No v0.90.4 PR is open in the current static GitHub snapshot.",
      tone: "unknown"
    }
  ],
  freshness: [
    {
      label: "Milestone docs",
      state: "good",
      note: "Tracked v0.90.4 docs, WBS, checklist, and issue wave are aligned to the opened wave."
    },
    {
      label: "Issue snapshot",
      state: "good",
      note: "GitHub issue snapshot captured at generation time shows WP-01 and WP-02 closed, with WP-03 onward still open."
    },
    {
      label: "PR/check snapshot",
      state: "unknown",
      note: "No open milestone PR exists in the current snapshot, so live check posture is unknown rather than green."
    },
    {
      label: "Staleness guard",
      state: "good",
      note: "If this snapshot is older than 24 hours, the dashboard should be treated as stale until refreshed."
    }
  ],
  nextActions: [
    "Execute WP-03 #2422 contract schema so the contract-market lane has a canonical parent artifact before bid and selection logic widen.",
    "Keep WP-04 through WP-07 behind schema and authority acceptance instead of widening into runner or demo work early.",
    "Preserve WP-14A as the feature-proof lane before quality, review, or release-tail convergence.",
    "Keep payment rails, settlement, inter-polis economics, and governed tool execution out of v0.90.4 claims."
  ],
  watchlist: [
    {
      label: "Contract schema",
      state: "active",
      note: "WP-03 is the live execution edge; downstream contract-market semantics should not widen before it lands."
    },
    {
      label: "Authority transitions",
      state: "blocked",
      note: "Transition, lifecycle, delegation, and counterparty proof remain blocked on the early schema lane."
    },
    {
      label: "Runner and demo proof",
      state: "blocked",
      note: "The fixture set, runner, bounded demo, and demo matrix are intentionally later and should not be shown as landed."
    },
    {
      label: "Review tail",
      state: "blocked",
      note: "WP-15 through WP-20 remain gated on feature proof, review convergence, and accepted-finding disposition."
    }
  ],
  prChecks: [
    {
      label: "Open milestone PRs",
      state: "unknown",
      note: "No open PR with label version:v0.90.4 was present in the GitHub snapshot used for this dashboard."
    },
    {
      label: "Status checks",
      state: "unknown",
      note: "There is no live milestone PR to summarize, so check posture is intentionally unknown instead of implied green."
    },
    {
      label: "Next PR expectation",
      state: "warn",
      note: "The first new milestone PR should appear when WP-03 contract schema work is published."
    }
  ],
  reviewTail: [
    {
      label: "WP-15 quality gate and docs convergence",
      state: "blocked",
      note: "Quality/docs convergence waits on WP-14A feature proof coverage."
    },
    {
      label: "WP-16 internal review",
      state: "blocked",
      note: "Internal review should not run before the quality and proof surfaces converge."
    },
    {
      label: "WP-17 external review",
      state: "blocked",
      note: "External review remains a later gate after internal review."
    },
    {
      label: "WP-18 remediation",
      state: "blocked",
      note: "Accepted findings cannot be fixed until reviews exist."
    },
    {
      label: "WP-19 planning handoff",
      state: "blocked",
      note: "Next-milestone handoff waits for review disposition."
    },
    {
      label: "WP-20 ceremony",
      state: "blocked",
      note: "Release ceremony remains gated on planning handoff and final release-truth alignment."
    }
  ],
  lanes: [
    {
      id: "lane-readiness",
      title: "Readiness and schema foundations",
      status: "active",
      wps: ["WP-01", "WP-02", "WP-03", "WP-04", "WP-05", "WP-06", "WP-07"],
      purpose: "Promote the package, inherit citizen-state authority, and land the contract, bid, evaluation, transition, and lifecycle substrate."
    },
    {
      id: "lane-market",
      title: "Counterparty and market mechanics",
      status: "blocked",
      wps: ["WP-08", "WP-09", "WP-10", "WP-11", "WP-12", "WP-13"],
      purpose: "Bound counterparties, delegation, resource stewardship, fixtures, runner output, and reviewer-facing summaries."
    },
    {
      id: "lane-proof",
      title: "Demo and proof coverage",
      status: "blocked",
      wps: ["WP-14", "WP-14A"],
      purpose: "Prove the bounded contract-market path end to end and make feature-proof coverage explicit before review convergence."
    },
    {
      id: "lane-release-tail",
      title: "Review and release tail",
      status: "blocked",
      wps: ["WP-15", "WP-16", "WP-17", "WP-18", "WP-19", "WP-20"],
      purpose: "Converge docs, perform internal and external review, fix accepted findings, hand off the next milestone, and close cleanly."
    }
  ],
  workPackages: [
    {
      id: "WP-01",
      issue: "#2420",
      title: "Promote v0.90.4 milestone package",
      queue: "docs",
      status: "complete",
      validation: "docs/package",
      checks: "landed",
      action: "closed",
      evidence: "Tracked package promoted, issue wave opened, and card authoring normalized."
    },
    {
      id: "WP-02",
      issue: "#2421",
      title: "Economics inheritance and authority audit",
      queue: "docs",
      status: "complete",
      validation: "docs/audit",
      checks: "landed",
      action: "closed",
      evidence: "Authority inheritance and contract-market dependency audit landed."
    },
    {
      id: "WP-03",
      issue: "#2422",
      title: "Contract schema",
      queue: "runtime",
      status: "active",
      validation: "schema/examples/negative fixtures",
      checks: "no PR yet",
      action: "execute next",
      evidence: "Open issue and current edge; canonical contract artifact still needs to land."
    },
    {
      id: "WP-04",
      issue: "#2423",
      title: "Bid schema",
      queue: "runtime",
      status: "blocked",
      validation: "schema/fixtures",
      checks: "blocked by WP-03",
      action: "wait on contract schema",
      evidence: "Bid artifact should not widen before the parent contract schema is accepted."
    },
    {
      id: "WP-05",
      issue: "#2424",
      title: "Evaluation and selection model",
      queue: "runtime",
      status: "blocked",
      validation: "selection tests",
      checks: "blocked by WP-03/WP-04",
      action: "wait on schema lane",
      evidence: "Selection logic should stay blocked until contract and bid artifacts exist."
    },
    {
      id: "WP-06",
      issue: "#2425",
      title: "Transition authority model",
      queue: "runtime",
      status: "blocked",
      validation: "authority tests",
      checks: "blocked by WP-02-WP-05",
      action: "wait on readiness lane",
      evidence: "Transition authority depends on inherited authority plus schema acceptance."
    },
    {
      id: "WP-07",
      issue: "#2426",
      title: "Contract lifecycle state",
      queue: "runtime",
      status: "blocked",
      validation: "state-machine fixtures",
      checks: "blocked by WP-06",
      action: "wait on transition authority",
      evidence: "Lifecycle state should not land before transition authority exists."
    },
    {
      id: "WP-08",
      issue: "#2427",
      title: "External counterparty model",
      queue: "runtime",
      status: "blocked",
      validation: "denial cases",
      checks: "blocked by WP-02/WP-06",
      action: "wait on authority model",
      evidence: "Counterparty participation remains blocked on authority and lifecycle semantics."
    },
    {
      id: "WP-09",
      issue: "#2428",
      title: "Delegation and subcontract model",
      queue: "runtime",
      status: "blocked",
      validation: "trace-link tests",
      checks: "blocked by WP-03-WP-08",
      action: "wait on market mechanics",
      evidence: "Delegation should not widen before core contract and authority boundaries exist."
    },
    {
      id: "WP-10",
      issue: "#2429",
      title: "Resource stewardship bridge",
      queue: "docs",
      status: "blocked",
      validation: "boundary notes and fixture",
      checks: "blocked by WP-03-WP-09",
      action: "wait on contract market core",
      evidence: "Resource stewardship remains later and must not imply payment rails or tool authority."
    },
    {
      id: "WP-11",
      issue: "#2430",
      title: "Contract-market fixture set",
      queue: "runtime",
      status: "blocked",
      validation: "fixture packet",
      checks: "blocked by WP-10",
      action: "wait on resource bridge",
      evidence: "Canonical fixture packet depends on the earlier contract-market semantics."
    },
    {
      id: "WP-12",
      issue: "#2431",
      title: "Contract-market runner",
      queue: "runtime",
      status: "blocked",
      validation: "runner proof artifacts",
      checks: "blocked by WP-11",
      action: "wait on fixtures",
      evidence: "Runner proof remains later and should not be shown as landed."
    },
    {
      id: "WP-13",
      issue: "#2432",
      title: "Review summary shape",
      queue: "docs",
      status: "blocked",
      validation: "summary schema",
      checks: "blocked by WP-11/WP-12",
      action: "wait on runner outputs",
      evidence: "Reviewer-facing summary depends on fixture and runner output."
    },
    {
      id: "WP-14",
      issue: "#2433",
      title: "Bounded contract-market demo and negative cases",
      queue: "demo",
      status: "blocked",
      validation: "demo and negative packet",
      checks: "blocked by WP-06-WP-13",
      action: "wait on implementation lane",
      evidence: "The bounded demo remains unstarted until the market substrate lands."
    },
    {
      id: "WP-14A",
      issue: "#2434",
      title: "Demo matrix and feature proof demos",
      queue: "demo",
      status: "blocked",
      validation: "demo matrix",
      checks: "blocked by WP-03-WP-14",
      action: "preserve before review",
      evidence: "Feature-proof coverage must remain explicit before review convergence."
    },
    {
      id: "WP-15",
      issue: "#2435",
      title: "Quality gate, docs, and review convergence",
      queue: "docs",
      status: "blocked",
      validation: "quality/docs",
      checks: "blocked by WP-14A",
      action: "hold",
      evidence: "Docs/review convergence should not be green before proof coverage exists."
    },
    {
      id: "WP-16",
      issue: "#2436",
      title: "Internal review",
      queue: "review",
      status: "blocked",
      validation: "review",
      checks: "blocked by WP-15",
      action: "hold",
      evidence: "Internal review is intentionally later and still blocked."
    },
    {
      id: "WP-17",
      issue: "#2437",
      title: "External review",
      queue: "review",
      status: "blocked",
      validation: "review",
      checks: "blocked by WP-16",
      action: "hold",
      evidence: "External review follows the internal review gate."
    },
    {
      id: "WP-18",
      issue: "#2438",
      title: "Review findings remediation",
      queue: "review",
      status: "blocked",
      validation: "review",
      checks: "blocked by WP-16/WP-17",
      action: "hold",
      evidence: "Accepted findings remediation remains blocked until reviews exist."
    },
    {
      id: "WP-19",
      issue: "#2439",
      title: "Next milestone planning handoff",
      queue: "docs",
      status: "blocked",
      validation: "docs",
      checks: "blocked by WP-18",
      action: "hold",
      evidence: "Handoff remains release-tail work after findings disposition."
    },
    {
      id: "WP-20",
      issue: "#2440",
      title: "Release ceremony",
      queue: "release",
      status: "blocked",
      validation: "release",
      checks: "blocked by WP-19",
      action: "hold",
      evidence: "Ceremony remains the last milestone step and is not currently green."
    }
  ],
  validationProfiles: [
    {
      label: "Dashboard and docs/static tooling",
      status: "active",
      profile: "Focused local validation plus static integrity scan is acceptable for narrow docs and static-tooling changes.",
      command: "bash adl/tools/test_milestone_dashboard.sh"
    },
    {
      label: "Runtime, schema, and authority work",
      status: "blocked",
      profile: "Contract-market implementation work still requires issue-specific schema, runtime, fixture, and negative-test proof rather than the dashboard smoke test.",
      command: "issue-specific cargo/tests/fixtures"
    },
    {
      label: "Release tail and ceremony",
      status: "blocked",
      profile: "No dashboard state can approve review or release; human ceremony and release-truth evidence remain required.",
      command: "WP-20 release closeout only"
    }
  ],
  releaseBlockers: [
    "The contract schema and bid schema are not both landed yet.",
    "Evaluation, transition authority, lifecycle, counterparty, delegation, and resource-stewardship proof remain open.",
    "The canonical fixture packet, runner, reviewer summary, bounded demo, and feature-proof coverage lane remain open.",
    "Internal review, external review, accepted-finding remediation, planning handoff, and ceremony remain open.",
    "No payment, settlement, or governed-tool execution claims are releasable in v0.90.4."
  ],
  deferredFindings: [
    {
      label: "Payment rails and settlement",
      state: "guarded",
      note: "Lightning, x402, stablecoins, banking, invoicing, and settlement remain explicitly out of scope for v0.90.4."
    },
    {
      label: "Governed tool execution",
      state: "guarded",
      note: "Contracts may describe tool requirements only as constraints; v0.90.5 owns UTS, ACC, tool registry binding, executor authority, denial records, and model testing."
    },
    {
      label: "Full inter-polis economics",
      state: "guarded",
      note: "v0.90.4 is a bounded contract-market substrate, not a full economic civilization milestone."
    },
    {
      label: "Production counterparty verification",
      state: "guarded",
      note: "KYC, billing, tax, legal, and production trust onboarding remain later-scope work."
    }
  ]
};
