import gradient from 'gradient-string';
import inquirer from 'inquirer';
import { CONFIG, isConfiguredUrl } from './config.js';

export interface MenuChoice {
	name: string;
	value: string;
}

export const getMenuOptions = () => [
	{
		type: 'list',
		name: 'action',
		message: gradient.cristal('What would you like to do?'),
		choices: [
			isConfiguredUrl(CONFIG.urls.email)
				? { name: `📧  ${gradient.passion('Send an Email')}`, value: 'email' }
				: null,
			isConfiguredUrl(CONFIG.urls.resume)
				? { name: `📥  ${gradient.morning('View Resume')}`, value: 'viewResume' }
				: null,
			isConfiguredUrl(CONFIG.urls.portfolio)
				? { name: `🌐  ${gradient.teen('Visit Portfolio')}`, value: 'viewPortfolio' }
				: null,
			isConfiguredUrl(CONFIG.urls.github)
				? { name: `💻  ${gradient.atlas('View GitHub')}`, value: 'viewGitHub' }
				: null,
			isConfiguredUrl(CONFIG.urls.linkedin)
				? { name: `💼  ${gradient.fruit('View LinkedIn')}`, value: 'viewLinkedIn' }
				: null,
			isConfiguredUrl(CONFIG.urls.twitter)
				? { name: `🐦  ${gradient.cristal('View Twitter')}`, value: 'viewTwitter' }
				: null,
			{ name: gradient.rainbow('🚪  Exit'), value: 'quit' },
		].filter((choice): choice is MenuChoice => Boolean(choice)),
	},
];

export const promptUser = async (): Promise<{ action: string }> => {
	return inquirer.prompt(getMenuOptions());
};
