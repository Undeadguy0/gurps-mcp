-- Черта: преимущество, недостаток, причуда, перк.
-- id = trait:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE trait SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON trait TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON trait TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON trait TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON trait TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON trait TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON trait TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON trait TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON trait TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON trait TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE trait_sid ON trait FIELDS sid;

DEFINE FIELD OVERWRITE kind ON trait TYPE string ASSERT $value IN ['adv', 'disadv', 'quirk', 'perk'];
DEFINE FIELD OVERWRITE cost ON trait TYPE object;
DEFINE FIELD OVERWRITE cost.type ON trait TYPE string ASSERT $value IN ['fixed', 'per_level', 'base_plus_level'];
DEFINE FIELD OVERWRITE cost.base ON trait TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE cost.per_level ON trait TYPE option<int>;
DEFINE FIELD OVERWRITE cost.max_level ON trait TYPE option<int>;
DEFINE FIELD OVERWRITE self_control ON trait TYPE option<record<ref_self_control>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE categories ON trait TYPE array<record<ref_trait_category>> DEFAULT [] ASSERT fn::refs_ok($value);
DEFINE FIELD OVERWRITE modifiers_allowed ON trait TYPE array<record<modifier>> DEFAULT [] ASSERT fn::refs_ok($value);
-- Пререквизиты: дерево (см. 00_common/prereq.surql)
DEFINE FIELD OVERWRITE prereqs ON trait TYPE option<object> FLEXIBLE ASSERT $value = NONE OR fn::prereq_ok($value);
-- Типизированные эффекты (см. 00_common/feature.surql)
DEFINE FIELD OVERWRITE features ON trait TYPE array<object> FLEXIBLE DEFAULT [] ASSERT $value.all(|$f| fn::feature_ok($f));

DEFINE FIELD OVERWRITE _refs ON trait TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[pack], categories, [self_control].filter(|$v| $v != NONE), modifiers_allowed, fn::prereq_refs(prereqs), fn::feature_refs(features)]));
