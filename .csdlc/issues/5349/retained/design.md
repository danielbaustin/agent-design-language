# #5349 Design: Provider And Governed-Tool Adapters

Status: preparation-only design. No product path is claimed and no product
implementation may begin until the executable dependency gate is `ready` at
the current `origin/main`.

## Decision

Implement one small adapter crate at `adl-v2/crates/adl-adapters` after the
required WP-06 engine ports and WP-07 record contracts exist on current
`origin/main`. Receipts are non-blocking audit evidence. The crate provides four bounded
adapter families behind predecessor-owned typed ports:

1. deterministic mock provider and governed-tool adapters;
2. an HTTPS client adapter with injected endpoint authority and strict limits;
3. a governed-tool adapter that consumes verified authorization and preserves
   denial without acquiring policy authority; and
4. a narrow compatibility adapter that translates approved incumbent
   envelopes without importing incumbent implementation.

Runtime v3 remains the execution and admission authority. The adapters perform
one admitted, bounded port operation; they do not schedule, retry, supervise,
select models, grant capabilities, evaluate policy, sign records, or control
lifecycle state. Provider expansion issue #5526 consumes the frozen adapter
interfaces later and is not absorbed into this issue.

## Dependency Gate

Product work may start when the required predecessor interfaces exist:

| Gate | Source truth | Blocking |
| --- | --- | --- |
| WP-06 #5340 | `adl-engine` provider/tool ports exist on `origin/main` | yes |
| WP-07 #5342 | `adl-records` trust/digest contracts exist on `origin/main` | yes |
| WP-08 #5341 | Runtime v3 integration seam exists on `origin/main` | no; later wiring observation |

WP-07 #5342 and Runtime v3 ingress #5591 are transitive execution inputs
through terminal #5341. The canonical v0.91.8 wave names WP-06 and WP-08 as
WP-09's direct dependencies; the older issue body naming WP-06 and WP-07 does
not override that checked-in order.

`.csdlc/prepared/issues/5349/dependency_gate.rb` checks source-interface
availability and claim collisions without network mutation. It never reads a
receipt, so receipt absence cannot block execution.

## Authority Boundary

| Owner | Owns | #5349 may do | #5349 must not do |
| --- | --- | --- | --- |
| WP-06 #5340 | typed provider/tool ports, bounded engine outcomes, retry and cancellation semantics | implement one call behind a terminal port contract and preserve its outcome | schedule, retry, reorder, join, resume, or manufacture success |
| WP-08 #5341 and Runtime v3 | canonical admission, execution authority, provider/tool invocation context, supervision and continuity | consume an admitted invocation context and return a typed outcome | bypass ingress, admit work, supervise, mutate runtime state, or reopen admission |
| WP-07 #5342 | record, signature, provenance and verification contracts | preserve verified identifiers and evidence | sign, re-sign, weaken verification, or rewrite history |
| Governance/Freedom Gate | capability, commitment and lawful-execution decisions | require a verified authorization envelope and preserve denial | decide policy, mint authority, widen scope, or reinterpret refusal |
| Provider child #5526 | vendor/model identities, capability matrices, bounded credentialed smoke and provider-specific normalization | expose the frozen generic adapter contract it consumes | claim vendor/model live readiness or absorb provider expansion |
| C-SDLC v2 | cards, claims, review, publication and closeout | retain issue-local lifecycle proof | grant product, review, merge, or closeout authority |

## Design-By-Contract Adapter Surface

Every adapter implements a predecessor-owned typed port with explicit
preconditions, postconditions, invariants, and stable error classes.

### Common preconditions

- accepted invocation context is canonical, versioned, bounded, and already
  admitted by Runtime v3;
- provider/tool identity and operation kind are explicit;
- deadlines, body/response limits, redirect policy, and cancellation are
  explicit, never process-global defaults;
- any secret is carried as `secrecy::SecretString`, never serializable or
  printable;
- governed-tool calls carry a verified authorization envelope whose subject,
  action, resource, scope, expiry, and request digest match the invocation;
- HTTP calls carry a runtime-issued endpoint permit. A URL alone is not
  network authority.

### Common postconditions

- exactly one bounded operation result is returned for one accepted call;
- request identity, idempotency key, provider/tool identity, normalized error,
  usage, and retained evidence are deterministic for identical observed input;
- cancellation, timeout, saturation, denial, unsupported capability,
  malformed response, rate limit, authentication failure, and unavailable
  provider remain distinct typed outcomes;
- no response body, header, error, trace, debug output, fixture, or evidence
  contains a credential or secret-derived value;
- adapter output cannot grant more authority than the accepted input.

### Mock adapter

