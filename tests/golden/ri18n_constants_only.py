from __future__ import annotations

from dataclasses import dataclass

@dataclass(frozen=True)
class SupportLanguage:
	cn_name: str
	en_name: str
	iso639_code: str
	native_name: str

def list_support_language() -> list[SupportLanguage]:
	return [
		SupportLanguage(iso639_code="ar", cn_name="阿拉伯语", en_name="Arabic", native_name="العربية"),
		SupportLanguage(iso639_code="de", cn_name="德语", en_name="German", native_name="Deutsch"),
		SupportLanguage(iso639_code="en", cn_name="英语", en_name="English", native_name="English"),
		SupportLanguage(iso639_code="es", cn_name="西班牙语", en_name="Spanish", native_name="español"),
		SupportLanguage(iso639_code="ja", cn_name="日语", en_name="Japanese", native_name="日本語"),
		SupportLanguage(iso639_code="zh", cn_name="中文", en_name="Chinese", native_name="中文"),
	]

class Iso639Codes:
	AR = "ar"
	DE = "de"
	EN = "en"
	ES = "es"
	JA = "ja"
	ZH = "zh"

class Keys:
	ANIMAL_CAT_DESC = "animal.cat.desc"
	ANIMAL_CAT_NAME = "animal.cat.name"
	ANIMAL_CAT_NUMBER_PLURAL = "animal.cat.number.plural"
	ANIMAL_CAT_NUMBER_SINGLE = "animal.cat.number.single"
	HUMAN_MAN_NAME = "human.man.name"
	HUMAN_WOMAN_NAME = "human.woman.name"
