import { createSpinner } from 'nanospinner';

export const sleep = (ms: number): Promise<void> => {
	return new Promise((resolve) => setTimeout(resolve, ms));
};

export const createAnimatedSpinner = async (text: string, duration = 500) => {
	const spinner = createSpinner(text).start();
	await sleep(duration);
	return spinner;
};

export const animateText = async (text: string, speed = 10): Promise<void> => {
	process.stdout.write('\n');
	for (const char of text) {
		process.stdout.write(char);
		await sleep(speed);
	}
	process.stdout.write('\n');
};
