import { readFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import type { AppConfig } from './types.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Load configuration from the central config.json file
function loadConfig(): AppConfig {
	const possiblePaths = [
		join(__dirname, 'config.json'), // Bundled with dist (production)
		join(__dirname, '../config.json'), // One level up from dist
		join(__dirname, '../../config.json'), // From src in development
		join(__dirname, '../../../config.json'), // From nested src structure
	];

	for (const configPath of possiblePaths) {
		try {
			const configData = readFileSync(configPath, 'utf-8');
			return JSON.parse(configData) as AppConfig;
		} catch (error) {
			// Continue to next path
			continue;
		}
	}

	throw new Error(
		'Failed to load configuration file. Please ensure config.json exists.\n' +
			`Tried paths: ${possiblePaths.join(', ')}`
	);
}

export const CONFIG: AppConfig = loadConfig();

export function normalizeConfigUrl(url?: string): string {
	return (url ?? '').trim();
}

export function isConfiguredUrl(url?: string): boolean {
	return normalizeConfigUrl(url).length > 0;
}

export function getDisplayUrl(url: string): string {
	const normalized = normalizeConfigUrl(url);
	if (!normalized) {
		return '';
	}

	try {
		const parsed = new URL(normalized);
		const path = parsed.pathname.replace(/\/$/, '');
		return `${parsed.host}${path}`;
	} catch {
		return normalized.replace(/^https?:\/\//, '').replace(/\/$/, '');
	}
}

export function getSocialTag(url: string): string {
	const normalized = normalizeConfigUrl(url);
	if (!normalized) {
		return '';
	}

	try {
		const parsed = new URL(normalized);
		const segments = parsed.pathname.split('/').filter(Boolean);
		if (!segments.length) {
			return '';
		}
		const handle = segments[segments.length - 1].replace(/^@/, '');
		return handle ? `@${handle}` : '';
	} catch {
		const segments = normalized.split('/').filter(Boolean);
		const handle = (segments[segments.length - 1] || '').replace(/^@/, '');
		return handle ? `@${handle}` : '';
	}
}
