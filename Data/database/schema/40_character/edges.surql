-- Рёбра персонаж -> каталог. Хранятся только вложенные очки и выбор игрока; note — пояснение игрока.
-- Навык: выбранная специализация (из skill.specializations; обязательна при spec_mode = 'required').
DEFINE TABLE OVERWRITE has_skill TYPE RELATION IN character OUT skill SCHEMAFULL;
DEFINE FIELD OVERWRITE points ON has_skill TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE spec ON has_skill TYPE option<string>
	ASSERT $value IN $this.out.specializations;
DEFINE EVENT OVERWRITE has_skill_spec_required ON has_skill WHEN $after.spec = NONE AND $after.out.spec_mode = 'required'
	THEN { THROW 'spec: специализация обязательна для этого навыка'; };
DEFINE FIELD OVERWRITE tl ON has_skill TYPE option<record<ref_tl>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE note ON has_skill TYPE option<string>;

DEFINE TABLE OVERWRITE has_technique TYPE RELATION IN character OUT technique SCHEMAFULL;
DEFINE FIELD OVERWRITE points ON has_technique TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE note ON has_technique TYPE option<string>;

DEFINE TABLE OVERWRITE has_spell TYPE RELATION IN character OUT spell SCHEMAFULL;
DEFINE FIELD OVERWRITE points ON has_spell TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE note ON has_spell TYPE option<string>;

DEFINE TABLE OVERWRITE has_trait TYPE RELATION IN character OUT trait SCHEMAFULL;
DEFINE FIELD OVERWRITE level ON has_trait TYPE option<int>;
DEFINE FIELD OVERWRITE modifiers ON has_trait TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE modifiers[*].modifier ON has_trait TYPE record<modifier> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE modifiers[*].levels ON has_trait TYPE option<int>;
DEFINE FIELD OVERWRITE note ON has_trait TYPE option<string>;

-- Предмет. Вложенность: contained_in -> другое ребро owns_item того же персонажа, чей предмет — контейнер.
DEFINE TABLE OVERWRITE owns_item TYPE RELATION IN character OUT item SCHEMAFULL;
DEFINE FIELD OVERWRITE qty ON owns_item TYPE int DEFAULT 1 ASSERT $value >= 1;
DEFINE FIELD OVERWRITE equipped ON owns_item TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE location ON owns_item TYPE option<string>;
DEFINE FIELD OVERWRITE note ON owns_item TYPE option<string>;
DEFINE FIELD OVERWRITE contained_in ON owns_item TYPE option<record<owns_item>>
	ASSERT $value = NONE OR (record::exists($value) AND $value != $this.id AND $value.in = $this.in AND $value.out.container != NONE);
