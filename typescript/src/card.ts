import boxen from 'boxen';
import chalk from 'chalk';
import gradient from 'gradient-string';
import { CONFIG, getDisplayUrl, getSocialTag, isConfiguredUrl } from './config.js';
import { sleep } from './utils.js';

export const profileCard = async (): Promise<void> => {
	const cardData = {
		name: gradient.pastel(CONFIG.personalInfo.name),
		title: chalk.white(CONFIG.personalInfo.title),
		company: CONFIG.personalInfo.company
			? `${chalk.white('Working at')} ${gradient.morning(CONFIG.personalInfo.company)}`
			: '',
		location: `📍 ${chalk.gray(CONFIG.personalInfo.location)}`,
		github: `${chalk.white('{')} ${chalk.green(getDisplayUrl(CONFIG.urls.github))} ${chalk.white(
			'}'
		)}${getSocialTag(CONFIG.urls.github) ? ` ${chalk.gray(getSocialTag(CONFIG.urls.github))}` : ''}`,
		linkedin: `${chalk.white('{')} ${chalk.blue(getDisplayUrl(CONFIG.urls.linkedin))} ${chalk.white(
			'}'
		)}${getSocialTag(CONFIG.urls.linkedin) ? ` ${chalk.gray(getSocialTag(CONFIG.urls.linkedin))}` : ''}`,
		twitter: isConfiguredUrl(CONFIG.urls.twitter)
			? `${chalk.white('{')} ${chalk.cyan(getDisplayUrl(CONFIG.urls.twitter || ''))} ${chalk.white(
					'}'
				)}${getSocialTag(CONFIG.urls.twitter || '') ? ` ${chalk.gray(getSocialTag(CONFIG.urls.twitter || ''))}` : ''}`
			: '',
		web: `${chalk.white('{')} ${chalk.cyan(getDisplayUrl(CONFIG.urls.portfolio))} ${chalk.white('}')}`,
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
		isConfiguredUrl(CONFIG.urls.github) && `📦 GitHub:    ${cardData.github}`,
		isConfiguredUrl(CONFIG.urls.linkedin) && `💼 LinkedIn:  ${cardData.linkedin}`,
		cardData.twitter && `🐦 Twitter:   ${cardData.twitter}`,
		isConfiguredUrl(CONFIG.urls.portfolio) && `🌐 Website:   ${cardData.web}`,
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
