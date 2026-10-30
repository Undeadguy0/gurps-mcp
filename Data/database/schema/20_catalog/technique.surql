-- Техника. Приёмы MA — через derived_from (шаблон attack_mode или другая техника).
-- id = technique:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE technique SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON technique TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON technique TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON technique TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON technique TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON technique TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON technique TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON technique TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON technique TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON technique TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE technique_sid ON technique FIELDS sid;

DEFINE FIELD OVERWRITE parent_skill ON technique TYPE record<skill> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE parent_spec ON technique TYPE option<string>
	ASSERT $value = NONE OR $value IN $this.parent_skill.specializations;
DEFINE FIELD OVERWRITE difficulty ON technique TYPE record<ref_difficulty>
	ASSERT record::exists($value) AND $value.for_techniques = true;
DEFINE FIELD OVERWRITE default_mod ON technique TYPE int;
DEFINE FIELD OVERWRITE max_raw ON technique TYPE option<string>;
DEFINE FIELD OVERWRITE max_mod ON technique TYPE option<int>;
DEFINE FIELD OVERWRITE derived_from ON technique TYPE option<record<attack_mode | technique>> ASSERT fn::ref_ok($value);
-- Изменения относительно базовой атаки: {damage_mod, reach, to_hit_mod}
DEFINE FIELD OVERWRITE attack_delta ON technique TYPE option<object>;
DEFINE FIELD OVERWRITE attack_delta.damage_mod ON technique TYPE option<int>;
DEFINE FIELD OVERWRITE attack_delta.to_hit_mod ON technique TYPE option<int>;
DEFINE FIELD OVERWRITE attack_delta.reach ON technique TYPE option<array<int>>;
-- Пререквизиты: дерево (см. 00_common/prereq.surql)
DEFINE FIELD OVERWRITE prereqs ON technique TYPE option<object> FLEXIBLE ASSERT $value = NONE OR fn::prereq_ok($value);
-- Типизированные эффекты (см. 00_common/feature.surql)
DEFINE FIELD OVERWRITE features ON technique TYPE array<object> FLEXIBLE DEFAULT [] ASSERT $value.all(|$f| fn::feature_ok($f));

DEFINE FIELD OVERWRITE _refs ON technique TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([source_refs.map(|$s| $s.book), [pack, parent_skill, difficulty], [derived_from].filter(|$v| $v != NONE), fn::prereq_refs(prereqs), fn::feature_refs(features)]));
