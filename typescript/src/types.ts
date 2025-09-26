export interface PersonalInfo {
	name: string;
	title: string;
	company?: string;
	location: string;
	skills: string[];
}

export interface URLs {
	email: string;
	resume: string;
	portfolio: string;
	github: string;
	linkedin: string;
	twitter?: string;
}

export interface ThemeConfig {
	borderColor: string;
	backgroundColor: string;
	animationSpeed: {
		fast: number;
		medium: number;
		slow: number;
	};
}

export interface AppConfig {
	personalInfo: PersonalInfo;
	urls: URLs;
	theme: ThemeConfig;
}
