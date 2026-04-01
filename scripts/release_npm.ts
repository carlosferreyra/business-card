// @ts-nocheck
import { mkdir } from 'node:fs/promises';
import { join } from 'node:path';

class PackageMetadata {
	constructor(
		public readonly name: string,
		public readonly version: string,
		public readonly description: string,
		public readonly repository: string,
		public readonly license: string
	) {}

	get repositoryUrl(): string {
		return this.repository.endsWith('.git') ? this.repository : `${this.repository}.git`;
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
		name: meta.name,
		version: meta.version,
		description: meta.description,
		license: meta.license,
		type: 'module',
		bin: { [meta.name]: `./bin/${meta.name}.cjs` },
		files: ['bin', 'README.md'],
		repository: { type: 'git', url: meta.repositoryUrl },
		publishConfig: { access: 'public' },
		engines: { node: '>=18' },
	}),

	cliWrapper: (meta: PackageMetadata) =>
		`
import { spawnSync } from "node:child_process";
import { platform } from "node:os";

const BIN_NAME = "${meta.name}";
const REPOSITORY = "${meta.repository}".replace(/\\/$/, "");
const VERSION = "${meta.version}";

function bootstrapBinary() {
  const tag = "v" + VERSION;
  const currentPlatform = platform();

  if (currentPlatform === "win32") {
    const url = \`\${REPOSITORY}/releases/download/\${tag}/\${BIN_NAME}-installer.ps1\`;
    spawnSync("powershell", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", "iwr -useb '" + url + "' | iex"], { stdio: "inherit" });
  } else {
    const url = \`\${REPOSITORY}/releases/download/\${tag}/\${BIN_NAME}-installer.sh\`;
    spawnSync("sh", ["-c", "curl -LsSf '" + url + "' | sh"], { stdio: "inherit" });
  }
}

const args = process.argv.slice(2);
const result = spawnSync(BIN_NAME, args, { stdio: "inherit", shell: platform() === "win32" });

if (result.error) {
  bootstrapBinary();
  const retry = spawnSync(BIN_NAME, args, { stdio: "inherit", shell: platform() === "win32" });
  process.exit(retry.status ?? 1);
}

process.exit(result.status ?? 0);
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

		console.log(`📦 Bundling ${meta.name}...`);

		const buildResult = await Bun.build({
			entrypoints: [entryPath],
			outdir: binDir,
			target: 'node',
			format: 'cjs',
			naming: `${meta.name}.cjs`,
			// Optimization flags
			minify: true,
			sourcemap: 'none',
			banner: '#!/usr/bin/env node',
			// Future-proofing: Set to true to generate a standalone binary (executables)
			// Note: compile: true usually requires target: "bun"
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

		console.log(`✅ Optimized NPM wrapper generated for ${meta.name} v${meta.version}`);
	} catch (err) {
		console.error('Error:', err instanceof Error ? err.message : err);
		process.exit(1);
	}
}

main();
