# package: com.example.ri18n

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

def translate_with_args(iso639_code: str, translation_key: str, *args: object) -> str | None:
	translation_key_template = translate(iso639_code, translation_key)
	if not translation_key_template:
		return None
	return translation_key_template.format(*args)

def translate(iso639_code: str, translation_key: str) -> str | None:
	if iso639_code == "ar":
		return match_entries_ar(translation_key)
	if iso639_code == "de":
		return match_entries_de(translation_key)
	if iso639_code == "en":
		return match_entries_en(translation_key)
	if iso639_code == "es":
		return match_entries_es(translation_key)
	if iso639_code == "ja":
		return match_entries_ja(translation_key)
	if iso639_code == "zh":
		return match_entries_zh(translation_key)
	return None

def match_entries_ar(translation_key: str) -> str | None:
	if translation_key == Keys.ANIMAL_CAT_DESC:
		return "هذا نمر، اسمه {}، عمره {} سنة، ووزنه {} كيلوغرام"
	elif translation_key == Keys.ANIMAL_CAT_NAME:
		return "نمر"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_PLURAL:
		return "هناك {} نمور في المجموع"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_SINGLE:
		return "هناك {} نمر"
	elif translation_key == Keys.HUMAN_MAN_NAME:
		return "بوب"
	elif translation_key == Keys.HUMAN_WOMAN_NAME:
		return "أليس"
	return None

def match_entries_de(translation_key: str) -> str | None:
	if translation_key == Keys.ANIMAL_CAT_DESC:
		return "Dies ist ein Tiger, sein Name ist {}, er ist {} Jahre alt und wiegt {} Kilogramm"
	elif translation_key == Keys.ANIMAL_CAT_NAME:
		return "Tiger"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_PLURAL:
		return "Es gibt insgesamt {} Tiger"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_SINGLE:
		return "Es gibt {} Tiger"
	elif translation_key == Keys.HUMAN_MAN_NAME:
		return "Bob"
	elif translation_key == Keys.HUMAN_WOMAN_NAME:
		return "Alice"
	return None

def match_entries_en(translation_key: str) -> str | None:
	if translation_key == Keys.ANIMAL_CAT_DESC:
		return "This is a tiger, his name is {}, age is {} and weight is {}"
	elif translation_key == Keys.ANIMAL_CAT_NAME:
		return "Tiger"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_PLURAL:
		return "There are {} tigers"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_SINGLE:
		return "There is {} tiger"
	elif translation_key == Keys.HUMAN_MAN_NAME:
		return "Bob"
	elif translation_key == Keys.HUMAN_WOMAN_NAME:
		return "Alice"
	return None

def match_entries_es(translation_key: str) -> str | None:
	if translation_key == Keys.ANIMAL_CAT_DESC:
		return "Este es un tigre, se llama {}, tiene {} años y pesa {} kilos"
	elif translation_key == Keys.ANIMAL_CAT_NAME:
		return "Tigre"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_PLURAL:
		return "Hay {} tigres"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_SINGLE:
		return "Hay {} tigre"
	elif translation_key == Keys.HUMAN_MAN_NAME:
		return "Roberto"
	elif translation_key == Keys.HUMAN_WOMAN_NAME:
		return "Alicia"
	return None

def match_entries_ja(translation_key: str) -> str | None:
	if translation_key == Keys.ANIMAL_CAT_DESC:
		return "これは虎です。名前は{}、年齢は{}歳、体重は{}キロです"
	elif translation_key == Keys.ANIMAL_CAT_NAME:
		return "タイガー"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_PLURAL:
		return "虎が{}頭います"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_SINGLE:
		return "虎が{}頭います"
	elif translation_key == Keys.HUMAN_MAN_NAME:
		return "ボブ"
	elif translation_key == Keys.HUMAN_WOMAN_NAME:
		return "アリス"
	return None

def match_entries_zh(translation_key: str) -> str | None:
	if translation_key == Keys.ANIMAL_CAT_DESC:
		return "这是一只老虎，它的名字叫{}，年龄{}岁，体重{}公斤"
	elif translation_key == Keys.ANIMAL_CAT_NAME:
		return "老虎"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_PLURAL:
		return "共有{}只老虎"
	elif translation_key == Keys.ANIMAL_CAT_NUMBER_SINGLE:
		return "有{}只老虎"
	elif translation_key == Keys.HUMAN_MAN_NAME:
		return "鲍勃"
	elif translation_key == Keys.HUMAN_WOMAN_NAME:
		return "爱丽丝"
	return None
