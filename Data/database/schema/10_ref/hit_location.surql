-- Зона попадания (torso, skull, arm, ...). Используется и для мест брони.
-- Броски 3d6 и состав зон — в плане тела (body_plan.surql), здесь — свойства самой зоны.
DEFINE TABLE OVERWRITE ref_hit_location SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_hit_location:torso); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_hit_location TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_hit_location TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_hit_location TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_hit_location TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_hit_location TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_hit_location TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_hit_location TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_hit_location_code ON ref_hit_location FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE to_hit_mod ON ref_hit_location TYPE int DEFAULT 0;
-- Парная зона (рука, нога, кисть, ступня): сторона — по броску плана тела или 1d (note).
DEFINE FIELD OVERWRITE paired ON ref_hit_location TYPE bool DEFAULT false;
-- Дополнительная DR зоны (череп: +2).
DEFINE FIELD OVERWRITE extra_dr ON ref_hit_location TYPE int DEFAULT 0 ASSERT $value >= 0;
DEFINE FIELD OVERWRITE cripple_divisor ON ref_hit_location TYPE option<float>;
-- Нокдаун/оглушение: модификатор броска HT; NONE — особого правила у зоны нет.
-- knockdown_damage_types — к каким типам повреждений правило относится ([] = к любым).
DEFINE FIELD OVERWRITE knockdown_mod ON ref_hit_location TYPE option<int>;
DEFINE FIELD OVERWRITE knockdown_damage_types ON ref_hit_location TYPE array<record<ref_damage_type>> DEFAULT [] ASSERT fn::refs_ok($value);
-- Множители раны по типу повреждения; condition — особое условие (узкий луч и т. п.), без него — всегда.
DEFINE FIELD OVERWRITE wound_mults ON ref_hit_location TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE wound_mults[*].damage_type ON ref_hit_location TYPE record<ref_damage_type> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE wound_mults[*].mult ON ref_hit_location TYPE float;
DEFINE FIELD OVERWRITE wound_mults[*].condition ON ref_hit_location TYPE option<record<ref_wound_condition>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE wound_mults[*].note ON ref_hit_location TYPE option<string>;
DEFINE FIELD OVERWRITE _refs ON ref_hit_location TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([source_refs.map(|$s| $s.book), [pack], knockdown_damage_types,
		wound_mults.map(|$w| $w.damage_type), wound_mults.map(|$w| $w.condition).filter(|$v| $v != NONE)]));

-- Условие особого множителя раны (narrow_beam — узкий луч, ...). Названия — в translation.
DEFINE TABLE OVERWRITE ref_wound_condition SCHEMAFULL;
DEFINE FIELD OVERWRITE code ON ref_wound_condition TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_wound_condition TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_wound_condition TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_wound_condition TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_wound_condition TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_wound_condition TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_wound_condition TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_wound_condition_code ON ref_wound_condition FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_wound_condition TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));
