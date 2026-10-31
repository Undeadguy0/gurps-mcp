-- Числовые правила для расчётов в коде (gurps-mcp calc). Формулы — в коде, числа — здесь.

-- Константа правил: value (число) или text (строка), по code. Используемые коды:
--   basic_lift_divisor        — делитель ST*ST для Basic Lift
--   basic_lift_round_min      — с какого значения Basic Lift округляется до целого
--   parry_base                — прибавка к парированию (база + навык / делитель)
--   parry_skill_divisor       — делитель навыка для парирования
--   limitation_floor_percent  — нижний предел суммы модификаторов черты, %
--   skill_cost_extra_points   — очков за каждый уровень навыка сверх таблицы ref_skill_cost
--   spell_attribute (text)    — код ref_attribute, от которого считаются заклинания
--   magery_trait (text)       — sid черты, уровень которой проверяет prereq { type: 'magery' }
--   st_damage_lookup (text)   — ST между строками ref_st_damage: 'floor' — строка с наибольшим st <= ST
--   st_damage_extra_st        — ST выше последней строки: шаг ST ...
--   st_damage_extra_dice      — ... и сколько кубиков добавляется к thr и sw за каждый полный шаг (B15)
--   dodge_attribute (text)    — код ref_attribute, от которого считается Dodge
--   dodge_base                — прибавка к dodge_attribute (Dodge = floor(атрибут) + dodge_base)
--   encumbrance_move_min      — нижняя граница Move под нагрузкой (если Basic Move > 0)
--   dodge_min                 — нижняя граница Dodge под нагрузкой (B17)
--   move_attribute (text)     — код ref_attribute, который умножается на move_mult нагрузки
--   уровни нагрузки — таблица ref_encumbrance (schema/10_ref/encumbrance.surql)
DEFINE TABLE OVERWRITE ref_rule SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_rule:parry_base); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_rule TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_rule TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_rule TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_rule TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_rule TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_rule TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_rule TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_rule_code ON ref_rule FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE value ON ref_rule TYPE option<number>;
DEFINE FIELD OVERWRITE text ON ref_rule TYPE option<string>;
DEFINE EVENT OVERWRITE ref_rule_value_required ON ref_rule WHEN ($after.value = NONE) = ($after.text = NONE)
	THEN { THROW 'ref_rule: нужно ровно одно из value / text'; };
DEFINE FIELD OVERWRITE _refs ON ref_rule TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));

-- Прогрессия цены навыка: points очков -> уровень = атрибут + base_mod сложности + level_offset.
-- Сверх последней строки: +1 уровень за каждые ref_rule:skill_cost_extra_points очков.
DEFINE TABLE OVERWRITE ref_skill_cost SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_skill_cost:p1); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_skill_cost TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_skill_cost TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_skill_cost TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_skill_cost TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_skill_cost TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_skill_cost TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_skill_cost TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_skill_cost_code ON ref_skill_cost FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE points ON ref_skill_cost TYPE int ASSERT $value > 0;
DEFINE FIELD OVERWRITE level_offset ON ref_skill_cost TYPE int;
DEFINE INDEX OVERWRITE ref_skill_cost_points ON ref_skill_cost FIELDS points UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_skill_cost TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));

-- Таблица урона от ST: st -> thr/sw { dice, mod }.
DEFINE TABLE OVERWRITE ref_st_damage SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_st_damage:st10); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_st_damage TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_st_damage TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_st_damage TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_st_damage TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_st_damage TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_st_damage TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_st_damage TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_st_damage_code ON ref_st_damage FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE st ON ref_st_damage TYPE int ASSERT $value >= 0;
DEFINE INDEX OVERWRITE ref_st_damage_st ON ref_st_damage FIELDS st UNIQUE;
DEFINE FIELD OVERWRITE thr ON ref_st_damage TYPE object;
DEFINE FIELD OVERWRITE thr.dice ON ref_st_damage TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE thr.mod ON ref_st_damage TYPE int;
DEFINE FIELD OVERWRITE sw ON ref_st_damage TYPE object;
DEFINE FIELD OVERWRITE sw.dice ON ref_st_damage TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE sw.mod ON ref_st_damage TYPE int;
DEFINE FIELD OVERWRITE _refs ON ref_st_damage TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));
