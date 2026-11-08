package com.example.ri18n;

public class Ri18n {

	public static class SupportLanguage {
		public String cnName;
		public String enName;
		public String iso639Code;
		public String nativeName;

		SupportLanguage(String cnName, String enName, String iso639Code, String nativeName) {
			this.cnName = cnName;
			this.enName = enName;
			this.iso639Code = iso639Code;
			this.nativeName = nativeName;
		}

	}

	public static String getByLocale(String iso639Code, String translationKey, Object... args) {
		String translationKeyTemplate = getByLocale(iso639Code, translationKey);
		if (translationKeyTemplate == null || translationKeyTemplate.isEmpty()) {
			return null;
		}
		return translationKeyTemplate.formatted(args);
	}

	public static final java.util.ArrayList<SupportLanguage> LANGUAGE_LIST = new java.util.ArrayList<SupportLanguage>();
	public static class Iso639Codes {
		public static final String AR = "ar";
		public static final String DE = "de";
		public static final String EN = "en";
		public static final String ES = "es";
		public static final String JA = "ja";
		public static final String ZH = "zh";

	}

	static {
		LANGUAGE_LIST.add(new SupportLanguage("阿拉伯语", "Arabic", "ar", "العربية"));
		LANGUAGE_LIST.add(new SupportLanguage("德语", "German", "de", "Deutsch"));
		LANGUAGE_LIST.add(new SupportLanguage("英语", "English", "en", "English"));
		LANGUAGE_LIST.add(new SupportLanguage("西班牙语", "Spanish", "es", "español"));
		LANGUAGE_LIST.add(new SupportLanguage("日语", "Japanese", "ja", "日本語"));
		LANGUAGE_LIST.add(new SupportLanguage("中文", "Chinese", "zh", "中文"));
	}

	public static class Keys {
		public static final String ANIMAL_CAT_DESC = "animal.cat.desc";
		public static final String ANIMAL_CAT_NAME = "animal.cat.name";
		public static final String ANIMAL_CAT_NUMBER_PLURAL = "animal.cat.number.plural";
		public static final String ANIMAL_CAT_NUMBER_SINGLE = "animal.cat.number.single";
		public static final String HUMAN_MAN_NAME = "human.man.name";
		public static final String HUMAN_WOMAN_NAME = "human.woman.name";

	}

	public static String getByLocale(String iso639Code, String translationKey) {
		if ("ar".equals(iso639Code)) {
			return matchEntriesAr(translationKey);
		}
		if ("de".equals(iso639Code)) {
			return matchEntriesDe(translationKey);
		}
		if ("en".equals(iso639Code)) {
			return matchEntriesEn(translationKey);
		}
		if ("es".equals(iso639Code)) {
			return matchEntriesEs(translationKey);
		}
		if ("ja".equals(iso639Code)) {
			return matchEntriesJa(translationKey);
		}
		if ("zh".equals(iso639Code)) {
			return matchEntriesZh(translationKey);
		}
		return null;
	}

	static String matchEntriesAr(String translationKey) {
		switch (translationKey) {
			case Keys.ANIMAL_CAT_DESC:
				return "هذا نمر، اسمه %s، عمره %d سنة، ووزنه %f كيلوغرام";
			case Keys.ANIMAL_CAT_NAME:
				return "نمر";
			case Keys.ANIMAL_CAT_NUMBER_PLURAL:
				return "هناك %d نمور في المجموع";
			case Keys.ANIMAL_CAT_NUMBER_SINGLE:
				return "هناك %d نمر";
			case Keys.HUMAN_MAN_NAME:
				return "بوب";
			case Keys.HUMAN_WOMAN_NAME:
				return "أليس";
		}
		return null;
	}

	static String matchEntriesDe(String translationKey) {
		switch (translationKey) {
			case Keys.ANIMAL_CAT_DESC:
				return "Dies ist ein Tiger, sein Name ist %s, er ist %d Jahre alt und wiegt %f Kilogramm";
			case Keys.ANIMAL_CAT_NAME:
				return "Tiger";
			case Keys.ANIMAL_CAT_NUMBER_PLURAL:
				return "Es gibt insgesamt %d Tiger";
			case Keys.ANIMAL_CAT_NUMBER_SINGLE:
				return "Es gibt %d Tiger";
			case Keys.HUMAN_MAN_NAME:
				return "Bob";
			case Keys.HUMAN_WOMAN_NAME:
				return "Alice";
		}
		return null;
	}

	static String matchEntriesEn(String translationKey) {
		switch (translationKey) {
			case Keys.ANIMAL_CAT_DESC:
				return "This is a tiger, his name is %s, age is %d and weight is %f";
			case Keys.ANIMAL_CAT_NAME:
				return "Tiger";
			case Keys.ANIMAL_CAT_NUMBER_PLURAL:
				return "There are %d tigers";
			case Keys.ANIMAL_CAT_NUMBER_SINGLE:
				return "There is %d tiger";
			case Keys.HUMAN_MAN_NAME:
				return "Bob";
			case Keys.HUMAN_WOMAN_NAME:
				return "Alice";
		}
		return null;
	}

	static String matchEntriesEs(String translationKey) {
		switch (translationKey) {
			case Keys.ANIMAL_CAT_DESC:
				return "Este es un tigre, se llama %s, tiene %d años y pesa %f kilos";
			case Keys.ANIMAL_CAT_NAME:
				return "Tigre";
			case Keys.ANIMAL_CAT_NUMBER_PLURAL:
				return "Hay %d tigres";
			case Keys.ANIMAL_CAT_NUMBER_SINGLE:
				return "Hay %d tigre";
			case Keys.HUMAN_MAN_NAME:
				return "Roberto";
			case Keys.HUMAN_WOMAN_NAME:
				return "Alicia";
		}
		return null;
	}

	static String matchEntriesJa(String translationKey) {
		switch (translationKey) {
			case Keys.ANIMAL_CAT_DESC:
				return "これは虎です。名前は%s、年齢は%d歳、体重は%fキロです";
			case Keys.ANIMAL_CAT_NAME:
				return "タイガー";
			case Keys.ANIMAL_CAT_NUMBER_PLURAL:
				return "虎が%d頭います";
			case Keys.ANIMAL_CAT_NUMBER_SINGLE:
				return "虎が%d頭います";
			case Keys.HUMAN_MAN_NAME:
				return "ボブ";
			case Keys.HUMAN_WOMAN_NAME:
				return "アリス";
		}
		return null;
	}

	static String matchEntriesZh(String translationKey) {
		switch (translationKey) {
			case Keys.ANIMAL_CAT_DESC:
				return "这是一只老虎，它的名字叫%s，年龄%d岁，体重%f公斤";
			case Keys.ANIMAL_CAT_NAME:
				return "老虎";
			case Keys.ANIMAL_CAT_NUMBER_PLURAL:
				return "共有%d只老虎";
			case Keys.ANIMAL_CAT_NUMBER_SINGLE:
				return "有%d只老虎";
			case Keys.HUMAN_MAN_NAME:
				return "鲍勃";
			case Keys.HUMAN_WOMAN_NAME:
				return "爱丽丝";
		}
		return null;
	}

}