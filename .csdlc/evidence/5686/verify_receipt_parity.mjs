#!/usr/bin/env node

import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const commonDir = execFileSync(
  "git",
  ["rev-parse", "--path-format=absolute", "--git-common-dir"],
  { cwd: root, encoding: "utf8" },
).trim();

const trackedReceiptPath = join(
  root,
  ".csdlc/evidence/5686/5662-closeout-receipt.json",
);
const canonicalReceiptPath = join(commonDir, "csdlc-v2/closeout/5662.json");
const receipt = JSON.parse(readFileSync(trackedReceiptPath, "utf8"));
const projection = JSON.parse(
  readFileSync(join(root, ".csdlc/issues/5662/index.json"), "utf8"),
);
const retainedManifest = JSON.parse(
  readFileSync(
    join(root, ".csdlc/evidence/5686/retained-projection-sha256.json"),
    "utf8",
  ),
);

const canonicalize = (value) => {
  if (Array.isArray(value)) {
    return value.map(canonicalize);
  }
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalize(value[key])]),
    );
  }
  return value;
};
const canonicalJson = (value) => JSON.stringify(canonicalize(value));
const failures = [];

if (existsSync(canonicalReceiptPath)) {
  const canonicalReceipt = JSON.parse(readFileSync(canonicalReceiptPath, "utf8"));
  if (canonicalJson(receipt) !== canonicalJson(canonicalReceipt)) {
    failures.push("tracked receipt differs from canonical closeout receipt");
  }
}

if (canonicalJson(projection) !== canonicalJson(receipt.record)) {
  failures.push("projected issue record differs from receipt.record");
}

for (const [card, expected] of Object.entries(receipt.cards)) {
  const actual = JSON.parse(
    readFileSync(join(root, `.csdlc/issues/5662/cards/${card}.values.json`), "utf8"),
  );
  if (canonicalJson(actual) !== canonicalJson(expected)) {
    failures.push(`${card}.values.json differs from receipt.cards.${card}`);
  }
}

for (const [relativePath, expected] of Object.entries(receipt.authored_artifacts)) {
  const actual = readFileSync(join(root, relativePath), "utf8");
  if (actual !== expected) {
    failures.push(`${relativePath} differs from receipt.authored_artifacts`);
  }
}

const expectedProjectionPaths = Object.keys(retainedManifest.files).sort();
const enumerateFiles = (directory, prefix) =>
  readdirSync(join(root, directory), { withFileTypes: true }).flatMap((entry) => {
    const relativePath = `${prefix}/${entry.name}`;
    return entry.isDirectory()
      ? enumerateFiles(relativePath, relativePath)
      : [relativePath];
  });
const actualProjectionPaths = [
  ...enumerateFiles(".csdlc/issues/5662", ".csdlc/issues/5662"),
  ".csdlc/publication/5662.intent.json",
].sort();
if (canonicalJson(actualProjectionPaths) !== canonicalJson(expectedProjectionPaths)) {
  failures.push("projected path set differs from the two retained commits");
}

for (const [relativePath, expectedDigest] of Object.entries(
  retainedManifest.files,
)) {
  const actualDigest = createHash("sha256")
    .update(readFileSync(join(root, relativePath)))
    .digest("hex");
  if (actualDigest !== expectedDigest) {
    failures.push(`${relativePath} differs from retained projection manifest`);
  }
}

const result = {
  schema: "adl.csdlc_terminal_projection_parity.v1",
  issue: 5662,
  receipt_ref: receipt.receipt_ref,
  tracked_receipt: ".csdlc/evidence/5686/5662-closeout-receipt.json",
  canonical_receipt_compared: existsSync(canonicalReceiptPath),
  receipt_digest: receipt.record.digest,
  projection_digest: projection.digest,
  phase: projection.phase,
  generation: projection.generation,
  retained_projection_manifest:
    ".csdlc/evidence/5686/retained-projection-sha256.json",
  source_revisions: retainedManifest.source_revisions,
  expected_projection_path_count: expectedProjectionPaths.length,
  projected_path_count: actualProjectionPaths.length,
  parity: failures.length === 0,
  failures,
};

process.stdout.write(`${JSON.stringify(result)}\n`);
if (failures.length > 0) {
  process.exitCode = 1;
}
