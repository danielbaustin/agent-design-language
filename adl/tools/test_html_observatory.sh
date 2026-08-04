#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_JS="${ROOT_DIR}/demos/html-observatory/app.js"
CONFIG_JSON="${ROOT_DIR}/demos/html-observatory/runtime-v3.config.json"

node - <<'NODE' "${APP_JS}" "${CONFIG_JSON}"
const fs = require("fs");
const vm = require("vm");
const assert = require("assert");

const appPath = process.argv[2];
const configPath = process.argv[3];
const source = fs.readFileSync(appPath, "utf8");
const config = JSON.parse(fs.readFileSync(configPath, "utf8"));

(async () => {
const observatoryFeed = {
  schema: "adl.runtime_v3.observatory_feed.v2",
  runtime_instance_id: "runtime-v3-test",
  runtime_process_id: 12345,
  runtime_selection: "runtime_v3_explicit_opt_in",
  control: {
    port: 20997,
    read_endpoint: "/v1/observatory",
    websocket_endpoint: "/v1/observatory/ws",
    signed_command_endpoint: "/v1/control",
    signed_commands_required_for_mutation: true,
    bearer_token_required_for_read: false,
    login_required_for_mutation: true,
    browser_mutation_authority: true
  },
  health: {
    observability_ready: true,
    snapshot: {
      schema: "adl.runtime.control_snapshot.v1",
      revision: 1,
      topology_generation: 1,
      components: { runtime_api: "running" },
      restart_counts: {},
      queues: {},
      clock: { status: "authoritative" },
      continuity_head: { generation: 1, accepted_through: 1, topology_hash: "t", config_hash: "c", integrity: "verified" },
      lifecycle: "running",
      event_count: 1,
      observability_ready: true
    }
  },
  weather: {
    schema: "adl.runtime.weather_health.v1",
    resource_state: "healthy",
    shutdown_decision: "continue",
    gpu_proof_state: "unavailable_not_pass"
  },
  weather_freshness: {
    observed_at_unix_millis: 1785778500000,
    age_millis: 1,
    stale_after_millis: 30000,
    stale: false
  },
  agents: {
    total_count: 1,
    rendered_sample_count: 1,
    sample: [{ id: "agent-0001", label: "Shepherd", role: "runtime shepherd", state: "running", detail: "operator-addressable" }]
  },
  continuity: {},
  proof: {
    default_runtime_switch_authorized: false,
    runtime_v2_decommission_authorized: false,
    sidecar_required: false
  },
  events: [{ sequence: 1, component: "operator", event: "agent_ready", correlation_id: "test-1" }]
};

const readiness = {
  schema: "adl.runtime_v3.readiness.v1",
  ready: true,
  degraded_reasons: [],
  observability_ready: true,
  runtime_instance_id: "runtime-v3-test"
};

const calls = [];
const context = {
  console,
  URL,
  URLSearchParams,
  location: { search: "" },
  window: { location: { search: "" } },
  fetch: async (url, options = {}) => {
    calls.push({ url: String(url), options });
    if (String(url) === "https://localhost:20997/v1/observatory") {
      return { ok: true, status: 200, json: async () => observatoryFeed };
    }
    if (String(url) === "https://localhost:20997/v1/ready") {
      return { ok: true, status: 200, json: async () => readiness };
    }
    if (String(url) === "https://localhost:20997/v1/control") {
      const body = JSON.parse(String(options.body || "{}"));
      assert.equal(options.method, "POST");
      assert.equal(options.headers["Content-Type"], "application/json");
      assert.equal(body.schema, "adl.runtime.control_command.v1");
      return {
        ok: true,
        status: 200,
        json: async () => ({
          schema: "adl.runtime.control_response.v1",
          command_id: body.command_id,
          correlation_id: body.correlation_id,
          outcome: { snapshot: { lifecycle: "running" } }
        })
      };
    }
    return { ok: false, status: 404, json: async () => ({ code: "not_found" }) };
  },
  globalThis: {}
};
context.globalThis = context;
vm.runInNewContext(source, context);
const api = context.AdlHtmlObservatory;
api.applyRuntimeV3Config(config);

assert.equal(api.requestedRuntimeSelection(), "v3");
assert.equal(api.getQueryApiBase(), "https://localhost:20997");
assert.equal(api.getRuntimeV3Config().signed_command_endpoint, "/v1/control");

const eventCheck = await api.checkEventsEndpoint(api.getQueryApiBase());
assert.equal(eventCheck.schema, "adl.html_observatory.runtime_v3_event_check.v1");
assert.equal(eventCheck.events[0].event, "agent_ready");
assert.equal(api.normalizeEventEntries(eventCheck).length, 1);

const command = {
  schema: "adl.runtime.control_command.v1",
  runtime_instance_id: "runtime-v3-test",
  command_id: "operator-message-1",
  correlation_id: "operator-message-1",
  principal: "operator",
  action: { action: "snapshot" },
  signing_algorithm: "ed25519",
  signing_key_id: "operator-key",
  signature: "signed-fixture"
};
const response = await api.submitRuntimeV3SignedControlCommand(api.getQueryApiBase(), command);
assert.equal(response.schema, "adl.runtime.control_response.v1");
assert.equal(response.command_id, "operator-message-1");
assert(calls.some((call) => call.url === "https://localhost:20997/v1/control" && call.options.method === "POST"));

assert.throws(
  () => api.normalizeTrustedRuntimeV3ApiBase("https://operator:token@localhost:20997"),
  /trusted HTTPS localhost:20997/
);

await assert.rejects(
  () => api.submitRuntimeV3SignedControlCommand("https://localhost:20997", { schema: "wrong" }),
  /adl.runtime.control_command.v1/
);
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

echo "PASS: HTML Observatory Runtime v3 default, event check, and signed command POST contract"
