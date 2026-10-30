-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
-- Безоружные шаблоны (natural = true). Урон — заглушка.
UPSERT attack_mode:['basic', 'punch'] SET natural = true, limb = 'arm', skill = skill:['basic', 'unarmed_a'],
	damage = { raw: 'PLACEHOLDER', base: 'thr', mod: 0, type: ref_damage_type:cr }, reach = [0],
	parry_raw = '0', parry = { value: 0 }, defaults = [{ attribute: ref_attribute:DX, mod: 0 }];
UPSERT attack_mode:['basic', 'kick'] SET natural = true, limb = 'leg', skill = skill:['basic', 'unarmed_a'],
	damage = { raw: 'PLACEHOLDER', base: 'thr', mod: 0, type: ref_damage_type:cr }, reach = [1],
	skill_modifier = -1, defaults = [{ attribute: ref_attribute:DX, mod: -1 }, { skill: skill:['basic', 'sword_a'], mod: -1 }],
	skill_damage_bonus = { min_relative: 1, per_die: 1 };
UPSERT attack_mode:['basic', 'bite'] SET natural = true, limb = 'head',
	damage = { raw: 'PLACEHOLDER', base: 'thr', mod: 0, type: ref_damage_type:cr }, reach = [0];
