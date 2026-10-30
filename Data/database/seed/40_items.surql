-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
UPSERT item:['basic', 'sword_item_a'] SET category = 'melee', tl_raw = '2', tl = ref_tl:TL2, cost = 1, weight = 1,
	attack_modes = [
		{ name: 'swing', skill: skill:['basic', 'sword_a'], damage: { raw: 'PLACEHOLDER', base: 'sw', mod: 0, type: ref_damage_type:cut },
		  reach_raw: '1', reach: [1], parry_raw: '0', parry: { value: 0 } },
		{ name: 'thrust', skill: skill:['basic', 'sword_a'], damage: { raw: 'PLACEHOLDER', base: 'thr', mod: 0, type: ref_damage_type:imp },
		  reach_raw: '1', reach: [1], notes: [ref_weapon_note:dagger] }
	];
UPSERT item:['basic', 'shield_a'] SET category = 'shield', tl = ref_tl:TL0, cost = 1, weight = 1,
	shield = { db: 1, parry: 0, dr: 1, hp: 1 },
	attack_modes = [{ name: 'bash', damage: { raw: 'PLACEHOLDER', base: 'thr', type: ref_damage_type:cr } }];
UPSERT item:['basic', 'armor_a'] SET category = 'armor', tl = ref_tl:TL2, cost = 1, weight = 1,
	armor = { dr_raw: 'PLACEHOLDER', locations: [{ loc: ref_hit_location:torso, dr: 1, dr_vs: [{ damage_type: ref_damage_type:cr, dr: 0 }] }] };
UPSERT item:['basic', 'bag_a'] SET category = 'gear', cost = 1, weight = 1, container = { capacity_weight: 10 };
UPSERT item:['basic', 'knuckles_a'] SET category = 'gear', cost = 1, weight = 1,
	features = [{ kind: 'damage_bonus', attack_mode: attack_mode:['basic', 'punch'], amount: 1 }];
