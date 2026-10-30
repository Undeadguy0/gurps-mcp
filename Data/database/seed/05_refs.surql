-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
-- Только минимум кодов, нужный остальным сидам. Названия — в 90_translations.
-- Атрибуты: числа (база, цена уровня, делители) — ЗАГЛУШКИ, не значения GURPS.
UPSERT ref_attribute:ST SET code = 'ST', kind = 'basic', base_value = 5, cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:DX SET code = 'DX', kind = 'basic', base_value = 5, cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:IQ SET code = 'IQ', kind = 'basic', base_value = 5, cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:HT SET code = 'HT', kind = 'basic', base_value = 5, cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:Will SET code = 'Will', kind = 'secondary', base_from = [ref_attribute:IQ], cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:Per SET code = 'Per', kind = 'secondary', base_from = [ref_attribute:IQ], cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:HP SET code = 'HP', kind = 'secondary', base_from = [ref_attribute:ST], cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:FP SET code = 'FP', kind = 'secondary', base_from = [ref_attribute:HT], cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:Speed SET code = 'Speed', kind = 'secondary', step = 0.25, base_from = [ref_attribute:DX, ref_attribute:HT],
	base_divisor = 2, cost_per_level = 1, pack = pack:basic;
UPSERT ref_attribute:Move SET code = 'Move', kind = 'secondary', base_from = [ref_attribute:Speed], cost_per_level = 1, pack = pack:basic;

UPSERT ref_difficulty:E SET code = 'E', base_mod = 0, pack = pack:basic;
UPSERT ref_difficulty:A SET code = 'A', base_mod = 0, for_techniques = true, technique_costs = [1], pack = pack:basic;
UPSERT ref_difficulty:H SET code = 'H', base_mod = 0, for_techniques = true, technique_costs = [1], pack = pack:basic;
UPSERT ref_difficulty:VH SET code = 'VH', base_mod = 0, pack = pack:basic;

-- Правила расчёта (gurps-mcp calc) — ЗАГЛУШКИ.
UPSERT ref_rule:basic_lift_divisor SET code = 'basic_lift_divisor', value = 1, pack = pack:basic;
UPSERT ref_rule:basic_lift_round_min SET code = 'basic_lift_round_min', value = 1, pack = pack:basic;
UPSERT ref_rule:parry_base SET code = 'parry_base', value = 1, pack = pack:basic;
UPSERT ref_rule:parry_skill_divisor SET code = 'parry_skill_divisor', value = 1, pack = pack:basic;
UPSERT ref_rule:limitation_floor_percent SET code = 'limitation_floor_percent', value = -50, pack = pack:basic;
UPSERT ref_rule:skill_cost_extra_points SET code = 'skill_cost_extra_points', value = 1, pack = pack:basic;
UPSERT ref_rule:spell_attribute SET code = 'spell_attribute', text = 'IQ', pack = pack:basic;
UPSERT ref_rule:magery_trait SET code = 'magery_trait', text = 'magery', pack = pack:basic;
UPSERT ref_skill_cost:p1 SET code = 'p1', points = 1, level_offset = 0, pack = pack:basic;
FOR $st IN 0..=12 {
	UPSERT type::record('ref_st_damage', 'st' + <string> $st) SET code = 'st' + <string> $st, st = $st,
		thr = { dice: 1, mod: 0 }, sw = { dice: 1, mod: 1 }, pack = pack:basic;
};

UPSERT ref_skill_category:melee SET code = 'melee', pack = pack:basic;
UPSERT ref_trait_category:mental SET code = 'mental', axis = 'nature', pack = pack:basic;
UPSERT ref_trait_category:physical SET code = 'physical', axis = 'nature', pack = pack:basic;
UPSERT ref_self_control:cr12 SET code = 'cr12', roll = 12, cost_mult = 1.0, pack = pack:basic;

UPSERT ref_damage_type:cut SET code = 'cut', pack = pack:basic;
UPSERT ref_damage_type:cr SET code = 'cr', pack = pack:basic;
UPSERT ref_damage_type:imp SET code = 'imp', pack = pack:basic;
UPSERT ref_hit_location:torso SET code = 'torso', pack = pack:basic;
UPSERT ref_weapon_note:dagger SET code = 'dagger', symbol = '†', column = 'st', pack = pack:basic;
UPSERT ref_tl:TL0 SET code = 'TL0', level = 0, pack = pack:basic;
UPSERT ref_tl:TL2 SET code = 'TL2', level = 2, pack = pack:basic;
UPSERT ref_tl:TL3 SET code = 'TL3', level = 3, pack = pack:basic;

-- Магия
UPSERT ref_college:college_a SET code = 'college_a', pack = pack:basic;
UPSERT ref_college:college_b SET code = 'college_b', pack = pack:basic;
UPSERT ref_spell_class:regular SET code = 'regular', role = 'main', pack = pack:basic;
UPSERT ref_spell_class:resisted SET code = 'resisted', role = 'modifier', pack = pack:basic;
UPSERT ref_spell_system:standard SET code = 'standard', pack = pack:basic;
UPSERT ref_mana_level:normal SET code = 'normal', rank = 2, pack = pack:basic;
UPDATE pack:basic SET spell_system = ref_spell_system:standard;
