-- Типизированные эффекты (features) черт, навыков, предметов. Поле kind:
--   attribute_bonus { attribute, amount }
--   skill_bonus     { skill? | category?, spec?, amount }            — бонус к уровню навыка
--   dr_bonus        { amount, locations?: [ref_hit_location], damage_type? }
--   damage_bonus    { attack_mode? | skill?, amount?, per_die?, damage_type?, reach? }
--                   — op 'add' (прибавка, per_die = на кубик) или 'set' (замена типа/досягаемости, когти)
--   cost_reduction  { attribute, percent }
--   reaction_bonus  { amount, situation? }
-- Общие: op ('add' | 'set', по умолчанию 'add'), per_level (bool). Итоговый расчёт — в коде.
DEFINE FUNCTION OVERWRITE fn::feature_ok($f: any) -> bool {
	IF !type::is_object($f) { RETURN false; };
	LET $k = $f.kind;
	LET $common = ['kind', 'op', 'per_level'];
	LET $allowed = IF $k = 'attribute_bonus' { ['attribute', 'amount'] }
		ELSE IF $k = 'skill_bonus' { ['skill', 'category', 'spec', 'amount'] }
		ELSE IF $k = 'dr_bonus' { ['amount', 'locations', 'damage_type'] }
		ELSE IF $k = 'damage_bonus' { ['attack_mode', 'skill', 'amount', 'per_die', 'damage_type', 'reach'] }
		ELSE IF $k = 'cost_reduction' { ['attribute', 'percent'] }
		ELSE IF $k = 'reaction_bonus' { ['amount', 'situation'] }
		ELSE { NONE };
	IF $allowed = NONE { RETURN false; };
	IF !object::keys($f).all(|$x| $x IN array::concat($common, $allowed)) { RETURN false; };
	LET $op = $f.op ?? 'add';
	IF !($op IN ['add', 'set']) OR !fn::opt_bool($f.per_level) { RETURN false; };
	RETURN IF $k = 'attribute_bonus' { fn::rec_of($f.attribute, ['ref_attribute']) AND type::is_number($f.amount) }
		ELSE IF $k = 'skill_bonus' { ($f.skill != NONE OR $f.category != NONE)
			AND fn::opt_rec_of($f.skill, ['skill']) AND fn::opt_rec_of($f.category, ['ref_skill_category'])
			AND fn::opt_str($f.spec) AND type::is_number($f.amount) }
		ELSE IF $k = 'dr_bonus' { type::is_int($f.amount)
			AND ($f.locations = NONE OR (type::is_array($f.locations) AND $f.locations.all(|$l| fn::rec_of($l, ['ref_hit_location']))))
			AND fn::opt_rec_of($f.damage_type, ['ref_damage_type']) }
		ELSE IF $k = 'damage_bonus' { ($f.attack_mode != NONE OR $f.skill != NONE)
			AND fn::opt_rec_of($f.attack_mode, ['attack_mode']) AND fn::opt_rec_of($f.skill, ['skill'])
			AND fn::opt_num($f.amount) AND fn::opt_bool($f.per_die)
			AND fn::opt_rec_of($f.damage_type, ['ref_damage_type'])
			AND ($f.reach = NONE OR (type::is_array($f.reach) AND $f.reach.all(|$r| type::is_int($r))))
			AND IF $op = 'add' { type::is_number($f.amount) } ELSE { $f.damage_type != NONE OR $f.reach != NONE } }
		ELSE IF $k = 'cost_reduction' { fn::rec_of($f.attribute, ['ref_attribute']) AND type::is_int($f.percent) AND $f.percent > 0 AND $f.percent <= 100 }
		ELSE IF $k = 'reaction_bonus' { type::is_int($f.amount) AND fn::opt_str($f.situation) }
		ELSE { false };
};
DEFINE FUNCTION OVERWRITE fn::feature_refs($fs: any) -> array {
	RETURN array::flatten(($fs ?? []).map(|$f| array::flatten([
		[$f.attribute, $f.skill, $f.category, $f.damage_type, $f.attack_mode], $f.locations ?? []
	]))).filter(|$v| type::is_record($v));
};
