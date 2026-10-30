-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
UPSERT spell:['basic', 'spell_a'] SET colleges = [ref_college:college_a], classes = [ref_spell_class:regular],
	difficulty = ref_difficulty:H, casting_time = { raw: 'PLACEHOLDER', seconds: 1 },
	duration = { raw: 'PLACEHOLDER', kind: 'timed', seconds: 60 }, cost = { raw: 'PLACEHOLDER', base: 1, per: 'casting' },
	maintain = { raw: 'PLACEHOLDER', cost: 1 }, prereqs = { type: 'magery', level: 1 };
UPSERT spell:['basic', 'spell_b'] SET colleges = [ref_college:college_a, ref_college:college_b],
	classes = [ref_spell_class:regular, ref_spell_class:resisted], resist = ref_attribute:HT,
	difficulty = ref_difficulty:VH, casting_time = { raw: 'PLACEHOLDER' }, duration = { raw: 'PLACEHOLDER', kind: 'instant' },
	cost = { raw: 'PLACEHOLDER' },
	prereqs = { type: 'and', of: [
		{ type: 'magery', level: 2 },
		{ type: 'spell', spell: spell:['basic', 'spell_a'] },
		{ type: 'or', of: [
			{ type: 'spells_from_college', college: ref_college:college_a, count: 2 },
			{ type: 'spells_any', count: 5 },
			{ type: 'attribute', attribute: ref_attribute:IQ, min: 12 }
		] }
	] };
