const { test } = require('node:test');
const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const { mkdtempSync, copyFileSync, readFileSync, rmSync } = require('node:fs');
const { tmpdir } = require('node:os');
const { join, resolve } = require('node:path');
const vm = require('node:vm');

const directory = mkdtempSync(join(tmpdir(), 'business-card-test-'));
let source;
try {
  copyFileSync(resolve('Cargo.toml'), join(directory, 'Cargo.toml'));
  execFileSync('bun', [resolve('scripts/release_npm.ts')], { cwd: directory });
  source = readFileSync(join(directory, '.release/npm/bin/carlosferreyra.cjs'), 'utf8');
} finally {
  rmSync(directory, { recursive: true, force: true });
}

function runWrapper({ platform = 'linux', exists = true, results = [] } = {}) {
  const calls = [];
  let output = '';
  let exit;
  const stop = {};
  const mocks = {
    'node:os': { platform: () => platform, homedir: () => '/mock-home' },
    'node:fs': { existsSync: () => exists },
    'node:path': require('node:path'),
    'node:child_process': { spawnSync: (...args) => { calls.push(args); return results.shift(); } },
  };
  try {
    vm.runInNewContext(source, {
      require: name => mocks[name],
      process: { argv: ['node', 'wrapper', '--version'], stderr: { write: text => { output += text; } }, exit: code => { exit = code; throw stop; } },
    });
  } catch (error) {
    if (error !== stop) throw error;
  }
  return { calls, output, exit };
}

test('launch error returns nonzero and explains failure', () => {
  const result = runWrapper({ results: [{ status: null, error: new Error('ENOENT') }] });
  assert.equal(result.exit, 1);
  assert.match(result.output, /Failed to launch binary.*ENOENT/);
});

test('Windows bootstrap fails before downloading', () => {
  const result = runWrapper({ platform: 'win32', exists: false });
  assert.equal(result.exit, 1);
  assert.equal(result.calls.length, 0);
  assert.match(result.output, /unsupported on win32/);
});

test('installer failure stops binary launch', () => {
  const result = runWrapper({ exists: false, results: [{ status: 22 }] });
  assert.equal(result.exit, 1);
  assert.equal(result.calls.length, 1);
  assert.deepEqual(Array.from(result.calls[0][1]).slice(0, 3), ['-o', 'pipefail', '-c']);
});

test('arguments and binary exit code are forwarded', () => {
  const result = runWrapper({ results: [{ status: 7 }] });
  assert.equal(result.exit, 7);
  assert.deepEqual(Array.from(result.calls[0][1]), ['--version']);
});
