-- Класс легальности (LC0–LC4).
DEFINE TABLE OVERWRITE ref_lc SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_lc:LC4).
DEFINE FIELD OVERWRITE code ON ref_lc TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_lc TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_lc TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_lc TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_lc TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_lc TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_lc TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_lc TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_lc TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_lc_code ON ref_lc FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE level ON ref_lc TYPE int ASSERT $value >= 0 AND $value <= 4;

-- Технологический уровень. Суффиксы (3^, 2+1) хранятся в *_raw сущности; здесь — базовый TL.
DEFINE TABLE OVERWRITE ref_tl SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_tl:TL3).
DEFINE FIELD OVERWRITE code ON ref_tl TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_tl TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_tl TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_tl TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_tl TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_tl TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_tl TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_tl TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_tl TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_tl_code ON ref_tl FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE level ON ref_tl TYPE int ASSERT $value >= 0 AND $value <= 12;
