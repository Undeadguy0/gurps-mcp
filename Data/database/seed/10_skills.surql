-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
UPSERT skill:['basic', 'sword_a'] SET attribute = ref_attribute:DX, difficulty = ref_difficulty:A,
	categories = [ref_skill_category:melee], source_refs = [{ book: ref_book:B, page: NONE }];
UPSERT skill:['basic', 'sword_b'] SET attribute = ref_attribute:DX, difficulty = ref_difficulty:A, categories = [ref_skill_category:melee];
UPSERT skill:['basic', 'unarmed_a'] SET attribute = ref_attribute:DX, difficulty = ref_difficulty:E,
	-- правило безоружного боя: +1 к урону на кубик для атак этим навыком
	features = [{ kind: 'damage_bonus', skill: skill:['basic', 'unarmed_a'], amount: 1, per_die: true }];
UPSERT skill:['basic', 'outdoor_a'] SET attribute = ref_attribute:Per, difficulty = ref_difficulty:A,
	spec_mode = 'required', specializations = ['spec_a', 'spec_b'];
UPSERT skill:['basic', 'lore_a'] SET attribute = ref_attribute:IQ, difficulty = ref_difficulty:H,
	prereqs = { type: 'attribute', attribute: ref_attribute:IQ, min: 10 };
-- Override: тот же sid в хоумбрю-паке (priority выше)
UPSERT skill:['homebrew', 'sword_a'] SET attribute = ref_attribute:DX, difficulty = ref_difficulty:E;
