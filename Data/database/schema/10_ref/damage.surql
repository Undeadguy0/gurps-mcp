-- Тип повреждений (cut, cr, imp, ...).
DEFINE TABLE OVERWRITE ref_damage_type SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_damage_type:cut); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_damage_type TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_damage_type TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_damage_type TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_damage_type TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_damage_type TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_damage_type TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_damage_type TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_damage_type_code ON ref_damage_type FIELDS code UNIQUE;
-- Множитель раны; NONE — особый случай.
DEFINE FIELD OVERWRITE wound_mult ON ref_damage_type TYPE option<float>;
DEFINE FIELD OVERWRITE drains ON ref_damage_type TYPE option<string> ASSERT $value = NONE OR $value IN ['HP', 'FP'];
DEFINE FIELD OVERWRITE _refs ON ref_damage_type TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));

-- Модификатор урона (ex, frag, inc, ...). Делитель брони — числом в damage.armor_divisor.
DEFINE TABLE OVERWRITE ref_damage_modifier SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_damage_modifier:ex); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_damage_modifier TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_damage_modifier TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_damage_modifier TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_damage_modifier TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_damage_modifier TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_damage_modifier TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_damage_modifier TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_damage_modifier_code ON ref_damage_modifier FIELDS code UNIQUE;
-- Что пишется после кода модификатора: none — ничего; dice — кубики (frag: осколки 2d);
-- number — число; multiplier — множитель (×N); divisor — делитель; text — свободный текст.
DEFINE FIELD OVERWRITE value_kind ON ref_damage_modifier TYPE string DEFAULT 'none'
	ASSERT $value IN ['none', 'dice', 'number', 'multiplier', 'divisor', 'text'];
DEFINE FIELD OVERWRITE _refs ON ref_damage_modifier TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));
