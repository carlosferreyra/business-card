#!/usr/bin/env node

import chalk from 'chalk';
import gradient from 'gradient-string';
import { actionHandlers } from './actions.js';
import { welcomeBanner } from './banner.js';
import { profileCard } from './card.js';
import { promptUser } from './menu.js';
import { animateText } from './utils.js';

const main = async (): Promise<void> => {
	try {
		// Show welcome banner
		await welcomeBanner();

		// Display profile card
		await profileCard();

		// Show helpful tip
		console.log(
			gradient.passion('\n💡 Tip: Use ') +
				chalk.cyan('cmd/ctrl + click') +
				gradient.passion(' on links to open directly.\n')
		);

		// Main interaction loop
		while (true) {
			const { action } = await promptUser();

			if (action === 'quit') {
				await animateText(gradient.rainbow('\n👋 Thanks for stopping by! Have a great day!\n'));
				break;
			}

			// Execute the selected action
			if (action in actionHandlers) {
				await actionHandlers[action as keyof typeof actionHandlers]();
			}
		}
	} catch (error) {
		console.error(chalk.red('\n❌ An error occurred:'), (error as Error).message);
		process.exit(1);
	}
};

// Run the application
main().catch(console.error);
