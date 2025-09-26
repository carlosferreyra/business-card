import boxen from 'boxen';
import chalk from 'chalk';
import gradient from 'gradient-string';
import { CONFIG } from './config.js';
import { sleep } from './utils.js';

export const profileCard = async (): Promise<void> => {
	const cardData = {
		name: gradient.pastel(CONFIG.personalInfo.name),
		title: chalk.white(CONFIG.personalInfo.title),
		company: CONFIG.personalInfo.company
			? `${chalk.white('Working at')} ${gradient.morning(CONFIG.personalInfo.company)}`
			: '',
		location: `📍 ${chalk.gray(CONFIG.personalInfo.location)}`,
		github: `${chalk.white('{')} ${chalk.gray('github.com/')}${chalk.green(
			'carlosferreyra'
		)} ${chalk.white('}')}`,
		linkedin: `${chalk.white('{')} ${chalk.gray('linkedin.com/in/')}${chalk.blue(
			'carlosferreyra'
		)} ${chalk.white('}')}`,
		twitter: CONFIG.personalInfo.name.toLowerCase().includes('carlos')
			? `${chalk.white('{')} ${chalk.gray('twitter.com/')}${chalk.cyan(
					'carlosferreyra'
			  )} ${chalk.white('}')}`
			: '',
		web: `${chalk.white('{')} ${chalk.cyan(
			CONFIG.urls.portfolio.replace('https://', '')
		)} ${chalk.white('}')}`,
		npx: `${chalk.red('npx')} ${chalk.white('carlosferreyra')}`,
		skills: gradient.cristal(CONFIG.personalInfo.skills.join(' | ')),
	};

	const cardContent = [
		cardData.name,
		cardData.title,
		'',
		cardData.company && cardData.company,
		cardData.location,
		'',
		`⚡ Skills: ${cardData.skills}`,
		'',
		`📦 GitHub:    ${cardData.github}`,
		`💼 LinkedIn:  ${cardData.linkedin}`,
		cardData.twitter && `🐦 Twitter:   ${cardData.twitter}`,
		`🌐 Website:   ${cardData.web}`,
		'',
		`📇 Card:      ${cardData.npx}`,
		'',
		gradient.passion('🚀 Available for exciting opportunities and collaborations!'),
		gradient.cristal("💭 Let's connect and create something amazing together!"),
	]
		.filter(Boolean)
		.join('\n');

	const card = boxen(cardContent, {
		padding: 1,
		margin: 1,
		borderStyle: 'round',
		borderColor: CONFIG.theme.borderColor as any,
		float: 'center',
		backgroundColor: CONFIG.theme.backgroundColor,
		title: chalk.cyan.bold(`${CONFIG.personalInfo.name}'s Business Card`),
		titleAlignment: 'center',
	});

	for (const line of card.split('\n')) {
		console.log(line);
		await sleep(CONFIG.theme.animationSpeed.fast);
	}
};
