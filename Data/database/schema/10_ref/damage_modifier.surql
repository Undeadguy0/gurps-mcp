-- Модификатор урона (ex, frag, inc, ...). Делитель брони — числом в damage.armor_divisor.
DEFINE TABLE OVERWRITE ref_damage_modifier SCHEMAFULL;
-- Общие поля справочника. id записи = code (например ref_damage_modifier:ex).
DEFINE FIELD OVERWRITE code ON ref_damage_modifier TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE name_ru ON ref_damage_modifier TYPE string;
DEFINE FIELD OVERWRITE name_en ON ref_damage_modifier TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON ref_damage_modifier TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_damage_modifier TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_damage_modifier TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_damage_modifier TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_damage_modifier TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_damage_modifier TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_damage_modifier_code ON ref_damage_modifier FIELDS code UNIQUE;
-- Есть ли у модификатора значение (например, кубы осколков у frag).
DEFINE FIELD OVERWRITE has_value ON ref_damage_modifier TYPE bool DEFAULT false;
