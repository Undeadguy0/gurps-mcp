-- Атрибут (ST, DX, IQ, HT, Will, Per, HP, FP, Speed, Move).
DEFINE TABLE OVERWRITE ref_attribute SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_attribute:DX); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_attribute TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_attribute TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_attribute TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_attribute TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_attribute TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_attribute TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_attribute TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_attribute_code ON ref_attribute FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE kind ON ref_attribute TYPE string ASSERT $value IN ['basic', 'secondary'];
-- Цена уровня в очках (заглушка), шаг уровня (Speed: 0.25)
DEFINE FIELD OVERWRITE cost_per_level ON ref_attribute TYPE option<int>;
DEFINE FIELD OVERWRITE step ON ref_attribute TYPE number DEFAULT 1;
DEFINE FIELD OVERWRITE _refs ON ref_attribute TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];

-- Сложность (E, A, H, VH, W).
DEFINE TABLE OVERWRITE ref_difficulty SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_difficulty:A); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_difficulty TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_difficulty TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_difficulty TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_difficulty TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_difficulty TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_difficulty TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_difficulty TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_difficulty_code ON ref_difficulty FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE base_mod ON ref_difficulty TYPE option<int>;
DEFINE FIELD OVERWRITE for_techniques ON ref_difficulty TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE _refs ON ref_difficulty TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];

-- Категория навыка (melee, outdoor, ...).
DEFINE TABLE OVERWRITE ref_skill_category SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_skill_category:melee); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_skill_category TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_skill_category TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_skill_category TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_skill_category TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_skill_category TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_skill_category TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_skill_category TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_skill_category_code ON ref_skill_category FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_skill_category TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];
