import figlet from 'figlet';
import gradient from 'gradient-string';
import { CONFIG } from './config.js';
import { animateText, createAnimatedSpinner, sleep } from './utils.js';

export const welcomeBanner = async (): Promise<void> => {
  console.clear();
  console.log('\n');

  const spinner = await createAnimatedSpinner('Initializing...', 300);
  spinner.success();

  return new Promise((resolve) => {
    figlet(CONFIG.personalInfo.name, {
      font: 'Big',
      horizontalLayout: 'default',
      verticalLayout: 'default',
    }, async (err, data) => {
      if (!err && data) {
        const lines = data.split('\n');
        for (const line of lines) {
          console.log(gradient.rainbow(line));
          await sleep(CONFIG.theme.animationSpeed.medium);
        }
        await animateText(
          `{ ${CONFIG.personalInfo.title} }`,
          CONFIG.theme.animationSpeed.slow
        );
      }
      resolve();
    });
  });
};
