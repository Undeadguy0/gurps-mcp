-- ВНИМАНИЕ: данные набраны по памяти из GURPS Basic Set 4e и НЕ СВЕРЕНЫ с книгой.
-- Номера страниц не указаны (page = NONE) — проверить цифры и страницы по книгам.
UPSERT trait:combat_reflexes SET name = 'Combat Reflexes', name_ru = 'Боевые рефлексы', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], kind = 'adv', cost = { type: 'fixed', base: 15 },
	categories = [ref_trait_category:mental], features = [{ target: 'active_defense', op: 'add', value: 1 }, { target: 'fright_check', op: 'add', value: 2 }];
UPSERT trait:magery SET name = 'Magery', name_ru = 'Магические способности', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], kind = 'adv',
	cost = { type: 'base_plus_level', base: 5, per_level: 10 }, categories = [ref_trait_category:mental, ref_trait_category:supernatural];
UPSERT trait:bad_temper SET name = 'Bad Temper', name_ru = 'Вспыльчивость', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], kind = 'disadv', cost = { type: 'fixed', base: -10 },
	self_control = ref_self_control:cr12, categories = [ref_trait_category:mental];
UPSERT modifier:one_college SET name = 'One College Only', name_ru = 'Только одна школа', pack = pack:basic,
	type = 'lim', value = { kind: '%', amount: -40 }, applies_to = ['Magery'];
UPDATE trait:magery SET modifiers_allowed = [modifier:one_college];
