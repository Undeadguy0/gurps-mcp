-- Тип повреждений (cut, cr, imp, ...).
DEFINE TABLE OVERWRITE ref_damage_type SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_damage_type:cut).
DEFINE FIELD OVERWRITE code ON ref_damage_type TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_damage_type TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_damage_type TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_damage_type TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_damage_type TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_damage_type TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_damage_type TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_damage_type TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_damage_type TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_damage_type_code ON ref_damage_type FIELDS code UNIQUE;
-- Множитель раны; NONE — особый случай (например, спец. урон).
DEFINE FIELD OVERWRITE wound_mult ON ref_damage_type TYPE option<float>;
DEFINE FIELD OVERWRITE drains ON ref_damage_type TYPE option<string> ASSERT $value = NONE OR $value IN ['HP', 'FP'];
