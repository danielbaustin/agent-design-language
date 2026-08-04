import { execFile } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname } from "node:path";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);
const [cli, project, url, tool, output] = process.argv.slice(2);

if (![cli, project, url, tool, output].every(Boolean)) {
  throw new Error("usage: capture_unity_mcp_image.mjs <cli> <project> <url> <tool> <output>");
}

const { stdout } = await execFileAsync(
  process.execPath,
  [
    cli,
    "run-tool",
    tool,
    "--path",
    project,
    "--url",
    url,
    "--input",
    "{}",
    "--raw",
  ],
  { maxBuffer: 16 * 1024 * 1024 },
);

const response = JSON.parse(stdout);
if (response.status !== "success" || !Array.isArray(response.content)) {
  throw new Error(`Unity-MCP ${tool} did not return a successful content response`);
}

const image = response.content.find(
  (item) => item?.type === "image" && item?.mimeType === "image/png" && item?.data,
);
if (!image) {
  throw new Error(`Unity-MCP ${tool} did not return a PNG image`);
}

await mkdir(dirname(output), { recursive: true });
await writeFile(output, Buffer.from(image.data, "base64"));
process.stdout.write(`${output}\n`);
