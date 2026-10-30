-- Снаряжение. Сырые строки книги (*_raw, с † ‡ * U F) храним рядом с разобранными полями.
-- id = item:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE item SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON item TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON item TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON item TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON item TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON item TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON item TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON item TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON item TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON item TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE item_sid ON item FIELDS sid;

DEFINE FIELD OVERWRITE category ON item TYPE string ASSERT $value IN ['melee', 'ranged', 'armor', 'shield', 'gear'];
DEFINE FIELD OVERWRITE weight ON item TYPE option<number>;
DEFINE FIELD OVERWRITE weight_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE cost ON item TYPE option<number>;
DEFINE FIELD OVERWRITE tl_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE tl ON item TYPE option<record<ref_tl>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE lc ON item TYPE option<record<ref_lc>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE quantity ON item TYPE int DEFAULT 1;
-- Встроенные режимы атаки (ключевые ссылки проверяются)
DEFINE FIELD OVERWRITE attack_modes ON item TYPE array<object> FLEXIBLE DEFAULT [];
DEFINE FIELD OVERWRITE attack_modes[*].name ON item TYPE string;
DEFINE FIELD OVERWRITE attack_modes[*].skill ON item TYPE option<record<skill>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE attack_modes[*].damage ON item TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE attack_modes[*].damage.raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE attack_modes[*].damage.base ON item TYPE option<string> ASSERT $value = NONE OR $value IN ['thr', 'sw'];
DEFINE FIELD OVERWRITE attack_modes[*].damage.dice ON item TYPE option<int>;
DEFINE FIELD OVERWRITE attack_modes[*].damage.mod ON item TYPE option<int>;
DEFINE FIELD OVERWRITE attack_modes[*].damage.type ON item TYPE option<record<ref_damage_type>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE attack_modes[*].damage.modifiers ON item TYPE option<array<record<ref_damage_modifier>>> ASSERT fn::refs_ok($value);
DEFINE FIELD OVERWRITE attack_modes[*].damage.armor_divisor ON item TYPE option<number>;
DEFINE FIELD OVERWRITE attack_modes[*].reach_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE attack_modes[*].reach ON item TYPE option<array<int>>;
DEFINE FIELD OVERWRITE attack_modes[*].parry_raw ON item TYPE option<string>;
DEFINE FIELD OVERWRITE attack_modes[*].parry ON item TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE attack_modes[*].notes ON item TYPE option<array<record<ref_weapon_note>>> ASSERT fn::refs_ok($value);

-- Броня: {dr_raw, locations[{loc, dr, dr_vs[{damage_type, dr}]}], flexible, ...}
DEFINE FIELD OVERWRITE armor ON item TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE armor.locations ON item TYPE option<array<object>> FLEXIBLE;
DEFINE FIELD OVERWRITE armor.locations[*].loc ON item TYPE record<ref_hit_location> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE armor.locations[*].dr ON item TYPE option<int>;
DEFINE FIELD OVERWRITE armor.locations[*].dr_vs ON item TYPE option<array<object>>;
DEFINE FIELD OVERWRITE armor.locations[*].dr_vs[*].damage_type ON item TYPE record<ref_damage_type> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE armor.locations[*].dr_vs[*].dr ON item TYPE int;
-- Щит (обязателен при category = 'shield'): DB, модификатор парирования, DR/HP
DEFINE FIELD OVERWRITE shield ON item TYPE option<object> ASSERT $this.category = 'shield';
DEFINE EVENT OVERWRITE item_shield_required ON item WHEN $after.category = 'shield' AND $after.shield = NONE
	THEN { THROW 'shield: обязателен для category = shield'; };
DEFINE FIELD OVERWRITE shield.db ON item TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE shield.parry ON item TYPE option<int>;
DEFINE FIELD OVERWRITE shield.dr ON item TYPE option<int>;
DEFINE FIELD OVERWRITE shield.hp ON item TYPE option<int>;
-- Контейнер: вместимость (вложенность — owns_item.contained_in)
DEFINE FIELD OVERWRITE container ON item TYPE option<object>;
DEFINE FIELD OVERWRITE container.capacity_weight ON item TYPE option<number>;
DEFINE FIELD OVERWRITE container.capacity_raw ON item TYPE option<string>;
-- Типизированные эффекты (см. 00_common/feature.surql)
DEFINE FIELD OVERWRITE features ON item TYPE array<object> FLEXIBLE DEFAULT [] ASSERT $value.all(|$f| fn::feature_ok($f));

DEFINE FIELD OVERWRITE _refs ON item TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([
		[pack, tl, lc],
		array::flatten(attack_modes.map(|$m| array::flatten([[$m.skill, $m.damage.type], $m.damage.modifiers ?? [], $m.notes ?? []]))),
		array::flatten((armor.locations ?? []).map(|$l| array::flatten([[$l.loc], ($l.dr_vs ?? []).map(|$d| $d.damage_type)]))),
		fn::feature_refs(features)
	])).filter(|$v| $v != NONE);
