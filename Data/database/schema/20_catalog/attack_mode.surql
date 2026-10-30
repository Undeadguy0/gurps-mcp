-- Шаблон режима атаки. natural = true — безоружные (удар рукой, ногой, укус).
-- Модификации (когти, бонусы навыков) — features черт/навыков/предметов; расчёт в коде.
-- id = attack_mode:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE attack_mode SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON attack_mode TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON attack_mode TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON attack_mode TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON attack_mode TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON attack_mode TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON attack_mode TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON attack_mode TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON attack_mode TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON attack_mode TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE attack_mode_sid ON attack_mode FIELDS sid;

DEFINE FIELD OVERWRITE natural ON attack_mode TYPE bool DEFAULT false;
-- Часть тела / конечность (для правил вида «когти меняют тип урона руки»)
DEFINE FIELD OVERWRITE limb ON attack_mode TYPE option<string> ASSERT $value = NONE OR $value IN ['arm', 'leg', 'head', 'other'];
DEFINE FIELD OVERWRITE skill ON attack_mode TYPE option<record<skill>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE damage ON attack_mode TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE damage.raw ON attack_mode TYPE option<string>;
DEFINE FIELD OVERWRITE damage.base ON attack_mode TYPE option<string> ASSERT $value = NONE OR $value IN ['thr', 'sw'];
DEFINE FIELD OVERWRITE damage.dice ON attack_mode TYPE option<int>;
DEFINE FIELD OVERWRITE damage.mod ON attack_mode TYPE option<int>;
DEFINE FIELD OVERWRITE damage.type ON attack_mode TYPE option<record<ref_damage_type>> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE damage.modifiers ON attack_mode TYPE option<array<record<ref_damage_modifier>>> ASSERT fn::refs_ok($value);
DEFINE FIELD OVERWRITE damage.armor_divisor ON attack_mode TYPE option<number>;
DEFINE FIELD OVERWRITE reach_raw ON attack_mode TYPE option<string>;
DEFINE FIELD OVERWRITE reach ON attack_mode TYPE option<array<int>>;
DEFINE FIELD OVERWRITE parry_raw ON attack_mode TYPE option<string>;
DEFINE FIELD OVERWRITE parry ON attack_mode TYPE option<object> FLEXIBLE;
DEFINE FIELD OVERWRITE notes ON attack_mode TYPE option<array<record<ref_weapon_note>>> ASSERT fn::refs_ok($value);
-- Общие поля атаки (см. 00_common/attack.surql)
DEFINE FIELD OVERWRITE defaults ON attack_mode TYPE array<object> FLEXIBLE DEFAULT [] ASSERT $value.all(|$d| fn::attack_default_ok($d));
DEFINE FIELD OVERWRITE skill_modifier ON attack_mode TYPE option<int>;
DEFINE FIELD OVERWRITE skill_damage_bonus ON attack_mode TYPE option<object>;
DEFINE FIELD OVERWRITE skill_damage_bonus.min_relative ON attack_mode TYPE int;
DEFINE FIELD OVERWRITE skill_damage_bonus.per_die ON attack_mode TYPE int;
DEFINE FIELD OVERWRITE technique ON attack_mode TYPE option<record<technique>> ASSERT fn::ref_ok($value);

DEFINE FIELD OVERWRITE _refs ON attack_mode TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([source_refs.map(|$s| $s.book), [pack, skill, damage.type], damage.modifiers ?? [], notes ?? [], fn::attack_refs($this)])).filter(|$v| $v != NONE);
