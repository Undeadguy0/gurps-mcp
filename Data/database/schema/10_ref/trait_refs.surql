-- Категория черты. axis: nature / origin / flag.
DEFINE TABLE OVERWRITE ref_trait_category SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_trait_category:mental); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_trait_category TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_trait_category TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_trait_category TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_trait_category TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_trait_category TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_trait_category TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_trait_category TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_trait_category_code ON ref_trait_category FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE axis ON ref_trait_category TYPE string ASSERT $value IN ['nature', 'origin', 'flag'];
DEFINE FIELD OVERWRITE _refs ON ref_trait_category TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));

-- Самоконтроль (CR): бросок и множитель стоимости.
DEFINE TABLE OVERWRITE ref_self_control SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_self_control:cr12); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_self_control TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_self_control TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_self_control TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_self_control TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_self_control TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_self_control TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_self_control TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_self_control_code ON ref_self_control FIELDS code UNIQUE;
-- kind: roll — бросок самоконтроля roll (6/9/12/15); always — «N/A», броска нет, недостаток действует всегда.
DEFINE FIELD OVERWRITE kind ON ref_self_control TYPE string DEFAULT 'roll' ASSERT $value IN ['roll', 'always'];
-- Бросок 3d6; обязателен при kind = 'roll', NONE при 'always'.
DEFINE FIELD OVERWRITE roll ON ref_self_control TYPE option<int> ASSERT $value = NONE OR ($value >= 3 AND $value <= 18);
DEFINE EVENT OVERWRITE ref_self_control_roll ON ref_self_control WHEN $event != 'DELETE' AND ($after.kind = 'roll') != ($after.roll != NONE)
	THEN { THROW 'ref_self_control: roll обязателен при kind = roll и запрещён при kind = always'; };
DEFINE FIELD OVERWRITE cost_mult ON ref_self_control TYPE float;
DEFINE FIELD OVERWRITE _refs ON ref_self_control TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));
