export interface SupportLanguage {
	cnName: string;
	enName: string;
	iso639Code: string;
	nativeName: string;
}

export function listSupportLanguage(): Array<SupportLanguage> {
	return [
		{ iso639Code: "ar", cnName: "阿拉伯语", enName: "Arabic", nativeName: "العربية" },
		{ iso639Code: "de", cnName: "德语", enName: "German", nativeName: "Deutsch" },
		{ iso639Code: "en", cnName: "英语", enName: "English", nativeName: "English" },
		{ iso639Code: "es", cnName: "西班牙语", enName: "Spanish", nativeName: "español" },
		{ iso639Code: "ja", cnName: "日语", enName: "Japanese", nativeName: "日本語" },
		{ iso639Code: "zh", cnName: "中文", enName: "Chinese", nativeName: "中文" },
	];
}

export const Iso639Codes = {
	AR: "ar",
	DE: "de",
	EN: "en",
	ES: "es",
	JA: "ja",
	ZH: "zh",
} as const;

export const Keys = {
	ANIMAL_CAT_DESC: "animal.cat.desc",
	ANIMAL_CAT_NAME: "animal.cat.name",
	ANIMAL_CAT_NUMBER_PLURAL: "animal.cat.number.plural",
	ANIMAL_CAT_NUMBER_SINGLE: "animal.cat.number.single",
	HUMAN_MAN_NAME: "human.man.name",
	HUMAN_WOMAN_NAME: "human.woman.name",
} as const;
