-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
UPSERT modifier:['basic', 'limit_a'] SET type = 'lim', value = { kind: '%', amount: -10 };
UPSERT trait:['basic', 'magery'] SET kind = 'adv', cost = { type: 'base_plus_level', base: 0, per_level: 1 },
	categories = [ref_trait_category:mental], modifiers_allowed = [modifier:['basic', 'limit_a']];
UPSERT trait:['basic', 'reflexes_a'] SET kind = 'adv', cost = { type: 'fixed', base: 1 }, categories = [ref_trait_category:mental],
	features = [
		{ kind: 'attribute_bonus', attribute: ref_attribute:Per, amount: 1 },
		{ kind: 'reaction_bonus', amount: 1, situation: 'placeholder' }
	];
UPSERT trait:['basic', 'claws_a'] SET kind = 'adv', cost = { type: 'fixed', base: 1 }, categories = [ref_trait_category:physical],
	-- когти: заменяют тип урона удара рукой (op 'set')
	features = [{ kind: 'damage_bonus', op: 'set', attack_mode: attack_mode:['basic', 'punch'], damage_type: ref_damage_type:cut }];
UPSERT trait:['basic', 'talent_a'] SET kind = 'adv', cost = { type: 'per_level', base: 0, per_level: 1, max_level: 4 },
	features = [{ kind: 'skill_bonus', skill: skill:['basic', 'outdoor_a'], amount: 1, per_level: true }];
UPSERT trait:['basic', 'tough_a'] SET kind = 'adv', cost = { type: 'per_level', base: 0, per_level: 1 },
	features = [{ kind: 'dr_bonus', amount: 1, per_level: true, locations: [ref_hit_location:torso] }];
UPSERT trait:['basic', 'cheap_st_a'] SET kind = 'perk', cost = { type: 'fixed', base: 1 },
	features = [{ kind: 'cost_reduction', attribute: ref_attribute:ST, percent: 10 }],
	prereqs = { type: 'or', of: [{ type: 'advantage', trait: trait:['basic', 'tough_a'] }, { type: 'attribute', attribute: ref_attribute:ST, min: 10 }] };
UPSERT trait:['basic', 'temper_a'] SET kind = 'disadv', cost = { type: 'fixed', base: -1 },
	self_control = ref_self_control:cr12, categories = [ref_trait_category:mental];
