-- Пак (источник данных): книга, сторонний материал, хоумбрю или личные записи персонажа (custom).
-- priority: чем выше, тем приоритетнее при override (одинаковый sid).
DEFINE TABLE OVERWRITE pack SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON pack TYPE string ASSERT string::matches(<string> record::id($value), '^[a-z0-9_]+$');
DEFINE FIELD OVERWRITE name ON pack TYPE string;
DEFINE FIELD OVERWRITE abbrev ON pack TYPE string;
DEFINE FIELD OVERWRITE kind ON pack TYPE string ASSERT $value IN ['official', 'thirdparty', 'homebrew', 'custom'];
DEFINE FIELD OVERWRITE priority ON pack TYPE int DEFAULT 0;
-- Для kind = 'custom': владелец-персонаж (обязателен), для прочих — NONE.
DEFINE FIELD OVERWRITE owner ON pack TYPE option<record<character>>
	ASSERT $this.kind = 'custom' AND record::exists($value);
-- ASSERT не срабатывает на NONE, поэтому обязательность — событием.
DEFINE EVENT OVERWRITE pack_owner_required ON pack WHEN $after.kind = 'custom' AND $after.owner = NONE
	THEN { THROW 'owner: обязателен для пака kind = custom'; };
-- Система магии пака (ref_spell_system); обязательна для паков с заклинаниями.
DEFINE FIELD OVERWRITE spell_system ON pack TYPE option<record<ref_spell_system>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE notes ON pack TYPE option<string>;
DEFINE INDEX OVERWRITE pack_abbrev ON pack FIELDS abbrev UNIQUE;
