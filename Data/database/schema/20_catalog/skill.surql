-- Навык. Одна запись на навык; специализации — списком, выбранная — на ребре has_skill.
-- id = skill:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE skill SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON skill TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON skill TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON skill TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON skill TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON skill TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON skill TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON skill TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON skill TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON skill TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE skill_sid ON skill FIELDS sid;

DEFINE FIELD OVERWRITE attribute ON skill TYPE record<ref_attribute> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE difficulty ON skill TYPE record<ref_difficulty> ASSERT record::exists($value);
-- Специализации: none (нет), required, optional, tl (по TL). Названия — translation field 'spec.<sid>'.
DEFINE FIELD OVERWRITE spec_mode ON skill TYPE string DEFAULT 'none' ASSERT $value IN ['none', 'required', 'optional', 'tl'];
DEFINE FIELD OVERWRITE specializations ON skill TYPE array<string> DEFAULT []
	ASSERT $value.all(|$s| string::matches($s, '^[a-z0-9_]+$')) AND array::len(array::distinct($value)) = array::len($value)
		AND ($this.spec_mode IN ['required', 'optional'] OR array::len($value) = 0);
DEFINE FIELD OVERWRITE tl_dependent ON skill TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE encumbrance_penalty ON skill TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE categories ON skill TYPE array<record<ref_skill_category>> DEFAULT [] ASSERT fn::refs_ok($value);
-- Пререквизиты: дерево (см. 00_common/prereq.surql)
DEFINE FIELD OVERWRITE prereqs ON skill TYPE option<object> FLEXIBLE ASSERT $value = NONE OR fn::prereq_ok($value);
-- Типизированные эффекты (см. 00_common/feature.surql)
DEFINE FIELD OVERWRITE features ON skill TYPE array<object> FLEXIBLE DEFAULT [] ASSERT $value.all(|$f| fn::feature_ok($f));

DEFINE FIELD OVERWRITE _refs ON skill TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([source_refs.map(|$s| $s.book), [pack, attribute, difficulty], categories, fn::prereq_refs(prereqs), fn::feature_refs(features)]));
