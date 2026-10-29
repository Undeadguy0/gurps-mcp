-- ВНИМАНИЕ: данные набраны по памяти из GURPS Basic Set 4e и НЕ СВЕРЕНЫ с книгой.
-- Номера страниц не указаны (page = NONE) — проверить цифры и страницы по книгам.
UPSERT item:broadsword SET name = 'Broadsword', name_ru = 'Палаш', pack = pack:basic, category = 'melee',
	source_refs = [{ book: 'B', page: NONE }], tl_raw = '2', tl = ref_tl:TL2, cost = 500, weight = 3,
	attack_modes = [
		{ name: 'swing', skill: 'Broadsword',
		  damage: { raw: 'sw+1 cut', base: 'sw', dice: NONE, mod: 1, type: ref_damage_type:cut, armor_divisor: NONE, followup: NONE },
		  reach_raw: '1', reach: [1], parry_raw: '0', parry: { value: 0, unbalanced: false, fencing: false, none: false },
		  min_st_raw: '10', min_st: 10 },
		{ name: 'thrust', skill: 'Broadsword',
		  damage: { raw: 'thr+1 cr', base: 'thr', dice: NONE, mod: 1, type: ref_damage_type:cr },
		  reach_raw: '1', reach: [1], parry_raw: '0', parry: { value: 0, unbalanced: false, fencing: false, none: false },
		  min_st_raw: '10', min_st: 10 }
	];
UPSERT item:regular_bow SET name = 'Regular Bow', name_ru = 'Обычный лук', pack = pack:basic, category = 'ranged',
	source_refs = [{ book: 'B', page: NONE }], tl_raw = '0', tl = ref_tl:TL0, cost = 100, weight = 2,
	attack_modes = [
		{ name: 'shoot', skill: 'Bow',
		  damage: { raw: 'thr+1 imp', base: 'thr', mod: 1, type: ref_damage_type:imp },
		  acc: 2, range_raw: 'x15/x20', range: { half: NONE, max: NONE, st_mult: [15, 20] },
		  rof_raw: '1', shots_raw: '1(2)', bulk: -7, min_st_raw: '10†', min_st: 10, notes: [ref_weapon_note:dagger] }
	];
UPSERT item:mail_shirt SET name = 'Mail Shirt', name_ru = 'Кольчужная рубаха', pack = pack:basic, category = 'armor',
	source_refs = [{ book: 'B', page: NONE }], tl_raw = '2', tl = ref_tl:TL2, cost = 150, weight = 16,
	armor = { dr_raw: '4/2*', locations: [{ loc: ref_hit_location:torso, dr: 4, dr_vs: [{ damage_type: ref_damage_type:cr, dr: 2 }], side: NONE }],
	          flexible: true, layer: NONE, don_time: NONE };
