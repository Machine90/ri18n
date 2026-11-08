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

#[macro_export]
macro_rules! translate_with_args {
    ($code:expr, $key:expr) => {
        $crate::ri18n::translate($code, $key)
    };
    ($code:expr, $key:expr, $($values:tt)*) => {{
        #[allow(unused_imports)] use formatx::formatx;
        if let Some(template) = $crate::ri18n::translate($code, $key) {
            Some(formatx!(template, $($values)*))
        } else {
            None
        }
    }};
}


pub fn translate(iso_639_code: &str, translation_key: &str) -> Option<String> {
	match iso_639_code {
		"ar" => match_entries_ar(translation_key),
		"de" => match_entries_de(translation_key),
		"en" => match_entries_en(translation_key),
		"es" => match_entries_es(translation_key),
		"ja" => match_entries_ja(translation_key),
		"zh" => match_entries_zh(translation_key),
		_ => None,
	}
}

fn match_entries_ar(translation_key: &str) -> Option<String> {
	let content = match translation_key {
		keys::ANIMAL_CAT_DESC => "هذا نمر، اسمه {}، عمره {} سنة، ووزنه {} كيلوغرام",
		keys::ANIMAL_CAT_NAME => "نمر",
		keys::ANIMAL_CAT_NUMBER_PLURAL => "هناك {} نمور في المجموع",
		keys::ANIMAL_CAT_NUMBER_SINGLE => "هناك {} نمر",
		keys::HUMAN_MAN_NAME => "بوب",
		keys::HUMAN_WOMAN_NAME => "أليس",
		_ => return None,
	}
	.to_string();
	Some(content)
}

fn match_entries_de(translation_key: &str) -> Option<String> {
	let content = match translation_key {
		keys::ANIMAL_CAT_DESC => "Dies ist ein Tiger, sein Name ist {}, er ist {} Jahre alt und wiegt {} Kilogramm",
		keys::ANIMAL_CAT_NAME => "Tiger",
		keys::ANIMAL_CAT_NUMBER_PLURAL => "Es gibt insgesamt {} Tiger",
		keys::ANIMAL_CAT_NUMBER_SINGLE => "Es gibt {} Tiger",
		keys::HUMAN_MAN_NAME => "Bob",
		keys::HUMAN_WOMAN_NAME => "Alice",
		_ => return None,
	}
	.to_string();
	Some(content)
}

fn match_entries_en(translation_key: &str) -> Option<String> {
	let content = match translation_key {
		keys::ANIMAL_CAT_DESC => "This is a tiger, his name is {}, age is {} and weight is {}",
		keys::ANIMAL_CAT_NAME => "Tiger",
		keys::ANIMAL_CAT_NUMBER_PLURAL => "There are {} tigers",
		keys::ANIMAL_CAT_NUMBER_SINGLE => "There is {} tiger",
		keys::HUMAN_MAN_NAME => "Bob",
		keys::HUMAN_WOMAN_NAME => "Alice",
		_ => return None,
	}
	.to_string();
	Some(content)
}

fn match_entries_es(translation_key: &str) -> Option<String> {
	let content = match translation_key {
		keys::ANIMAL_CAT_DESC => "Este es un tigre, se llama {}, tiene {} años y pesa {} kilos",
		keys::ANIMAL_CAT_NAME => "Tigre",
		keys::ANIMAL_CAT_NUMBER_PLURAL => "Hay {} tigres",
		keys::ANIMAL_CAT_NUMBER_SINGLE => "Hay {} tigre",
		keys::HUMAN_MAN_NAME => "Roberto",
		keys::HUMAN_WOMAN_NAME => "Alicia",
		_ => return None,
	}
	.to_string();
	Some(content)
}

fn match_entries_ja(translation_key: &str) -> Option<String> {
	let content = match translation_key {
		keys::ANIMAL_CAT_DESC => "これは虎です。名前は{}、年齢は{}歳、体重は{}キロです",
		keys::ANIMAL_CAT_NAME => "タイガー",
		keys::ANIMAL_CAT_NUMBER_PLURAL => "虎が{}頭います",
		keys::ANIMAL_CAT_NUMBER_SINGLE => "虎が{}頭います",
		keys::HUMAN_MAN_NAME => "ボブ",
		keys::HUMAN_WOMAN_NAME => "アリス",
		_ => return None,
	}
	.to_string();
	Some(content)
}

fn match_entries_zh(translation_key: &str) -> Option<String> {
	let content = match translation_key {
		keys::ANIMAL_CAT_DESC => "这是一只老虎，它的名字叫{}，年龄{}岁，体重{}公斤",
		keys::ANIMAL_CAT_NAME => "老虎",
		keys::ANIMAL_CAT_NUMBER_PLURAL => "共有{}只老虎",
		keys::ANIMAL_CAT_NUMBER_SINGLE => "有{}只老虎",
		keys::HUMAN_MAN_NAME => "鲍勃",
		keys::HUMAN_WOMAN_NAME => "爱丽丝",
		_ => return None,
	}
	.to_string();
	Some(content)
}
