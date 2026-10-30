-- План тела (B552-555): humanoid, quadruped, ... Ровно один план is_default = true
-- (используется, если у персонажа body_plan = NONE). Названия — в translation.
DEFINE TABLE OVERWRITE ref_body_plan SCHEMAFULL;
DEFINE FIELD OVERWRITE code ON ref_body_plan TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_body_plan TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_body_plan TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_body_plan TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_body_plan TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_body_plan TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_body_plan TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_body_plan_code ON ref_body_plan FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE is_default ON ref_body_plan TYPE bool DEFAULT false;
DEFINE EVENT OVERWRITE ref_body_plan_one_default ON ref_body_plan WHEN $after.is_default = true
	AND count(SELECT id FROM ref_body_plan WHERE is_default = true AND id != $after.id) > 0
	THEN { THROW 'ref_body_plan: план по умолчанию (is_default) уже есть'; };
DEFINE FIELD OVERWRITE _refs ON ref_body_plan TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));

-- Зона в плане тела: id = ['план', 'зона'], например ref_body_plan_location:['humanoid', 'arm'].
-- rolls — диапазоны 3d6 [от, до] (у руки: [[8, 8], [12, 12]]); [] — зона только прицельно (глаз, жизненно важные органы).
-- to_hit_mod — переопределение ref_hit_location.to_hit_mod для этого плана (NONE — как у зоны).
DEFINE TABLE OVERWRITE ref_body_plan_location SCHEMAFULL;
DEFINE FIELD OVERWRITE plan ON ref_body_plan_location TYPE record<ref_body_plan>
	ASSERT record::exists($value) AND record::id($value) = record::id($this.id)[0];
DEFINE FIELD OVERWRITE location ON ref_body_plan_location TYPE record<ref_hit_location>
	ASSERT record::exists($value) AND record::id($value) = record::id($this.id)[1];
DEFINE FIELD OVERWRITE pack ON ref_body_plan_location TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_body_plan_location TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_body_plan_location TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_body_plan_location TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_body_plan_location TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_body_plan_location TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE rolls ON ref_body_plan_location TYPE array<array<int, 2>> DEFAULT []
	ASSERT $value.all(|$r| $r[0] >= 3 AND $r[0] <= $r[1] AND $r[1] <= 18);
DEFINE FIELD OVERWRITE to_hit_mod ON ref_body_plan_location TYPE option<int>;
DEFINE INDEX OVERWRITE ref_body_plan_location_uniq ON ref_body_plan_location FIELDS plan, location UNIQUE;
-- Диапазоны бросков внутри одного плана не пересекаются.
DEFINE EVENT OVERWRITE ref_body_plan_location_overlap ON ref_body_plan_location WHEN $event != 'DELETE' THEN {
	LET $other = array::flatten((SELECT VALUE rolls FROM ref_body_plan_location WHERE plan = $after.plan AND id != $after.id));
	IF $after.rolls.any(|$a| $other.any(|$b| $a[0] <= $b[1] AND $b[0] <= $a[1])) {
		THROW 'ref_body_plan_location: диапазоны бросков пересекаются с другой зоной плана';
	};
};
DEFINE FIELD OVERWRITE _refs ON ref_body_plan_location TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[pack, plan, location], source_refs.map(|$s| $s.book)]));
