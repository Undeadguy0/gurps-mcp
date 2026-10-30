-- Зона попадания (torso, skull, ...). Используется и для мест брони.
DEFINE TABLE OVERWRITE ref_hit_location SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_hit_location:torso); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_hit_location TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_hit_location TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_hit_location TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_hit_location TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_hit_location TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_hit_location TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_hit_location TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_hit_location_code ON ref_hit_location FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE to_hit_mod ON ref_hit_location TYPE int DEFAULT 0;
-- Диапазон 3d6 на таблице попаданий: [от, до]
DEFINE FIELD OVERWRITE roll_range ON ref_hit_location TYPE option<array<int, 2>>;
DEFINE FIELD OVERWRITE cripple_divisor ON ref_hit_location TYPE option<float>;
DEFINE FIELD OVERWRITE wound_mults ON ref_hit_location TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE wound_mults[*].damage_type ON ref_hit_location TYPE record<ref_damage_type> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE wound_mults[*].mult ON ref_hit_location TYPE float;
DEFINE FIELD OVERWRITE _refs ON ref_hit_location TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], wound_mults.map(|$w| $w.damage_type)]));
