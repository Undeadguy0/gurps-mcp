-- Снаряжение. Принцип: сырые строки книги (*_raw, с † ‡ * U F) храним рядом с разобранными полями.
DEFINE TABLE OVERWRITE item SCHEMAFULL;
-- Общие поля каталожной сущности
DEFINE FIELD OVERWRITE name ON item TYPE string;
DEFINE FIELD OVERWRITE name_ru ON item TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON item TYPE record<pack>;
DEFINE FIELD OVERWRITE source_refs ON item TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON item TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON item TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON item TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON item TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON item TYPE option<string>;
DEFINE FIELD OVERWRITE override_of ON item TYPE option<record<item>>;
DEFINE INDEX OVERWRITE item_name ON item FIELDS name;
DEFINE INDEX OVERWRITE item_name_pack ON item FIELDS name, pack UNIQUE;
DEFINE INDEX OVERWRITE item_ft_name ON item FIELDS name FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE item_ft_name_ru ON item FIELDS name_ru FULLTEXT ANALYZER gurps_ru BM25;

-- Базовые поля
DEFINE FIELD OVERWRITE category ON item TYPE string ASSERT $value IN ['melee', 'ranged', 'armor', 'gear'];
DEFINE FIELD OVERWRITE weight ON item TYPE option<number>;
DEFINE FIELD OVERWRITE weight_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE cost ON item TYPE option<number>;
DEFINE FIELD OVERWRITE tl ON item TYPE option<int>;
DEFINE FIELD OVERWRITE lc ON item TYPE option<int>;
DEFINE FIELD OVERWRITE quantity ON item TYPE int DEFAULT 1;
-- Режимы атаки (структура внутри объекта гибкая: см. README)
DEFINE FIELD OVERWRITE attack_modes ON item TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE attack_modes[*] ON item TYPE object FLEXIBLE;
-- Броня: {dr_raw, locations[{loc, dr, dr_vs, side}], flexible, layer, don_time}
DEFINE FIELD OVERWRITE armor ON item TYPE option<object> FLEXIBLE;
-- Прочее: {container_capacity, uses}
DEFINE FIELD OVERWRITE gear ON item TYPE option<object> FLEXIBLE;
