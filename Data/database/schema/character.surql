-- Персонажи и их связи с каталогом.
DEFINE TABLE OVERWRITE character SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON character TYPE string;
DEFINE FIELD OVERWRITE player ON character TYPE option<string>;
DEFINE FIELD OVERWRITE campaign ON character TYPE option<string>;
DEFINE FIELD OVERWRITE tl ON character TYPE int DEFAULT 3;
-- Основные атрибуты
DEFINE FIELD OVERWRITE attributes ON character TYPE object;
DEFINE FIELD OVERWRITE attributes.ST ON character TYPE int DEFAULT 10;
DEFINE FIELD OVERWRITE attributes.DX ON character TYPE int DEFAULT 10;
DEFINE FIELD OVERWRITE attributes.IQ ON character TYPE int DEFAULT 10;
DEFINE FIELD OVERWRITE attributes.HT ON character TYPE int DEFAULT 10;
-- Вторичные: смещения от базы (HP, Will, Per, FP, Basic Speed, Basic Move)
DEFINE FIELD OVERWRITE secondary ON character TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE secondary.HP ON character TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE secondary.Will ON character TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE secondary.Per ON character TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE secondary.FP ON character TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE secondary.speed ON character TYPE number DEFAULT 0;
DEFINE FIELD OVERWRITE secondary.move ON character TYPE int DEFAULT 0;
-- Очки
DEFINE FIELD OVERWRITE points ON character TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE points.total ON character TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE points.unspent ON character TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE points.disadv_limit ON character TYPE option<int>;
DEFINE FIELD OVERWRITE notes ON character TYPE option<string>;
DEFINE FIELD OVERWRITE created_at ON character TYPE datetime DEFAULT time::now() READONLY;

DEFINE TABLE OVERWRITE has_skill TYPE RELATION IN character OUT skill SCHEMAFULL;
DEFINE FIELD OVERWRITE points ON has_skill TYPE int;
DEFINE FIELD OVERWRITE level ON has_skill TYPE option<int>;
DEFINE FIELD OVERWRITE specialization ON has_skill TYPE option<string>;

DEFINE TABLE OVERWRITE has_technique TYPE RELATION IN character OUT technique SCHEMAFULL;
DEFINE FIELD OVERWRITE points ON has_technique TYPE int;
DEFINE FIELD OVERWRITE level ON has_technique TYPE option<int>;

DEFINE TABLE OVERWRITE has_trait TYPE RELATION IN character OUT trait SCHEMAFULL;
DEFINE FIELD OVERWRITE level ON has_trait TYPE option<int>;
DEFINE FIELD OVERWRITE points ON has_trait TYPE int;
DEFINE FIELD OVERWRITE modifiers ON has_trait TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE modifiers[*].modifier ON has_trait TYPE record<modifier>;
DEFINE FIELD OVERWRITE modifiers[*].levels ON has_trait TYPE option<int>;
DEFINE FIELD OVERWRITE note ON has_trait TYPE option<string>;

DEFINE TABLE OVERWRITE owns_item TYPE RELATION IN character OUT item SCHEMAFULL;
DEFINE FIELD OVERWRITE qty ON owns_item TYPE int DEFAULT 1;
DEFINE FIELD OVERWRITE equipped ON owns_item TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE location ON owns_item TYPE option<string>;
