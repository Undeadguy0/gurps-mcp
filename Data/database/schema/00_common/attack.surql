-- Общие части режима атаки (attack_mode и item.attack_modes[*]).
-- defaults: уровни, если у персонажа нет навыка атаки: [{ attribute | skill, mod }] (ровно одна цель).
-- skill_modifier: прибавка к уровню навыка (например, «Навык-2» для пинка).
-- skill_damage_bonus: { min_relative, per_die } — +per_die к урону на кубик, если навык ≥ атрибут+min_relative.
-- technique: техника, заменяющая уровень навыка, если она есть у персонажа.
DEFINE FUNCTION OVERWRITE fn::attack_default_ok($d: any) -> bool {
	IF !type::is_object($d) { RETURN false; };
	IF !object::keys($d).all(|$k| $k IN ['attribute', 'skill', 'mod']) { RETURN false; };
	RETURN type::is_int($d.mod) AND (($d.attribute != NONE) != ($d.skill != NONE))
		AND fn::opt_rec_of($d.attribute, ['ref_attribute']) AND fn::opt_rec_of($d.skill, ['skill']);
};
-- Ссылки общих полей атаки (для _refs).
DEFINE FUNCTION OVERWRITE fn::attack_refs($m: any) -> array {
	RETURN array::flatten([
		[$m.technique], ($m.defaults ?? []).map(|$d| $d.attribute ?? $d.skill)
	]).filter(|$v| type::is_record($v));
};
