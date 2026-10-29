-- Модификатор черты: усиление (enh) / ограничение (lim).
DEFINE TABLE OVERWRITE modifier SCHEMAFULL;
-- Общие поля каталожной сущности
DEFINE FIELD OVERWRITE name ON modifier TYPE string;
DEFINE FIELD OVERWRITE name_ru ON modifier TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON modifier TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON modifier TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON modifier TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON modifier TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON modifier TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON modifier TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON modifier TYPE option<string>;
DEFINE FIELD OVERWRITE override_of ON modifier TYPE option<record<modifier>> ASSERT fn::ref_ok($value);
DEFINE INDEX OVERWRITE modifier_name ON modifier FIELDS name;
DEFINE INDEX OVERWRITE modifier_name_pack ON modifier FIELDS name, pack UNIQUE;
DEFINE INDEX OVERWRITE modifier_ft_name ON modifier FIELDS name FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE modifier_ft_name_ru ON modifier FIELDS name_ru FULLTEXT ANALYZER gurps_ru BM25;

-- Поля модификатора
DEFINE FIELD OVERWRITE type ON modifier TYPE string ASSERT $value IN ['enh', 'lim'];
DEFINE FIELD OVERWRITE value ON modifier TYPE object;
DEFINE FIELD OVERWRITE value.kind ON modifier TYPE string ASSERT $value IN ['%', 'pts', 'per_level'];
DEFINE FIELD OVERWRITE value.amount ON modifier TYPE number;
DEFINE FIELD OVERWRITE levels ON modifier TYPE option<int>;
DEFINE FIELD OVERWRITE applies_to ON modifier TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE incompatible ON modifier TYPE array<record<modifier>> DEFAULT [] ASSERT fn::refs_ok($value);
