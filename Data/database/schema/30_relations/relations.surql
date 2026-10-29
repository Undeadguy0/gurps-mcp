-- Связи (графовые рёбра) каталога.

-- Дефолт навыка: skill -> skill. mod: штраф (-5), any_spec: любая специальность, cap: потолок.
DEFINE TABLE OVERWRITE defaults_to TYPE RELATION IN skill | technique OUT skill SCHEMAFULL;
DEFINE FIELD OVERWRITE mod ON defaults_to TYPE int;
DEFINE FIELD OVERWRITE spec ON defaults_to TYPE option<string>;
DEFINE FIELD OVERWRITE any_spec ON defaults_to TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE cap ON defaults_to TYPE option<int>;

-- Дефолт навыка от атрибута (DX-5 и т.п.).
DEFINE TABLE OVERWRITE attr_default SCHEMAFULL;
DEFINE FIELD OVERWRITE skill ON attr_default TYPE record<skill> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE attribute ON attr_default TYPE record<ref_attribute> REFERENCE ON DELETE REJECT ASSERT record::exists($value);
DEFINE FIELD OVERWRITE mod ON attr_default TYPE int;

-- Пререквизит: что угодно -> skill|trait. min_level — минимальный уровень/порог.
DEFINE TABLE OVERWRITE requires TYPE RELATION IN skill | technique | trait OUT skill | trait | technique SCHEMAFULL;
DEFINE FIELD OVERWRITE min_level ON requires TYPE option<int>;
DEFINE FIELD OVERWRITE spec ON requires TYPE option<string>;
DEFINE FIELD OVERWRITE note ON requires TYPE option<string>;

-- Взаимоисключение черт.
DEFINE TABLE OVERWRITE excludes TYPE RELATION IN trait OUT trait SCHEMAFULL;

-- Таланты: trait -> skill, бонус на уровень.
DEFINE TABLE OVERWRITE affects_skill TYPE RELATION IN trait OUT skill SCHEMAFULL;
DEFINE FIELD OVERWRITE bonus_per_level ON affects_skill TYPE int DEFAULT 1;

-- Альтернативные черты (например, вариант с другими условиями).
DEFINE TABLE OVERWRITE alternative_of TYPE RELATION IN trait OUT trait SCHEMAFULL;

-- Override: хоумбрю/сторонняя запись -> оригинальная. Побеждает пак с большим priority.
DEFINE TABLE OVERWRITE overrides TYPE RELATION IN skill | technique | trait | modifier | item OUT skill | technique | trait | modifier | item SCHEMAFULL;
DEFINE FIELD OVERWRITE reason ON overrides TYPE option<string>;
