-- Техника (technique).
DEFINE TABLE OVERWRITE technique SCHEMAFULL;
-- Общие поля каталожной сущности
DEFINE FIELD OVERWRITE name ON technique TYPE string;
DEFINE FIELD OVERWRITE name_ru ON technique TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON technique TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON technique TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON technique TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON technique TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON technique TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON technique TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON technique TYPE option<string>;
DEFINE FIELD OVERWRITE override_of ON technique TYPE option<record<technique>> ASSERT fn::ref_ok($value);
DEFINE INDEX OVERWRITE technique_name ON technique FIELDS name;
DEFINE INDEX OVERWRITE technique_name_pack ON technique FIELDS name, pack UNIQUE;
DEFINE INDEX OVERWRITE technique_ft_name ON technique FIELDS name FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE technique_ft_name_ru ON technique FIELDS name_ru FULLTEXT ANALYZER gurps_ru BM25;

-- Поля техники
DEFINE FIELD OVERWRITE parent_skill ON technique TYPE record<skill> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE parent_spec ON technique TYPE option<string>;
DEFINE FIELD OVERWRITE difficulty ON technique TYPE record<ref_difficulty> REFERENCE ON DELETE REJECT
	ASSERT record::exists($value) AND $value.for_techniques = true;
DEFINE FIELD OVERWRITE default_mod ON technique TYPE int;
DEFINE FIELD OVERWRITE max_raw ON technique TYPE option<string>;
DEFINE FIELD OVERWRITE max_mod ON technique TYPE option<int>;
-- Пререквизиты: свободная форма; связи — через requires (relations.surql)
DEFINE FIELD OVERWRITE prereqs ON technique TYPE array<object> FLEXIBLE DEFAULT [];
