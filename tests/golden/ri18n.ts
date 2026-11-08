// module path: com/example/ri18n

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

function formatTemplate(template: string, ...args: unknown[]): string {
	let i = 0;
	return template.replace(/\{\}/g, () => String(args[i++]));
}

export function translateWithArgs(iso639Code: string, translationKey: string, ...args: unknown[]): string | null {
	const translationKeyTemplate = translate(iso639Code, translationKey);
	if (translationKeyTemplate == null || translationKeyTemplate === "") {
		return null;
	}
	return formatTemplate(translationKeyTemplate, ...args);
}

export function translate(iso639Code: string, translationKey: string): string | null {
	if (iso639Code === "ar") {
		return matchEntriesAr(translationKey);
	}
	if (iso639Code === "de") {
		return matchEntriesDe(translationKey);
	}
	if (iso639Code === "en") {
		return matchEntriesEn(translationKey);
	}
	if (iso639Code === "es") {
		return matchEntriesEs(translationKey);
	}
	if (iso639Code === "ja") {
		return matchEntriesJa(translationKey);
	}
	if (iso639Code === "zh") {
		return matchEntriesZh(translationKey);
	}
	return null;
}

function matchEntriesAr(translationKey: string): string | null {
	switch (translationKey) {
		case Keys.ANIMAL_CAT_DESC:
			return "هذا نمر، اسمه {}، عمره {} سنة، ووزنه {} كيلوغرام";
		case Keys.ANIMAL_CAT_NAME:
			return "نمر";
		case Keys.ANIMAL_CAT_NUMBER_PLURAL:
			return "هناك {} نمور في المجموع";
		case Keys.ANIMAL_CAT_NUMBER_SINGLE:
			return "هناك {} نمر";
		case Keys.HUMAN_MAN_NAME:
			return "بوب";
		case Keys.HUMAN_WOMAN_NAME:
			return "أليس";
		default:
			return null;
	}
}

function matchEntriesDe(translationKey: string): string | null {
	switch (translationKey) {
		case Keys.ANIMAL_CAT_DESC:
			return "Dies ist ein Tiger, sein Name ist {}, er ist {} Jahre alt und wiegt {} Kilogramm";
		case Keys.ANIMAL_CAT_NAME:
			return "Tiger";
		case Keys.ANIMAL_CAT_NUMBER_PLURAL:
			return "Es gibt insgesamt {} Tiger";
		case Keys.ANIMAL_CAT_NUMBER_SINGLE:
			return "Es gibt {} Tiger";
		case Keys.HUMAN_MAN_NAME:
			return "Bob";
		case Keys.HUMAN_WOMAN_NAME:
			return "Alice";
		default:
			return null;
	}
}

function matchEntriesEn(translationKey: string): string | null {
	switch (translationKey) {
		case Keys.ANIMAL_CAT_DESC:
			return "This is a tiger, his name is {}, age is {} and weight is {}";
		case Keys.ANIMAL_CAT_NAME:
			return "Tiger";
		case Keys.ANIMAL_CAT_NUMBER_PLURAL:
			return "There are {} tigers";
		case Keys.ANIMAL_CAT_NUMBER_SINGLE:
			return "There is {} tiger";
		case Keys.HUMAN_MAN_NAME:
			return "Bob";
		case Keys.HUMAN_WOMAN_NAME:
			return "Alice";
		default:
			return null;
	}
}

function matchEntriesEs(translationKey: string): string | null {
	switch (translationKey) {
		case Keys.ANIMAL_CAT_DESC:
			return "Este es un tigre, se llama {}, tiene {} años y pesa {} kilos";
		case Keys.ANIMAL_CAT_NAME:
			return "Tigre";
		case Keys.ANIMAL_CAT_NUMBER_PLURAL:
			return "Hay {} tigres";
		case Keys.ANIMAL_CAT_NUMBER_SINGLE:
			return "Hay {} tigre";
		case Keys.HUMAN_MAN_NAME:
			return "Roberto";
		case Keys.HUMAN_WOMAN_NAME:
			return "Alicia";
		default:
			return null;
	}
}

function matchEntriesJa(translationKey: string): string | null {
	switch (translationKey) {
		case Keys.ANIMAL_CAT_DESC:
			return "これは虎です。名前は{}、年齢は{}歳、体重は{}キロです";
		case Keys.ANIMAL_CAT_NAME:
			return "タイガー";
		case Keys.ANIMAL_CAT_NUMBER_PLURAL:
			return "虎が{}頭います";
		case Keys.ANIMAL_CAT_NUMBER_SINGLE:
			return "虎が{}頭います";
		case Keys.HUMAN_MAN_NAME:
			return "ボブ";
		case Keys.HUMAN_WOMAN_NAME:
			return "アリス";
		default:
			return null;
	}
}

function matchEntriesZh(translationKey: string): string | null {
	switch (translationKey) {
		case Keys.ANIMAL_CAT_DESC:
			return "这是一只老虎，它的名字叫{}，年龄{}岁，体重{}公斤";
		case Keys.ANIMAL_CAT_NAME:
			return "老虎";
		case Keys.ANIMAL_CAT_NUMBER_PLURAL:
			return "共有{}只老虎";
		case Keys.ANIMAL_CAT_NUMBER_SINGLE:
			return "有{}只老虎";
		case Keys.HUMAN_MAN_NAME:
			return "鲍勃";
		case Keys.HUMAN_WOMAN_NAME:
			return "爱丽丝";
		default:
			return null;
	}
}
