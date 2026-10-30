-- Заклинание. Пак обязан иметь spell_system.
-- id = spell:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE spell SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON spell TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON spell TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON spell TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON spell TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON spell TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON spell TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON spell TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON spell TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON spell TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE spell_sid ON spell FIELDS sid;

-- Система магии берётся из пака (обязательна)
DEFINE FIELD OVERWRITE spell_system ON spell TYPE record<ref_spell_system> VALUE $this.pack.spell_system ASSERT record::exists($value);
DEFINE FIELD OVERWRITE colleges ON spell TYPE array<record<ref_college>> ASSERT array::len($value) > 0 AND fn::refs_ok($value);
-- Хотя бы один основной класс (role = 'main'); модификаторы (Resisted и т.п.) — дополнительно.
DEFINE FIELD OVERWRITE classes ON spell TYPE array<record<ref_spell_class>>
	ASSERT fn::refs_ok($value) AND $value.any(|$c| $c.role = 'main');
DEFINE FIELD OVERWRITE difficulty ON spell TYPE record<ref_difficulty> ASSERT $value IN [ref_difficulty:H, ref_difficulty:VH];
-- Сопротивление: атрибут (обязателен, если есть класс-модификатор Resisted — проверяется кодом)
DEFINE FIELD OVERWRITE resist ON spell TYPE option<record<ref_attribute>> ASSERT fn::ref_ok($value);
-- Время сотворения: сырой текст + секунды
DEFINE FIELD OVERWRITE casting_time ON spell TYPE object;
DEFINE FIELD OVERWRITE casting_time.raw ON spell TYPE string;
DEFINE FIELD OVERWRITE casting_time.seconds ON spell TYPE option<number>;
-- Длительность
DEFINE FIELD OVERWRITE duration ON spell TYPE object;
DEFINE FIELD OVERWRITE duration.raw ON spell TYPE string;
DEFINE FIELD OVERWRITE duration.kind ON spell TYPE string ASSERT $value IN ['instant', 'timed', 'permanent', 'special'];
DEFINE FIELD OVERWRITE duration.seconds ON spell TYPE option<number>;
-- Стоимость и поддержание (сырое + простые типизированные поля)
DEFINE FIELD OVERWRITE cost ON spell TYPE object;
DEFINE FIELD OVERWRITE cost.raw ON spell TYPE string;
DEFINE FIELD OVERWRITE cost.base ON spell TYPE option<int>;
DEFINE FIELD OVERWRITE cost.per ON spell TYPE option<string> ASSERT $value = NONE OR $value IN ['casting', 'yard', 'level', 'subject', 'other'];
DEFINE FIELD OVERWRITE maintain ON spell TYPE option<object>;
DEFINE FIELD OVERWRITE maintain.raw ON spell TYPE string;
DEFINE FIELD OVERWRITE maintain.cost ON spell TYPE option<int>;
-- Пререквизиты: дерево (см. 00_common/prereq.surql)
DEFINE FIELD OVERWRITE prereqs ON spell TYPE option<object> FLEXIBLE ASSERT $value = NONE OR fn::prereq_ok($value);

DEFINE FIELD OVERWRITE _refs ON spell TYPE array<record> REFERENCE ON DELETE REJECT
	VALUE array::distinct(array::flatten([[pack, difficulty], colleges, classes, [resist].filter(|$v| $v != NONE), fn::prereq_refs(prereqs)]));
