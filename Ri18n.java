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

	public static String getByLocale(String iso639Code, String i18NKey, Object... args) {
		String i18NKeyTemplate = getByLocale(iso639Code, i18NKey);
		if (i18NKeyTemplate == null || i18NKeyTemplate.isEmpty()) {
			return null;
		}
		return i18NKeyTemplate.formatted(args);
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

	public static String getByLocale(String iso639Code, String i18NKey) {
		if (iso639Code == "ar") {
			switch (i18NKey) {
				case "animal.cat.desc":
					return "هذا نمر، اسمه %s، عمره %d سنة، ووزنه %f كيلوغرام";
				case "animal.cat.name":
					return "نمر";
				case "animal.cat.number.plural":
					return "هناك %d نمور في المجموع";
				case "animal.cat.number.single":
					return "هناك %d نمر";
				case "human.man.name":
					return "بوب";
				case "human.woman.name":
					return "أليس";
			}
		}
		if (iso639Code == "de") {
			switch (i18NKey) {
				case "animal.cat.desc":
					return "Dies ist ein Tiger, sein Name ist %s, er ist %d Jahre alt und wiegt %f Kilogramm";
				case "animal.cat.name":
					return "Tiger";
				case "animal.cat.number.plural":
					return "Es gibt insgesamt %d Tiger";
				case "animal.cat.number.single":
					return "Es gibt %d Tiger";
				case "human.man.name":
					return "Bob";
				case "human.woman.name":
					return "Alice";
			}
		}
		if (iso639Code == "en") {
			switch (i18NKey) {
				case "animal.cat.desc":
					return "This is a tiger, his name is %s, age is %d and weight is %f";
				case "animal.cat.name":
					return "Tiger";
				case "animal.cat.number.plural":
					return "There are %d tigers";
				case "animal.cat.number.single":
					return "There is %d tiger";
				case "human.man.name":
					return "Bob";
				case "human.woman.name":
					return "Alice";
			}
		}
		if (iso639Code == "es") {
			switch (i18NKey) {
				case "animal.cat.desc":
					return "Este es un tigre, se llama %s, tiene %d años y pesa %f kilos";
				case "animal.cat.name":
					return "Tigre";
				case "animal.cat.number.plural":
					return "Hay %d tigres";
				case "animal.cat.number.single":
					return "Hay %d tigre";
				case "human.man.name":
					return "Roberto";
				case "human.woman.name":
					return "Alicia";
			}
		}
		if (iso639Code == "ja") {
			switch (i18NKey) {
				case "animal.cat.desc":
					return "これは虎です。名前は%s、年齢は%d歳、体重は%fキロです";
				case "animal.cat.name":
					return "タイガー";
				case "animal.cat.number.plural":
					return "虎が%d頭います";
				case "animal.cat.number.single":
					return "虎が%d頭います";
				case "human.man.name":
					return "ボブ";
				case "human.woman.name":
					return "アリス";
			}
		}
		if (iso639Code == "zh") {
			switch (i18NKey) {
				case "animal.cat.desc":
					return "这是一只老虎，它的名字叫%s，年龄%d岁，体重%f公斤";
				case "animal.cat.name":
					return "老虎";
				case "animal.cat.number.plural":
					return "共有%d只老虎";
				case "animal.cat.number.single":
					return "有%d只老虎";
				case "human.man.name":
					return "鲍勃";
				case "human.woman.name":
					return "爱丽丝";
			}
		}
		return null;
	}

}