The mock is a deterministic scripted port implementation, not a reduced live
provider. It matches canonical request predicates, consumes each scripted step
once, records stable observations, and fails on unexpected order, identity,
or input. It supports every positive and negative outcome required by the
matrix without clock, environment, filesystem, network, or credentials.

### HTTPS adapter

The HTTP adapter uses `reqwest` with Rustls and no default features. It accepts
only `https` endpoints from a validated runtime-issued permit, rejects URL
userinfo and fragments, disables redirects, applies explicit connect/request
deadlines and body limits, honors cancellation, and returns bounded bytes for
provider-specific normalization. It does not discover hosts, read proxy
environment variables, select credentials, or infer retryability from status
alone. Test transport uses `wiremock`; production code does not roll its own
HTTP client or TLS.

### Governed-tool adapter

The governed-tool adapter validates authorization-envelope binding before
dispatch, calls only the named typed tool port, and preserves refusal and
appeal/review evidence. It cannot mint a capability, alter commitment scope,
skip Freedom Gate, execute a shell directly, resolve arbitrary paths, or turn
denial into fallback execution.

### Compatibility adapter

The compatibility surface is an explicit, versioned translation allowlist.
It consumes only fixtures and contracts approved by #5337/#5336, rejects
unknown fields/versions and lossy mappings, and emits a compatibility
classification. It contains no incumbent ADL source, dynamic plugin loading,
reflection, shell execution, implicit provider aliases, or default routing.

## Preparation And Future Protected Paths

The preparation claim protects exactly:

- `.csdlc/issues/5349`
- `.csdlc/locks/5349.lock`
- `.csdlc/prepared/issues/5349`
- `.csdlc/evidence/5349`

No product path is claimed while dependencies are open. After the dependency
gate passes, a typed `csdlc-bind` amendment may add exactly:

- `adl-v2/crates/adl-adapters`

That future path is disjoint from `adl-language`, `adl-compiler`, `adl-engine`,
the planned records crate, `adl-runtime-v3-adapter`, Runtime v3 source trees,
the thin CLI, and provider child #5526 issue-local records. Shared workspace
manifests are excluded. If implementation needs another path, stop and
typed-replan instead of widening implicitly.

## COTS Inventory

Exact preparation pins use the current stable crate releases verified on
2026-07-21. Implementation must re-run `cargo info`/locked-tree review before
the first product commit and use exact lockfile resolutions.

| Class | Crate and version | Features/purpose | Boundary |
| --- | --- | --- | --- |
| production | `reqwest 0.13.4` | `default-features = false`, `rustls`, `json`, `stream`; maintained HTTPS client | no native TLS, cookies, proxy discovery, redirects, multipart, HTTP/3 or blocking client |
| production | `secrecy 0.10.3` | non-serializable/non-debuggable secret wrappers | secrets never enter records or adapter diagnostics |
| production | `url 2.5.8` | structural endpoint validation | URL validity is not endpoint authority |
| production | `serde 1.0.229` | typed wire envelopes only | deny unknown fields on adapter-owned wire types |
| production | `serde_json 1.0.151` | bounded JSON encoding/decoding | no arbitrary-depth or preserve-order features |
| production | `tokio 1.53.1` | minimal `rt`, `sync`, `time` and I/O features required by terminal port contracts | no process/fs/signal features without typed replan |
| dev | `wiremock 0.6.5` | maintained local HTTP mock server | test-only; no credential or external-network proof |

In-repo path dependencies are terminal #5340 engine ports, terminal #5341
Runtime adapter contracts, and terminal #5342 record/trust contracts. No
provider SDK, AWS SDK, shell/process crate, dynamic loading crate, policy
engine, database, or alternate HTTP/TLS stack is allowed.

## Source And Test Budgets

Budgets count sorted tracked Rust files with physical lines; generated output
and Cargo build artifacts are excluded.

| Surface | Hard budget | Additional rule |
| --- | ---: | --- |
| production Rust under `src/` | 1,500 lines | no production module over 350 lines |
| Rust tests under `tests/` | 2,500 lines | at least 30 explicit tests; no inline test modules |
| direct production COTS | 6 | only the six declared crates and terminal path dependencies |
| direct dev COTS | 1 | only `wiremock` |
| credentialed/live tests | 0 required for parent acceptance | absence never becomes a live-provider claim |

A breach stops work for typed replanning and exact review. Tests, negative
proof, secret hygiene, and denial semantics cannot be removed to make a budget
pass.

## Validation-Time Budget

All Rust build output uses `/Volumes/FastWork/adl-5349`. The planning profile
sets a 7,200-second hard ceiling. The required lanes allocate 3,600 seconds;
the remaining 3,600 seconds is bounded contingency for one clean rebuild or
one deterministic rerun after a fixed failure, not authority to skip a lane.

