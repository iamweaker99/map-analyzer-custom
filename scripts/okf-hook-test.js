const assert = require("node:assert/strict");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const repositoryRoot = path.resolve(__dirname, "..");
const hook = path.join(repositoryRoot, ".codex", "hooks", "user-prompt-submit.js");
const { runHook } = require(hook);

function run(prompt, cwd = repositoryRoot) {
  return spawnSync(process.execPath, [hook], {
    cwd: repositoryRoot,
    input: JSON.stringify({ prompt, cwd }),
    encoding: "utf8",
    windowsHide: true,
  });
}

function context(result) {
  assert.equal(result.status, 0, result.stderr);
  return result.stdout ? JSON.parse(result.stdout).hookSpecificOutput?.additionalContext ?? "" : "";
}

const semantic = context(run("Why did we choose the forward-density window this way?"));
assert.match(semantic, /decision\.forward-density-window/);
const ordinary = run("Where is StreamProfile rendered?");
assert.equal(ordinary.status, 0, ordinary.stderr);
assert.equal(ordinary.stdout, "");
const sourceOnly = context(run("Does a stream at the end of the map get counted?"));
assert.match(sourceOnly, /coverage: source_only/);
assert.match(sourceOnly, /reference\.github-issue-6/);
const discussion = run("read this file: Keep_LOCAL\\OKF_Routing Stage 2 end-to-end acceptance verification.md and tell me what you would do next. Be concise.\n\nDon't execute anything yet, we are in discussion");
assert.equal(discussion.status, 0, discussion.stderr);
assert.equal(discussion.stdout, "");
const failure = run("Why did we choose the forward-density window this way?", path.join(repositoryRoot, "missing-hook-cwd"));
assert.equal(failure.status, 0, failure.stderr);
assert.equal(failure.stdout, "");

const validInput = JSON.stringify({ prompt: "test" });
const successful = runHook(validInput, {
  spawnSync: (command) => command === "git"
    ? { status: 0, stdout: `${repositoryRoot}\n` }
    : { status: 0, stdout: "OKF CONTEXT\ncoverage: semantic\nOKF_CONTEXT_OBJECT decision.forward-density-window" },
});
assert.match(successful, /decision\.forward-density-window/);
for (const childResult of [
  { status: 1, stdout: "" },
  { status: null, stdout: "", error: new Error("spawn failed") },
]) {
  assert.equal(runHook(validInput, {
    spawnSync: (command) => command === "git"
      ? { status: 0, stdout: `${repositoryRoot}\n` }
      : childResult,
  }), "");
}
assert.equal(runHook("not json"), "");
assert.equal(runHook("{}"), "");
assert.equal(runHook(validInput, { spawnSync: () => { throw new Error("unexpected"); } }), "");
console.log("OKF hook tests passed");
