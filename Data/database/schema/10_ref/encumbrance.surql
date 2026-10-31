-- Уровни нагрузки (encumbrance). Значения правил — только в сидах (заполняет Мастер), в схеме их нет.
-- Уровень действует, пока вес груза <= max_load_bl * Basic Lift; ищется наименьший такой level.
-- Итоги в коде: Move = floor(Basic Move * move_mult) (минимум — ref_rule:encumbrance_move_min),
-- Dodge = база Dodge (ref_rule:dodge_*) + dodge_mod. Названия уровней — в translation (field = name).
DEFINE TABLE OVERWRITE ref_encumbrance SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_encumbrance:light); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_encumbrance TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_encumbrance TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_encumbrance TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_encumbrance TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_encumbrance TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_encumbrance TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_encumbrance TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_encumbrance_code ON ref_encumbrance FIELDS code UNIQUE;
-- Порядковый номер уровня (0 = без нагрузки), уникален.
DEFINE FIELD OVERWRITE level ON ref_encumbrance TYPE int ASSERT $value >= 0;
DEFINE INDEX OVERWRITE ref_encumbrance_level ON ref_encumbrance FIELDS level UNIQUE;
-- Верхняя граница веса уровня, в кратных Basic Lift.
DEFINE FIELD OVERWRITE max_load_bl ON ref_encumbrance TYPE number ASSERT $value > 0;
-- Множитель Basic Move на этом уровне, (0; 1].
DEFINE FIELD OVERWRITE move_mult ON ref_encumbrance TYPE number ASSERT $value > 0 AND $value <= 1;
-- Модификатор Dodge (0 или отрицательный).
DEFINE FIELD OVERWRITE dodge_mod ON ref_encumbrance TYPE int ASSERT $value <= 0;
DEFINE FIELD OVERWRITE _refs ON ref_encumbrance TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));