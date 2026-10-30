-- Дерево пререквизитов (для skill, technique, trait, spell). Узлы (поле type):
--   and / or            { of: [узлы] }  (не пусто)
--   spell               { spell: spell:[..] }
--   spells_from_college { college: ref_college:x, count: int }
--   spells_any          { count: int }
--   magery              { level: int }
--   attribute           { attribute: ref_attribute:IQ, min: int }
--   advantage           { trait: trait:[..], level?: int }
--   skill               { skill: skill:[..], spec?: string, min?: int }
--   technique           { technique: technique:[..], min?: int }
-- Лишние ключи запрещены. Вычисление выполнения — в коде.
DEFINE FUNCTION OVERWRITE fn::prereq_ok($n: any) -> bool {
	IF !type::is_object($n) { RETURN false; };
	LET $t = $n.type;
	LET $keys = object::keys($n);
	LET $allowed = IF $t IN ['and', 'or'] { ['type', 'of'] }
		ELSE IF $t = 'spell' { ['type', 'spell'] }
		ELSE IF $t = 'spells_from_college' { ['type', 'college', 'count'] }
		ELSE IF $t = 'spells_any' { ['type', 'count'] }
		ELSE IF $t = 'magery' { ['type', 'level'] }
		ELSE IF $t = 'attribute' { ['type', 'attribute', 'min'] }
		ELSE IF $t = 'advantage' { ['type', 'trait', 'level'] }
		ELSE IF $t = 'skill' { ['type', 'skill', 'spec', 'min'] }
		ELSE IF $t = 'technique' { ['type', 'technique', 'min'] }
		ELSE { [] };
	IF !$keys.all(|$k| $k IN $allowed) { RETURN false; };
	RETURN IF $t IN ['and', 'or'] { type::is_array($n.of) AND array::len($n.of) > 0 AND $n.of.all(|$c| fn::prereq_ok($c)) }
		ELSE IF $t = 'spell' { fn::rec_of($n.spell, ['spell']) }
		ELSE IF $t = 'spells_from_college' { fn::rec_of($n.college, ['ref_college']) AND type::is_int($n.count) AND $n.count > 0 }
		ELSE IF $t = 'spells_any' { type::is_int($n.count) AND $n.count > 0 }
		ELSE IF $t = 'magery' { type::is_int($n.level) AND $n.level >= 0 }
		ELSE IF $t = 'attribute' { fn::rec_of($n.attribute, ['ref_attribute']) AND type::is_int($n.min) }
		ELSE IF $t = 'advantage' { fn::rec_of($n.trait, ['trait']) AND fn::opt_int($n.level) }
		ELSE IF $t = 'skill' { fn::rec_of($n.skill, ['skill']) AND fn::opt_str($n.spec) AND fn::opt_int($n.min) }
		ELSE IF $t = 'technique' { fn::rec_of($n.technique, ['technique']) AND fn::opt_int($n.min) }
		ELSE { false };
};
-- Все ссылки дерева (для _refs / защиты от удаления).
DEFINE FUNCTION OVERWRITE fn::prereq_refs($n: any) -> array {
	IF $n = NONE { RETURN []; };
	RETURN IF $n.type IN ['and', 'or'] { array::flatten($n.of.map(|$c| fn::prereq_refs($c))) }
		ELSE { [$n.spell, $n.college, $n.attribute, $n.trait, $n.skill, $n.technique].filter(|$v| type::is_record($v)) };
};
