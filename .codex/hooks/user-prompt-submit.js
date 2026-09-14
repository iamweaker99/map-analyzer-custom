const fs = require("node:fs");
const { spawnSync: defaultSpawnSync } = require("node:child_process");
const path = require("node:path");

function runHookUnsafe(rawInput, { spawnSync = defaultSpawnSync } = {}) {
  const input = JSON.parse(rawInput);
  if (!input || typeof input !== "object") return "";
  const prompt = typeof input.prompt === "string" ? input.prompt : "";
  const cwd = typeof input.cwd === "string" ? input.cwd : process.cwd();
  if (!prompt) return "";

  const root = spawnSync("git", ["rev-parse", "--show-toplevel"], {
    cwd, encoding: "utf8", timeout: 1000, windowsHide: true,
  });
  if (root.status !== 0 || !root.stdout) return "";

  const repositoryRoot = root.stdout.trim();
  const command = process.platform === "win32" ? "powershell.exe" : path.join(repositoryRoot, "scripts", "okf");
  const commandArgs = process.platform === "win32"
    ? ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", path.join(repositoryRoot, "scripts", "okf.ps1"), "context", "--query", prompt]
    : ["context", "--query", prompt];
  const result = spawnSync(command, commandArgs, {
    cwd: repositoryRoot, encoding: "utf8", timeout: 18000, windowsHide: true,
  });
  const output = result.status === 0 ? result.stdout.trim() : "";
  if (!output || output.includes("coverage: none")) return "";

  const response = JSON.stringify({
    hookSpecificOutput: {
      hookEventName: "UserPromptSubmit",
      additionalContext: `Retrieved project memory from OKF:\n${output}`,
    },
  });
  return response;
}

function runHook(rawInput, dependencies) {
  try {
    return runHookUnsafe(rawInput, dependencies);
  } catch {
    return "";
  }
}

if (require.main === module) {
  try {
    const output = runHookUnsafe(fs.readFileSync(0, "utf8"));
    if (output) process.stdout.write(output);
  } catch {
    // OKF is enrichment only; retrieval failures must never block a prompt.
  }
  process.exitCode = 0;
}

module.exports = { runHook };
