import gradient from 'gradient-string';
import inquirer from 'inquirer';

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
			{ name: `📧  ${gradient.passion('Send an Email')}`, value: 'email' },
			{ name: `📥  ${gradient.morning('View Resume')}`, value: 'viewResume' },
			{ name: `🌐  ${gradient.teen('Visit Portfolio')}`, value: 'viewPortfolio' },
			{ name: `💻  ${gradient.atlas('View GitHub')}`, value: 'viewGitHub' },
			{ name: `💼  ${gradient.fruit('View LinkedIn')}`, value: 'viewLinkedIn' },
			{ name: `🐦  ${gradient.cristal('View Twitter')}`, value: 'viewTwitter' },
			{ name: gradient.rainbow('🚪  Exit'), value: 'quit' },
		] as MenuChoice[],
	},
];

export const promptUser = async (): Promise<{ action: string }> => {
	return inquirer.prompt(getMenuOptions());
};
