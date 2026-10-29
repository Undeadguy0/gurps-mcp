-- Черта (trait): преимущество, недостаток, причуда, перк.
DEFINE TABLE OVERWRITE trait SCHEMAFULL;
-- Общие поля каталожной сущности
DEFINE FIELD OVERWRITE name ON trait TYPE string;
DEFINE FIELD OVERWRITE name_ru ON trait TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON trait TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON trait TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON trait TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON trait TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON trait TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON trait TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON trait TYPE option<string>;
DEFINE FIELD OVERWRITE override_of ON trait TYPE option<record<trait>> ASSERT fn::ref_ok($value);
DEFINE INDEX OVERWRITE trait_name ON trait FIELDS name;
DEFINE INDEX OVERWRITE trait_name_pack ON trait FIELDS name, pack UNIQUE;
DEFINE INDEX OVERWRITE trait_ft_name ON trait FIELDS name FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE trait_ft_name_ru ON trait FIELDS name_ru FULLTEXT ANALYZER gurps_ru BM25;

-- Поля черты
DEFINE FIELD OVERWRITE kind ON trait TYPE string ASSERT $value IN ['adv', 'disadv', 'quirk', 'perk'];
DEFINE FIELD OVERWRITE cost ON trait TYPE object;
DEFINE FIELD OVERWRITE cost.type ON trait TYPE string ASSERT $value IN ['fixed', 'per_level', 'base_plus_level'];
DEFINE FIELD OVERWRITE cost.base ON trait TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE cost.per_level ON trait TYPE option<int>;
DEFINE FIELD OVERWRITE cost.max_level ON trait TYPE option<int>;
-- Самоконтроль (CR): ссылка на ref_self_control или NONE
DEFINE FIELD OVERWRITE self_control ON trait TYPE option<record<ref_self_control>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE categories ON trait TYPE array<record<ref_trait_category>> DEFAULT [] ASSERT fn::refs_ok($value);
-- Эффекты: {target: 'skill:Stealth' | 'attr:DX' | 'fright_check', op: 'add'|'mul'|'set', value}
DEFINE FIELD OVERWRITE features ON trait TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE features[*].target ON trait TYPE string;
DEFINE FIELD OVERWRITE features[*].op ON trait TYPE string ASSERT $value IN ['add', 'mul', 'set'];
DEFINE FIELD OVERWRITE features[*].value ON trait TYPE number;
DEFINE FIELD OVERWRITE modifiers_allowed ON trait TYPE array<record<modifier>> DEFAULT [] ASSERT fn::refs_ok($value);

-- Защита от удаления используемых справочных записей
DEFINE FIELD OVERWRITE _refs ON trait TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([categories, [self_control].filter(|$v| $v != NONE)]));
