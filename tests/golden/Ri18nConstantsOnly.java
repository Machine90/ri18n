package com.example.ri18n;

public class Ri18nConstantsOnly {

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

}