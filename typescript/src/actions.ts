import chalk from 'chalk';
import gradient from 'gradient-string';
import open from 'open';
import { CONFIG } from './config.js';
import { animateText, createAnimatedSpinner } from './utils.js';

export const actionHandlers = {
  email: async (): Promise<void> => {
    const spinner = await createAnimatedSpinner('Opening mail client...');
    await open(CONFIG.urls.email);
    spinner.success({ text: gradient.passion('📧 Email client opened!') });
    await animateText(chalk.green('Looking forward to hearing from you!'));
  },

  viewResume: async (): Promise<void> => {
    const spinner = await createAnimatedSpinner('Preparing to open resume...');
    try {
      await open(CONFIG.urls.resume);
      spinner.success({ text: chalk.green('Resume opened in your browser! 🎉') });
      await animateText(chalk.gray('Tip: You can download it directly'));
    } catch (err) {
      spinner.error({ text: chalk.red('Failed to open resume 😢') });
      console.error(chalk.red('Error:'), (err as Error).message);
      throw err;
    }
  },

  viewPortfolio: async (): Promise<void> => {
    const spinner = await createAnimatedSpinner('Loading portfolio...');
    await open(CONFIG.urls.portfolio);
    spinner.success({ text: gradient.teen('🌐 Portfolio opened!') });
    await animateText(chalk.green('Hope you enjoy exploring my work!'));
  },

  viewGitHub: async (): Promise<void> => {
    const spinner = await createAnimatedSpinner('Opening GitHub...');
    await open(CONFIG.urls.github);
    spinner.success({ text: gradient.atlas('💻 GitHub profile opened!') });
    await animateText(chalk.green('Check out my latest projects!'));
  },

  viewLinkedIn: async (): Promise<void> => {
    const spinner = await createAnimatedSpinner('Opening LinkedIn...');
    await open(CONFIG.urls.linkedin);
    spinner.success({ text: gradient.morning('💼 LinkedIn profile opened!') });
    await animateText(chalk.green('Let\'s connect professionally!'));
  },

  viewTwitter: async (): Promise<void> => {
    if (CONFIG.urls.twitter) {
      const spinner = await createAnimatedSpinner('Opening Twitter...');
      await open(CONFIG.urls.twitter);
      spinner.success({ text: gradient.cristal('🐦 Twitter profile opened!') });
      await animateText(chalk.green('Follow me for tech updates!'));
    }
  },
};