| PVF lane | Proof role | Budget |
| --- | --- | ---: |
| dependency and ownership gate | required interfaces and claim disjointness | 60 s |
| deterministic mock matrix | success plus every typed failure class | 300 s |
| HTTPS contract matrix | permit, TLS-only, limits, redirect, cancellation and normalization | 600 s |
| governed-tool policy matrix | authorization binding, denial, appeal evidence and no bypass | 600 s |
| compatibility matrix | approved translations, unknown/lossy rejection and stable classification | 300 s |
| secret/redaction and negative authority suite | no leakage, escalation, shell, AWS, Runtime v2 or direct runtime access | 480 s |
| complete all-target suite | full crate regression | 420 s |
| strict format and Clippy | warning-free all targets/features | 300 s |
| COTS, LoC, module, test and path inventory | exact budgets and scope | 240 s |
| exact-revision lifecycle truth | diff hygiene, typed doctor, review identity | 300 s |

No required acceptance lane may be skipped, ignored, degraded, fixture-only,
prose-only, pending, waived, or replaced by CI-only evidence.

## No-Deferral Acceptance Matrix

| Acceptance | Positive proof | Negative proof | Deferral policy |
| --- | --- | --- | --- |
| AC-1 dependency authority | required engine and record interfaces exist | missing required source interface fails; receipts are not consulted | none |
| AC-2 mock determinism | repeated canonical requests produce identical scripted observations | unexpected order/input/identity and exhausted scripts fail | none |
| AC-3 HTTPS bounds | permitted HTTPS request returns one bounded normalized outcome | HTTP, redirect, userinfo, oversized body, timeout, cancellation and malformed response fail distinctly | none |
| AC-4 governed tool | verified envelope invokes exactly one named tool and preserves evidence | mismatch, expiry, denial, scope widening and direct execution fail | none |
| AC-5 compatibility | approved versions translate losslessly with stable classification | unknown, lossy, ambiguous and alias-drift mappings fail | none |
| AC-6 error fidelity | all typed failures preserve class, retryability source and retained identity | adapter retry, fallback and manufactured success fail | none |
| AC-7 secret hygiene | approved secret injection succeeds without retention | canary absent from stdout, stderr, debug, error, trace, fixture and artifact surfaces | none |
| AC-8 scope/COTS/budgets | exact locked inventory and line/test counts pass | undeclared dependency, path growth, Runtime v2/AWS/provider SDK fails | none |
| AC-9 validation budget | all required lanes fit the 3,600-second allocation and 7,200-second hard ceiling | timeout, omitted lane, or unbounded rerun fails | none |
| AC-10 exact integration | all FastWork lanes, review, CI, merge and post-merge validation agree | stale review or red CI fails; receipt state is non-blocking | none |
| AC-11 live-claim boundary | parent records deterministic adapter proof only | no credential means no direct-provider/live-production claim; #5526 owns bounded live smoke | none; live proof is outside parent claim, not deferred parent acceptance |
| AC-12 rollback | explicit opt-in removal returns `adapter_unavailable` | silent incumbent, alternate-provider, raw-HTTP, or Runtime v2 fallback fails | none |

## No-Credential Live-Claim Gate

The parent issue requires no provider credential and performs no live provider
call during deterministic implementation or acceptance. A missing credential
is never reported as a passed or skipped parent test because no credentialed
lane belongs to the parent proof set. Live direct-provider evidence belongs to
#5526 and must identify the exact vendor, resolved model, capability, revision,
credential source class, and retained redacted outcome. OpenRouter, mocks, and
wire compatibility do not count as direct-provider proof. No AWS or Bedrock
lane is required or permitted.

## Rollback

The crate remains opt-in behind typed ports until later selector/cutover work.
Rollback removes the adapter registration and returns the typed
`adapter_unavailable` outcome; it never falls back silently to incumbent ADL,
an alternate provider, an ungoverned tool, raw HTTP, or Runtime v2. Mock and
compatibility registrations are explicit and cannot become production defaults.

## Stop Conditions

- a required WP-06 engine-port or WP-07 record-contract source interface is
  absent from current `origin/main` when product work would start;
- a live or recovered claim overlaps the issue-local preparation paths or
  future `adl-v2/crates/adl-adapters` path;
- terminal port contracts contradict this design and typed replanning has not
  completed;
- implementation requires a shared manifest, sibling owner path, Runtime v2,
  Runtime v3 internal, C-SDLC product, provider SDK, AWS, shell/process, policy
  authority, credential file scan, or additional network stack;
- any required deterministic, denial, secret, error, COTS, budget,
  exact-revision, review, CI, post-merge or closeout proof is absent or weak;
- source, module, test, COTS, 3,600-second lane allocation, or 7,200-second
  hard validation ceiling cannot be met without weakening functionality or proof.
