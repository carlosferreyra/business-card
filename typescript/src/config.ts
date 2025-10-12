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
