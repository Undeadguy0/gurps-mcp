-- Тестовый персонаж.
UPSERT character:test_hero SET name = 'Тестовый герой', player = 'Undeadguy', campaign = 'test', tl = 3,
	attributes = { ST: 11, DX: 12, IQ: 10, HT: 11 }, points = { total: 150, unspent: 0 };
DELETE has_skill WHERE in = character:test_hero; DELETE has_trait WHERE in = character:test_hero; DELETE owns_item WHERE in = character:test_hero;
RELATE character:test_hero->has_skill->skill:broadsword SET points = 4, level = 13;
RELATE character:test_hero->has_skill->skill:survival SET points = 2, specialization = 'Woodlands';
RELATE character:test_hero->has_trait->trait:combat_reflexes SET points = 15;
RELATE character:test_hero->has_trait->trait:magery SET level = 1, points = 15;
RELATE character:test_hero->owns_item->item:broadsword SET equipped = true, location = 'belt';
