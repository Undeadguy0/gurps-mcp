-- Переводы: (record, lang, field) -> text. Из этой таблицы позже генерируются .ftl.
-- field: name | description | notes | spec.<sid> (название специализации навыка).
-- Удаление записи удаляет её переводы (REFERENCE ON DELETE CASCADE).
DEFINE TABLE OVERWRITE translation SCHEMAFULL;
DEFINE FIELD OVERWRITE record ON translation TYPE record REFERENCE ON DELETE CASCADE ASSERT record::exists($value);
DEFINE FIELD OVERWRITE lang ON translation TYPE string ASSERT string::matches($value, '^[a-z]{2}$');
DEFINE FIELD OVERWRITE field ON translation TYPE string ASSERT string::matches($value, '^(name|description|notes|spec\\.[a-z0-9_]+)$');
DEFINE FIELD OVERWRITE text ON translation TYPE string ASSERT string::len(string::trim($value)) > 0;
DEFINE INDEX OVERWRITE translation_uniq ON translation FIELDS record, lang, field UNIQUE;
-- Полнотекст: по вычисляемому полю на язык, у каждого свой анализатор (стемминг ru/en).
-- Поиск: `text_ru @1@ $q` / `text_en @1@ $q`. Для других языков индекса нет.
DEFINE FIELD OVERWRITE text_en ON translation TYPE option<string> VALUE IF $this.lang = 'en' { $this.text } ELSE { NONE };
DEFINE FIELD OVERWRITE text_ru ON translation TYPE option<string> VALUE IF $this.lang = 'ru' { $this.text } ELSE { NONE };
DEFINE INDEX OVERWRITE translation_ft_en ON translation FIELDS text_en FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE translation_ft_ru ON translation FIELDS text_ru FULLTEXT ANALYZER gurps_ru BM25;
