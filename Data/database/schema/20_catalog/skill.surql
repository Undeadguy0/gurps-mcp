-- Навык (skill). Специальности — дочерние записи через parent.
DEFINE TABLE OVERWRITE skill SCHEMAFULL;
-- Общие поля каталожной сущности
DEFINE FIELD OVERWRITE name ON skill TYPE string;
DEFINE FIELD OVERWRITE name_ru ON skill TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON skill TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON skill TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON skill TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON skill TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON skill TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON skill TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON skill TYPE option<string>;
DEFINE FIELD OVERWRITE override_of ON skill TYPE option<record<skill>> ASSERT fn::ref_ok($value);
DEFINE INDEX OVERWRITE skill_name ON skill FIELDS name;
DEFINE INDEX OVERWRITE skill_name_pack ON skill FIELDS name, pack UNIQUE;
DEFINE INDEX OVERWRITE skill_ft_name ON skill FIELDS name FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE skill_ft_name_ru ON skill FIELDS name_ru FULLTEXT ANALYZER gurps_ru BM25;

-- Поля навыка
DEFINE FIELD OVERWRITE attribute ON skill TYPE record<ref_attribute> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE difficulty ON skill TYPE record<ref_difficulty> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE specialization ON skill TYPE option<object>;
DEFINE FIELD OVERWRITE specialization.mode ON skill TYPE string ASSERT $value IN ['required', 'optional', 'tl'];
DEFINE FIELD OVERWRITE specialization.options ON skill TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE tl_dependent ON skill TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE encumbrance_penalty ON skill TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE categories ON skill TYPE array<record<ref_skill_category>> DEFAULT [] ASSERT fn::refs_ok($value);
DEFINE FIELD OVERWRITE parent ON skill TYPE option<record<skill>> ASSERT fn::ref_ok($value);
-- Пороговые атрибуты-пререквизиты (например IQ 12+): [{attribute, min}]
DEFINE FIELD OVERWRITE attr_prereqs ON skill TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE attr_prereqs[*].attribute ON skill TYPE record<ref_attribute> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE attr_prereqs[*].min ON skill TYPE int;

-- Защита от удаления используемых справочных записей (REFERENCE работает только на корневом поле)
DEFINE FIELD OVERWRITE _refs ON skill TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[attribute, difficulty], categories, attr_prereqs.map(|$p| $p.attribute)]));
