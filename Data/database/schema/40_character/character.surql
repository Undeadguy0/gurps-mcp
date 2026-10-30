-- Персонаж: только ссылки на каталог + личные записи (пак kind = 'custom', owner = персонаж).
-- Производные величины (уровни навыков, HP, Speed, сумма очков) считаются в коде.
DEFINE TABLE OVERWRITE character SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON character TYPE string;
DEFINE FIELD OVERWRITE player ON character TYPE option<string>;
DEFINE FIELD OVERWRITE campaign ON character TYPE option<string>;
DEFINE FIELD OVERWRITE tl ON character TYPE record<ref_tl> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE mana_level ON character TYPE option<record<ref_mana_level>> ASSERT fn::ref_ok($value);
-- Купленные уровни атрибутов (смещение от базы, может быть < 0): [{attribute, levels}].
-- levels — число шагов ref_attribute.step (Speed: levels = 1 -> +0.25); цена = levels * cost_per_level.
DEFINE FIELD OVERWRITE attributes ON character TYPE array<object> DEFAULT []
	ASSERT array::len(array::distinct($value.map(|$a| $a.attribute))) = array::len($value);
DEFINE FIELD OVERWRITE attributes[*].attribute ON character TYPE record<ref_attribute> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE attributes[*].levels ON character TYPE int;
DEFINE FIELD OVERWRITE disadv_limit ON character TYPE option<int>;
DEFINE FIELD OVERWRITE quirk_limit ON character TYPE option<int>;
DEFINE FIELD OVERWRITE notes ON character TYPE option<string>;
-- Текущее состояние (сессия): текущие HP и FP; NONE = полные (равны максимуму, считается в коде).
DEFINE FIELD OVERWRITE current_hp ON character TYPE option<int>;
DEFINE FIELD OVERWRITE current_fp ON character TYPE option<int>;
DEFINE FIELD OVERWRITE created_at ON character TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE _refs ON character TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[tl, mana_level], attributes.map(|$a| $a.attribute)])).filter(|$v| $v != NONE);
