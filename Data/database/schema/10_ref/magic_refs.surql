-- Школа магии.
DEFINE TABLE OVERWRITE ref_college SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_college:college_a); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_college TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_college TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_college TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_college TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_college TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_college TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_college TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_college_code ON ref_college FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_college TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];

-- Класс заклинания. role: main (Regular, Area, ...) или modifier (Resisted, ...).
DEFINE TABLE OVERWRITE ref_spell_class SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_spell_class:regular); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_spell_class TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_spell_class TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_spell_class TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_spell_class TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_spell_class TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_spell_class TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_spell_class TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_spell_class_code ON ref_spell_class FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE role ON ref_spell_class TYPE string ASSERT $value IN ['main', 'modifier'];
DEFINE FIELD OVERWRITE _refs ON ref_spell_class TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];

-- Система магии (указывается в pack.spell_system).
DEFINE TABLE OVERWRITE ref_spell_system SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_spell_system:standard); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_spell_system TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_spell_system TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_spell_system TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_spell_system TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_spell_system TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_spell_system TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_spell_system TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_spell_system_code ON ref_spell_system FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_spell_system TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];

-- Уровень маны. rank: 0 = нет, 1 = низкий, 2 = обычный, ...
DEFINE TABLE OVERWRITE ref_mana_level SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_mana_level:normal); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_mana_level TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_mana_level TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_mana_level TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_mana_level TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_mana_level TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_mana_level TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_mana_level TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_mana_level_code ON ref_mana_level FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE rank ON ref_mana_level TYPE int ASSERT $value >= 0 AND $value <= 5;
DEFINE FIELD OVERWRITE _refs ON ref_mana_level TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];
