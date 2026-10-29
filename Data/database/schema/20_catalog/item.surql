-- Снаряжение. Принцип: сырые строки книги (*_raw, с † ‡ * U F) храним рядом с разобранными полями.
DEFINE TABLE OVERWRITE item SCHEMAFULL;
-- Общие поля каталожной сущности
DEFINE FIELD OVERWRITE name ON item TYPE string;
DEFINE FIELD OVERWRITE name_ru ON item TYPE option<string>;
DEFINE FIELD OVERWRITE pack ON item TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON item TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON item TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON item TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON item TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON item TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE notes ON item TYPE option<string>;
DEFINE FIELD OVERWRITE override_of ON item TYPE option<record<item>> ASSERT fn::ref_ok($value);
DEFINE INDEX OVERWRITE item_name ON item FIELDS name;
DEFINE INDEX OVERWRITE item_name_pack ON item FIELDS name, pack UNIQUE;
DEFINE INDEX OVERWRITE item_ft_name ON item FIELDS name FULLTEXT ANALYZER gurps_en BM25;
DEFINE INDEX OVERWRITE item_ft_name_ru ON item FIELDS name_ru FULLTEXT ANALYZER gurps_ru BM25;

-- Базовые поля
DEFINE FIELD OVERWRITE category ON item TYPE string ASSERT $value IN ['melee', 'ranged', 'armor', 'gear'];
DEFINE FIELD OVERWRITE weight ON item TYPE option<number>;
DEFINE FIELD OVERWRITE weight_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE cost ON item TYPE option<number>;
-- TL: сырой текст книги (например '3^') + ссылка на базовый TL
DEFINE FIELD OVERWRITE tl_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE tl ON item TYPE option<record<ref_tl>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE lc ON item TYPE option<record<ref_lc>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE quantity ON item TYPE int DEFAULT 1;

-- Режимы атаки. Объект гибкий (сырые строки *_raw и разобранные поля),
-- но ключевые ссылки проверяются явно.
DEFINE FIELD OVERWRITE attack_modes ON item TYPE array<object> FLEXIBLE DEFAULT [];
DEFINE FIELD OVERWRITE attack_modes[*].name ON item TYPE string;
DEFINE FIELD OVERWRITE attack_modes[*].damage ON item TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE attack_modes[*].damage.raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE attack_modes[*].damage.type ON item TYPE option<record<ref_damage_type>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE attack_modes[*].damage.modifiers ON item TYPE option<array<record<ref_damage_modifier>>> ASSERT fn::refs_ok($value);
DEFINE FIELD OVERWRITE attack_modes[*].damage.armor_divisor ON item TYPE option<number>;
-- Разобранные сноски († ‡ * U F) рядом с *_raw
DEFINE FIELD OVERWRITE attack_modes[*].notes ON item TYPE option<array<record<ref_weapon_note>>> ASSERT fn::refs_ok($value);

-- Броня: {dr_raw, locations[{loc, dr, dr_vs[{damage_type, dr}], side}], flexible, layer, don_time}
DEFINE FIELD OVERWRITE armor ON item TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE armor.locations ON item TYPE option<array<object>> FLEXIBLE;
DEFINE FIELD OVERWRITE armor.locations[*].loc ON item TYPE record<ref_hit_location> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE armor.locations[*].dr ON item TYPE option<int>;
DEFINE FIELD OVERWRITE armor.locations[*].dr_vs ON item TYPE option<array<object>>;
DEFINE FIELD OVERWRITE armor.locations[*].dr_vs[*].damage_type ON item TYPE record<ref_damage_type> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE armor.locations[*].dr_vs[*].dr ON item TYPE int;
-- Прочее: {container_capacity, uses}
DEFINE FIELD OVERWRITE gear ON item TYPE option<object> FLEXIBLE;

-- Все справочные ссылки предмета — для защиты от удаления (REFERENCE только на корневом поле)
DEFINE FIELD OVERWRITE _refs ON item TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([
		[tl, lc],
		array::flatten(attack_modes.map(|$m| array::flatten([[$m.damage.type], $m.damage.modifiers ?? [], $m.notes ?? []]))),
		array::flatten((armor.locations ?? []).map(|$l| array::flatten([[$l.loc], ($l.dr_vs ?? []).map(|$d| $d.damage_type)])))
	])).filter(|$v| $v != NONE);
