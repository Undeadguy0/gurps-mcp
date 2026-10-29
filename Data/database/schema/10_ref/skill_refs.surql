-- Атрибут (ST, DX, IQ, HT, Will, Per).
DEFINE TABLE OVERWRITE ref_attribute SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_attribute:DX).
DEFINE FIELD OVERWRITE code ON ref_attribute TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_attribute TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_attribute TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_attribute TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_attribute TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_attribute TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_attribute TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_attribute TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_attribute TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_attribute_code ON ref_attribute FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE kind ON ref_attribute TYPE string ASSERT $value IN ['basic', 'secondary'];

-- Сложность навыка (E, A, H, VH, W).
DEFINE TABLE OVERWRITE ref_difficulty SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_difficulty:A).
DEFINE FIELD OVERWRITE code ON ref_difficulty TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_difficulty TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_difficulty TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_difficulty TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_difficulty TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_difficulty TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_difficulty TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_difficulty TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_difficulty TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_difficulty_code ON ref_difficulty FIELDS code UNIQUE;
-- Смещение уровня при 1 очке относительно атрибута (E=0, A=-1, ...)
DEFINE FIELD OVERWRITE base_mod ON ref_difficulty TYPE option<int>;
DEFINE FIELD OVERWRITE for_techniques ON ref_difficulty TYPE bool DEFAULT false;

-- Категория навыка (melee, outdoor, ...).
DEFINE TABLE OVERWRITE ref_skill_category SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_skill_category:melee).
DEFINE FIELD OVERWRITE code ON ref_skill_category TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_skill_category TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_skill_category TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_skill_category TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_skill_category TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_skill_category TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_skill_category TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_skill_category TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_skill_category TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_skill_category_code ON ref_skill_category FIELDS code UNIQUE;
