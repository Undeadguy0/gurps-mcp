-- Категория черты. axis: nature (mental/physical/social), origin (exotic/supernatural), flag (прочее).
DEFINE TABLE OVERWRITE ref_trait_category SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_trait_category:mental).
DEFINE FIELD OVERWRITE code ON ref_trait_category TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_trait_category TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_trait_category TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_trait_category TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_trait_category TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_trait_category TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_trait_category TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_trait_category TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_trait_category TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_trait_category_code ON ref_trait_category FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE axis ON ref_trait_category TYPE string ASSERT $value IN ['nature', 'origin', 'flag'];

-- Самоконтроль (CR): бросок и множитель стоимости.
DEFINE TABLE OVERWRITE ref_self_control SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_self_control:cr12).
DEFINE FIELD OVERWRITE code ON ref_self_control TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_self_control TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_self_control TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_self_control TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_self_control TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_self_control TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_self_control TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_self_control TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_self_control TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_self_control_code ON ref_self_control FIELDS code UNIQUE;
-- NONE — «всегда/без броска»
DEFINE FIELD OVERWRITE roll ON ref_self_control TYPE option<int>;
DEFINE FIELD OVERWRITE cost_mult ON ref_self_control TYPE float;
