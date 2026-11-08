#[derive(Debug, Clone)]
pub struct SupportLanguage {
	pub cn_name: String,
	pub en_name: String,
	pub iso_639_code: String,
	pub native_name: String,
}

pub fn list_support_language() -> Vec<SupportLanguage> {
	[
		("ar", "阿拉伯语", "Arabic", "العربية"),
		("de", "德语", "German", "Deutsch"),
		("en", "英语", "English", "English"),
		("es", "西班牙语", "Spanish", "español"),
		("ja", "日语", "Japanese", "日本語"),
		("zh", "中文", "Chinese", "中文"),
	]
	.iter()
	.map(
		|(iso_639_code, cn_name, en_name, native_name)| {
			SupportLanguage {
				iso_639_code: iso_639_code.to_string(),
				cn_name: cn_name.to_string(),
				en_name: en_name.to_string(),
				native_name: native_name.to_string(),
			}
		}
	)
	.collect::<Vec<_>>()
}

pub mod iso639_codes {
	pub const AR: &str = "ar";
	pub const DE: &str = "de";
	pub const EN: &str = "en";
	pub const ES: &str = "es";
	pub const JA: &str = "ja";
	pub const ZH: &str = "zh";
}

pub mod keys {
	pub const ANIMAL_CAT_DESC: &str = "animal.cat.desc";
	pub const ANIMAL_CAT_NAME: &str = "animal.cat.name";
	pub const ANIMAL_CAT_NUMBER_PLURAL: &str = "animal.cat.number.plural";
	pub const ANIMAL_CAT_NUMBER_SINGLE: &str = "animal.cat.number.single";
	pub const HUMAN_MAN_NAME: &str = "human.man.name";
	pub const HUMAN_WOMAN_NAME: &str = "human.woman.name";
}
