// @ts-nocheck
import { mkdir } from 'node:fs/promises';
import { join } from 'node:path';

class PackageMetadata {
	constructor(
		public readonly rawName: string,
		public readonly version: string,
		public readonly description: string,
		public readonly repository: string,
		public readonly license: string
	) {}

	/**
	 * NPM and bin scripts must be lowercase to avoid "name cleaned" warnings.
	 */
	get safeName(): string {
		return this.rawName.toLowerCase().replace(/[^a-z0-9-_]/g, '');
	}

	/**
	 * Normalizes repository URL to the format NPM prefers.
	 */
	get repositoryUrl(): string {
		let url = this.repository;
		if (url.startsWith('https://github.com/')) {
			url = `git+${url}`;
		}
		return url.endsWith('.git') ? url : `${url}.git`;
	}

	static async fromCargo(): Promise<PackageMetadata> {
		const content = await Bun.file('Cargo.toml').text();
		const extract = (key: string) => {
			const match = content.match(new RegExp(`${key}\\s*=\\s*"([^"]+)"`));
			return match ? match[1] : '';
		};

		const name = extract('name');
		const version = extract('version');

		if (!name || !version) {
			throw new Error('Cargo.toml is missing name/version.');
		}

		return new PackageMetadata(
			name,
			version,
			extract('description') || 'Rust CLI wrapper',
			extract('repository'),
			extract('license') || 'MIT'
		);
	}
}

const Templates = {
	packageJson: (meta: PackageMetadata) => ({
		name: meta.safeName,
		version: meta.version,
		description: meta.description,
		license: meta.license,
		type: 'module',
		bin: { [meta.safeName]: `./bin/${meta.safeName}.cjs` },
		files: ['bin', 'README.md'],
		repository: { type: 'git', url: meta.repositoryUrl },
		homepage: meta.repository,
		publishConfig: { access: 'public' },
		engines: { node: '>=18' },
	}),

	cliWrapper: (meta: PackageMetadata) =>
		`
import { spawnSync } from "node:child_process";
import { platform, homedir } from "node:os";
import { join } from "node:path";
import { existsSync } from "node:fs";

const BIN_NAME = "${meta.rawName}";
const REPOSITORY = "${meta.repository}".replace(/\\/$/, "");

function bootstrapBinary() {
  const currentPlatform = platform();

  if (currentPlatform !== "darwin" && currentPlatform !== "linux") {
    process.stderr.write("Automatic installation is unsupported on " + currentPlatform + ".\\n");
    return false;
  }
  const url = \`\${REPOSITORY}/releases/latest/download/\${BIN_NAME}-installer.sh\`;
  const result = spawnSync("bash", ["-o", "pipefail", "-c", "curl -LsSf '" + url + "' | sh"], { stdio: ["inherit", 2, 2] });
  if (result.error || result.status !== 0) {
    process.stderr.write("Failed to install binary: " + (result.error?.message ?? result.status) + "\\n");
    return false;
  }
  return true;
}

const args = process.argv.slice(2);
const isWin = platform() === "win32";

// Define the absolute path where cargo-dist installs the binary.
// This prevents the wrapper from finding itself in the PATH (recursion).
const binPath = join(
  homedir(),
  ".cargo",
  "bin",
  isWin ? \`\${BIN_NAME}.exe\` : BIN_NAME
);

function hasUpdater() {
  if (!existsSync(binPath)) return false;
  const result = spawnSync(binPath, ["--version", "--__business-card-update-restarted"], { encoding: "utf8", timeout: 5000 });
  if (result.error || result.status !== 0) return false;
  const match = result.stdout.trim().match(/^carlosferreyra (\\d+)\\.(\\d+)\\.(\\d+)$/);
  if (!match) return false;
  const version = match.slice(1).map(Number);
  const minimum = [1, 2, 17];
  for (let i = 0; i < minimum.length; i++) {
    if (version[i] !== minimum[i]) return version[i] > minimum[i];
  }
  return true;
}

if (!hasUpdater()) {
  process.stderr.write("Installing the latest business card...\\n");
  if (!bootstrapBinary()) {
    if (!existsSync(binPath)) process.exit(1);
    process.stderr.write("Continuing with the installed card.\\n");
  }
}

// Always run from the absolute path to bypass the NPM shim
const result = spawnSync(binPath, args, {
  stdio: "inherit",
  shell: isWin
});

if (result.error) {
  process.stderr.write("Failed to launch binary: " + result.error.message + "\\n");
}
process.exit(result.status ?? 1);
`.trim(),
};

async function main() {
	try {
		const meta = await PackageMetadata.fromCargo();
		const outDir = '.release/npm';
		const srcDir = join(outDir, 'src');
		const binDir = join(outDir, 'bin');

		await mkdir(srcDir, { recursive: true });
		await mkdir(binDir, { recursive: true });

		// 1. Generate package.json
		await Bun.write(
			join(outDir, 'package.json'),
			JSON.stringify(Templates.packageJson(meta), null, 2)
		);

		// 2. Generate and Build CLI Wrapper
		const entryPath = join(srcDir, 'cli.ts');
		await Bun.write(entryPath, Templates.cliWrapper(meta));

		console.log(`📦 Bundling ${meta.safeName}...`);

		const buildResult = await Bun.build({
			entrypoints: [entryPath],
			outdir: binDir,
			target: 'node',
			format: 'cjs',
			naming: `${meta.safeName}.cjs`,
			minify: true,
			sourcemap: 'none',
			banner: '#!/usr/bin/env node',
			compile: false,
		});

		if (!buildResult.success) {
			console.error('Build failed:', buildResult.logs);
			process.exit(1);
		}

		// 3. Sync README
		const readme = Bun.file('README.md');
		if (await readme.exists()) {
			await Bun.write(join(outDir, 'README.md'), readme);
		}

		console.log(`✅ Optimized NPM wrapper generated: ${meta.safeName} v${meta.version}`);
	} catch (err) {
		console.error('Error:', err instanceof Error ? err.message : err);
		process.exit(1);
	}
}

main();
