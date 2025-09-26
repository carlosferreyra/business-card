#!/usr/bin/env bun

// Development utilities for the CLI business card
import chalk from 'chalk';
import { CONFIG } from './src/config.js';

const command = process.argv[2];

switch (command) {
	case 'config':
		console.log(chalk.cyan('\n📋 Current Configuration:'));
		console.log(chalk.gray('='.repeat(50)));
		console.log(chalk.yellow('Personal Info:'));
		console.log(`  Name: ${CONFIG.personalInfo.name}`);
		console.log(`  Title: ${CONFIG.personalInfo.title}`);
		console.log(`  Company: ${CONFIG.personalInfo.company}`);
		console.log(`  Location: ${CONFIG.personalInfo.location}`);
		console.log(`  Skills: ${CONFIG.personalInfo.skills.join(', ')}`);

		console.log(chalk.yellow('\nURLs:'));
		Object.entries(CONFIG.urls).forEach(([key, value]) => {
			console.log(`  ${key}: ${value}`);
		});

		console.log(chalk.yellow('\nTheme:'));
		console.log(`  Border Color: ${CONFIG.theme.borderColor}`);
		console.log(`  Background Color: ${CONFIG.theme.backgroundColor}`);
		console.log(`  Animation Speeds: ${JSON.stringify(CONFIG.theme.animationSpeed)}`);
		break;

	case 'run':
		console.log(chalk.green('🚀 Running CLI Business Card...\n'));
		await import('./src/index.js');
		break;

	default:
		console.log(chalk.cyan('\n🛠️  Development Utils for CLI Business Card'));
		console.log(chalk.gray('='.repeat(50)));
		console.log(chalk.yellow('Available commands:'));
		console.log('  bun dev-utils.ts config  - Show current configuration');
		console.log('  bun dev-utils.ts run     - Run the business card');
		console.log('  bun run dev              - Run with hot reload');
		console.log('  bun run build            - Build for production');
		console.log('  bun run start            - Run built version');
		console.log('\n💡 Tip: Edit src/config.ts to customize your business card');
}
