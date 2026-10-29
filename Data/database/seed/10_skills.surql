-- ВНИМАНИЕ: данные набраны по памяти из GURPS Basic Set 4e и НЕ СВЕРЕНЫ с книгой.
-- Номера страниц не указаны (page = NONE) — проверить цифры и страницы по книгам.
UPSERT skill:broadsword SET name = 'Broadsword', name_ru = 'Палаш', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], attribute = ref_attribute:DX, difficulty = ref_difficulty:A, categories = [ref_skill_category:melee];
UPSERT skill:shortsword SET name = 'Shortsword', name_ru = 'Короткий меч', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], attribute = ref_attribute:DX, difficulty = ref_difficulty:A, categories = [ref_skill_category:melee];
UPSERT skill:two_handed_sword SET name = 'Two-Handed Sword', name_ru = 'Двуручный меч', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], attribute = ref_attribute:DX, difficulty = ref_difficulty:A, categories = [ref_skill_category:melee];
UPSERT skill:stealth SET name = 'Stealth', name_ru = 'Скрытность', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], attribute = ref_attribute:DX, difficulty = ref_difficulty:A, encumbrance_penalty = true;
UPSERT skill:naturalist SET name = 'Naturalist', name_ru = 'Натуралист', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], attribute = ref_attribute:IQ, difficulty = ref_difficulty:H;
UPSERT skill:survival SET name = 'Survival', name_ru = 'Выживание', pack = pack:basic,
	source_refs = [{ book: 'B', page: NONE }], attribute = ref_attribute:Per, difficulty = ref_difficulty:A,
	specialization = { mode: 'required', options: ['Arctic', 'Desert', 'Island/Beach', 'Jungle', 'Mountain', 'Plains', 'Swampland', 'Woodlands'] };
-- Специальности как дочерние записи
UPSERT skill:survival_woodlands SET name = 'Survival (Woodlands)', name_ru = 'Выживание (леса)', pack = pack:basic,
	attribute = ref_attribute:Per, difficulty = ref_difficulty:A, parent = skill:survival;
UPSERT skill:survival_mountain SET name = 'Survival (Mountain)', name_ru = 'Выживание (горы)', pack = pack:basic,
	attribute = ref_attribute:Per, difficulty = ref_difficulty:A, parent = skill:survival;

-- Хоумбрю-override палаша (для проверки приоритета)
UPSERT skill:hb_broadsword SET name = 'Broadsword', name_ru = 'Палаш (хоумбрю)', pack = pack:homebrew,
	attribute = ref_attribute:DX, difficulty = ref_difficulty:E, override_of = skill:broadsword, notes = 'тестовый override';
