-- Пак (источник данных): книга, сторонний материал или хоумбрю.
-- priority: чем выше, тем приоритетнее при override.
DEFINE TABLE OVERWRITE pack SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON pack TYPE string;
DEFINE FIELD OVERWRITE abbrev ON pack TYPE string;
DEFINE FIELD OVERWRITE kind ON pack TYPE string ASSERT $value IN ['official', 'thirdparty', 'homebrew'];
DEFINE FIELD OVERWRITE priority ON pack TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON pack TYPE option<string>;
DEFINE INDEX OVERWRITE pack_abbrev ON pack FIELDS abbrev UNIQUE;
