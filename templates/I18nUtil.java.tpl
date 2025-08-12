
public class I18nUtil {

	public static enum Iso639Code {
		ZH("zh"),
		EN("en")
		;

		private String code;

		Iso639Code(String code) {
			this.code = code;
		}
	}

	static public class SupportLanguage {
		public Iso639Code code;
		public String nativeName;
		public String enName;

		SupportLanguage(Iso639Code code, String nativeName, String enName) {
			this.nativeName = nativeName;
			this.enName = enName;
			this.code = code;
		}
	}

	static String buildI18nKey(Iso639Code iso639, String key) {
		if (iso639 == null || key == null || key.isEmpty()) {
			return null;
		}
		return String.format("%s.%s", iso639.code, key);
	}

	public static String getByLocale(Iso639Code locale, String key) {
		String i18nKey = buildI18nKey(locale, key);
		return languageMapper.get(i18nKey);
	}

	public static String getByLocale(Iso639Code locale, String key, Object... args) {
		String i18nKey = buildI18nKey(locale, key);
		String template = languageMapper.get(i18nKey);
		if (template == null || template.isEmpty()) {
			return null;
		}
		return template.formatted(args);
	}

	public static java.util.ArrayList<SupportLanguage> supportLanguages = new java.util.ArrayList<>();
	static {
		supportLanguages.add(new SupportLanguage(Iso639Code.EN, "English", "English"));
		supportLanguages.add(new SupportLanguage(Iso639Code.ZH, "中文", "Chinese"));
	}

	public static final String TEST_CONTENT = "test.content";
	public static final String TEST_TITLE = "test.title";
	public static final String PLAYER_HEALTH = "player.health";

	static final java.util.HashMap<String, String> languageMapper = new java.util.HashMap<>();
	static {
		languageMapper.put(buildI18nKey(Iso639Code.EN, TEST_TITLE), "hello");
		languageMapper.put(buildI18nKey(Iso639Code.ZH, TEST_TITLE), "你好");
		languageMapper.put(buildI18nKey(Iso639Code.EN, TEST_CONTENT), "this is content");
		languageMapper.put(buildI18nKey(Iso639Code.ZH, TEST_CONTENT), "这是内容");
		languageMapper.put(buildI18nKey(Iso639Code.EN, PLAYER_HEALTH), "Health");
		languageMapper.put(buildI18nKey(Iso639Code.ZH, PLAYER_HEALTH), "血条");
	}
}